//! Trusted policy core. Only `syscall` is a user-facing operation, and it derives
//! identity from the scheduler. The platform layer must protect this object's
//! memory, deliver preemption, and enforce mappings before running user code.
#![no_std]
#![forbid(unsafe_code)]

pub const MAX_PROCESSES: usize = 8;
pub const MAX_CAPABILITIES: usize = 8;
const _: () = assert!(MAX_PROCESSES == 8 && MAX_CAPABILITIES == 8);
pub use bastion_policy as decisions;
pub mod net;
pub mod relay;

/// Compatibility helpers backed by the C compiled from Bastion.Runtime.
pub mod policy {
    pub fn reserve(used: u64, requested: u64, limit: u64) -> Option<u64> {
        if super::decisions::can_reserve(used, requested, limit) != 0 {
            Some(super::decisions::reserve_value(used, requested, limit))
        } else {
            None
        }
    }

    pub use super::decisions::charge;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidLimit,
    ProcessTableFull,
    InvalidProcess,
    InvalidHandle,
    PermissionDenied,
    PageLimit,
    GlobalPageLimit,
    CapabilityLimit,
    InsufficientPages,
    NoCurrentProcess,
    ClockWentBackwards,
    IdentifierExhausted,
}

/// IDs are names, not authority. A syscall must also supply a valid capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessId(u64);

impl ProcessId {
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Handles are looked up only in the current process's private capability table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle(u64);

impl Handle {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rights(u8);

impl Rights {
    pub const NONE: Self = Self(0);
    pub const INSPECT: Self = Self(1);
    pub const TERMINATE: Self = Self(2);
    pub const ALL: Self = Self(3);
    pub fn contains(self, other: Self) -> bool {
        decisions::rights_allow(self.0.into(), other.0.into()) != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Maximum owned page reservations, including initial code and stack pages.
    pub pages: u64,
    pub capabilities: usize,
    /// CPU time in platform clock units, per global fixed-length period.
    pub cpu_budget: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usage {
    pub pages: u64,
    pub capabilities: usize,
    pub cpu_remaining: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub id: ProcessId,
    pub limits: Limits,
    pub usage: Usage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    SelfInfo,
    Inspect(Handle),
    Terminate(Handle),
    Restrict { handle: Handle, rights: Rights },
    Close(Handle),
    Yield,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    Done,
    Info(Snapshot),
}

#[derive(Clone, Copy, Debug)]
struct Capability {
    handle: Handle,
    target: ProcessId,
    rights: Rights,
}

#[derive(Clone, Copy, Debug)]
struct Process {
    id: ProcessId,
    limits: Limits,
    pages: u64,
    remaining: u64,
    caps: [Option<Capability>; MAX_CAPABILITIES],
}

impl Process {
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            id: self.id,
            limits: self.limits,
            usage: Usage {
                pages: self.pages,
                capabilities: self.caps.iter().flatten().count(),
                cpu_remaining: self.remaining,
            },
        }
    }
}

/// Single-CPU kernel state. Mutation needs an exclusive borrow: the platform
/// must enter with interrupts masked, or add synchronization before SMP use.
pub struct Kernel {
    processes: [Option<Process>; MAX_PROCESSES],
    current: Option<usize>,
    cursor: usize,
    next_id: u64,
    page_capacity: u64,
    used_pages: u64,
    period: u64,
    quantum: u64,
    now: u64,
}

impl Kernel {
    pub fn new(page_capacity: u64, period: u64, quantum: u64) -> Result<Self, Error> {
        if decisions::abi_version(0) != 1 || decisions::valid_config(period, quantum) == 0 {
            return Err(Error::InvalidLimit);
        }
        Ok(Self {
            processes: [None; MAX_PROCESSES],
            current: None,
            cursor: 0,
            next_id: 1,
            page_capacity,
            used_pages: 0,
            period,
            quantum,
            now: 0,
        })
    }

    fn allocate_id(&mut self) -> Result<u64, Error> {
        let id = self.next_id;
        let next = decisions::next_identifier(id);
        if next == 0 {
            return Err(Error::IdentifierExhausted);
        }
        self.next_id = next;
        Ok(id)
    }

    fn index(&self, id: ProcessId) -> Result<usize, Error> {
        self.processes
            .iter()
            .position(|p| p.is_some_and(|p| decisions::same_identity(p.id.0, id.0) != 0))
            .ok_or(Error::InvalidProcess)
    }

    /// Trusted loader API. No process creation syscall is exposed, so spawning
    /// cannot multiply a user's CPU or memory budget.
    pub fn spawn(&mut self, limits: Limits, initial_pages: u64) -> Result<ProcessId, Error> {
        if decisions::valid_limits(
            limits.pages,
            initial_pages,
            limits.capabilities as u64,
            limits.cpu_budget,
            self.period,
        ) == 0
        {
            return Err(Error::InvalidLimit);
        }
        if decisions::can_reserve(self.used_pages, initial_pages, self.page_capacity) == 0 {
            return Err(Error::GlobalPageLimit);
        }
        let slot = self
            .processes
            .iter()
            .position(Option::is_none)
            .ok_or(Error::ProcessTableFull)?;
        let id = ProcessId(self.allocate_id()?);
        self.processes[slot] = Some(Process {
            id,
            limits,
            pages: initial_pages,
            remaining: limits.cpu_budget,
            caps: [None; MAX_CAPABILITIES],
        });
        self.used_pages =
            decisions::reserve_value(self.used_pages, initial_pages, self.page_capacity);
        Ok(id)
    }

    /// Trusted allocator API: reserve before mapping, roll back if mapping fails.
    /// This accounts for pages; it does not itself allocate or protect memory.
    pub fn reserve_pages(&mut self, id: ProcessId, pages: u64) -> Result<(), Error> {
        let i = self.index(id)?;
        let p = self.processes[i].as_mut().unwrap();
        match decisions::reservation_status(
            p.pages,
            self.used_pages,
            pages,
            p.limits.pages,
            self.page_capacity,
        ) {
            0 => {}
            1 => return Err(Error::PageLimit),
            _ => return Err(Error::GlobalPageLimit),
        }
        p.pages = decisions::reserve_value(p.pages, pages, p.limits.pages);
        self.used_pages = decisions::reserve_value(self.used_pages, pages, self.page_capacity);
        Ok(())
    }

    /// Trusted allocator API: unmap and invalidate translations before releasing.
    pub fn release_pages(&mut self, id: ProcessId, pages: u64) -> Result<(), Error> {
        let i = self.index(id)?;
        let p = self.processes[i].as_mut().unwrap();
        if decisions::can_release(p.pages, pages) == 0 {
            return Err(Error::InsufficientPages);
        }
        p.pages = decisions::release_value(p.pages, pages);
        self.used_pages = decisions::release_value(self.used_pages, pages);
        Ok(())
    }

    /// Trusted bootstrap/manager API. User code cannot mint capabilities.
    pub fn grant(
        &mut self,
        owner: ProcessId,
        target: ProcessId,
        rights: Rights,
    ) -> Result<Handle, Error> {
        let i = self.index(owner)?;
        self.index(target)?;
        let p = self.processes[i].as_ref().unwrap();
        if decisions::can_grant(
            p.caps.iter().flatten().count() as u64,
            p.limits.capabilities as u64,
        ) == 0
        {
            return Err(Error::CapabilityLimit);
        }
        let slot = p
            .caps
            .iter()
            .position(Option::is_none)
            .ok_or(Error::CapabilityLimit)?;
        let handle = Handle(self.allocate_id()?);
        self.processes[i].as_mut().unwrap().caps[slot] = Some(Capability {
            handle,
            target,
            rights,
        });
        Ok(handle)
    }

    pub fn snapshot(&self, id: ProcessId) -> Result<Snapshot, Error> {
        Ok(self.processes[self.index(id)?].as_ref().unwrap().snapshot())
    }

    pub const fn used_pages(&self) -> u64 {
        self.used_pages
    }
    pub const fn now(&self) -> u64 {
        self.now
    }
    pub fn current(&self) -> Option<ProcessId> {
        self.current.and_then(|i| self.processes[i].map(|p| p.id))
    }

    /// Charge elapsed execution time on EVERY trap before handling a syscall,
    /// fault, or timer. Uses a monotonic clock. Yielding never replenishes budget.
    /// If the platform delivers a late interrupt across multiple periods, charge
    /// the final period too; skipped windows do not accumulate credit.
    pub fn account_until(&mut self, now: u64) -> Result<(), Error> {
        if decisions::clock_valid(self.now, now) == 0 {
            return Err(Error::ClockWentBackwards);
        }
        for (i, slot) in self.processes.iter_mut().enumerate() {
            if let Some(p) = slot {
                p.remaining = decisions::account(
                    p.remaining,
                    p.limits.cpu_budget,
                    self.now,
                    now,
                    self.period,
                    u64::from(self.current == Some(i)),
                );
            }
        }
        self.now = now;
        Ok(())
    }

    /// End the previous dispatch and select the next funded process, round robin.
    /// Call account_until first, while current still identifies the trapped task.
    pub fn schedule(&mut self) -> Option<ProcessId> {
        self.current = None;
        let mut mask = 0u64;
        for (i, slot) in self.processes.iter().enumerate() {
            if let Some(p) = slot
                && decisions::runnable(1, p.remaining) != 0
            {
                mask |= 1 << i;
            }
        }
        let i = decisions::next_slot(mask, self.cursor as u64) as usize;
        if i >= MAX_PROCESSES {
            return None;
        }
        let p = self.processes[i].as_ref()?;
        self.current = Some(i);
        self.cursor = decisions::next_cursor(i as u64) as usize;
        Some(p.id)
    }

    /// Arm an interrupt no later than this absolute clock value before user
    /// entry. With no runnable process, wake at the next replenishment boundary.
    pub fn deadline(&self) -> u64 {
        let remaining = self
            .current
            .map_or(0, |i| self.processes[i].as_ref().unwrap().remaining);
        decisions::deadline(
            self.now,
            self.period,
            self.quantum,
            remaining,
            u64::from(self.current.is_some()),
        )
    }

    fn cap_slot(&self, i: usize, handle: Handle) -> Result<usize, Error> {
        self.processes[i]
            .as_ref()
            .unwrap()
            .caps
            .iter()
            .position(|cap| {
                cap.is_some_and(|cap| decisions::same_identity(cap.handle.0, handle.0) != 0)
            })
            .ok_or(Error::InvalidHandle)
    }

    fn authorized(&self, i: usize, handle: Handle, needed: Rights) -> Result<ProcessId, Error> {
        let slot = self.cap_slot(i, handle)?;
        let cap = self.processes[i].as_ref().unwrap().caps[slot].unwrap();
        let owner = self.processes[i].as_ref().unwrap().id;
        let caller = self.current().ok_or(Error::NoCurrentProcess)?;
        if decisions::authorized(
            caller.0,
            owner.0,
            u64::from(self.index(cap.target).is_ok()),
            cap.rights.0.into(),
            needed.0.into(),
        ) == 0
        {
            return Err(Error::PermissionDenied);
        }
        self.index(cap.target)?;
        Ok(cap.target)
    }

    /// The ONLY untrusted entry point. Identity comes from current, never from
    /// a PID supplied by userspace. Hardware adapters decode registers into Call.
    pub fn syscall(&mut self, call: Call) -> Result<Reply, Error> {
        let i = self.current.ok_or(Error::NoCurrentProcess)?;
        let id = self.processes[i].as_ref().unwrap().id;
        match call {
            Call::SelfInfo => Ok(Reply::Info(self.snapshot(id)?)),
            Call::Inspect(h) => {
                let target = self.authorized(i, h, Rights::INSPECT)?;
                Ok(Reply::Info(self.snapshot(target)?))
            }
            Call::Terminate(h) => {
                let target = self.authorized(i, h, Rights::TERMINATE)?;
                self.remove(target)?;
                Ok(Reply::Done)
            }
            Call::Restrict { handle, rights } => {
                let slot = self.cap_slot(i, handle)?;
                let cap = self.processes[i].as_mut().unwrap().caps[slot]
                    .as_mut()
                    .unwrap();
                let restricted = decisions::restrict_rights(cap.rights.0.into(), rights.0.into());
                if restricted > 3 {
                    return Err(Error::PermissionDenied);
                }
                cap.rights = Rights(restricted as u8);
                Ok(Reply::Done)
            }
            Call::Close(h) => {
                let slot = self.cap_slot(i, h)?;
                self.processes[i].as_mut().unwrap().caps[slot] = None;
                Ok(Reply::Done)
            }
            Call::Yield => {
                self.current = None;
                Ok(Reply::Done)
            }
            Call::Exit => {
                self.remove(id)?;
                Ok(Reply::Done)
            }
        }
    }

    /// Trusted fault handler API. The hardware layer must drop access to old
    /// mappings and scrub backing pages before allocating them to a new process.
    pub fn fault_current(&mut self) -> Result<ProcessId, Error> {
        let id = self.current().ok_or(Error::NoCurrentProcess)?;
        self.remove(id)?;
        Ok(id)
    }

    fn remove(&mut self, id: ProcessId) -> Result<(), Error> {
        let i = self.index(id)?;
        let p = self.processes[i].take().unwrap();
        self.used_pages = decisions::release_value(self.used_pages, p.pages);
        if self.current == Some(i) {
            self.current = None;
        }
        // Bounded global revocation prevents stale authority when a slot is reused.
        for p in self.processes.iter_mut().flatten() {
            for cap in &mut p.caps {
                if cap.is_some_and(|cap| decisions::revoke_target(cap.target.0, id.0) != 0) {
                    *cap = None;
                }
            }
        }
        Ok(())
    }
}

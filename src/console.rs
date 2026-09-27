//! Fixed-storage serial line editor and output queue. No terminal escape engine.
use crate::decisions;
pub const LINE_CAPACITY: usize = 64;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Processes,
    Limits,
    Network,
    Uptime,
    Version,
    Empty,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    None,
    Echo(u8),
    Erase,
    Cancel,
    Clear(usize),
    Submit(Command),
    Rejected,
}

pub struct LineEditor {
    bytes: [u8; LINE_CAPACITY],
    len: usize,
    discard: bool,
    after_cr: bool,
}
impl Default for LineEditor {
    fn default() -> Self {
        Self::new()
    }
}
impl LineEditor {
    pub const fn new() -> Self {
        Self {
            bytes: [0; LINE_CAPACITY],
            len: 0,
            discard: false,
            after_cr: false,
        }
    }
    /// UART framing/parity/overrun errors poison the line, including lost bytes.
    pub fn reject(&mut self) {
        self.discard = true;
    }
    pub fn feed(&mut self, byte: u8) -> Event {
        if self.after_cr && byte == b'\n' {
            self.after_cr = false;
            return Event::None;
        }
        self.after_cr = byte == b'\r';
        match decisions::console_action(byte.into(), self.len as u64, u64::from(self.discard)) {
            1 => {
                self.bytes[self.len] = byte;
                self.len += 1;
                Event::Echo(byte)
            }
            2 => {
                self.len -= 1;
                self.bytes[self.len] = 0;
                Event::Erase
            }
            3 => {
                let event = if self.discard {
                    Event::Rejected
                } else {
                    Event::Submit(self.command())
                };
                self.clear();
                event
            }
            4 => {
                self.clear();
                Event::Cancel
            }
            5 => {
                self.discard = true;
                Event::None
            }
            6 => {
                let n = self.len;
                self.clear();
                Event::Clear(n)
            }
            _ => Event::None,
        }
    }
    fn clear(&mut self) {
        self.bytes.fill(0);
        self.len = 0;
        self.discard = false;
    }
    fn command(&self) -> Command {
        // Lean admits only printable ASCII; invalid UTF-8 still fails closed.
        match core::str::from_utf8(&self.bytes[..self.len])
            .unwrap_or("")
            .trim()
        {
            "help" | "?" => Command::Help,
            "ps" => Command::Processes,
            "limits" => Command::Limits,
            "net status" => Command::Network,
            "uptime" => Command::Uptime,
            "version" => Command::Version,
            "" => Command::Empty,
            _ => Command::Unknown,
        }
    }
}

/// A full queue refuses new output, preserving already queued bytes in order.
pub struct OutputQueue<const N: usize> {
    bytes: [u8; N],
    read: usize,
    len: usize,
}
impl<const N: usize> Default for OutputQueue<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> OutputQueue<N> {
    pub const fn new() -> Self {
        Self {
            bytes: [0; N],
            read: 0,
            len: 0,
        }
    }
    pub const fn available(&self) -> usize {
        N - self.len
    }
    /// All-or-nothing so CRLF pairs and cursor-erasure sequences stay intact.
    pub fn push(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() > self.available() {
            return false;
        }
        for &byte in bytes {
            self.bytes[(self.read + self.len) % N] = byte;
            self.len += 1;
        }
        true
    }
    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }
        let byte = self.bytes[self.read];
        self.read = (self.read + 1) % N;
        self.len -= 1;
        Some(byte)
    }
}

/// Bounded raw UART input queue. Hardware/software overruns invalidate pending
/// input; the next reader receives an error so a truncated command cannot run.
pub struct InputQueue<const N: usize> {
    bytes: OutputQueue<N>,
    error: bool,
}
impl<const N: usize> Default for InputQueue<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> InputQueue<N> {
    pub const fn new() -> Self {
        Self {
            bytes: OutputQueue::new(),
            error: false,
        }
    }
    pub fn push(&mut self, byte: Result<u8, ()>) {
        if self.error {
            return;
        }
        if let Ok(b) = byte
            && self.bytes.push(&[b])
        {
            return;
        }
        self.bytes = OutputQueue::new();
        self.error = true;
    }
    pub fn pop(&mut self) -> Option<Result<u8, ()>> {
        if self.error {
            self.error = false;
            Some(Err(()))
        } else {
            self.bytes.pop().map(Ok)
        }
    }
}

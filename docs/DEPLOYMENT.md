# Direct hypervisor boot

`dist/bastion.iso` contains an x86-64 kernel and BIOS/UEFI boot files. It runs as the
guest operating system; it does not require Linux or nested virtualization inside
the guest. Build it with `cargo xtask build`.

## Compatibility requirements

- x86-64 CPU with NX, a usable TSC, and PC-compatible PIC/PIT devices.
- Custom ISO attachment and BIOS/UEFI boot, with Secure Boot disabled for this unsigned image.
- A graphical or serial console; there is no SSH server.
- One active CPU. Additional CPUs are not started.
- Legacy/transitional virtio-net for the current network driver.

Local QEMU validation used 64 MB under BIOS and 128 MB under UEFI. No cloud provider
has been tested. Modern-only virtio, DHCP, provider IP/gateway configuration, and
a persistent boot disk are not implemented. The UDP laboratory uses QEMU's static
`10.0.2.15` address; a console boot on a VPS does not imply working public networking.

## Boot procedure

1. Build the normal release ISO; the test ISO deliberately exits QEMU.
2. Attach the unprovisioned ISO through the hypervisor's custom-image workflow.
3. Open its console and check for `ALL BOOT CHECKS PASSED`.
4. Leave the ISO attached. Bastion runs from its boot image and does not install to disk.

Do not publish relay-provisioned images: their boot modules contain private link
keys. Replay counters are volatile, so relay keys and deployment epoch must change
after every reboot. The [network guide](NETWORK.md) describes the experimental
provisioning format and its limits.

A hosted service still needs provider network configuration, durable provisioning,
modern hardware support, a loader, storage, and capability-controlled user services.

# Security

Bastion is experimental and has no production-supported releases.
Use it in an isolated development environment. The implemented proofs cover
selected policy functions, not the entire kernel, drivers, network protocols, compiler chain,
or hypervisor. [Architecture](docs/ARCHITECTURE.md) and [networking](docs/NETWORK.md)
document the trusted components and limitations.

For a suspected vulnerability, use [GitHub private vulnerability reporting](https://github.com/moritayasuaki/bastion/security/advisories/new).
Include the affected commit and a minimal reproduction. Do not
post real credentials or sensitive packet captures in an issue.
Ordinary non-sensitive bugs and documentation corrections can use public issues.

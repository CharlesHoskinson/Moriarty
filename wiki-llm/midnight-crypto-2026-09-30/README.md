# Midnight Rust cryptography research and implementation

User-requested follow-up to the published authoring beta. [Measured results and remaining gates](RESULT.md), [Rust harness](../../experiments/midnight-crypto/README.md), [design](DESIGN.md), [implementation plan](PLAN.md), [independent expected outcomes](audits/test-contract.md). Acquisition uses official pinned Git source; the local Rust tests are distinct from production financial acceptance.

**Security currentness (2026-10-14, Opus).** These are reviewed primitive-level local experiments (PR12, candidate `7e7f3861`). The k6 proof used an unsafe test SRS. Neither those results nor the secp256k1 signature tests qualify the current signed-v2 path, M8, or any target. See [security currentness](../SECURITY-CURRENTNESS-2026-10-14.md).

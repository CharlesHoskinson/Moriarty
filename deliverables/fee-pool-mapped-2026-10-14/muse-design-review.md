# Muse independent design vote — FeePool source component

## Verdict

**APPROVE** — for the exact frozen proposal plus the signed-scope addendum and its design QA only.

This is a consequential-design vote that the proposed finite FeePool component is a valid foundation toward full M11. It is not implementation approval, not a source/result audit, not an execution grant, and not native, proof, ledger, or full-M11 acceptance. Native authority remains zero. No product code exists yet and none was authorized by this vote.

## Identity (requested vs returned, recorded separately)

| Field | Value |
|---|---|
| Requested reviewer | `muse-wonder-1.0`, HIGH effort, independent non-author design audit |
| Returned identity | Per provider/host telemetry (authority for identity); this report does not self-certify the returned model identity |
| Designer of record | GPT-6.1 Sol (proposal author); this vote is the independent agreeing vote required before author edits |
| Scope read | Complete normative proposal, frozen references, proposed Quint model/tests and actual final logs, signed-scope diagnostic, and applicable tracked Moriarty source interfaces. No peer reviewer verdicts were read. No tests, builds, provider, network, native SDK, k/check/model, key, SRS, cache, credential, or private material was accessed |

Guarded CLI status was inspected at startup (`status --json`): capability `SP01.6 loan-swap-subset`, blocked action `implementation/repair of loan-swap-subset` on stale admission inputs, no pending transactions. That stale loan-swap admission does not block this independent read-only design review, and no native campaign was dispatched.

## Frozen bytes verified

All nine proposal hashes in `design-freeze.json` were recomputed and match exactly (`design.md/json`, `interface.md/json`, `arithmetic-expected.md/json`, `quint-scope.md/json`, `reference-manifest.json`). Both frozen references match `reference-manifest.json`: `fee_pool.rs` sha256 `193863e8…f7ae` (34,143 bytes) and `fee_pool.qnt` sha256 `9dc8e8b1…080a` (32,067 bytes), pinned at Fenrir commit `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`. Base Moriarty commit `dc31345850b5800687b16b26bfa20f1d66d94211`, tree `f6248b06c486b4bc008a2a9e40cf206fc1a71424`. The Quint candidate hashes in `results.json`/`root-validation.json` also match the files read (model `fb9b41a0…d3`, tests `16de81f9…04`, claim-coverage test `5a9465da…ae`).

## Why the design is sound

1. **Arithmetic is exact over the full reference U64 domain with no new primitive.** The relation `q*d + r == n*w` with `0 <= r < d`, `d > 0`, and all operands range-checked to U64 maximum `M = 18446744073709551615` before any product uniquely constrains the Euclidean quotient/remainder. Worst case `M*M + M = 2^128 - 2^64` fits checked Core `u128` (`crates/mori-core/src/ir.rs:13`, checked add/mul in `runtime.rs`). Source syntax has no division, so untrusted quotient/remainder witnesses constrained by this equation are the correct mechanism; neither witness is trusted. Range checks are mandated before arithmetic, which is required because Core boolean evaluation is eager. Denominator discipline (original aggregate, or 1 only when the aggregate is 0, which forces all effective weights and payouts to 0) preserves independent original-weight floor shares and allocation-order independence.
2. **Monetary semantics match the frozen reference.** Costs are metered first (`metered <= gross`, surplus `s = gross - metered`); splits are independent floors 55/10/5/15 against the original `s`; Midnight receives the exact residual, bounded by floor(15%) through floor(15%)+4 (surplus 99 attains +4); the 5% watcher share is a held reserve, never a seat reward and never on a 30-day clock; only operator/DA rewards use `epochClose + 2592000` (30 days, op/DA30-day, not a watcher 30-day rule); backstop refill is `min(amount, max(target - balance, 0))` expressed without unsafe subtraction, and lowering the target preserves an existing above-target balance; finishing sweeps only operator+DA residue after every registered reward seat (including zero-weight seats) is allocated; claims are positive, immutable in amount/recipient, one-shot, and remain legal after finish. Spot checks against `fee_pool.rs` (`settle_epoch`, `allocate_reward`, `finish_distribution`, `claim`, `set_backstop_target`, `record_epoch`) confirm each of these.
3. **Custody and effects are funded and complete, with no double counting.** `intake` performs one real `transfer(FEES, "funding-source", "fee-pool", grossAmount)`; entitlement creation uses `liability(...)` exactly once per named duty and does not move funds; each claim executes `transfer` then `discharge` then marks claimed. The invariant `gross == claimedTotal + Σ liability_amount + backstop + watcherHeld + carried` separates delivered funds, outstanding duties, held reserves, and unallocated custody. Runtime spot checks confirm the needed semantics exist: zero-amount liabilities are created present, `discharge` retains a present-zero duty, and zero transfers succeed when both balance entries exist (scenario seeding is an explicit premise).
4. **Width is finite and honestly bounded.** 54 states + 23 actions + 1 asset = 78 declarations (parser limit 256, `ir.rs:8`), 9 duties (signed registry limit 16), fixed slots for 2 operator seats, 1 DA seat, 1 watcher, and 1+1+1 receipt slots. Finite slots, role/import/clock/beneficiary premises, the unqualified target/governance premise, single-epoch scope (including exclusion of the reference's cross-epoch global watcher-reserve spend), non-kind0 cost kinds, `signal_step_down`, general registries, Capacity Exchange, and native acceptance all remain explicit open items. The component does not pretend to replace final M11 scope.
5. **Implementability in current Moriarty source is established.** Checked `u128` UInt with `+ - *` and comparisons, 
...[truncated 4992 chars]
# Next capability: Preprod-sized, Preprod-budgeted settlement scripts (source work only)

These are reviewer recommendations (Opus security/ledger judgment). They are not root decisions and create no authority. The root routes new source when it is ready. Nothing here is an implementation.

## Why the current S2 cannot go to Preprod

1. **Size (hard).** The applied S2 is 30,764 compiled bytes. Every transaction that carries it, whether as a witness or as the `script_ref` of a new output, is therefore at least that large. Observed Preprod `maxTxSize` is 16,384, and the ledger enforces `txsize ≤ maxTxSize` on every transaction (pinned implementation and formal source). A reference script does not help, because its creating transaction must also fit. S2 must lose about 50% of its bytes, or be split into scripts that each fit with margin.
2. **Cost (strong host signal).** After removing the test-harness decode cost, a single-thread settle is still ≈2.8–3.7× the per-tx memory limit (17.5 M) and ≈1.4–1.9× the step limit (10 B), on aiken's default cost model. The two-thread atomic case (P07) is likely above the per-block memory limit, because S2 runs its full logic once per thread input. Splitting S2 fixes the size and removes the per-input duplication. It does **not** by itself fix single-thread cost. Reducing the per-settle work is required as well.

## Smallest concrete source work, in order

Each step is source-only. It needs its own fresh audit, and any later execution needs its own authorization.

**W0. Cost attribution (source for a later run).** Add per-component cost tests: each named gate component of `settle_facts`, run alone on the p01, p04 and p07 contexts, plus a decode-only baseline. *Predicate:* the components' costs sum, within a stated tolerance, to the facts cost. This decides where W2 cuts instead of guessing.

**W1. Split production gates from diagnostics.** Production C2 and S2 should evaluate unnamed Boolean gates. The named `*_parts` lists and their ByteArray labels should move to test-only code (C2 bytecode currently contains `own_token`, `single_token`, `burned`, `claimant_signs` and `spent_or_expired`). *Obligation:* define each component once and use it in both builds, so that `gate ⇔ all(parts)` holds by construction. Short-circuiting may turn an ERROR-PATH case into a False case. Both refuse, but the set of accepted transactions must be identical. The old ERROR-PATH and BOOLEAN class of each case must be recorded before any run.

**W2. Cheaper signed-intent check (consequential design: needs the two-vote rule).** If W0 shows that on-chain JSON rendering and hashing of owner intents dominate, replace the signed bytes with a domain-separated digest over the typed intent: tag ‖ network ‖ S2 hash ‖ thread outref ‖ seq ‖ blake2b-256(`serialise_data(intent)`). The wallet display must render from the same typed intent. *Preserve:* exact asset identity, recipients, gross-debit, fee, liability and net caps, the validity window, replay and revocation, per-phase authority, and native co-authority. This changes the Rust Core bridge and the signer format, so it needs FX1 test keys and a fresh security review.

*W2b (alternative, larger).* Move the Core recomputation (`recomputed`, `intent_effects`, `statement_digest`) into the Groth16 relation that C2 already verifies, so that S2 checks only digest binding, payouts, continuation and global outputs. *Obligation:* the proved relation must include exactly those conditions (product contract); otherwise this is a trust downgrade.

**W3. Split S2, keeping the hash chain acyclic: C2 → V → S2′.**
- **S2′** (mint + spend, one hash = policy id = lock credential), parameterized by `c2_hash` and the V hash. It keeps genesis and burn (H39, H46–H48, H58), and Revoke/Close if they fit. On `Settle` it requires (a) a withdraw-0 of V's script credential in the same transaction and (b) that V's redeemer, read from `self.redeemers`, names S2′'s own policy and lists `own_ref`.
- **V** (withdraw purpose, parameterized by `c2_hash`) runs **once per transaction**. It validates every input that carries the named thread policy and must cover exactly the set of S2′ inputs. It checks each thread and enforces global output allocation across all threads (H45, H70), the claims (H33, H35, H36, H64, H59) and the C2 release binding.
- *Deployment:* register V's stake credential. The deposit is observed as 2 ADA. The Conway witness rule for registering a script credential is not in the packet; pin it.

## Split-specific adversarial cases (freeze before any run)

1. Settle via S2′ without V's withdrawal.
2. V's redeemer names another policy.
3. V covers one of two S2′ inputs.
4. A V built for a foreign C2 (different V hash).
5. Two programs in one V call with inconsistent per-thread data.
6. V's redeemer replayed or bound to the wrong input set.
7. Revoke or Close used to bypass the settle checks.
8. Own-policy mint outside genesis.
9. V reached through any other purpose (its `else` handler must fail).
10. An unregistered V credential (expected phase-1 rejection; record it as a liveness/deploy fact).

## What must be measured and proved afterwards (each run needs separate authorization)

| ID | Measure or proof | Pass condition (reviewer-recommended) |
|---|---|---|
| M-SZ | Per-script compiled bytes (A2-style), plus the actual serialized size of each deploy transaction | Each script ≤ 14,336 B, and each deploy tx ≤ 15,872 B, which keeps ≥ 512 B below 16,384 |
| M-EU | X1: the compiled scripts evaluated on ledger-shaped ScriptContext Data under Preprod's PlutusV3 cost model from a pinned source with a pinned name mapping | Single-thread settle tx, counting S2′ × inputs + V once + C2 release, ≤ 80% of per-tx limits (≤ 14.0 M mem, ≤ 8.0 B steps). P07 two-thread ≤ 100% of per-tx limits. Report the largest thread count that fits |
| M-EQ | Case-level equivalence: every P and H case keeps its frozen verdict (accept or refuse); the isolation-name mapping is frozen before the run | No verdict change; no refit of the 146 R1 expectations (a new candidate gets a new frozen index) |
| M-ADV | The 10 split-specific cases above | All refuse; case 10 rejected in phase 1 |
| M-SIG | FX1 positives and negatives with test-generated keys only | Valid accepts; any change to domain, network, script, outref or seq refuses |
| M-ENC | OEX7: on-chain `serialise_data` against the original transaction datum bytes, including non-canonical encodings | Documented equality, or an explicit fail-closed rule |
| M-HASH | Independent script-hash recomputation (non-aiken) | Equal to the tool hashes |
| M-G16 | C2 `issue` ExUnits with a genuine new proof (K2, separate authority) | Fits per-tx limits in its own transaction |

## Duties that must survive, mapped to cases

| Duty | Cases |
|---|---|
| Financial limits | H03–H06, H09, H43, H13 |
| Authority | H01, H02, H42, H65, H66, H38, H49, H52, H53, P15, P17 |
| Core recomputation and statement | H11, H12, H34, H37, H60, H56, H57, P16, H69 |
| Payouts and double satisfaction | H20–H25, H32, H44, H45, H70 |
| Frame and continuation | H26–H28, H30, H31 |
| Residual duties | H29, H51, P03 (present-zero), unit tests (absent-duty discharge refused, row erasure refused) |
| Windows | H07, H61, H62, P14 |
| Recursion and claim binding | H33, H35, H36, H58, H59, H64, P12, P13, H54, H55 |
| Multi-parent atomicity | P07.0, P07.1 |

The phase-2 failure and collateral outcome for every settle failure must be stated, and it must keep residual duties (product contract).

## Explicitly not part of this capability

No native, Rust, key, wallet, proof, SDK, ledger, network or funding action. No retry of P3S1-R1. No new controller or framework: reuse the existing literal-step pattern. RR-03, the Rust R1 ABI2/BUILD2 (mount view required), the real Groth16 proof, and every broad exit stay open.

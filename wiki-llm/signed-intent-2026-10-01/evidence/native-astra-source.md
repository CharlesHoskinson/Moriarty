# Independent native numerical source/resource review

Reviewer: fresh delegated GPT-6 Astra, requested medium effort; independent of G3 author. Date: 2026-09-30 local session. Worktree `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`, branch `feat/signed-intent-runner-20261001`, HEAD `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3`.

**Vote: APPROVE BOUNDED — one local numerical native proof experiment under the conditions below.** This is a source/resource vote, not execution admission, actual-result approval, full candidate approval, signed-finance approval, recursive PCD approval, or Preview approval. No blocking source defect was found for this narrow experiment. Whole-candidate and actual-result reviews remain owed.

Read the repository AGENTS, installed and checked-in develop skill, execution-focus reference, orchestration stop rules, complete proposal, complete circuit and test sources, feature export, Cargo manifest/lock, and old native equality precursor for context. Guarded status reports unresolved operational history, stale admission inputs, missing current accounting and unavailable live resource state for the loan campaign; pending transactions are empty. This vote does not clear those dependent campaign gates.

## Frozen SHA256

- `src/financial_transfer.rs`: `f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4`
- `tests/financial_transfer.rs`: `41e632534b815f984cd72bdb0f49153ed3602bfe95b34a8bb19bc61305406f6e`
- `src/lib.rs`: `396f0e86dcd72a9e4dfd7684df872b88c9f4415cab42a96779c11e3622a7d041`
- `Cargo.toml`: `15dedfcb94a3d8ffe8290e30e4c9858e200b9a10682de3db759f93e402204388`
- `Cargo.lock`: `53731faef893a205c4c96933845d72253e3b55277b3bdefc6cf7c355d8b46bab`
- `fixtures/moriarty.json`: `0761254e1166e5ba3ec3f6059d9fa2540d54cedf4accfee7107ba9ecd73095d5`

Paths above are under `experiments/midnight-crypto/`.

## Numbered findings

1. **Repository observation / numerical range relation:** All 28 public values and eight private hints receive sixteen byte steps with eight boolean bits, zero initial accumulator, and final equality to the relation cell. The high bit of the first byte is zero for fields 7 through 12. The pinned Fq implementation explicitly exports little-endian canonical representations and has modulus `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`, exceeding every possible two-UInt128 sum. Thus the recurrence is an actual UInt128 range argument; sums and bounded differences cannot conceal wraparound. Signed127 money/caps and range-constrained amount-minus-one enforce positive amount without trusting host hints.

2. **Repository observation / arithmetic completeness for the scoped relation:** The gates constrain gross=amount+fee; sender debit; recipient and fee-recipient credits; gross/fee caps; net floor; round window; one work unit consumed and spent; gross allowance consumed and spent; and four ordered numerical effect slots. Pre work and allowance total witnesses are UInt128 bounded. Paired exact deltas imply conserved post totals. Saturating host hint generation cannot accept an invalid relation because every hint is constrained by a gate and range. Fee zero remains permitted with unchanged fee balance and zero fee-effect numeric slot.

3. **Repository observation / adapter boundary:** The adapter checks exactly three distinct ordered account roles, one owner allowance, canonical decimal fields, exact five/six effect count and ordered Debit/Credit/(fee Credit)/UseAllowance/UseReplay/AdvanceHead kinds. It projects the selected fixture's numeric values. It does not constrain replay/head payloads, full state preservation, identity authentication, asset origin, registry, nonce, hashes or source semantics. Missing/arbitrary nonnumeric metadata is not comprehensively validated. This is acceptable only because these are explicitly excluded. Do not describe strict projected layout as validation of all Core5 semantics or all fixture identities.

4. **Repository observation / public binding:** Every numerical relation cell is constrained to the corresponding instance row, including version=1. Caps and window endpoints can legitimately vary across different valid statements; changing a public input against one existing proof must fail. Public binding establishes neither source-hash binding nor authority. No account/token/source/policy/head/nonce digest is computed by this circuit.

5. **Repository observation / real verification path:** The ignored test contains exactly one `unsafe_setup`, bounded `keygen_vk_with_k`, one keygen_pk and one create_proof. It checks k<=10 before setup, proves the fee fixture, and verifies through prepare plus guard.verify. I inspected the pinned KZG Guard implementation: verify calls check, which executes multi_miller_loop and final_exponentiation, checking identity. Transcript.assert_empty compares consumed position with complete buffer length. The test requires valid-proof acceptance, each of 28 public mutations to refuse, first-byte corruption and half truncation to refuse, and one trailing byte to refuse. It does not produce an invalid-witness proof or a zero-fee native proof. Such claims would be unsupported. Proof-byte mutations cover these exact cases, not arbitrary malformed transcripts.

6. **Experiment observation / cheap diagnostics only:** I ran the existing executable directly, with native test excluded and two test threads. All seven constraint/adapter diagnostics passed in 1.89 seconds; RowSizer returned k=10 and domain=1024. Executable SHA256 was `bf7a83ea553faf7d1d2a00a54db45ab80f95529d812609db4e134b2cd52d7d32`. This direct invocation avoids compilation and setup; source-to-binary rebuild freshness was not independently established by this invocation. Source was separately inspected and hashed. MockProver success is only constraint-diagnostic evidence. No native experiment was executed by this reviewer.

7. **Repository observation / dependency separation:** Manifest and lock pin native proof git source to midnight-zk `0ededef0e605701fc5139ebdcf011b11f3d86ba7` with proofs0.8.0 and curves0.3.0. Locked ledger dependencies also contain proofs0.7.3/curves0.2.1. The feature-gated export is present. The old equality precursor is insufficient evidence for this new relation. There is no inferred Preview or native ledger verifier compatibility, production SRS trust, recursion, or source/compiler correspondence.

8. **Resource observation and execution condition:** At review, target apparent size was 2,186,144,188 bytes and filesystem free was 26,549,297,152 bytes. These satisfy the proposed launch thresholds now, but must be freshly checked immediately before execution. Disk ceilings are not enforced inside the test or by the bare timeout command. Root's authorized bounded runner must enforce target<=8GiB and free>=10GiB throughout and stop before approaching them. One concurrent native process, at most two Cargo jobs, at most 600 seconds total, k<=10, and exactly one proof creation are the permitted allocation. Cargo jobs limit compilation jobs, not Rayon worker threads; if the allocation means two CPU workers as well, explicitly cap Rayon in the outer environment. No increase of k, timeout extension, R3 retry, second proof attempt, upstream suite, SRS download, network, wallet or production key use is covered. Unsafe ephemeral SRS is local test material only.

## Proposed command and independent expected result

Execute once from the reviewed worktree, only after the other required independent substantive agreeing vote, fresh source hashes, live resource preflight, and applicable admission. The proposal's child command is:

```sh
timeout 600s env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/charl/research/moriarty-crypto-2026-09-30/target cargo test --offline --locked --manifest-path experiments/midnight-crypto/Cargo.toml --features native-proof --test financial_transfer native_transfer_proof_bounded_k10 -- --ignored --exact --nocapture
```

The outer bounded runner must retain raw exit and logs, enforce disk thresholds and terminate the entire child process group at the 600-second ceiling; the bare timeout alone is not evidence of those controls. A process that ignores TERM must not remain alive beyond the ceiling. Preserve failed/partial receipts without retry.

Independent prediction: exactly one ignored test becomes selected, k remains10, a nonempty fee-transfer proof is generated, correct public values pass strict EOF and final pairing, and all 31 negative verification checks fail (28 changed fields, corruption, truncation, trailing byte). Raw exit0 and that test's success are required for the local numerical observation. Timeout, resource stop, keygen/proof error, panic or any accepted mutation is failure of this attempt. Native invalid-witness refusal, zero-fee native proof, identity/source binding, signed financial execution and Preview settlement remain untested even if the experiment succeeds.

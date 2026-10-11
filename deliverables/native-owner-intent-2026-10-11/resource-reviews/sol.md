# Sol independent Task4 resource vote

**Decision: APPROVE — exact bounded, one-shot resource scope only.**

Reviewer: fresh independent GPT-6.1 Sol, requested HIGH effort, dispatched as `/root/sol_signed_native_resource_check`, 2026-10-11. The host supplied no separate external returned-model identity. This is a substantive resource vote on the frozen candidate below; it grants no whole-source approval, actual-result approval, publication/merge approval, native-success claim, or reuse of prior allocations.

## Exact commitment

Checkout `/home/charl/Moriarty/.worktrees/signed-qa-sol-20261011` is clean at commit `ee963f61f225b140c0b0f43ca83b677fc9d36497`, tree `6e1542165799732019a08c09a7b9bf0c32dd7a20`. Runtime base is `/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-4-signed-native/`.

| Selected artifact | SHA256 independently checked |
|---|---|
| `task4-native-config.json` | `56ae9a0e848f0b750511b6d6c1c598fd6fa83d416ee48b82876d1578f915062d` |
| `mori-backend-task4-frozen-final` | `39b137137b2c3a380204f392f1096ca6d25c1de9e03fdf160c8dbb4d84edc4d4` |
| Runtime and tracked `task4-native-driver.py` | `4b6c8931bf73bb75e22181ad32804504ca27587e1f4946bce5c3d025170ba366` |
| `frozen-bindings-index-final.json` | `93f7f786c93de8e03ea3989bd17bca5fe1390cfa721a0ee1cd4de6a87cda5e24` |
| `recipes-frozen-final/plan.json` | `ffcdb02ab7b7d97461f53b40a7f78ec3c5cba4aa2c6e3d8b8f9a2f9fe412cffa` |
| `author-handoff.json` | `91f5201b6e858dd030295d2bd1eb0608f3ac70fa57875456444ea55f6e3ac43d` |
| Author report | `61e3b311c71d19ffc15e43bda5fde40aa3e02b368ced74f71a00366e0862ebb4` |
| Normative design | `a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142` |
| Normative plan | `6f1696fa2ab7ee7c279dcbd38500f35cc48c82c382061a6829abcb64384d4891` |
| Global constraints | `c7d33f0697022b60cc8bcac1a1a0278c13ca3baeea9b4146f10bb2cf3254ce8a` |
| Strict original integrated IR | `b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff` |
| Selected layout | `68db1985c7fccbde6374b9bc4dc214e9ead76494caf4019b10318506a7b449a1` |

The executable is exactly 422,531,128 bytes, mode `0555`; its SHA also matches the emitted compiler executable and the successful frozen no-run compiler-artifact receipt (test profile, opt-level 0, debuginfo 2, no package features). All 61 ordinary pin entries across config/handoff matched, including fixture copies, original runtime inputs, source files and captures. Both `execution/` and `wrapper-execution/` were absent at review. The config/index commit every selected recipe and statement; this approval does not select any draft binary or recipe.

Only the original independent-payment `settle` shape is admitted: ledger revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, k13, 4,193 model rows, 254 instructions, 32 financial plus 15 authorization public values, 83 MOI1 fields and five flattened private fields. Transcript statement has binding_input plus all 47 values. Its honest vector SHA is `42ab592bdc2c10dde0c2af3da7ed77bd431de83d6187806a7f85c1e3a0b0c146`; Claim SHA is `7e6d6432dce0fc79625803448e48e30851131951cbe02b72a41b90722711cb8d`.

Cached SRS metadata is taken solely from the existing pinned JSON snapshot: `/home/charl/.cache/midnight/zk-params/bls_midnight_2p13`, 1,573,252 bytes, SHA `d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803`. I did not open or hash that payload. Production loading charges first, requires a regular opened file, bounds its read to 128 MiB plus one byte, checks the hardcoded upstream SHA and little-endian degree before unchecked decoding, and requires full decoder consumption. The provider refuses another k. Existing embedded verifier parameters at degree14 initialize inside the charged verifier operation and remain subject to the same child limits; they are not a new selectable proving degree or acquisition.

## Exact allocation and enforcement

Ten sequential process groups, one invocation per case, in exactly this order: positive, raw-honest, response-plus-one, wrong-private-key, tight-gross, tight-fee, tight-net, tight-duty, alternative, reused-proof. Each child has RLIMIT_AS soft and hard **2,147,483,648 bytes**, core-file limit zero, **60 seconds** deadline, then SIGTERM with at most **2 seconds** grace before SIGKILL. `RAYON_NUM_THREADS=1`; the test harness selects only `proving::task4_native_driver::task4_native_resource_gated_entry` with one test thread. No parallel native cases, retry, degree search, acquisition or resource increase is admitted.

Aggregate child deadline allowance is **600 seconds plus 20 seconds kill grace**, at most ten children. These are child execution limits, not a promise that wrapper hashing/capture overhead or OS process reaping completes within exactly620 wall-clock seconds. RLIMIT_AS bounds the child's address space, not measured RSS. The inspected native path makes no separate child process requiring additional allocation.

Categories are `[params_load, params_get, keygen, keygen_vk, prove, prove_unchecked, verify]`.

| Case | Maximum calls in category order |
|---|---|
| positive | 3 / 4 / 1 / 2 / 1 / 0 / 3 |
| raw-honest | 1 / 2 / 1 / 0 / 0 / 1 / 1 |
| each of six hostile raw cases | 1 / 2 / 1 / 0 / 0 / 1 / 1 |
| alternative | 1 / 1 / 0 / 1 / 0 / 0 / 1 |
| reused-proof | 1 / 1 / 0 / 1 / 0 / 0 / 6 |
| aggregate | **12 / 20 / 8 / 4 / 1 / 7 / 17** |

These reservations occur before operations and are flushed to create-once JSONL. Counts must exactly equal each case's limits for success; the wrapper also checks aggregate bounds and exact final totals. Upstream keygen and keygen_vk each make one provider request; prove/prove_unchecked each make one more. Internal PK initialization, VK initialization and static verifier initialization are included in their enclosing charged operations rather than represented as independent calls.

The wrapper checks selected binary/wrapper/source/fixture/recipe hashes, commit/tree and checkout cleanliness before launching. It refuses existing allocation output roots, creates captures exclusively, and stops globally on any failed/timeout/nonzero/incomplete case. The Rust entry independently checks its executing binary SHA, explicit admission and external RLIMIT_AS; case directories and results are create-once. Positive must pass before later cases; raw-honest must pass before hostile raw cases. Panics, OOM and timeout cannot qualify as hostile constraint evidence. Prior k11 allocation and stale SP01/A4/A5 history remain consumed/unresolved and supply no authority here.

## Substantive control assessment

The positive uses actual production prove_signed, producer self verification, production verify_signed, and a fresh reconstruction of strict original IR/Claim/VK for another native verification. Complete-effect readback requires alice28/bob80/treasury2, paid70, payment30/reserve5/present-zero0, ordered Transfer70/Fee2/Liability50-to30/State0-to70/Emit70 and caps72/2/68/35. A malformed external s+1 attachment must refuse through both production callers with all native counters zero.

Raw-honest calibrates the identical five-field raw format and full memory/PIs through strict original keygen/prove_unchecked/verifier before hostile work. The guard COPY is used only by the ordinary checker and test-only memory evaluator. Actual keygen and prove_unchecked receive `p.ir`, the strict original, and verification uses that original key. Frozen data differ only at same-index identified checks: s+1 at222; private/public key equality at202/203 for a valid secret11 signature against public PK7; gross/fee/net/duty comparisons at243/246/249/252 for freshly valid signed limits71/1/69/34. No original identifiers, typed inputs, outputs, financial slots or Impacts are removed. Every raw recipe has all108 memory identifiers and five private field values. Hostile qualification requires an actual nonempty bounded proof followed by actual native rejection; a generic prover error is explicitly unqualified failure.

The alternative valid attachment keeps the honest Claim/public statement and reuses the positive proof through production verification. This correctly tests the existential native R/s boundary. Reused-proof controls invoke six actual native verifications: altered bob financial value with rebuilt MOI1/execution hash/Claim binding; rebuilt nonce/message; rebuilt valid PK11 payload/attachment; in-interval at16; appended proof byte; flipped byte0 bit0. Host metadata refusals are saved separately and do not substitute for native rejection.

I independently recomputed all14 frozen statement-vector digests, all applicable canonical Claim hashes and binding_input reductions; public_values exactly equal the 47 statement values after binding_input. I checked each guard-copy diff against the strict IR, including every top-level field and neutralized site's original/replacement, and recomputed all ten memory/raw-private receipt hashes. These are ordinary data checks, not circuit or cryptographic execution evidence.

## Safety and feasibility premises

The bounded attempt is justified by an already measured k13 whole financial circuit, a present metadata-pinned small SRS, a compiled exact executable, modest fixed47-value statements and sequential execution. Host memory metadata during review reported about41,013,440 kB available; that is an observation, not reserved capacity. The debug executable's422 MB size and its initial full-byte self-hash consume part of the2GiB allowance. Native peak allocation and elapsed time are unmeasured. No estimate establishes that keygen/proving will finish: timeout, OOM or a genuine upstream synthesis/prover failure is an acceptable bounded experimental failure that consumes the allocation and stops all remaining cases. Tight hostile synthesis may fail before producing a proof; that outcome cannot be relabeled successful constraint evidence or retried under this vote.

Approval assumes the controller invokes this exact config and wrapper using ordinary assertion-enabled Python (no `-O`/`-OO`/PYTHONOPTIMIZE), preserves all captures, and rechecks pins immediately before admission. The wrapper's essential policy checks use Python assertions. The literal admission environment token does not validate reviewer identity or replace both fresh substantive agreeing exact-resource votes; the controller must establish them before setting it. Direct reviewer/controller native calls outside the wrapper are excluded. Source/config/binary changes invalidate this vote.

## Review evidence and nonclaims

Required AGENTS/develop startup and guarded status --json were completed. Status has no pending transaction notifications and still reports stale SP01 candidate/binding/accounting/resource history; this resource vote does not clear it. I read the normative contract/plan/global constraints, complete production proving.rs/parameters.rs/signed_relation.rs, test-only driver, wrapper, fixture/schema/layout and frozen recipes/index. The normative literal vector parsed to the pinned companion data. Existing final ordinary captures show backend80 passed/0 failed/2 ignored and final library16 passed/0 failed/1 ignored; the no-run capture succeeded. I inspected these captures and did not rerun their tests. My own wrapper dry-run passed with zero native calls and no output-root creation; git diff --check passed and the checkout remained clean.

No source edit, delegation, network request, SRS payload read, SRS decoding, MockProver, ignored-test execution, native keygen/prove/verify or experiment occurred in this review. The only write is this create-once report. There is no resource blocker within the exact scope stated above. Both fresh resource votes remain required for controller admission; after execution both fresh whole-source and actual-result reviews remain mandatory. Native success/soundness, full maximum16-shape support beyond its retained1MiB refusal, financial ledger acceptance, account/key association, ledger time/state/history/nonce consumption, creditor consent, recursion/private handoff, Preview settlement, fraud cost/64KB transaction qualification and full U0-U7/M1-M17 remain open.

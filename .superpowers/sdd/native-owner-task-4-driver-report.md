# Task4 signed native driver — compiled, unexecuted

## Scope and basis

Repository observation, 2026-10-11. This is Task4 driver authoring and ordinary
QA only, under the current GPT-6.1 Sol HIGH author routing. Author task is
`/root/sol_signed_native_driver_impl`; the host provided no separate external
returned model identity. No author self-review counts as an independent vote.

Worktree: `/home/charl/Moriarty/.worktrees/signed-qa-sol-20261011`. Basis:
`becf2daa91bca6d544de71cfc7a1107a6f2d33bd`, tree
`25bd5db3a97f74dfc979be1f709f6af599e0d2ec`. Root supplied Task3's fresh Sol
approval report, SHA256
`bbe8b40bac43be2a207748a3891afd23b2b06eabb3ccb021a10e294cbc92eb30`.
That source approval does not approve this driver, resources or native results.
Root authorized this sole writer in the sibling checkout. The original global
constraints, external Fenrir, original inputs, primary checkout and earlier
captures remain untouched. No delegation, dependency expansion, protocol/API
change, push, merge, history rewrite or native experiment occurred.

The required Moriarty skill and guarded status were applied. Status still names
unresolved stale campaign/accounting/resource history; no transaction notification
is pending. Source/ordinary QA authorization does not resolve dependent dispatch.
Normative design SHA256 remains
`a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142`;
golden vector remains
`9c035c8e42e2b378e8ed2b3388767d3a8d60ac7691aafd3d45dc7066eeb285f0`.
The normative plan was not changed.

## Driver and recipes

`crates/mori-backend/src/task4_native_driver.rs` is compiled only under
`cfg(test)`, as a private proving submodule. Its ignored entry invokes the actual
production `prove_signed` and `verify_signed`. Test-only counters surround their
parameter load/provider, keygen, prove and verifier calls. One `cfg(test)` helper
in `signed_relation.rs` exposes existing reconstruction before host cap checks.
All additions to production source disappear in non-test builds.

Ten exact Task3 fixture files are copied byte-for-byte under
`crates/mori-backend/tests/support/task4-native/` and embedded for portable ordinary
QA. The native entry additionally hashes the original runtime fixture paths.
Only the full independent-payment `settle` circuit is selected: k13, 4,193 model
rows, 254 instructions, 32 financial plus15 authorization public values, 83 MOI1
fields, three typed/five flattened private fields and108 memory identifiers.
Original strict IR SHA256:
`b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff`.
Original layout SHA256:
`68db1985c7fccbde6374b9bc4dc214e9ead76494caf4019b10318506a7b449a1`.

The pinned upstream revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`
provides public `Preprocessed` and `prove_unchecked`, but preprocessing itself is
crate-private. A bounded, test-only evaluator implements exactly the frozen
instruction vocabulary. The upstream ordinary `recipe.check` must accept first;
the evaluator checks all types, ranges, equality/assertions, complete transcript
consumption, public impacts and identifiers. It computes and retains the full
memory map. Pinned native synthesis independently computes each result and
checks alignment against retained memory. No unchecked fabricated flag or
missing financial slot is supplied.

Hostile preprocessing changes only these existing same-index checks in a COPY
of the original IR. Assertions become `Assert(1)`; equalities become `a=a`.
Every identifier, output, public/private input format and Impact is retained.
Only the STRICT ORIGINAL IR is passed to native keygen and `prove_unchecked`;
the original key verifies the produced proof. The copied recipe never supplies
native keys or replaces the whole financial circuit.

| Raw control | Only neutralized preprocessing indices | Recipe IR SHA256 |
|---|---|---|
| response s+1, checked canonical | 222, strict FENS | `a0c1cd461a4fd77c4cdf04da472356e304fac7f78b2ade2270d3ad208403ab4b` |
| genuine secret11/nonce31 signature, expected public PK7 | 202/203, private/public PK coordinate equality | `93a6357e8cc05cffc691cbab9a9c70f0806c442ec94f40a332d7a494a35c6a18` |
| fresh signed gross cap71 | 243, gross72 comparison | `bcff61b6b07ebec14f4c0b1219e87874dccad9484a187925a02fd4062fc84099` |
| fresh signed fee cap1 | 246, fee2 comparison | `cb837f93877e2df4d287aef61c131361a10ca654d29c04f05cc0cc9345f4c770` |
| fresh signed net floor69 | 249, net68 comparison | `d74873e90a1150a7938bca94653e29870a54e5a81997c16de9ebeeb135ff2778` |
| fresh signed duty cap34 | 252, complete closing duty35 comparison | `f0433d6f2b087e5999db715c6f841ceb5342429bddda5456eb6a0b53f8043b97` |

All tight-cap payload SHA limbs, full MOI1 message and Claim/binding are rebuilt;
attachments are actual valid secret7/nonce31 signatures. Both the original
ordinary refusal and acceptance after only the identified neutralization are
checked. These are ordinary recipes, not native constraint evidence. An honest
raw witness must produce an accepted original-circuit proof before any hostile
raw attempt. Every unexpected panic or prover error fails as unqualified.
This driver conservatively accepts hostile evidence only when an actual produced
proof is rejected by the pinned native verifier. It never treats arbitrary
synthesis errors as intended constraint refusal. No unspecified identity-point,
canonical-response-plus-order, torsion, fuzzing or k search was added.

Positive receipt checks retain the complete Execution: alice28, bob80, treasury2,
paid70, payment30/reserve5/present-zero0, and exact ordered
Transfer70/Fee2/Liability50-to30/State0-to70/Emit70. Production producer self
verification, production consumer verification, and another freshly reconstructed
original IR/Claim/VK verification are distinct calls.

Invalid s+1 external attachment is checked through both production entrypoints
before SRS, with zero native counters. A separately frozen valid nonce31 attachment
under PK7 retains the identical Claim/public statement and reuses the honest
proof through production verification. The native statement excludes attached
R/s; no native attachment-byte commitment is claimed.

Six actual verifier controls reuse the honest proof: financial post-bob+1,
reconstructed nonce2/message, public PK11 with a fresh valid attachment, current
at16, raw appended zero byte, and byte0 bit0 flip. Financial post-bob+1 retains
all other financial values, rebuilds full MOI1, updates the claimed Execution hash,
and rehashes the complete Claim/binding. It is explicitly a direct raw invalid
financial report, not a host-accepted execution. The other changed claims use
independently reconstructed host-valid payloads/signatures and include separate
production host metadata-comparison refusals, which never count as native
verification. Every native statement includes binding_input and all47 values.

## Frozen artifacts and specified resource envelope

Runtime: `/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-4-signed-native/`.
Authoritative final recipes: `recipes-frozen-final/`; all draft captures remain.
`frozen-bindings-index-final.json` SHA256
`93f7f786c93de8e03ea3989bd17bca5fe1390cfa721a0ee1cd4de6a87cda5e24`
contains every exact input, source, layout, IR, recipe, statement, Claim, raw
private witness and memory binding. `recipes-frozen-final/plan.json` SHA256
`ffcdb02ab7b7d97461f53b40a7f78ec3c5cba4aa2c6e3d8b8f9a2f9fe412cffa`.
Honest full statement SHA256 (pretty JSON vector, no terminal newline):
`42ab592bdc2c10dde0c2af3da7ed77bd431de83d6187806a7f85c1e3a0b0c146`.

Frozen executable: `task-4-signed-native/mori-backend-task4-frozen-final`,
**422,531,128 bytes**, SHA256
`39b137137b2c3a380204f392f1096ca6d25c1de9e03fdf160c8dbb4d84edc4d4`.
This is a create-once copy of the emitted test executable, chmod0555. Earlier
frozen draft binaries are preserved and are not the selected candidate.
The native entry hashes its executing binary and requires the controller's exact
SHA, plus explicit admission and external RLIMIT_AS. No output may be overwritten.

Wrapper: `task4-native-driver.py`, same bytes as tracked
`crates/mori-backend/tests/support/task4-native-driver.py`, SHA256
`4b6c8931bf73bb75e22181ad32804504ca27587e1f4946bce5c3d025170ba366`.
It adapts the retained liability driver pattern, validates exact candidate/tree,
clean checkout, binary/source/input/recipe pins, then launches sequential process
groups with2GiB address space,60-second deadlines and2-second termination grace.
Config cannot raise these limits or change the ten-case order. It stops globally
on any failure, retains stdout/stderr and timeout/OOM/panic evidence, and refuses
consumed execution/wrapper output roots. Its `--dry-run` launches no test/native
child and hashes no SRS payload. Frozen config and its exact post-commit dry-run
capture are finalized in runtime and returned in the handoff.

Case environment is `MORIARTY_TASK4_CASE`; binary pin is
`MORIARTY_TASK4_BINARY_SHA`. The explicit controller admission token is
`MORIARTY_TASK4_NATIVE_ADMISSION=root-approved-exact-resource-votes`, set only
AFTER both new exact votes. This token supplies no independent resource approval.
Invocation is the frozen binary with:
`--exact proving::task4_native_driver::task4_native_resource_gated_entry --ignored --nocapture --test-threads=1`.
Each case writes `execution/<case>/`: full Execution, recipe, charge-before-call
JSONL, full proof bytes when produced, receipts/refusals and structured result.
Wrapper captures go under `wrapper-execution/`.

Counter order is parameter-load/provider-get/keygen/keygen-VK/prove/unchecked-prove/verify.
Each reservation happens before its operation and refuses exceeded counts.

| Case/process | Maximum counters in that order |
|---|---|
| positive first | 3 / 4 / 1 / 2 / 1 / 0 / 3 |
| raw-honest next; then each of six hostile raw cases | 1 / 2 / 1 / 0 / 0 / 1 / 1 each |
| alternative attachment | 1 / 1 / 0 / 1 / 0 / 0 / 1 |
| reused-proof six negatives | 1 / 1 / 0 / 1 / 0 / 0 / 6 |
| aggregate, ten processes | 12 / 20 / 8 / 4 / 1 / 7 / 17 |

Aggregate child deadline budget is600 seconds plus20 seconds of explicit maximum
kill grace; ten processes, no retry. Internal PK initialization and embedded
verifier parameter initialization remain within the charged enclosing upstream
native call and the same process limits; they are not separate planned calls.
The ordinary Rust debug binary's size is stated explicitly for resource review.

Selected cached k13 SRS metadata supplied by root is1,573,252 bytes, SHA256
`d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803`, at
`/home/charl/.cache/midnight/zk-params/bls_midnight_2p13`.
Only the existing JSON snapshot was read here. The SRS file was not opened,
hashed again, decoded or acquired. Native production loading will validate exact
upstream SHA and degree before decoding under the admitted process.

## Actual checks and remaining gates

Every Cargo command used installed Rust1.99.0, `--locked --offline` and persistent
`CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target`.
No network installation or fallback occurred.

- `red-budget.log`: actual intended failing counter test; original capture kept.
- `recipes-frozen-final/` from `lib-ordinary-final.log`:16 passed,0 failed,1 ignored;
  includes both parameter-free new tests and all existing backend library tests.
- `backend-ordinary-final.log`:80 passed,0 failed,2 ignored across backend unit,
  integration and doc groups. This preceded only the final alternative-attachment
  recipe freeze; the complete final library suite was rerun after that change.
- `no-run-frozen-final.jsonl/.stderr`: final `cargo test -p mori-backend --lib --no-run
  --locked --offline --message-format=json`, exit0, exact executable frozen above.
- `fmt-frozen-final.log`, `git diff --check`, Python wrapper syntax compilation:
  exit0. No Rust warnings.

Initial JSON macro syntax failure, wrong binding-name selector and incorrect
UInt effect assertion type are preserved in separate captures. Fixes followed
actual diagnostics. No native ignored test, SRS decoding, native keygen/prove/
verify, MockProver or other native experiment ran.

This is compiled and ordinary-tested preparation, not Task4 native qualification,
resource approval or complete roadmap delivery. Root still owes both fresh
GPT-6.1 Sol HIGH/Grok4.6 HIGH EXACT resource votes before any native operation,
then fresh whole-source/actual-result reviews. Native feasibility, genuine hostile
constraint results, native financial acceptance, account/key ownership, ledger
state/time/history/nonce consumption, Preview settlement, recursion/private
handoff and full U0-U7/M1-M17 remain open. Task3's full maximum shape and observed
1MiB public-report refusal remain unchanged; no ablation, slot removal, degree
shrink or JSON profile increase replaces that failure.

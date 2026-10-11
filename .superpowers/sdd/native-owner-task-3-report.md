# Native owner intent Task3 — ordinary QA and exact shape report

## Frozen scope and routing

Repository/experiment observation, 2026-10-11. Implemented Task3 only in
`/home/charl/Moriarty/.worktrees/signed-qa-sol-20261011`, branch
`feat/moriarty-signed-qa-sol-20261011`. Base commit
`798829a5394bf1dda8d49c17bc79b25e3b17d89a`, tree
`49f2f60dd5aee7e67c770ec78a986331c75a55d8`. Root authorized this sibling checkout
as the sole exception to the global constraint's older checkout path. Primary,
source/frozen checkouts, old evidence and external Fenrir were not edited.

Latest user routing selects GPT-6.1 Sol HIGH implementation. This author task is
`/root/sol_signed_qa_impl`; no separate external returned author model identity
was provided. No author self-review is counted as independent approval.

Root reported Tasks1/2 source merged in PR27 at
`52ce700c586a697ce4483da9eb88e80364de689a`, with the same base tree, after both
fresh source checks approved. Captured Sol report SHA256
`68b0275a3a524e3481dc1fb20aa472ef7753e66521dc661d78a4e202d971235c` and Grok public
capture SHA256 `93ea6e85326f7c6ab1db857d08aae573831b974bd86320b746737a944fdf5077`
were independently rehashed here. These source approvals do not approve Task3's
new bytes, native results or resource use. Task3 is ready for fresh independent
scoped review; the final full source/result pair and Task4 remain open.

Normative spec bytes remain SHA256
`a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142`; golden vector
remains SHA256 `9c035c8e42e2b378e8ed2b3388767d3a8d60ac7691aafd3d45dc7066eeb285f0`.

Required Moriarty skill/startup status was applied. The guarded CLI reports stale
campaign bindings/accounting/resource history and no pending transactions.
Authorized Task3 source/QA work is independent of that blocked dispatch. No
campaign, scheduler, framework, delegation, goal-state workaround, network
installation, publishing, merging, rewriting history or credential work occurred.

## Implemented QA and limits

The standalone signed Quint model uses one cohesive shared local state and no
messages. Pure functions separately define source-wide exact admission, complete
presence/amount duty membership, ordered checked updates, each financial cap,
independent policy/domain/key/interval checks, whole-candidate signature premise,
accepted update and identified refusal. Thin actions publish one atomic state.
Refusal preserves prior financial state/effects, records a reason and consumes
one of twelve modeled attempts; terminal traces do not gain a blanket stutter.

The signature premise requires equality to the entire candidate Statement:
source, full opening frame, operations, authority, all domains and caps. It means
an authentic external verifier result, not a caller flag. Current logical at is
separate and checked but deliberately not signed. Full AssetId comparison uses
network/issuer/reference/scale; opaque integer references/names abstract labels.
Maps abstract canonical unique registries/limit membership. JSON, sorting,
hashing, source expression evaluation, cryptographic soundness, authenticated
account/key ownership, real duty completeness, predecessor/time/history and
source-to-circuit correspondence are not proved by this model. It does not
qualify M8 or native/ledger acceptance.

Actual mathematical UInt128 maximum
340282366920938463463374607431768211455 is used for all amounts and
outgoing/fee/credit/debit/gross/net/closing-duty sums. TypeScript evaluation
preserves this bound; the existing Rust evaluator's i64 limit is not substituted.

44 deterministic tests pass. The independent payment witness checks alice 28,
bob 80, treasury 2, paid 70, duties 30/5/present-zero and exact ordered
Transfer70/Fee2/Liability50→30/State0→70/Emit70. Other tests cover each freshly
signed cap refusal, state-only/zero finance, selftransfer/self-beneficiary fees,
partial/full/absent/present-zero duties, exact asset/source/payer/debtor mismatch,
all domain/policy/key/nonce/predecessor/interval controls, signature binding,
atomic intermediate failure, all accumulator bounds, UInt128 maximum and 16/17
asset/duty admission.

Final sampled safety run: 10,000 traces, 12 steps, seed 1011, TypeScript backend;
no safety violation. Witness counts: accepted payment 4,339; state-only 6,178;
selftransfer 6,123; partial discharge 4,319; full discharge 6,242; present-zero 6,046;
identified refusal 10,000. These counts are sampled reachability, not exhaustive
verification. Initial types/one-payment test and 100/4, then 1000/12 increments
preceded the final run. No Quint verify or Apalache was used.

`docs/NATIVE-CORE.md` now describes implemented revision3 signed compiler and
ordinary CLI behavior, separate legacy BIP340/ECDSA mode, explicit proof argument
groups, existential native signature scope and pending native qualification.
The QA README distinguishes the historical revision2 liability model/results
from this signed revision3 model and names its correspondence limits.

Only necessary ordinary measurement controls were added. Integration tests
capture exact full signed layouts and strict signature basis. A shared test-only
fixture defines zero/16-width cases. The only backend source-file change is
inside its existing `#[cfg(test)]` module: an internal maximum-shape binding
control asserts production preparation's report refusal. No production protocol
body, ABI, arithmetic, limits, dependencies or financial slots changed.

## Exact integrated measurements

The installed pinned upstream `IrSource::load`, IR optimal k and model rows/k
were used. There was no actual SRS read/decoding, key generation, genuine native
proving/verification, MockProver or ignored test. Ordinary host signatures and
`ProofPreimage::check` are separately scoped reference evidence.

| Exact fixture / action | Assets / duties | Financial / authorization / public fields | Message fields | Instructions / replay steps | Optimal k / model k | Model rows |
|---|---:|---:|---:|---:|---:|---:|
| golden-repayment / repay | 1 / 2 | 28 / 15 / 43 | 75 | 232 / 6 | 12 / 12 | 4,047 |
| independent-payment / settle | 1 / 3 | 32 / 15 / 47 | 83 | 254 / 6 | 13 / 13 | 4,193 |
| signed-observation / observe | 1 / 3 | 17 / 15 / 32 | 53 | 177 / 1 | 12 / 12 | 3,176 |
| zero-registry / settle | 0 / 0 | 7 / 11 / 18 | 27 | 92 / 2 | 12 / 12 | 2,435 |
| full-sixteen-registry / settle | 1 / 16 | 143 / 15 / 158 | 305 | 849 / 20 | 14 / 14 | 10,153 |
| max-sixteen-assets-duties / settle | 16 / 16 | 263 / 75 / 338 | 635 | 1884 / 50 | 15 / 15 | 23,814 |
| max-sixteen-observation / observe | 16 / 16 | 69 / 75 / 144 | 247 | 913 / 1 | 14 / 14 | 10,476 |

Every integrated circuit has exactly three typed private inputs, five flattened
private fields, N+11+4A public values, one unconditional Impact per public field,
13+2N+6A MOI1 fields and six FENS challenge fields. Original UInt/Bool financial
bindings remain complete. All measured optimal/model degrees agree and are at
most 19; no k search or slot/effect/signature reduction was used. Unsupported
higher degrees remain refused by existing production degree limits before SRS.

Strict signature basis: 30 instructions, three public fields, five private
fields, 1,529 rows, optimal/model k11. It was extracted from the actual compiler,
then serialized and loaded with the pinned upstream loader, with public message
replacing MOI1. It retains strict subgroup/private inputs, nonidentity, canonical
response and FENS/key binding checks, but excludes financial constraints, MOI1,
interval and caps. It is not the whole-circuit cost. Exact basis IR SHA256
`f85fbf00593015a5217d1fca4282a88742b95b14222a96129d37526945b486d6`.

The maximum settle fixture retains 16 exact assets, 16 duties, 48 touched balance
frames, 50 effects and all 338 public fields. Each alice balance becomes 8, each bob
and treasury balance 1, every duty becomes 1. Production `prepare_intent` refuses
with `mori::backend::limit`, `Export exceeds the bounded JSON profile:
mori::core::json: JSON exceeds 1 MiB`. This is a real observed bound, not a missing
signature or circuit shortcut. Compact Execution is 441,097 bytes; the bounded
pretty exporter exceeds 1 MiB. An internal ordinary reconstruction binds all
public values, exact payload/message and genuine fixed-QA signature and passes
the ordinary relation checker; this does not count as public preparation
acceptance. The full shape was preserved. Maximum observation preserves all 16
duty slots and 16 asset limits and prepares/host-verifies successfully.

Row/degree/IR/artifact measurements do not establish SRS/key size, peak memory,
proving time, ledger transaction/replay costs or native feasibility. No actual
prover resource observation is claimed.

## Exact bindings and retained evidence

Runtime root: `/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-3-signed-sol/`.
All new files are under this Task3 directory; earlier captures remain untouched.
`measurements-final/measurements.json` carries original actual row/layout counts;
`bindings-index.json` joins every source/scenario/Target/authority/program/context,
artifact/layout/IR/prepared/payload/wrapper/key/message/signature-byte SHA binding.
Its SHA256 is `560bcd6edb7735b674f877334595ec88e80f27a19c003ad296a0d59379931878`.
`measurements-final/max-sixteen-internal-statement.json` supplies the maximum
fixture's full actual internal public vector and signature/payload bytes while
identifying production report refusal. No null file masquerades as an accepted
max prepared payload.

Full fixture sources, scenarios, trusted Targets/authorities, canonical artifacts,
layouts, IR, accepted prepared payloads and fixed-QA signed wrappers are retained
in `measurements-final/`. Golden attachment is the immutable normative vector;
other signatures use PUBLIC laboratory secret 7/nonce 29. Specified alternative
controls use public laboratory secret 11/nonce 31. No wallet/private handoff data
or production key custody is included.

- `golden-repayment` source `cf653de549a82ac9535d868605d71c1df29d4bf3c0d6df4cf6a1a894e619ccb2`; artifact `ed20ecca8342b6943fd05a195c7574fc75305baf97e211d63e4ab608eb903d6e`; layout `1a5f96e46844c350954e159ca3824725549543d62cb288f145403b64059085f5`; IR `742ccde463d44c38a0c7ecf88427c4353e6d85e8f600fc34756a11d92513b95d`; payload `22e7eb4b5ceace5c6041b8662ce20de2bfa180451dd7856e0e08975977716e9e`; signature bytes `8665a2028bad9026f9ed81dcf1605213dfe98b3fd787d5591853f5212014fc6d`.
- `independent-payment` source `a446c4dc826c1706354dea6b1125cfb30019e0f16a90085ead9be9b1c8d5eb2d`; artifact `81ccb2ea7a08b186686a9f8e2f5f8729b68b7f0e6763a56dfd54e50be363758a`; layout `68db1985c7fccbde6374b9bc4dc214e9ead76494caf4019b10318506a7b449a1`; IR `b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff`; payload `628c659fd7e19255e58ad99ae5208ee68a09b5a502da25914d19b59114189146`; signature bytes `fe01ff1d35758331ed42cd644d03a624104910ab86571a547d6d14235be02cf1`.
- `signed-observation` source `a446c4dc826c1706354dea6b1125cfb30019e0f16a90085ead9be9b1c8d5eb2d`; artifact `81ccb2ea7a08b186686a9f8e2f5f8729b68b7f0e6763a56dfd54e50be363758a`; layout `ed9e4509d658819e039a1189ebd16d392533d7aa874df07ccb0ae4b6b3b42b6b`; IR `d44b98091910b8fe87534aeac628dd08dcfda3553dc1a75a5a8cb510616a174d`; payload `5ee14f51a06a4d164e3a1de8cb890ab64284d2336ffebad4fe7729635fbd6f0e`; signature bytes `fa676288461f2c7e3653feea044255063b5c9dee84faeff3a90af8e5419db27e`.
- `zero-registry` source `c227f272644eee7b83589539335a7d6e329562b7f65cf880f0fbb7f0457b68c8`; artifact `81804f5be88e2651622d1e8b6b053ce5d580efdcaeab01c2a9d61634049bd2dd`; layout `4d6ad3c6a41fa3716bf759bd7a1232251fc2498fe143f9555cdf4da3bce69e20`; IR `7543da4139aca91b99d544346be2436d43e928dd7c4675a42124656103d90b0c`; payload `f96d8ed510183493c98628da2b58337b7bb5b08c55dad0aac0825aa425f70ce1`; signature bytes `2327473753614b3bce5a1b333cda6d89dae8cfd74566628e48c1fba7021ba8a9`.
- `full-sixteen-registry` source `444a00f94b0d60641ee73436ce4f7a86cd35afdd3ea05fbd8046aabb93000de7`; artifact `d579e46e5b2909021d4be73c10f489ff4d633895d108c736380e56593a220c5d`; layout `cca9088e2277820a62f9d43426269b7bbae3f7b8cec31d98b17eb72c30eca6fb`; IR `8ec4c4bfb6e7db9ce589e2e5da82e37f4242552e2ad6e0c42d19aa9fe91b2913`; payload `7746dce7eda006398c16ad586fc6abd00244c0ee7997e1e546e7040343b73e9d`; signature bytes `a4b2be97dd3f8c76972c2d55169034e6af9b0568ba91d81285a3f4f8dd3abcc0`.
- `max-sixteen-assets-duties` source `b9693486fe8ea27d3a23cde5292d79d823703fea5162498357263824fa77ee26`; artifact `ba461ec4b1f29b648d16c1555f9f5440c4802d8d80725818f93b455dd75f206d`; layout `4bc9cf828b1ddefc3733e59445748d9095b5f0d886a95943cbb2a7bbe40d2289`; IR `e82ccc7b34dae051cd277b8ed5fa50c2ae2707e1478be6d03271e206aa5e02db`; payload `42321e7dafd708e847150b48a57ad44839d56e272f73dbc040f03cc7244a42ca`; signature bytes `c7a1e52e2a447a7615a6a2da56a2e7dc360c2f3bbb64121b6370535da9521ffb`.
- `max-sixteen-observation` source `b9693486fe8ea27d3a23cde5292d79d823703fea5162498357263824fa77ee26`; artifact `ba461ec4b1f29b648d16c1555f9f5440c4802d8d80725818f93b455dd75f206d`; layout `cb3d9a00689952ad4837a6ac8e06beb4c63bcf5063d5ed042c457647a81da4f4`; IR `ca7f2c5308cb6b5baa18564cfdccdd6cb47439297237c2caf0e36d61916eca57`; payload `5afd52f7852650d1d3b7d77db7ae4a72ad44508bf4bc700ad6723a7c3b359b90`; signature bytes `124185818eb002f67ce38d0e0fe7d8c9d1520f919622004b5edf608b8a5d020f`.

`candidate-index-final.json` hashes all owned source/model/docs bytes and retained
runtime evidence. The local author commit/tree is reported in the handoff and
binds this report itself. Current owned byte SHA256 values:

- `crates/mori-backend/src/signed_relation.rs`: `7aad7f73d241849f884b46544c7f63e89567dec33d036fe52892d0471d0de4e3`.
- `crates/mori-backend/tests/signed_relation.rs`: `64546d19a47f93c1084898a118982611678c6d0c78a02109a6287269ec1db95e`.
- `crates/mori-backend/tests/support/signed_shapes.rs`: `f9c1ddae3c4b6efaffb49fcc193575e93eb0807f6a13bf87442dbad7b102b331`.
- `experiments/moriarty-language/formal/quint/language/signed_transition.qnt`: `d64facc578cb537245895740114054394baaf35a90359c5e886ce9f5a63acced`.
- `experiments/moriarty-language/formal/quint/language/signed_transition_test.qnt`: `f12eccdcd1530084dd91b8e7b370a2e23a5a9e3610d2c9db47f8117d4299684f`.
- `experiments/moriarty-language/formal/quint/language/README.md`: `30b6285962e5927173124b50fa332cb6cbe6d7215121d3153654bb1f2a3de08a`.
- `docs/NATIVE-CORE.md`: `b9be84570f9bcccd2415f6ab07e0d3669b4f0700dc720071a35f2b1fa3ac05e1`.

## Actual ordinary and compatibility checks

Installed toolchain: rustc 1.99.0 (b940084d7 2026-09-28), Quint 0.32.0,
Node 24.21.0/npm 11.19.0. Every Cargo command explicitly set
`RUSTUP_TOOLCHAIN=1.99.0`, `CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target`,
`--locked --offline`; no absent toolchain or download fallback was invoked.

```sh
quint typecheck experiments/moriarty-language/formal/quint/language/signed_transition_test.qnt
quint test experiments/moriarty-language/formal/quint/language/signed_transition_test.qnt --main signed_transition_test --backend typescript --match 'Test$' --seed 1011
quint run experiments/moriarty-language/formal/quint/language/signed_transition.qnt --main signed_transition --backend typescript --invariant safety --witnesses acceptedPaymentWitness stateOnlyWitness selfTransferWitness partialWitness fullWitness presentZeroWitness refusalWitness --seed 1011 --max-samples 10000 --max-steps 12 --verbosity 1
MORIARTY_SIGNED_MEASUREMENT_DIR=<runtime>/measurements-final cargo test --workspace --locked --offline
```

`workspace-final.log`: exit 0, **275 passed, 0 failed, 3 existing ignored**, 32 result
groups, zero Rust warnings. This final full ordinary suite ran once after all
Rust changes. The maximum report refusal is an asserted expected control, not an
ignored failure. Existing fabricated hostile parameter-header tests do not read
real SRS. No native ignored tests ran.

`quint-full-test-fixed-2.log`: exit 0, 44 passing. `quint-final-typecheck.log`:
exit 0. `quint-final-run.log`: exit 0 and all seven witnesses nonzero.
`fmt-complete.log` and `diff-complete.log`: exit 0.

From `packages/moriarty-beta`, existing offline dependencies were temporarily
linked from the existing readiness checkout; the symlink was removed after QA.
`npm run build` generated only this checkout's ignored build outputs. No package
installation or dependency change occurred. Existing signature-only helpers were
used as follows:

```sh
env MORIARTY_CRYPTO_BINARY=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto MORIARTY_INTENT_FIXTURE=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture MORIARTY_REQUIRE_NATIVE=1 npm test
env MORIARTY_CRYPTO_BINARY=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto MORIARTY_INTENT_FIXTURE=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture MORIARTY_REQUIRE_NATIVE=1 npm run typecheck
```

`beta-test-built.log`: exit 0, **193 passed, 0 failed, 0 skipped**.
`beta-typecheck.log`: exit 0. Binary SHA256
`42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339`; intent fixture
SHA256 `e084f2ebc9dc74542e0e84c7d7607c11343bd8121021d11bedec33fa8a26c70c`.
These beta native adapters verify signatures only and do not qualify a native
financial transition proof.

Preserved failures: Quint reserved-name/return-annotation/record-spread syntax
captures, then the hidden-slot test's nonexisting-key `set` error (fixed to `put`);
measurement JSON macro recursion/trait-import compile failures; actual maximum
report-size refusal; beta fresh-checkout missing dist failure before the existing
offline build. Their original logs and initial partial measurements remain in the
Task3 directory. No failure was deleted or overwritten as success.

## Task4 expected controls and open work

`task4-expected-controls.json` SHA256
`858c2e81dfeee5f85a9cf58fcd4cf3249e314d6d135d114ff32e5bf127afa265` freezes
specified expected native/host outcomes and exact mutation recipes against the
actual golden and independent financial fixtures. It requires a genuine positive,
raw native canonical-response/FENS refusal, wrong private key/public equality,
authentic proof verification against changed public financial state, and freshly
signed gross/fee/net/full-duty cap refusals. Invalid external attachment is a
separate host control; another valid attachment can preserve the same existential
native statement. Native receipt and ordinary host/preimage refusal are never
interchanged.

This file is **specified-only**, not a native driver, resource vote or dispatch.
Root must select one exact measured case, freeze driver/binary/public/private-QA
inputs and pinned cached parameter bytes/checksum, then obtain both fresh exact
resource votes with explicit limits/counts before Task4. The independent payment
needs k13; the golden repayment needs k12. No prior consumed native allocation
is reused. No k search, retry, SRS discovery/download, degree increase, dropping
slots/effects or weaker signature is authorized here.

Outstanding: fresh independent Task3 scoped spec/quality review; exact new
resource decision; genuine integrated signed native proof/result pair; maximum
public report bound if broader source capabilities are pursued; authenticated
ledger account/key association, actual duties/predecessor/time/history/unique
consumption, Preview financial settlement, recursive/private handoff, compiler
soundness and all full U0–U7/M1–M17 acceptance gates. Tasks1/2 source merged,
Task3 ordinary QA measured, and the full language/native roadmap remains open.

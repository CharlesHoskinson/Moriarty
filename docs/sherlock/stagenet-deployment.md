# Sherlock stagenet deployment

Sherlock's terminal verification/settlement contract was deployed to Midnight ledger-9 stagenet on 2026-10-23. The transaction finalized at block **993565**. The selected runtime checked canonical inclusion, exact submitted bytes, complete contract-state/effect readback, fee bounds, wallet close and owned-process cleanup. Two independent actual-result reviews supported the deployment adoption.

| Public identity | Value |
| --- | --- |
| Contract | `80a0291cc688bc627b5dfef1a18d8e02ec5ed2f15343e60f18ee61e58a955371` |
| Transaction identifier | `0004799b402e5a83823d40bc6502078ee375ba6f653091c8520d9eabcb45baa283` |
| Transaction hash | `84566edeff7a1ea58b2e61cbd1ad8213cc02af1cf5700b4e62fc2d34b46592e2` |
| Finalized block hash | `2ea9a8230f7b0b3eb1d3e4b4fc92a2b62adea46aedf0315410b3a2b6543eb2f8` |
| Actual fee | `993524550460293` SPECK |

The SDK deployment pipeline completed in **82.6504 s**. The Root capture took **147.4486 s**; a conservative complete-process interval, including final cleanup and exit, was **149.0304 s**. These are nested timing scopes and must not be added. The pre-controller-exit service-cgroup sample recorded a peak counter of **1,349,775,360 bytes**, cumulative CPU usage of **59.408454 s**, and zero OOM events. These counters cover the service through that sample; they are not a final aggregate peak or isolated proof measurements.

The fee provider produced one DUST-spend proof and requested SRS parameters of degree 13. That is a proof count and requested parameter degree, not a payment of one DUST or a measured contract-circuit k. The accompanying [sample](../../deliverables/sherlock-stagenet-2026-10-23/samples/sdk06-deployment-baseline.json) and [public-safe receipt](../../deliverables/sherlock-stagenet-2026-10-23/receipts/sdk06-deployment-baseline.json) keep unmeasured values explicit.

This deployment has zero physical contract calls. It does not yet demonstrate the genuine recursive terminal-D proof call, altered-proof/replay refusals, the synthetic 16-transaction Fenrir rollup, or the finished universal interpreter. Those remain required. Deployment/readback closes one item in the first experiment: **5/10** accepted; the synthetic-rollup experiment remains **1/9**, and the full interpreter remains **0/6**. These are acceptance checklists, not implementation percentages.

The SDK correction compares serialized **TransactionCostModel** values while preserving owned-resource cleanup. The installed **CostModel** has no serialization method. The [exact policy and ownership helpers](../../tools/sherlock-sdk/producer-policy.mjs) are published with the [13 focused tests](../../tools/sherlock-sdk/model-api.test.mjs). Those source bytes passed all 13 tests and a separate real installed-SDK public-default smoke before this deployment. These support files do not add a public Moriarty CLI command.

Run the focused tests locally with Node 24.21.0, the version used for the recorded checks:

```sh
node --test tools/sherlock-sdk/model-api.test.mjs
```

Public provenance: source freeze `3f34e40ed63d888541cf894fb892cdb476e08ba6422fde4003d7b9fece2f3d09`; configuration `9e467975878fec787c8da507407ab75d86c779b913bf7fffcee82e16387c85c2`; Root deployment adoption `b1c5a10f7531e16beb64b4310ab7d05493874acd01815a05b732a76b108df14b`; independent actual reviews `a0938a07daadef3e655f1fbb37cc4cab799c2b5d3f4724b4e73b23ea858595d4` and `9d3801c164844f85ec3fbd8bd3699a60416b3badfcb0692c56f230d66c655a4d`. Hashes preserve local lineage; they do not authenticate the producer by themselves. Wallet material, signed transaction bodies, proof/key/SRS payloads and private runtime files are excluded.

The archived deployment is tied to parent VK
`afc4ec3a87499fedb60375d5b1ceb887d62b3049cb8c011e24c517d02cc7324b`,
raw terminal VK
`bd9e9d89ad0a5570571160b29dc8b2d89e11e3c6065b46638a2e978f9218776e`,
and the 2,806-byte serialized relation
`08b7f6db1db1d7143bfa7660dbb29c8070683cfca5757d690857f2e0a657785f`.
These identities come from the archived call expectations and key-copy regression.
The A4 deployment pins now live in
`crates/mori-recursion/tests/fixtures/stagenet-verifier.json`; A4 checks the parent
and serialized relation before keygen. The current A3 parent differs from this
historical parent. The old address and its pins remain unchanged until a new
approved deployment has finalized and been read back.

`tools/sherlock-sdk/sherlock-anchor.compact` retains the exact public anchor source
(SHA-256 `4f531ce016c697c973162b8f5f6a9ac4d034ba3e3ca15b919d05c00cb3252634`).
Its `acceptValidity` transcript addresses set index zero. The former A4 fixture
came from StrawAnchor and encoded index four at public slot 5. The corrected
fixture was compared against two generated SherlockAnchor public transcripts
with Compact 0.34.0, compact-runtime 0.19.0 and ledger-v9 1.0.0-rc.3. This is an
offline adapter comparison; a compiled proof cross-check and transaction
acceptance are separate obligations. Reproduce it after compiling the anchor
with `--feature-zkir-v3 --skip-zk` and installing the matching runtime:

```sh
node scripts/qa/capture-a4-template.mjs BUILD_DIRECTORY RUNTIME_INDEX_JS LEDGER_FS_JS \
  crates/mori-recursion/tests/fixtures/accept-template.json
```

Offline terminal preparation is available through
`scripts/doctorwatson heavy --id A4prepare -- scripts/qa/prepare-a4.sh`.
`MORIARTY_TERMINAL_KEY_DIR` optionally selects a retained public key directory;
its serialized relation must equal the freshly constructed relation byte for
byte, and the resulting proof must verify. Preparation never marks A4 as passing
and never submits a transaction. Its synthetic binding input and fresh in-memory
communication opening are for offline verification; a real call needs its own
final call bindings and fresh proof.

The corrected current-chain offline run produced k20 raw VK
`3f3b7e131e83101258ef080820362941218adf6d59cf6574c682267a01ff0083`
from serialized relation
`86dfecc6d4a09d10cea0508341bb435eeff32265b52b623fe84289aa91577905`.
The 6,432-byte proof passed native verification and the ledger reader; all 15
individual public-input mutations and a proof-byte mutation were rejected.
Key generation took 123.170 s and the prove/verify interval took 123.284 s. The
complete DoctorWatson run took 290.552 s with 16,587,382,784 bytes peak RSS
(`20261025T012424Z-A4prepare-corrected`). The complete interval contains both phase timers.
This key is a prepared candidate, not an installed verifier. A4 still refuses
the historical deployed parent before keygen, and no new transaction was submitted.

On 2026-10-25 the owner-authorized replacement for the current A2 → A3 → A4 chain
finalized on stagenet at block **1,013,319**. The current verifier contract is
`ac93b80e252f9cde794d22f76b2eb7d727f855f07ad77a0904afe5bac855388b`.
Its transaction identifier is
`00352295b794252756a67dcb63224a3a6b94fbbaf29a5f5c2b60343a48ff88af07`;
the independently queried indexer transaction hash is
`2444fb7c3969b7ab21b6b0a9fa09b6b9f00b8ad685e5ee9771f7173ab7b120dd`.
This supersedes contract
`80a0291cc688bc627b5dfef1a18d8e02ec5ed2f15343e60f18ee61e58a955371`
and its raw terminal key
`bd9e9d89ad0a5570571160b29dc8b2d89e11e3c6065b46638a2e978f9218776e`.
The archived deployment above remains a historical record; its parent and
relation are not inputs to the current chain.

The complete current key lineage is:

| Input or product | SHA-256 |
| --- | --- |
| A102 child VK (1,229 bytes) | `a6e6ed282daba635711e5d15863441a4888eb28407ec3899ee8722b5e1b02ab9` |
| A80 child VK (1,229 bytes) | `65414136eabb682558c39e8a76d8d5f436641fe2d76a87bbe88a5b26b093c104` |
| A3 parent VK (2,189 bytes) | `801467c825c79e063c0603b94c49eef63fd527ff8b3a6d42aa24022b108849a8` |
| Official k20 SRS | `1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1` |
| `accept-template.json` (3,246 bytes) | `d7a9f218630cfc1f25e4f474927a671755b929501171e3fe3121351f5bd29207` |
| Serialized decider relation (2,806 bytes) | `86dfecc6d4a09d10cea0508341bb435eeff32265b52b623fe84289aa91577905` |
| Raw decider VK (2,189 bytes, k20) | `3f3b7e131e83101258ef080820362941218adf6d59cf6574c682267a01ff0083` |
| Installed tagged `acceptValidity` verifier (2,217 bytes) | `10b2d9cfe7fd16df2616da4616f69c4f34f668e59f229e5855a1f6572fd46100` |
| Compiled `settleValidity` verifier (1,353 bytes) | `be8b0943d8e2acadeb2fd914e4bcc100bdcce72ffe2a8da4e34f9417141c0b5c` |
| Finalized contract state (4,604 bytes) | `3a44aa7e6cc54bcd79637a63a9b5d37b7a2d2e8113c0631175f4e854035342ae` |

The child producer is in `865f06402`, the parent port in `9d485819b`, the
corrected terminal template in `ed4d0181b`, and the terminal input guards in
`b0e8cbba8`. Deployment ran the SDK adapter from `9677548df` on `factory-a2`.
The anchor source remains SHA-256
`4f531ce016c697c973162b8f5f6a9ac4d034ba3e3ca15b919d05c00cb3252634`.
Its compiled build records Compact 0.34.0 and runtime 0.19.0. The deployment used
midnight-js 5.0.0-beta.7, wallet-sdk 2.0.0-beta.2 and proof server
9.0.0-rc.5_experimental, with `isCompleteWithin(50n)` for wallet synchronization.
The adapter overrides only the verifier material when installing `acceptValidity`;
it never installs that circuit's compiled verifier in place of the decider.

Before opening the wallet, the exact SDK constructor path passed checks for all
13 ledger fields, both empty statement sets and both verifier keys. The finalized
SDK readback passed those checks again. A separate HTTP indexer query returned
byte-identical state, and the Rust ledger reader extracted the exact raw decider
VK above (`k=20`, `hasIr=false`). The SDK interval, including wallet sync and
confirmation, was **316.524843 s**. DoctorWatson measured **318.176 s** and
**745,893,888 bytes** peak RSS for the complete deployment command. These are
nested scopes, not isolated circuit proving measurements. A subsequent independent indexer query reported `SUCCESS` and an actual fee of
**997,443,262,154,202 SPECK**. RPC readback matched canonical block hash
`5e4a30d96843d8f3689585c0940f59c986871be5ff5f2c5aae98789241f07b60`
and observed finalized height 1,013,360, above the deployment height.

The public receipt is retained locally at
`target/a4-anchor/deploy-run3/receipt.json`; the independent state query and key
reader outputs are `target/a4-anchor/new-contract-indexer.json` and
`target/a4-anchor/new-key-readback.json`. The measured factory trace is
`qa/sherlock/traces/20261025T023414Z-A4deploy.log`. Wallet material, the SDK's
local encrypted maintenance-key database and private proof preimages are excluded
from version control. The deployment contains no physical contract call. Native
A4 verification, a call with its actual bindings, local ledger acceptance and
settlement remain distinct checks; the deployed parent still covers only the
fixed A102 literal → A80 pair, not a Fenrir STF/A0 batch.

The fresh installed-key A4 check passed on 2026-10-25. It regenerated the exact
raw decider VK in **134.367 s**, then proved and verified a **6,432-byte** terminal
proof in **142.783 s**, including individual public-input and proof-byte refusal
checks. DoctorWatson measured **324.739 s** and **16,487,976,960 bytes** peak RSS
for the complete command. Its trace is
`qa/sherlock/traces/20261025T024457Z-A4.log`; the retained public proof bundle is
`target/sherlock-a4/run-Fqxp6gSV/artifacts`.

`doctorwatson check A5` also passed against independently fetched installed state.
The separate stock verifier accepted all four exact A2–A4 proofs, reconstructed
the carried accumulator from the exact child proof bytes, checked all statement
links and refused 23 mutations. Verification took **0.958 s**; the complete
factory check took **2.855 s** with **507,248,640 bytes** peak RSS. Its trace is
`qa/sherlock/traces/20261025T025102Z-A5.log`.

The A6 preparation uncovered a separate validator limitation: the installed JS
WASM `wellFormed` accepted a retained proof reused with different call bindings,
even with its contract-proof flag enabled. This matches the ledger-wasm source
configuration, which disables the ledger's default features; without
`proof-verifying`, `ProofMarker::proof_verify` returns success. A6 therefore uses
the optional `sherlock-ledger-validation` feature and native `validate_call`
example with `midnight-ledger-v9/proof-verifying` enabled. That verifier rejected
the same reused proof as `failed to verify proof: Invalid proof`. Its separate
all-checks profile also refused the unfunded fixture for a negative DUST balance.
This negative control is retained in
`target/a4-anchor/local-native-proof-rejected/negative-ledger-result.json`.
It is not a positive ledger-acceptance result or a stagenet call. Call preimages
and private openings reach the native prover only through a pipe.

`doctorwatson check A6` passed on 2026-10-25. The installed contract state
(SHA-256 `3a44aa7e…5342ae`) was put into a blank local ledger-9 by a bootstrap
deploy that waives fees. A freshly proved, actual-bound terminal D proof for
`acceptValidity` was then accepted and applied by the native
`midnight-ledger-v9/proof-verifying` validator. Contract proofs, signatures and
limits were on; balancing and native proofs were off. After the call,
`validity_ok` contained the statement, `settled` was empty, and root, height and
`batch_no` were unchanged. The native and JS complete post-state hashes match
(`c0c10aa5…d462e4`). In the same run, the reused offline proof was refused as
`Invalid proof`. The all-checks profile still refuses the unfunded local fixture
(`invalid balance … for token Dust`), so this is not a claim of full
transaction validity; that is A8 on stagenet. The check took **163.937 s** wall
time with **14,999,412,736 bytes** peak RSS. The trace is
`qa/sherlock/traces/20261025T042312Z-A6.log` and the receipt is
`target/sherlock-a6/run-4M1RU5hz/receipt.json`. Do not take any acceptance from
the installed SDK's JS WASM `wellFormed`: it does not verify proofs
cryptographically.

## First stagenet call (A8, A10)

On 2026-10-25 one `acceptValidity` call was submitted to `ac93b80e…55388b` with
`tools/sherlock-sdk/call-current.mjs --submit` under `doctorwatson heavy --id A8`.
- Transaction `a2b71fa213437f7c8470f60be871eed20ff1c78209ee5dcf29a91198f8e3af07`
  (id `0085e459…20611f`), block **1014763** (`5f5b16b7…c6f8d9`), result `SucceedEntirely`
  / indexer `SUCCESS`, fee `292262057142858`.
- Statement `deffb497…a5c25a` (public input 10 of the A4 terminal statement); fresh
  actual-bound D proof SHA-256 `7a8691fc…861ee9`.
- Pre-state `3a44aa7e…5342ae` (identical to the A6 state); indexed post-state
  `1eba3c3e…0349b9`, byte-equal to the independent VM prediction.
- Before balancing, the proven call was accepted and applied by native
  `proof-verifying` ledger-9 with contract proofs, signatures and limits on. After
  wallet balancing, the transaction was accepted again in the same profile. The
  local all-checks profile refuses only on DUST state absent from the blank local
  ledger, and that refusal is kept as evidence.
- Time: **475.682 s** wall, **14,999,293,952 B** peak RSS. Wallet sync took about
  287 s, the native D prove with key load about 163 s, native checks and balancing
  about 2 s, and submission to indexed readback about 20.5 s. Trace:
  `qa/sherlock/traces/20261025T045601Z-A8.log`. Outputs:
  `target/sherlock-a8/submit-20261025T045601/`.
- An earlier attempt (`20261025T044552Z-A8`) stopped before submission. It had passed
  a compact-runtime `ChargedState` to the ledger WASM `updateIndex`, and no
  transaction was sent.

`doctorwatson check A8` passed (indexer `ContractCall`). `doctorwatson check A10`
passed with `tools/sherlock-sdk/readback-call.mjs`, which reads only the pinned
address, the public A4 statement and the indexer. It took the deployment from the
call's `deploy` field and the pre-state from `contract(offset:{height:1014762})`,
and found the call to be the only action on the contract in its block. It
re-verified the indexed transaction's proof natively against that pre-state,
checked that public input 10 is the statement, replayed the call and matched the
complete indexed post-state. In the decoded state, `validity_ok` contains the
statement, `settled` is empty, and root, height, `batch_no` and every pinned field
are unchanged. Time: 2.336 s. Trace: `qa/sherlock/traces/20261025T050529Z-A10.log`.
Receipt: `target/sherlock-a10/run-48eEFirl/receipt.json`. Indexer note:
`contractAction` offsets select an exact block height, not "at or before".

## A9 settle deployments (D2, D3)

The A9 settle-path cases (s-c wrong predecessor, s-d reordered batch, s-e (iii) settle
replay and (iv) other domain) consume an accepted statement, which cannot be undone.
By coordinator decision on 2026-10-25 they run on two fresh, owner-authorized
deployments of the same source and keys. The A8 pin `ac93b80e…55388b` was not called
and remains the accept-level pin (s-a, s-b, s-e (i)/(ii)).

| | D2 (settle control, s-c, s-d, s-e iii) | D3 (s-e iv, other domain) |
| --- | --- | --- |
| Contract | `2fbe68b30cd302f9ea9c1c260385a02c679f746d3d05c2bf131f8d9fef4b59e0` | `03e750800b904d30fc813cc0d87f08dafd612c5ab1804d5a963134bfb7c2aa2b` |
| Deploy tx id | `009dda35e550cbb3f96ad1138970d72cf12524c67c3fb5e910671535989d460f07` | `0077429fcaffd690e3a2a09373b453edf220372d774f862abf0f9042dd2c4bb273` |
| Deploy tx hash | `b416ecf3c15439de61bc481beead8fb9174e8ede0c4191bd9f32855c56f34189` | `57239e060ad706ea7eb56c0b936e52e091526741c59004d6ae18c8650e5ff75f` |
| Block | **1016246** (`9aa4a30e…c3f162a`) | **1016306** (`daf94b32…450ef87`) |
| Indexer result, fee (SPECK) | `SUCCESS`, 997855085714286 | `SUCCESS`, 998403657142858 |
| instance_id | `489080d3…ecd45c00` (LE; identical to ac93b80e) | I2 = sha256("sherlock A9 s-e(iv) second instance_id") mod r = 18814733036858064687259868561697402537754088490782013248729309943217888935855 |
| Deployed state SHA-256 | `cf2e97d2…94c8eb44` | `f352e748…4782d70` |

Key lineage is unchanged from the current chain above. Both deployments use source
`4f531ce0…3252634`, the installed tagged `acceptValidity` decider verifier
`10b2d9cf…5fd46100` (raw decider VK `3f3b7e13…01ff0083`, relation `86dfecc6…91577905`,
A3 parent VK `801467c8…108849a8`), and the compiled `settleValidity` verifier
`be8b0943…1c0b5c`. The constructor is that of `target/a4-anchor/prepared-deployment.json`
(SHA-256 `f0d3ee9d…925e3f21`), with only argument 0 replaced on D3. Both used
`tools/sherlock-sdk/deploy-current.mjs` (D3 with `--instance-id`). Each passed the SDK
constructor check and the finalized readback of all 13 fields, both empty sets and both
verifier keys. Receipts: `target/sherlock-a9-deploy/{d2,d3}/receipt.json`.

D2 then received `acceptValidity(deffb497…a5c25a)`, the A8 statement, which is the
precondition for the settle control. The call used `tools/sherlock-sdk/call-current.mjs`
with `MORIARTY_SHERLOCK_DEPLOYMENT` set to the D2 receipt and a fresh actual-bound native
decider proof:
- Transaction `5f3ae140a257ee07e5b5f9c461f44df142fb28b9ebd2a8257b29b1fc9c9a85e5`
  (id `00d44e7d…0e52ffb2`), block **1016423** (`8d7f86df…ed33445`), `SucceedEntirely` /
  indexer `SUCCESS`, fee 292262057142858.
- Pre-state `cf2e97d2…94c8eb44`, indexed post-state `e4bb4589…a7d7a1b6`, byte-equal to
  the VM prediction. Native proof-verifying ledger-9 accepted and applied the call before
  balancing (contract proofs, signatures and limits on).
- Only the decider prove ran under `doctorwatson heavy --budget a4-terminal-d`. Wallet
  sync and indexer waits stayed outside the slot: **311.586 s** and **15,085,056,000 B**
  peak RSS (`20261025T074513Z-A9-D2accept-prove`). Outputs are in
  `target/sherlock-a9-deploy/d2-accept/`.

The settle attempts use `tools/sherlock-sdk/settle-harness.mjs`, which proves
`settleValidity` against a given snapshot with no chain re-fetch and never submits. Before
any transaction, `tools/sherlock-sdk/settle-rehearsal.mjs` passed under native
proof-verifying ledger-9 (`20261025T072650Z-A9settle-rehearsal`, receipt
`target/sherlock-a9-rehearsal/run1/receipt.json`):
- The control settle C applies over the A8 post-state.
- The stale (P1), forged-head (P2), reordered (P3) and other-domain (P4) settles each
  carry a real proof and are well-formed. Each applies over the snapshot it claims and
  fails to apply on its target.
- The honest second settle over post-C is refused locally (`SettleUnsatisfiable`).
Native refusal happens at guaranteed-transcript apply, not at `wellFormed`. Whether the
stagenet node refuses before inclusion is for the A9 evidence to show.

## Redeployment after the host loss (2026-10-26)

The old host and its deployer wallet were lost. The merged lanes changed the A3 parent key, so A4 failed against the installed contract `ac93b80e…`. On 2026-10-26 the A-chain was rebuilt (A2 and A3 PASS) and the terminal proof was prepared offline. SherlockAnchor was then redeployed from the new deployer wallet (`mn_addr_stagenet19mr7e738…gcwhe`).

| | Value |
| --- | --- |
| Contract | `2391d62fcfbf21b35cf911f651d71cb9d435fc3e962f67e2e94f91e85e91ff3f` |
| Deploy tx id | `009af03ef89880287f8e6bb17fa97c1e653fab235ab4b725f81548475775dac03b` |
| Deploy tx hash | `9c605e100f6595e0fee1d5d69450757b4ec59b2fe074a37bae771ea87d8e8741` |
| Block | **1032519** |
| Anchor source | `4f531ce0…3252634` (unchanged) |
| A3 parent VK | `642ef85409d8b55609b7ca5d45996b07e9ba6fdc1f8915697cfe7bd9200cb94e` |
| Raw decider VK (k20) | `dc5fc17f75e140e6bc22a5afc23588ee9cfe6947449dbaf888cfdfcb1190d5f7` |
| Decider relation | `9913df00292022582b29525abe053aee89f20099e37c57a6282cd96a9b2d964e` |
| Installed tagged `acceptValidity` verifier (2,217 bytes) | `3a7f4c5fed0441777b9fb50bd33192818ace065c9d03838648457fbe4564b198` |
| Compiled `settleValidity` verifier | `be8b0943…1c0b5c` (unchanged) |
| instance_id | `489080d3…ecd45c00` (LE; unchanged) |

The preparation `target/a4-anchor/prepared-deployment.json` is now produced by `tools/sherlock-sdk/prepare-deployment.py`. That file's producer had not been committed before. The SDK constructor dry run passed (`20261026T101939Z-A4deploy-dry`). The deployment ran under `20261026T103553Z-A4deploy` in 326.8 s at 626 MB peak RSS, using proof server `9.0.0-rc.5_experimental` (docker) and the pinned SDK set. The finalized readback matched all fields and both keys. `scripts/doctorwatson check A4` then passed against the installed key (`20261026T104216Z-A4`).

**Two earlier attempts each finalized a contract and then lost its maintenance signing key.** The SDK reads the private-state password only after the deployment is final:

- `f94fc52ce4d00e8540438b4962d5099bbe3ed5a412f94d0851cc0239fd1cca9c` (block 1032367, `deploy-run4`). The password file was missing.
- `81602c6f34be0094b9d019877f4f55214efe10b128d45627a492c224bf6b4279` (block 1032453, `deploy-run5`). The SDK rejected the password: it needs at least 3 character classes, and a hex password has 2.

Both orphan contracts hold correct state, and anyone can still call them, but nobody can maintain them. No receipt was written for either. Sherlock does not use them. `deploy-current.mjs`, `rollup-deploy.mjs` and `stagenet-session.mjs` now check the mode-600 password file and run the SDK's `validatePassword` before any transaction.

The SDK writes the maintenance key store to `midnight-level-db/` at the repository root, which is gitignored. It is backed up outside the repository. The SDK root also had to pin `@midnightntwrk/onchain-runtime-v4` to `4.0.0-rc.3`: a fresh install resolved a second copy (rc.5), and the SDK then failed with `expected instance of ChargedState`.

# Current Rust/Cardano local reference evidence

Run A passed9 tests. Run B passed11 tests: the same9 currentness tests plus2 chained staged tests. There are11 unique tests across the runs, not20. The2 staged tests compare8 complete outcomes against independently frozen expectations: delivery start/false/true and reserve start/firstfalse/firsttrue/secondfalse/secondtrue. Public raw-source APIs consume actual current Core parsing/staging and preserve complete metadata, Program, Plan, frames, captures, executions, ordered effects and work. See the exact source-hash manifest, independent freeze and verbatim commands/stdout/stderr/results in this directory.

This is local reference evidence only, partial SP1–5. NS1–17, NT1–24, SP6 and supplementary SP7 remain unexecuted. No stage-to-UTXO codec, layout/hash, target schema, Aiken, proof, authorization, custody, Preprod or ledger acceptance is established. The full goal, all20 pattern families, general obligations, recursion/privatePCD and U/M/MC/SP/G exits remain open.

Raw StageOutcome serialization is an internal test assertion alongside complete metadata carrying local-reference-stage / none / untracked. It is not a labelled persisted transition/target record. That separate consumer/codec requirement remains open. The retained root lock changes declared native source/build identities; see NATIVE-IDENTITY-DISCLOSURE.md. The separately scoped ledger source-closure qualification was not run.

The reviews apply to exact frozen uncommitted candidate bytes, not to an invented candidate commit. SOURCE-MAPPING.json distinguishes the actual base/main and earlier published feature reference from this local candidate. Publication of these proposed documents has not occurred in this task. Independent freeze lists51 original members; this curated evidence directory retains the original inventory and derivation, while the source candidate contains27 selected byte-exact test fixtures. The complete original packet is not duplicated here.

## Exact attributed Opus P2 disclosure

The following four paragraphs are copied verbatim from the P2 Opus report §6 (fc2687d903f7d27e8e5654468b1d3d8e8a9ba46e6fe3a463794c90e69b2359d2); their original historical candidate reference remains unchanged.

**Scope (projection /2).** The `mori-cardano` UNIT in-memory projection candidate reviewed under `current-cardano-p2-projection-20261015/REVIEW-INPUT.md` (SHA-256 `c3b64f7e85cf56d8bcfc9388a8daad1b95cc8db89c5b9d5051b0ba54d93db61e`) derives, from raw `moriarty/1` source only, per-action logical work W(a), static required-signer names, unreferenced bound names, and an in-memory positional body of state values, script-custodied amounts and duty cells. It reconstructs Core Scenarios from a body plus caller-supplied KeyHeld premises and runs the in-tree Core reference with cap W(a). It does not perform profile/1 admission and computes no layout descriptor, layout hash, sequence number, trace commitment, Plutus Data or CBOR bytes, validator, comparator outcome, credential or ledger observation. The metadata/1 API keeps its rule that every transfer, fee, debtor and creditor name is bound; the projection /2 API requires binding only for transfer and fee parties. "Unreferenced" means bound but absent from every transfer and fee statement; a name used only as a duty party is reported unreferenced. Account names and classes are symbolic and are not credentials or authorizations.

**Evidence limits.** KeyHeld premises are unauthenticated caller inputs, and framed amounts are body claims rather than observed UTxO value; reference results built from them are not holdings, custody or ledger evidence. W(a) is the exact logical work of a completed run under `moriarty-reference-logical-cost/1`, not ExUnits or a fee. An empty required-signer list means only that Core `/1` requires no one's signature for that action; it is not evidence of safety. A duty row records a source name; it does not show that the named debtor consented or is bound. The six positive cases are narrow evidence, and no refusal path of the projection has been tested.

**Invoice specimen (F-10).** The installment-invoice specimen (`installment-invoice.mori`, SHA-256 `105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9`) is a `moriarty/1` test Program, not a safe escrow. Core `/1` has no caller-authority construct. Its `partial` action debits only the script-custodied `escrow`, so under the proposed adapter profile/1 it requires no signer. From the specimen's post-`open` state (escrow 109, invoice residual 90, reserve 14), any party may submit `partial(1)` 36 times. Each call moves 2 to `fees` and 1 to `seller`. Escrow then holds 1 and the invoice residual is 54. A further `partial`, and `finish`, both refuse with `mori::core::balance`, and the thread cannot progress. At that point the seller is short 54, the reserve duty of 14 is unfunded, and `fees` has received 72. This is the specified `/1` behaviour of this Program, not an adapter defect. This metadata neither exercises, detects nor prevents it. The conditional-reserve specimen is likewise callable by any party, and its `approved` input confers no authority. Nothing in this candidate, its goldens or its tests is evidence that either specimen protects any party's funds.

**Projection /2 note on F-10.** The empty `required_signers` reported for `partial` and `finish` are this hazard expressed as metadata. The projection can reproduce the sequence locally through `project_reference`, but it neither detects nor prevents it. The positive tests do not exercise it; specified test NT-18 checks only its terminal refusals.

## Exact attributed Opus staged disclosures

The following two paragraphs are copied verbatim from the currentness Opus report §4 (d5d19cfc5777f695d05841cf86442554cb50adf5b4d9360e8897cb84fc560a12). No new security norm or author judgment is added.

**Delivery (F-11).** In `delivery.mori`, segment 1 of `deliver` debits only script-custodied `escrow` and requires no signer, and its `ready` binder is a caller-supplied value with authentication `none`. Any party holding or copying the continuation may resume with `ready = true` immediately after start and move the escrowed amount to `bob`. The await is not a delivery condition against an adversary. Nothing in these results is evidence of conditional settlement.

**Reserve (F-12).** In the reserve specimen, `quote` is checked only against `held` at the first await, and `quote <= 50` is checked after the second await. A caller supplying a larger `quote` commits the second segment (state, transfer and discharge) and leaves the action permanently unable to complete. Local reference results neither detect nor prevent this.

## Dated addendum: six source-membership tests, 2026-10-15 08:16 UTC

Earlier statements that the separately scoped source-closure check was unperformed
refer to Run A/B time and remain historical evidence. Root subsequently executed
the exact Opus-approved unchanged std-only test by direct rustc --test, under its
recorded caps. The actual result is PASS_CUR03_MEMBERSHIP_SELECTION_ONLY: six tests
passed; all159 pinned files stayed unchanged; no SDK was compiled or executed.
These six tests are separate from the eleven reference tests, and provide no
additional staged outcome coverage. See ledger-source-membership/COMMANDS.json,
RESULT.json, full logs, pre/post pins and exact Opus effect review. The test binary
is omitted; its recorded digest is retained.

Declared native identity remains unqualified. No closure hash, crypto, SDK,
semantic, native or ledger qualification follows. This addendum is transcription
of the owner scope and actual receipt, not a new security judgment or merge-ready
claim. Separate UTXO integration conflicts and review boundaries remain open.
Original PUBLIC-ARTIFACT-PINS.json retains the first proposal's identities; the
additive PUBLICATION-REVISION-2-PINS.json records current proposal bytes.

### Exact attributed owner limitations

The following is the verbatim §6 body from Opus effect review
3fda33854be9d9a41c6f184ab5dbff0439b838b843d74023626fcc57825a0370.


If C1–C7 all hold, CUR03 is satisfied for membership selection by this unchanged test at the frozen tree. Specifically, the declared `source_closure` `include_str!` list is all of the following:
- the frozen 61-file baseline followed by the 8 Core stage files, 69 in total, distinct and existing;
- covering every `.rs` file under `mori-core/src`;
- covering every `.rs` file under `mori-lexer/src` and `mori-ledger/src`, except three exclusions that are reasoned, `#[cfg(test)]`-declared and named nowhere else.

It also confirms the C1 cleanup at the file-membership level: any leftover Cardano or export `.rs` file under `mori-core/src` would fail tests 3 and 4.

It does **not** establish any of the following:
- The closure hash value, or that it matches any `PublicDerivedCallKey` snapshot. The disclosed root `Cargo.lock` addition changes the hashed closure. Any previously derived snapshot would therefore refuse with `derived-source-identity`, and build identity stays unqualified.
- That the actual Rust build used these inputs. The closure is declared identity, not build authentication.
- Coverage outside the three walked roots. This includes:
  - new files under `crates/mori/src` (only listed CLI files are pinned)
  - `build.rs` at crate roots
  - non-`.rs` files pulled in by `include_*`
  - non-test `#[path]` modules outside `src`
  - feature or `cfg` selection
  - git and SDK dependency sources
- Any semantic or native qualification: SRS provenance or setup, SDK keygen correspondence, the source–constraint theorem, proofs, verification, Cardano export semantics, settlement.

The author's source report counted an SDK effect based on the assumed Cargo path. For this direct path only, that effect finding is superseded. Independent source-comparison stages do not waive any other required native qualification.
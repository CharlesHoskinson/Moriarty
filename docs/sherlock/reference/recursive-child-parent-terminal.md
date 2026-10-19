# Recursive child, parent and terminal interfaces

Source-reference cut: 2026-10-19. This describes concrete code, not an observed recursive proof. The reviewed Fenrir parent is `sherlock_link.rs`, 37,710 bytes, SHA-256 `b0443be3d61e986871f6960f4dd4dffd3fcc333ab7b687833be267bb9c917e6a`. Its fresh non-author Opus 5.5 opinion approves the MF01–MF06 leaf, with typed API abstentions and composition requirements. It does not approve execution. The Moriarty S1/S2 candidate is 49,991 bytes, `2061d947fa6206fcd89460c3d08e6391e622b16a425a5a75ce4a7dc6e504a312`, unreviewed and uncompiled; the manifest/genesis/provenance leaf is still pending composition. Exact repository/source locations and read regions are in [source pins](../../results/sherlock-recursive-handoff-interfaces-20261019/SOURCE-PINS.json).

## From one Machine to two child instances

Moriarty's `produce_linked_requests(program, capacities, initial_fuel, stack_limit, identity)` returns `(LinkedRequests, LinkShape)`. The S1/S2 source pre-admits `LiteralLayout` and `ReturnPublicationLayout` before calling `produce_a102_literal_return_publication`, then checks produced capacities, identity, program and intermediate image. `LinkedRequests` has `a102: LiteralTransitionRequest` and `a80: ReturnPublicationRequest`; the latter carries `capacities`, `identity`, `values`.

A102 uses `ChildKey`; A80 uses `ReturnChildKey`, `ReturnChildClaim` and `ReturnChildProof`. Each proof carries its claim, full `instance`, processed `vk_bytes` and `proof`. `prove_linked_children` produces `LinkedChildren { a102, a80 }`. `verify_linked_children` returns `LinkVerification` with private fields and read-only `shape()`, `a102()`, `a80()` accessors. The private S1 seal is source tooling for verified export of the exact requests/children/verification records; it is not an in-circuit authentication of source, Init or a parent theorem. None of these new A80 calls is claimed compiled or executed.

Both child instances have `n = P + 2S + 5` scalar lanes, where `P` is the complete program lane count and `S` the complete state-image lane count. `LinkShape::offsets() -> Result<LinkOffsets, Error>` checks arithmetic before indexing.

| Child instance index/range | Content |
| --- | --- |
| `0` | claim binding |
| `1..3` | two context limbs |
| `3..3+P` | every ProgramBlock lane |
| `3+P..3+P+S` | complete pre image |
| `3+P+S..3+P+2S` | complete post image |
| `3+P+2S` | rule tag |
| `4+P+2S` | charge |

The envelopes are A102 `(3,1)` then A80 `(0,0)`. Program lanes agree and `A102.post == A80.pre`. A80's two images remain Step-mode Constant-space images, not canonical StateBlocks. The consumer pins both processed-key SHA-256 digests, both context hashes and the shape independently; `link.json` is checked data, never key authority. The wire is `moriarty-sherlock-link/1`, with scalar encoding `bls12-381-scalar/32-byte-little-endian-lowercase-hex`: exactly 64 lowercase hex characters, no `0x`, for one canonical scalar.

## Fenrir byte boundary and files

`read_link_children(dir, &LinkPins)` returns two `SherlockChild` values in A102/A80 order. `LinkPins` fields are `a102_vk_sha256`, `a80_vk_sha256`, `a102_context_hash`, `a80_context_hash`, `program_lanes`, `state_lanes`. `SherlockChild::from_bytes` checks the pinned key digest, processed-key byte round trip, instance count and proof size. This byte boundary avoids linking Moriarty Rust types across the proofs 0.8.3/0.8.2 crate graphs; it does not demonstrate cryptographic/transcript compatibility.

The reader requires `link.json` plus `a102.vk`, `a102.proof`, `a102.instance.json`, and corresponding `a80.*` files. The manifest must name `a102.claim.json` and `a80.claim.json`, but this reader does **not** read those claim files or authenticate them. Its manifest checks cover format/encoding, shape/offsets and each child's name, fixed file names, key digest, context, instance count and envelope. Extra producer fields are not an authentication mechanism. S3/N2 manifest, genesis and provenance reconciliation is pending; the displayed source format is not a completed producer-consumer compatibility result.

`SherlockLinkRelation::from_pins(a102_vk, a80_vk, &pins)` checks pinned key bytes and constructs the relation. `SherlockLinkWitness::from_children(&a102, &a80, h_pre)` carries both complete instances/proofs and the predecessor height. `check_witness`, `statement` and `instance` operate on that pair. The circuit aliases A80 program/pre cells to A102 program/post cells; it prepares the A102 own accumulator first, then A80, accumulates once, resolves merged fixed bases and collapses. The shared `-G` has one checked value; other fixed-base names must be distinct. The review retains an `msm.rs` same-name merging/API scope limitation; the encoding excerpts used for this reference do not discharge that source-review limitation.

## Parent statement: exactly 16 fields

`FIELD_NAMES` and `link_statement` expose this zero-based order. `h_mid = h_pre + 1` participates in the trace but is not a separate public field; `h_post = h_pre + 2`, with heights below 2^64. Height binding to previous state is an anchor obligation.

| Index | Name | Meaning |
| --- | --- | --- |
| 0 | `keys_id` | Poseidon key/shape identity |
| 1 | `a102_ctx_lo` | A102 context low128 limb |
| 2 | `a102_ctx_hi` | A102 context high128 limb |
| 3 | `a80_ctx_lo` | A80 context low128 limb |
| 4 | `a80_ctx_hi` | A80 context high128 limb |
| 5 | `program_c` | Poseidon commitment to every ProgramBlock lane |
| 6 | `pre_root` | Poseidon root of complete A102 pre image |
| 7 | `h_pre` | predecessor anchored height |
| 8 | `mid_root` | Poseidon root of complete A102 post / A80 pre image |
| 9 | `post_root` | Poseidon root of complete A80 post image |
| 10 | `h_post` | h_pre + 2 |
| 11 | `bind_a102` | A102 binding lane |
| 12 | `bind_a80` | A80 binding lane |
| 13 | `n_steps` | 2 |
| 14 | `work` | A102 charge + A80 charge = 1 |
| 15 | `trace_c` | ordered step/height trace commitment |

The tags are packed ASCII domain constants: `SHK_KEYS`, `SHK_PROG`, `SHK_STATE`, `SHK_TRACE`, `SHK_STMT`. `keys_id` hashes the two VK SHA-256 limb pairs, child instance length, `P` and `S` under `SHK_KEYS`. `trace_c` hashes exactly this 21-field preimage:

```text
SHK_TRACE, keys_id, a102_ctx_lo, a102_ctx_hi, a80_ctx_lo, a80_ctx_hi,
program_c, pre_root, h_pre, bind_a102, 3, 1, mid_root, h_mid,
bind_a80, 0, 0, post_root, h_post, 2, 1
```

All trace inputs are public statement fields or envelope/step constants. The trace supplies no independent height binding beyond the statement; the anchor must enforce the predecessor root/height and next batch equation.

## Collapsed accumulator: the following 12 fields

`instance()` is `statement || acc_pis(acc)` and has 28 fields. `AssignedAccumulator::as_public_input` serializes lhs then rhs. Each resolved/collapsed side is one point followed by scalar 1, with no unresolved fixed-base tail. The pinned Blstrs point encoder emits two x-coordinate chunks, two y-coordinate chunks and the identity bit in that order.

| Parent index | Field | Role |
| --- | --- | --- |
| 16 | `lhs.x[0]` | pinned point/coordinate encoding |
| 17 | `lhs.x[1]` | pinned point/coordinate encoding |
| 18 | `lhs.y[0]` | pinned point/coordinate encoding |
| 19 | `lhs.y[1]` | pinned point/coordinate encoding |
| 20 | `lhs.is_identity` | pinned point/coordinate encoding |
| 21 | `lhs.scalar` | `1` for a resolved/collapsed side |
| 22 | `rhs.x[0]` | pinned point/coordinate encoding |
| 23 | `rhs.x[1]` | pinned point/coordinate encoding |
| 24 | `rhs.y[0]` | pinned point/coordinate encoding |
| 25 | `rhs.y[1]` | pinned point/coordinate encoding |
| 26 | `rhs.is_identity` | pinned point/coordinate encoding |
| 27 | `rhs.scalar` | `1` for a resolved/collapsed side |

`x[i]`/`y[i]` mean the exact SDK `AssignedField::as_public_input` chunks, not raw 128-bit coordinates. The pinned foreign-field encoder encodes `coordinate - 1` into base limbs and packs ordered limb batches; the point encoder appends `is_identity`. Do not substitute a conventional affine-coordinate serialization. `reconstruct_acc` checks the 12-field shape, point decoding and scalar 1 for each side. A collapsed accumulator is a deferred verification obligation, not a recursive-proof acceptance result.

## Terminal decider and anchor handoff

`keygen_link_parent(params, s_g2, &rel, key_dir)` writes `link.vk` and `link.relation`; `load_link_parent_key` checks a separate parent VK digest and exact expected relation bytes. `prove_link_parent(params, s_g2, &rel, &vk, &w)` performs the source's child verification/folded pairing/prove/self-verify route and returns `LinkParentProof { vk, instance, proof }`. Its source calls `ChaCha20Rng::from_entropy`; deterministic inputs do not promise deterministic proof bytes. Actual degrees, costs and outputs remain unmeasured for this leaf.

`write_w_dir(dir, &parent, source)` writes `w.vk`, `w.proof`, `w.instance.json` and `SOURCE`. The new parent writer requires28 fields. The existing `main.rs` path still asserts the old 17 statement fields plus 12 accumulator fields (29 total); its unchanged command is not a usable Sherlock 28-field route.

`sherlock_decider_relation(parent_vk, s_g2, template)` builds `DeciderRelation` with `nr_cols=4`, `n_w_instance=28`, `dom_stmt=SHK_STMT`, and `stmt_fields=0..16`. `keygen_sherlock_decider(params, &rel, key_dir)` writes `decider.vk` and `decider.relation`. `DeciderWitness` carries `w_instance`, `w_proof`, `instance_id`, `batch_no`. The terminal folds the parent proof's own accumulator with its carried accumulator, reassigns the points with subgroup checks, and constrains the residue pairing. These are source operations, not observed checks.

Its statement digest is the pinned Poseidon of 19 fields:

```text
SHK_STMT, instance_id, batch_no, FIELD_NAMES[0], ..., FIELD_NAMES[15]
```

The terminal's public vector is **Template order**, not those 19 fields directly. `Template { slots }` uses `BindingInput`, `CommunicationCommitment`, `Const(F)`, `Stmt`; validation requires the first two slots and exactly one `Stmt`. The Sherlock helper refuses more than 1,023 public slots. The ledger-9 `acceptValidity(stmt)` transcript must be captured from the exact compiled anchor; its statement-slot index and constants cannot be invented.

A historical 2,618-byte `sherlock_anchor.compact` proposal exists. It declares `instance_id`, key/context/program pins, `s_root`, 64-bit `height`, 32-bit `batch_no`, `validity_ok` and `settled`; `acceptValidity` inserts a statement and `settleValidity` reconstructs the 19-field statement before changing state. It is uncompiled historical source, **not** a current reviewed integrated anchor or captured Template. The parent review saw no matching anchor/Template body. Its state/height/replay equation, key replacement, Compact/CPU/in-circuit Poseidon parity and target integration remain open. `genesis_root` is a host candidate over pinned A102 pre lanes; it proves no Init equations and does not authorize an anchor by itself.

## Remaining connected work

Compose/review the A80 manifest/genesis/provenance leaf and remaining tests with S1/S2; typecheck/register the parent and discharge the exact typed API abstentions; connect a Sherlock 28-field CLI with `s_g2` threaded through parent and decider keygen/prove; finish and review the anchor and capture its Template. Cross-graph 0.8.3→0.8.2 VK/transcript compatibility, same SRS [tau]2, source/claim authentication, Init, projectA, StepInv/F3–F5/G4 and full 39-row dispatch remain separately open. No public API requires a Moriarty maintainer's permission; the recorded reviews/operation conditions govern this project's own checked work.

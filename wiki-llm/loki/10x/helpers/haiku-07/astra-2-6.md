# astra-2-6: Native Sherlock fusion feasibility (A102 -> A80)

Scope: read-only audit of crates/mori-backend/src (no repo writes, no builds). Compile, keygen, prove and verify are UNOBSERVED in these files.

## Question
Could one stock-gate circuit directly constrain both steps (A102 Eval(Literal)->Return, then A80 Return->StepDone) and the terminal ledger binding, removing all child/parent verification while preserving the fixed A102->A80 statement? Which shared-assignment hooks exist, what witnesses are available versus the artifact-only child interface, what k/row bounds are recorded, what the 16-tx generalization excludes, and the relative engineering effort.

## Answer
- **Partial, at IR level only.** The two fragments are composable: `emit_return_publication_step` takes `pre: LiteralImage` as an input (`sherlock_machine/return_publication_step.rs:28-35`, fn at :226), and its doc says it is meant to run "immediately after A102 `emit_literal_step` in Step mode" (:1-25). So A102.post and A80.pre can be the same operands. That makes `a102.post == a80.pre` structural rather than a host equality (`sherlock_link.rs:76-81`, LINK_EQUALITIES).
- **Not yet a faithful fused step.** Both fragments constrain only a subset:
  - A102: scope is "exactly one charged Eval(Literal)->Return transition, not other rules, continuity or program finish"; projectA, StepInv, F4, G4 and literal uiValid remain open (`literal_bridge.rs:1-12`).
  - A80: "the circuit enforces only the line-80 fragment under a forced enable; full StepInv/O1-O4, F4, OBL.G4 and the other 38 dispatch rows remain open" (`sherlock_link.rs:83-88`).
  A fused circuit would therefore prove less than the full interpreter step. Closing those obligations is the bulk of the work, not the fusion itself.
- **Terminal ledger binding is not yet fused.** `LocalPublic` mode exposes post and envelope as public outputs (`literal_bridge.rs:157-168`). `LedgerCall` mode uses a communications commitment (`:164`, `CircuitMode` at :31-36). Fusion must choose one binding path; no fused mode exists.
- **Removing child verification is valid only if one custodian holds both steps.** The A102 child witness is built only from public instance lanes: `child_witness` assigns every `PublicInput` lane and leaves all other cells to the circuit (`sherlock_child.rs:244-311`, header :1-30). The A80 witness is the same carrier after `check_child_statement` (`sherlock_link.rs:1-8`). The child interface is therefore proof bytes plus full public statement lanes, and no trace is carried. In a fused circuit the post image would be an internal operand, which requires it as a PrivateInput. That input is available only to the same single `Machine` observation (`sherlock_link.rs:1-8`, H1 producer-asserted). The raw post image is not part of the child artifact interface.
- **Witness availability (summary).** Fused proving needs the full A102 pre, post, program and envelope, from the same custodian that ran both steps. Artifact-only children do not expose the post as private. The current stock LocalPublic path cannot hide it.
- **Row/k bounds: none recorded.** `budgets.json` a2-linked-children has `k: null`, wall/RSS null (`qa/sherlock/budgets.json`). `sherlock_link.rs:89-95` lists "k, rows, proof sizes, times and memory are unmeasured". Only the Fenrir fixed parent has k20 (6,432 bytes; `docs/sherlock/reference/recursion-evidence-2026-10-22.md:7`).
- **k cap conflict.** `MAX_K = 19` (`crates/mori-backend/src/proving.rs:22`). Both child and link producers refuse `options.max_k > MAX_K` (`sherlock_child.rs:135`, `sherlock_link.rs:567`) and `circuit_k > options.max_k` (`sherlock_child.rs:177`, `sherlock_link.rs:624`). A fused circuit whose k exceeds 19 is refused by this crate as written; the cap must be raised and measured.
- **16-tx generalization exclusions.** One fused unit covers exactly one charged A102 transition plus one uncharged A80 line-80 terminal publication. It does not cover a chain of transitions, continuity, program finish, other dispatch rows, or ledger-acceptance of more than one step (`literal_bridge.rs:1-12`, `sherlock_link.rs:83-95`). Sixteen transactions need recursion or a chunk chain between fused units.
- **Engineering effort (qualitative, not measured).** Low to moderate for the IR composition alone (reuse existing fragment signatures). High for closing the open obligations (StepInv O1-O4, F4, OBL.G4, dispatch rows), for a terminal ledger-call mode, for private-post witness handling, and for raising the k cap and measuring k/rows. Recursion between chunks remains necessary.

## Hooks (exact)
| Hook | Location | Role |
|---|---|---|
| A102 statement + enable + fragment | `crates/mori-backend/src/sherlock_machine/literal_bridge.rs:69-168` (`LiteralLayout::emit`) | statement operands, forced enable, `emit_literal_step` at :132, Output at :159 |
| A80 statement + enable + fragment | `crates/mori-backend/src/sherlock_link.rs:390-` (`ReturnPublicationLayout::emit`), `ENABLE_ID` :97, `emit_return_publication_step(` :444 | own statement prefix (`STATEMENT_PREFIX`, sherlock_link.rs:96), take_image of pre/post |
| Shared pre/post carrier | `sherlock_machine/literal_step.rs:12-15` (`LiteralImage`), `return_publication_step.rs:28-35` | composable operand images |
| Child witness (public-only) | `crates/mori-backend/src/sherlock_child.rs:244-311` | public lanes only |
| Host equality list | `sherlock_link.rs:76-81` | a102.post == a80.pre is host tooling today |
| k cap | `proving.rs:22`, `sherlock_child.rs:135-183`, `sherlock_link.rs:567-624` | MAX_K = 19 |

## Raw numbers table
| Quantity | Value | Source | Tag |
|---|---|---|---|
| MAX_K (native producer cap) | 19 | proving.rs:22 | CODE |
| MAX_PROOF_BYTES | 262,144 | proving.rs:21 | CODE |
| A102 child k | not recorded | budgets.json a2-linked-children k=null | UNOBSERVED |
| A80 child k | not recorded | sherlock_link.rs:89-95 | UNOBSERVED |
| Fixed parent k | 20; 6,432 bytes | recursion-evidence-2026-10-22.md:7 | CODE (cited) |
| Statement lane cap | 2^VERIFIER_MAX_DEGREE - 1 | literal_bridge.rs:39-41 | CODE (VERIFIER_MAX_DEGREE value not located locally) |

## Caveats
- Nothing was compiled or proved, so no fused k, row count, time or memory is established.
- The fusion mechanism is at the IR-composition level. Whether the pinned upstream accepts a single IR with two composed fragments, and whether its keygen and verifier resources fit, is UNOBSERVED.
- Host-side Machine observation is producer-asserted (H1, `sherlock_link.rs:67`). A fused circuit does not remove that trust: the genesis anchor and Init equations stay host subset checks (`sherlock_link.rs:89-95`).
- Sources: all Moriarty-factory paths above, read directly. The only external datum is the recursion-evidence doc line 7.

## Addendum: measured child k (verified by haiku-07)
- CODE: `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json` (children array): a102 `circuit_k` 13, `proof_bytes` 3440, `instance_lanes` 253; a80 `circuit_k` 12, `proof_bytes` 3440, `instance_lanes` 253. Program lanes 86, state lanes 81. Phases: a102-prove 2,098 ms, a80-prove 1,156 ms, verify-linked-children 3,436 ms (debug/unoptimized build, scope caveat per astra-2).
- Consequence: the earlier "row/k bounds: none recorded" line is superseded for the fixed two-step fixture. A fused single-relation k is UNMEASURED; astra-2's k14-k15 target is ESTIMATE. The 253-lane public statement is what a fused relation would shrink by making the A102 post internal.

## Addendum: binding preservation (for fused IR)
- Child binding is a host value, not a circuit-computed hash: `child_binding` = SHA-256 of the domain-separated claim (`crate::digest((CHILD_BINDING_VERSION, claim))`), split low128 + 2^128 * high128 in Fr (`sherlock_child.rs:196-201`). It becomes instance lane 0 (`sherlock_child.rs:183-186`). `return_binding` mirrors it under `RETURN_CHILD_BINDING_VERSION` (`sherlock_link.rs:648-652`).
- Binding is consumed as `binding_input` by the SDK (`sherlock_child.rs` witness: `Preprocessed.binding_input`, child_witness). Neither child recomputes it in circuit (`sherlock_link.rs` LINK_OPEN_OBLIGATIONS line 90).
- Fused preservation: a single relation needs one binding over a combined claim that names both contexts (A102 and A80 domains, program, A102.post == A80.pre). Using a new domain string keeps the two child bindings distinct. Whether the pinned SDK accepts a combined binding for a single IR is UNOBSERVED.
- The claim must still commit to the 253-lane statement or an equivalent commitment; host-side SHA-256 claim commitments are not circuit-checked today (LINK_OPEN_OBLIGATIONS line 90).

## Addendum: IR public-input exposure (pinned midnight-ledger 04c9c5d)
- Checked `~/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/zkir-v3/src/ir.rs`: no "Embedded" mode (grep count 0). `IrSource` has only `inputs`, `outputs`, `do_communications_commitment` and `instructions` (ir.rs ~:36-50).
- `PublicInput` (ir.rs:663-683) consumes the public transcript output; `PrivateInput` (ir.rs:900) is the prover-supplied free value. So an internal A102.post can be a `PrivateInput`, and only terminal slots need `PublicInput`. This matches astra-2's point that `IrSource::circuit` exposure cannot simply compose unchanged.
- Composition is therefore: concatenate the two emitter bodies in one `IrSource`, reuse the A102 post operands as the A80 pre (both use the same `LiteralImage` shape, `return_publication_step.rs:28-35`), with post operands as `PrivateInput`, and terminal slots as `PublicInput` with a fused binding. This is an engineering sketch; compile and SDK acceptance are UNOBSERVED.
- Witness: the child carrier (`child_witness`, sherlock_child.rs:244) fills only public lanes. A fused private post needs a new carrier that also fills private inputs from the same custodian's Machine observation. This is the custody change astra-2 flagged.

# astra-2 early draft corrections

Read the chair's in-progress `design.md` and `proposal.md` on 2026-11-07 at approximately 21:04Z. These comments precede the formal DRAFT broadcast.

## 1. One-hot composition must not conjoin inactive identities on the shared post-state

`design.md:2.2` says `post == pre` under `1 - sel[r]`. If each inactive fragment applies that condition to the shared post-state, it prevents another selected fragment from updating those lanes. Existing `c6_step_kit.rs:739` makes disabled identity explicit.

Use per-rule candidate outputs with one shared selection, or preserve a lane only when the selected rule does not write it. For candidate outputs, an inactive fragment's identity applies to its private candidate, not the selected shared post-state. Add a test with two available rules where one changes a lane that the other preserves. The universal leaf must accept the valid transition. Also test that no selected rule leaves a changed lane unconstrained.

## 2. Define who constrains each leaf lane

`design.md:3.2(3)` must not imply that `h_pre` equals the selector digest or enabled count. The leaf derives `rules_d` and `steps`. The node binds `h_pre` to its previous bounded height and checks the bounded sum. If the leaf also carries an absolute height in machine state, explicitly equate those representations. Otherwise state the node's ownership of this premise.

Source: `crates/mori-recursion/src/ivc.rs:191` and `:308`. An unconstrained semantic label in a standalone leaf is not automatically an end-to-end exploit when the composed node enforces the required condition.

## 3. Match genesis tests to the chosen semantics

Scaling the carry to zero at genesis removes its obligation. It does not necessarily reject every well-formed nonneutral carry supplied in that inactive branch. If the design requires refusal, also constrain the supplied genesis carry to the canonical neutral encoding. Otherwise test that the ignored carry cannot affect acceptance or the accumulated obligation. Keep malformed curve/encoding tests separate.

Source: `crates/mori-recursion/src/ivc.rs:315–368`. No cancellation exploit was demonstrated by this review.

## 4. Bound the Preview exit

`proposal.md:Exit of this change` allows every requirement to have a passing or explicitly open predicate. Require every mandatory security and assurance gate applicable to the claimed Preview profile to pass. Open successor semantics and conditional Mainnet work remain explicit, but cannot waive a required gate for that profile.

Source: UISEC-010/012, ZR16 and the council's `SND-001–007` proposals.

## 5. Preserve live recovery semantics

`design.md:5.2(5)` says an unsafe key means redeploy and retire. Define retirement as an observed on-chain state or explicit supported recovery policy. A new address or client routing change does not disable an old verifier. Preserve live duties and consumed identities. Empty disposable test deployments can use a narrower replacement procedure.

Source: ZR10/ZR15 in `docs/MORIARTY-BACKEND-REQUIREMENTS.md`.

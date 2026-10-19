# Normal report-wire proposal: source and evidence note

Author: GPT-6.1 Sol high, nonsecurity source proposal. Date: 2026-10-18. Proposal only; no adoption, design vote, independent-oracle qualification or actual behavioral result.

Only files in this outside-Git proposal directory were written. Source checkout inspected read-only: `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`; root-assigned current branch `doctorwatson-language/cli-native-feature-20261018`, assigned base `2c2b9182` and retained report tree `61bc8233eb3c0ce421ac5a80e4112eed952925a6`. Git was not queried or modified. Exact inspected file identities below are the actual byte pins for this note; no uncommitted tree identity is invented.

Develop skill/AGENTS/product contract were restored and read-only development CLI status exited0, retaining historical SP01 stale binding/candidate/accounting/resource blockers. No Cargo/rustc/rustfmt, Mori/parser/evaluator/model/SDK/VM, proof/key/wallet, private/cache/configuration/provider or network execution occurred. Existing public Rust types/parser/normal runtime/exporter source was read to ground the proposal. No pending custodian files or newly authored security tests were read. No expected JSON golden was generated, exported, executed or frozen.

## Source observations — not normative approval or run results

- `ir.rs:254` Step declares index, statement, Option<Value>, snapshot and effects. There is no skip_serializing_if on value. Value has type/content tags and UInt serializes as a decimal string. `ir.rs:263` Execution declares the complete execution members. The frozen v1 decode structures retain their own contracts; this proposal does not alter decoding.
- `runtime.rs:719` Let evaluates/captures a value and returns Some(v); Next returns Some(after), accepted predicates return Some(true), and Emit returns Some(v). The proposed Let rule agrees with inspected code but is presented as an explicit design choice, not as an independently established normative requirement.
- `runtime.rs:875..919` normal simulate starts a fresh effects vector per statement, appends it to all_effects, stores the fresh delta in Step.effects and stores post-statement snapshot. This corrects any informal reading of Step.effects as cumulative. The proposed cumulative prefix is derived, not a new wire field.
- `parser.rs:79` state initializers are direct literals. `parser.rs:258` assigns ExprId by expression-array insertion; expr creates child nodes before binary parents. The Counter illustration manually transcribes that public convention. It supplies no independent parser-correspondence result.
- `README.md` documents let capture/immediate next, the existing logical profile and Counter work16. `logical_cost.rs` wraps the same execution schema with CostReport. `runtime.rs:395` creates a fresh evaluation cache and charges uncached node visits. The proposed work table is ordinary arithmetic under that published schedule, not an observed execution.
- `simulate.rs:120/139` has the accepted metered and ordinary public envelope; `core_io.rs:247` exports then println adds final newline. Exporter source is `json.rs:117`. No product exporter was called to construct expected bytes here.
- `core_test_report.rs:109` contains the existing full normal Transfer literal oracle; its recorded freeze is preserved. Its Step values/effects/snapshots are consistency references, not proof of a Let rule or effect-prefix interpretation. `logical_meter.rs` contains the Counter source; its shared-evaluator equality is not treated as independent full normal-report evidence.
- Existing UNI-020 requires independently derived frozen observations; UNI-021 separates evidence/acceptance; UNI-024 retains the full report/public journey obligations. This proposal closes none.

## Design decision for root review

Recommended proposal: accepted Let reports Some(captured typed RHS); all five scoped normal completed kinds report a typed result; Step.effects remains a delta and Execution.effects is its ordered concatenation. The primary reason is a useful unambiguous per-statement observation that distinguishes expression reference from evaluated capture, while preserving current public shapes. This recommendation is not adopted. A different Let.value choice would need an explicit wire/version/compatibility decision before a new oracle is frozen; do not silently fit expectations to existing implementation or pending custodian output.

## Revision 2 — explicit zero-statement normal boundary

This revision remains PROPOSED / UNADOPTED / UNRUN. Root identified the missing N=0 boundary in the first draft; the three outside-Git proposal documents are updated together. The selected design includes otherwise valid empty actions rather than constraining N>=1: steps=[], post=pre, effects=[], with normal semantics/action identity retained. The final-snapshot clause is conditional on N>0, and the cumulative-effect definition explicitly uses its empty P[-1] identity at N=0. No Step or Step.value exists for an empty body.

Source-static check: `parser.rs:135..160` initializes an empty statements vector, may consume the closing brace without a statement iteration, and records that action. `runtime.rs:217..233` checks upper input/statement bounds and iterates statements without a nonempty-body condition. `runtime.rs:880..918` clones normalized pre into current, initializes empty Steps/effects, iterates zero times for an empty body, and returns current as post. `ir.rs:263` represents Steps/effects as Vec fields. These source facts support the proposed boundary; they are not an observed execution or an independent oracle. Read-only development CLI status was refreshed and retained the same historical blockers.

No runtime, tests, expected-golden generation, checkout/Git changes, security case/judgment or native operation occurred for this revision. Existing unobserved contract acceptance remains unobserved; separately reported CLI checks do not close this proposal's independent predicates.

First-draft document identities retained for the root's already-copied proposed evidence:

- `PROPOSED-CONTRACT.md` first-draft SHA256: `c5eccede7fb1a249c3d5108816bf0e7157af741ca31cf58a2f26b9289bc31bab`.
- `OPENSPEC-DELTA.md` first-draft SHA256: `8de857ad20a5567f8e98f6038282584d872c99272658ddd4515fb938b5e91a7d`.
- `SOURCE-NOTE.md` first-draft SHA256: `4019c4240ab9597487d500ec1378f205f7857867a04fde99a645e3e46a8144fd`.

Refreshed empty-action source identities:

- `crates/mori-core/src/parser.rs` SHA256 `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275`; unchanged from the first source note.
- `crates/mori-core/src/runtime.rs` SHA256 `eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930`; unchanged from the first source note.
- `crates/mori-core/src/ir.rs` SHA256 `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d`; unchanged from the first source note.

## Inspected public source identities

| Source path | SHA256 |
|---|---|
| `crates/mori-core/src/ir.rs` | `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d` |
| `crates/mori-core/src/parser.rs` | `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275` |
| `crates/mori-core/src/runtime.rs` | `eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930` |
| `crates/mori-core/src/json.rs` | `faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183` |
| `crates/mori-core/src/logical_cost.rs` | `8c1b38da8cbefb5f3bb83e89fc1fb471410e3886fc50b74e2af8c0d9dc9636cc` |
| `crates/mori-core/README.md` | `0d91facda4bcf06e545af2f9ced7c72b2b76220a9b53eb46a5a4caea66ebc4dc` |
| `crates/mori/src/commands/simulate.rs` | `53b1c0cb0bbaa94cf890890c37e1a282535e5e8bbcf29dafaaef498faed9ac11` |
| `crates/mori/src/commands/core_io.rs` | `6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594` |
| `crates/mori/tests/logical_meter.rs` | `c4c6de685036d8c9f36ccc01910a9f5b1c4948e70f0193ec942b1fbd65baa129` |
| `crates/mori/tests/core_test_report.rs` | `74e021ed57129647354f3ec226d5d3acdee314091d9ed77870b67c3b76ac2387` |
| `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md` | `66f85468058830690bd8e0d21e4f76a291dd99c572f1f5224cbf4b49b8de67b1` |

All full original scopes/exits and prior evidence remain retained. Source observations here are not compiler/test success, independent semantic correctness, native cost/correspondence or Midnight target qualification.

Revision2 companion document identities:

- `PROPOSED-CONTRACT.md` SHA256 `e858fa259e01c8cdaafb2e139c2b92f076c70789e87843463ce9aaf26caaa224`.
- `OPENSPEC-DELTA.md` SHA256 `f6bca6929a360bf13fd4eb1e10a711f830fef3e19e13f241cb5ed7781226e456`.

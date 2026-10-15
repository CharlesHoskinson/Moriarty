# Sol design: source provenance in the existing mapped-role static artifact

Status: read-only nonsecurity design, specified-only. Author/model: GPT-6.1 Sol high. Checkout: `/home/charl/Moriarty/.worktrees/core-source-provenance-20261015`, inspected HEAD `eb2577d6c67684b950288d769ea2fcb91e731bf1`. Installed develop skill, current AGENTS, product contract and execution-focus were restored; plugin `status --json` was run and retains historical SP01 blockers. Those blockers are not this task. No source/doc edits, builds, product execution, native operation, private-material access, index changes or commits occurred.

**VOTE: AGREE** on the exact mapped-role-only producer metadata design below: per-source-statement step/root-expression/instruction/group ranges plus actual per-group instruction and `replacement_fields`, without changing shared Group or ordinary/signed schemas or emitted IR. This is one substantive nonsecurity scoped vote. The second required agreeing vote and Opus compatibility/integrity/resource/command constraints are pending. Do not implement before root GO.

## Capability and limits of the claim

The existing public `compile-call --all-actions --authorization unsigned-mapped-roles` exporter should publish a direct map from each actual source statement/root expression to its actual emitted instruction/group ranges and template-replacement field positions. A reader can then locate the transfer, fee, state, duty and event fields in the existing tagged IR without reconstructing the compiler or running native preparation.

No missing Statement/operator arm or incorrect arithmetic/financial semantics has been established. Full source, ABI actions and IR already exist in the bundle. The missing capability is explicit producer provenance between those representations. The map supports U0's field/consumer contract and U2's admitted-grammar correspondence work under UNI-018/019/020; it does not establish runtime semantic equivalence, native proof, ledger effects or any parent exit. Different lowerers continue to describe different statements.

## Exact producer and consumer closure

Public Rust startup: `main.rs:16–20` parses `Cli` and constructs `Shell`; `commands/mod.rs:53` dispatches CompileCall. `commands/compile_call.rs:49–52` selects `run_static` only when `all_actions` is set. `run_static` reads explicit work limits, source and bindings and parses Core at201. The mapped-role mode has no signer policy. Export calls `static_compile::compile` at226, then publishes existing bundle leaves at234. Reopen calls `static_compile::reopen` at203.

`static_compile.rs:363–379` validates existing inputs/Core and selects `compile_mapped_roles`. At328 this obtains the private `layout::ValidatedSourceStaticBindings` via `layout::validate_source_static_roles` (layout.rs:692). That validator reparses the supplied source, compares source/program/ABI identities, and constructs the mapped-role ABI through `admit_bindings_inner` (layout.rs:151–290). `templates_hash` (layout.rs:124) hashes included `ops.rs` text; no templates are executed here.

`static_compile.rs:332–348` iterates real source actions in order, calls `relation::compile_source_static_roles` (relation/mod.rs:253), and serializes its IR/manifest. The wrapper calls `relation/compiler.rs:624`; it runs `compile_inner` with no signed bindings. The common loop at737 sets `statement_index` before `statement`; `statement` derives the actual root ExprId at502–511, reads required state names, emits the expression, then emits its operation/effect. `relation/arithmetic.rs:190–210` is the single `Builder::op` emission point: it applies the actual template replacement list and appends one `Group` and one `Instruction::Impact`.

Relevant helper calls are existing source-static arithmetic, `ops` template construction and `relation/event.rs` construction of event payload branches. `event.rs:15–18` calls `serialize_to_node_list_bounded` on public payload objects to determine emitted event-domain branches; this is an explicit source-call fact for Opus command classification, not a new execution judgment. This path does not select `relation::check`, `process`, preparation or proving entry points. Source inspection is not permission to execute any of those.

`relation/mod.rs:217–229` defines the mapped-role-only `SourceStaticManifest`. The ordinary/signed `Manifest` is a different type at28–43; shared `Group` at22–25 contains only field_count/origin. The mapped manifest's debit_sources is a set of roles, not a per-step map. Its existing `compiler_hash` calls `proving::source_closure` (proving.rs:150), which hashes included public source text.

`static_compile.rs:401–455` retains `source.mori`, ordered `action-NNNNNN.ir.bin` and `action-NNNNNN.manifest.json`, then emits `static-compile.json` with their digests. `SourceStaticArtifact::manifest_bytes` uses existing `call_artifacts::json_bytes` (call_artifacts.rs:433); IR uses `codec::encode` (codec.rs:8), a tagged serializer. CLI `publish_static` at248–269 writes source/action leaves before the marker. Reopen checks the supplied public inputs/marker, acquires leaves and recompiles at612; exact complete manifest/leaf equality at613–618 naturally includes new provenance bytes, then tagged IR is decoded for serialization checks at622–624. This is the existing real consumer; no new command or artifact leaf is needed.

The parallel `mori compile` command remains implemented (`commands/compile.rs:19–35` -> `mori_backend::compile/compile_signed` -> `lower_action` -> `.zkir` plus artifact.json). Its effect_frame already has step/kind/identity/public-position descriptors (`mori-backend/lib.rs:153–188`). It is neither replaced nor treated as an equivalent native-call statement.

## Exact proposed emitted shape

Add exactly one required field to the **mapped-role-only** action manifest: `source_provenance`. Do not change the shared Group fields, ordinary Manifest fields, SignedManifest fields, ABI, CLI flags, authorization selection or existing format/qualification strings. The proposed internal Serialize-only records use existing vector indexing conventions (`usize` for instruction/group/field indices, source step/root expression as `u16`). Opus must decide any compatibility/resource details before implementation.

```json
"source_provenance": {
  "statements": [
    {
      "step": 0,
      "root_expression": 2,
      "instructions": {"start": 8, "end": 15},
      "groups": {"start": 3, "end": 3}
    }
  ],
  "groups": [
    {"group": 0, "instruction": 3, "replacement_fields": []}
  ]
}
```

The numbers above illustrate field meaning only and are **not** an oracle or proposed test expectation.

Field meanings:

- `statements` has one ordered row for every admitted source statement, including pure statements that emit zero groups. `step` addresses `abi.actions[action].statements[step]`; the source Statement there supplies kind and exact source asset/from/to/duty/event identities. Do not duplicate those strings or invent a second financial DTO.
- `root_expression` is the ExprId actually passed to expression emission in `Compiler::statement`, not a second reimplementation of the Statement match. This is the parsed Core source expression-arena node ID, not a source span or a recursively expanded dependency map. The complete expression arena can be read from the existing source/program; the field does not claim an instruction or individual provenance row for every recursively evaluated expression node.
- Every instruction/group range is half-open, zero-based, local to that action IR: `[start,end)`. Record start immediately before processing the source statement and end immediately after successful processing. It includes that statement's state reads, arithmetic and operation templates. Adjacent statement ranges follow emission order. A zero-group statement has equal group endpoints. Prologue/epilogue instructions and ABI-root groups are not assigned a fictitious source step; their positions remain visible outside the statement ranges.
- The provenance `groups` vector has one row in emitted group order for **every** existing Group, including ABI-root groups. `group` indexes the existing manifest groups vector. `instruction` indexes its actual `Instruction::Impact` in the IR. A reader retrieves the actual guard and input operands from that instruction; no operand rendering/stringification is added to JSON.
- `replacement_fields` is the actual existing `Builder::op(replacements)` index list, in original call order, zero-based within that Impact's `inputs`. These are template slots replaced by the source/compiler operand, including when a source literal lowers to an Immediate. It is deliberately not inferred by scanning Variable operands. It does **not** assert that each replacement is an independent public input or that the fixed template fields are all named source data. Existing origin and field_count remain in Group. A reader derives each flattened field position from its group index and the prefix sum of existing group widths; no second copied public-transcript ABI is introduced.

Why this is the smallest useful shape: statement ranges locate all existing admitted constructs, group rows locate actual serialized fields, and root IDs connect to source expressions. Only numeric indices/offsets are added. Kind/identity, group width/origin and IR operands stay in existing authoritative representations. This is provenance, not an evaluator, instruction interpreter, semantic certificate or new audit register.

## Producer implementation outline, still no code edits

1. In `relation/mod.rs`, define private Serialize-only SourceProvenance/SourceStatement/SourceGroup/IndexRange structs and add source_provenance only to SourceStaticManifest. Existing Type/Action/Statement DTOs and shared Group stay untouched.
2. In `relation/arithmetic.rs`, add an **optional private** provenance vector to Builder, default None. At the actual op emission site, when enabled, append `{group: groups.len(), instruction: instructions.len(), replacement_fields: replacements.iter().map(index)}` immediately around the existing Group/Impact pushes. Do not alter the old Group, replacement loop, instruction, serial counter or guard. The collector stores indices only.
3. Enable capture only in `compiler::compile_source_static_roles`. Ordinary compile and signed compile keep capture disabled and retain their current public return types. The private compile_inner can receive an explicit capture boolean and return one additional optional provenance result; its three wrappers adapt the private tuple while ordinary/signed callers keep their current shapes. Avoid a general emitter refactor.
4. Change private `Compiler::statement` to return the root ExprId it already selected and emitted, rather than `()`. The sole calling loop snapshots instruction/group lengths, invokes it, then records that returned ExprId and final lengths when capture is enabled. No second Statement-to-expression match is introduced. Source step is the existing loop index.
5. The mapped wrapper returns the nonoptional collected map with the existing IR/groups/input_count and inserts it into SourceStaticManifest. The existing bundle/digest/publication/reopen path serializes and consumes it automatically; no static_compile.rs or CLI production change is currently needed.

These edits change source-closure/manifest digests through the **existing** compiler_hash mechanism, including source-closure hashes appearing elsewhere. Therefore do not promise all ordinary/signed artifact bytes remain identical. The design preserves their format/field sets and generated IR semantics; declared compiler/build hashes naturally refresh. Old bundle reopen compatibility is specifically for Opus/root to decide. Do not retain stale source identities or change the hash algorithm to conceal metadata source edits.

## Alternatives considered

1. **Recommended: mapped-only numeric provenance from actual emission.** Localized capture, no shared Group or signed/ordinary format change, consumed by existing manifests and reopen.
2. Add source fields directly to shared Group. Fewer plumbing changes, but unnecessarily changes ordinary and signed serializations and all their consumers. Not recommended for this child.
3. Derive a map later from source plus emitted IR, or infer replacements from Variable operands. This duplicates lowering knowledge, misses source-literal replacement slots and provides weaker evidence about the producer. Not recommended.

## Normal functional tests and independent expectations

The normal test source and expected values must be frozen by an independent nonauthor before any approved execution. Do not produce expected maps by importing the compiler or scanning its output and blessing that result. Compiler output is the actual side of a structural comparison; literal expectations and small financial arithmetic are the independent side.

Extend the existing public `mapped_role_static_io.rs` harness with one **exactly filtered normal test** that exports and cold-reopens the independent oracle's five-step specimen selected by root: a literal Transfer2; a Fee using input7 to the same treasury; a Let computing base3+1; a Transfer of that captured value4; and a Fee2 from the second mapped payer. Exact source identities, ExprIds, ranges and positions come from the frozen nonauthor oracle. The Let has no operation groups and must still retain its root ID and instruction range. These are ordinary financial and syntax combinations, not hostile inputs. The source collector covers every admitted Statement generically, but this initial regression claims only the frozen five-step scope. State updates, positional predicates, Bool/UInt emits, duties and additional actions are future coverage and require their own independent freezes before adding tests; they are not initial required expansion.

Required assertions against the freeze and actual public artifact:

- Exact five step and root-expression rows in original source order, including the final statement and the Let's empty group range; exact kind/asset/from/to reconstructed from the already-emitted ABI action.
- Exact literal expected instruction/group ranges for a hand-sized specimen; each group row addresses the actual Impact instruction and its original field count/origin. Each listed replacement offset locates the frozen expected source-derived operand position. Actual IR may be structurally decoded for this comparison; no SDK relation evaluation, interpreter/VM or proof is called.
- Keep an independently captured **base** IR byte/hash baseline for fixed public inputs before producer implementation. New tagged IR must match it, while manifests acquire provenance and fresh source-closure identities. The base-byte comparison is a compatibility observation, not an independent semantic oracle.
- Normal hand-derived reference finances retain complete ordered effects, gross debit, fees/net receipts, preserved unrelated balances/state and opening/accrued/discharged/closing duties. If reference execution is selected, freeze those values first and obtain command clearance separately; mapping data alone does not certify them.
- The new source_provenance field appears only in mapped-role output. Existing Group JSON fields remain exactly field_count/origin. Existing formats/qualification and ordinary/signed schema shapes are retained, with expected source-closure digest changes acknowledged. The exact permitted way to observe ordinary/signed output remains Opus-owned.
- Public `--inspect-static` succeeds in a separate subprocess with the same explicitly supplied source/bindings/limits. Existing exact recompile/leaf equality consumes the provenance. Do not replace public coverage with a helper-only compiler call.

This is generic construct/field provenance coverage, not a new application-specific compiler or a claim that one fixture completes admitted-grammar semantics. Full rule-level values/refusal/cost/native correspondence remains owed.

## Proposed write set and verification sequence

Proposed source files: `crates/mori-ledger/src/relation/{mod.rs,compiler.rs,arithmetic.rs}` and `crates/mori/tests/mapped_role_static_io.rs`. No product source writes have begun. Documentation/EARS/evidence updates follow root integration after the reviewed source capability works; do not create new requirement IDs, profiles, registries or framework files.

After two agreeing scoped design votes, Opus constraints, independent oracle and explicit root GO: prepare normal test source; observe actual RED against unchanged producer using the cleared exact filter; implement the selected capture; run the same exact filter GREEN; inspect/freeze full candidate and obtain fresh applicable nonauthor Muse and Opus audits. Root controls build execution/locks/limits and publication.

Candidate command proposal, **not run and not cleared by this design**:

```bash
cargo +1.99.0 --offline --locked test -p mori --test mapped_role_static_io -j2 source_step_field_correspondence_survives_public_export_and_reopen -- --exact --test-threads=1
```

The named test does not yet exist. No broad mapped-role, ledger or backend target is proposed; those contain additional signed/native/security paths. Exact normal subprocesses would use the existing mapped-role all-action export/reopen flags only. Opus must classify the complete selected command/source closure, including static event serialization, tagged encode/decode and ordinary build-time residuals. Native authority remains zero, and source-static compilation cannot imply authority for SDK check/degree/model/VM/proof/key/deploy/ledger operations or private artifact/cache access.

## Questions routed to Opus/root before implementation

- Opus: accept or refine mapped-only field shape/compatibility handling, existing publication/reopen integrity treatment and unchanged resource-limit accounting; supply all security tests and judgments. This design supplies no substitute assessment.
- Opus: classify the exact selected static export/reopen test and any ordinary/signed shape observations. No new execution authority is inferred from function names.
- Root/independent oracle author: freeze the lawful source, exact root IDs and literal mapping expectations, complete normal financial observations and base-IR compatibility baseline before execution/implementation.
- Root: confirm the second substantive agreeing scoped vote and issue GO after constraints resolve. Acceptance, native authority, counters, external target unknowns and all U/M/MC/SP/G exits remain unchanged.


## Adopted design vote after Opus review

**VOTE: AGREE** on Opus's exact `StatementEmission`/`DynamicField` shape in `OPUS-DESIGN.md`, superseding the initial two-vector shape above. Per-statement `statement`, `expression`, half-open instruction/group ranges and ordered absolute `(group, offset)` replacements supply the selected emission provenance with fewer redundant indices. The actual group-to-Impact instruction is derivable from existing IR; no per-group instruction vector is needed. All statement kinds are collected; no dependencies/activity/value equivalence is claimed. Root expression is only the actual root arena ID. Immediate replacement offsets remain included. Source kind/from/asset/to resolve only through existing ABI/source and are not claimed to be bound by the claim IR.

Adopt Opus's mapped relation leaf `/2` bump, unchanged bundle `/1` and qualifications, unchanged shared Group/ordinary/signed field shapes, and naturally changed sourceclosure/build/compiler identities. This is a substantive nonsecurity agreement, not a security judgment. Root granted source edits only after this agreement; source-only plumbing and the independent oracle's five-step normal test are prepared. Opus assembly-integrity code/security tests and all executable command clearance remain pending. No baseline IR digest was invented or captured.

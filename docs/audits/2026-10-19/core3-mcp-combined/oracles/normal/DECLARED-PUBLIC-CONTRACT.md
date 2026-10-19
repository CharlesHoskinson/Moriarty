# Proposed Core/3 public consumer contract

Status: DRAFT, not adopted; ordinary source-exposed design by Sol. Basis C214b377daf372f7df1506c49d3e15148f9931012/tree37504f3288875d1e169ed5e5d7ffd49bd1101ec7 and exact maina2d0f968720eda21f5b4d87b184716bead028b45/treee1d993e9edb53382a437284156eb379799f1745e. Main moved during preparation; no claim about the moving branch. All named CLI/backend consumer bodies are identical between these two pins; Core/3 implementation is only in C. Integrate/review the exact resulting tree separately. Existing Core/3 owner changes themselves remain subject to their actual adoption status.

Falsifiable delivery predicate: a source-authored language "moriarty/3" program reaches public source checking, complete source inspection, start/resume and a whole-report test manifest through the existing real load_v3/start/resume evaluator. The same exact source reaches a dedicated real staged compiler once its Core/3 artifact/context/registry contract is owner-specified and implemented. It is never relabelled /1 or /2, replaced with a host success, or desugared by inserting a synthetic liability. Real target compilation, native verification and complete ledger effects remain separate required predicates.

## 1. Public profile/API boundaries

Retain mori_core::parse and language as /1 and /1-or-/2 respectively; retain stage::load as /2 and load_v3 as /3. Retain require_core1/require_core2/require_core3 and legacy v1 decoding. No generic Program deserializer is widened, and no cross-profile conversion is added. Core/3 source remains parsed by parse_v3 internally and admitted by existing load_v3; public consumers cannot construct StagedProgram or bypass that path.

Proposed CLI selection uses the explicit source header, with no new runtime profile flag. Add a tool-private helper to existing check_source.rs (avoid a second parser or a new orchestration layer):

```rust
pub(super) fn public_core_language(source: &str) -> Result<&'static str, mori_core::Error> {
    match mori_core::language(source) {
        Ok(language) => Ok(language),
        Err(legacy_error) => match mori_core::language_v3(source) {
            Ok(mori_core::stage::LANGUAGE_3) => Ok(mori_core::stage::LANGUAGE_3),
            _ => Err(legacy_error),
        },
    }
}
```

This deliberately preserves existing malformed/unknown-header diagnostics byte-for-byte; only an exactly recognized /3 header selects its new route. Both producers perform full lexing today; no bounded-prefix equivalence or new lexical behavior is claimed. A /3 body error is then produced by load_v3. This is proposed compatibility behavior D1, not an adopted change. An internal load helper dispatches exactly LANGUAGE_2=>load, LANGUAGE_3=>load_v3; other profiles return the existing profile refusal. It never falls back to /1 on an error.

The entire currently admitted /3 grammar is the public local input: inherited UInt/Bool declarations, inputs, finite scalar expressions, ordered statements, financial operations, liability reads, awaits and source duty declarations. No selected-name whitelist, transfer-only subset or new smaller numeric profile is introduced. Existing unsupported constructs and bounds remain explicit. Beta/Source6/Core5/Sherlock do not acquire an embedding by this change; retained broader grammar obligations are not removed.

## 2. Source check and inspect

`mori check FILE [--json]` recognizes /3 through D1, calls load_v3, and gives the real Program to SourceCheckConsumer::core with staged=true. For /3, the accepted JSON uses the existing Core summary keys plus semantics:"moriarty-core/3"; the existing rejected key set and STAGE_LABELS are reused. Accepted language is /3, and actions retain complete admitted IR. Existing /1 and /2 key sets/bytes remain unchanged. This check report is deliberately an existing validity summary, not a complete Program serialization or a compiler result. Documentation/capabilities identify the load_v3 consumer and Core/3 semantics. Existing human rendering/error propagation remains. The shared consumer must cover every actual caller at integration (current C/a2d0 has CLI check only; in-flight MCP work must consume this dispatch rather than duplicate policy).

`mori inspect FILE` gets a /3 branch using load_v3 and the existing qualified stage printer. Proposed /3 success object keys are exactly status,language,semantics,agreement,states,assets,duties,actions,expressions,stage_plan,enforcement_report plus the three STAGE_LABELS. status=accepted; enforcement_report=null. duties is present even when empty; every declaration/action/arena node/plan site is retained. /1 and /2 report bytes remain unchanged; /2 does not acquire a duties key. Inspect rejection follows the current staged rejected/error envelope and labels. Source inspection reads source only; it does not read scenario, target, key or artifact. `inspect --target` is a different compiler/enforcement consumer: do not silently treat a null map as compiled. Until the Core/3 enforcement map is implemented under the backend contract, return an explicit unsupported-consumer error before target I/O/publication. This is a known uncompleted route, not an exemption from finishing it.

## 3. Public simulation

`mori simulate FILE --action NAME --scenario FILE [--logical-cap U128]` routes /3 to the existing staged start path using load_v3, including non-awaiting actions. Do not send non-awaiting /3 to legacy simulate. `--resume FILE --event FILE` loads source through D1 and the exact staged loader, then uses unchanged bounded continuation/event readers and stage::resume. Continue using the actual returned continuation; expected reports never supply it. Existing argument conflicts, optional action check, source/continuation/event read order, caps, decoding limits and failure publication are unchanged. These are owner-defined semantics, not re-decided by this ordinary proposal.

Every start/resume result is stage::outcome_report followed by the existing bounded pretty exporter and one final newline. Preserve all status-dependent keys, complete segment execution/Steps/statements/typed values/pre/post/effect deltas/ordered effects, frame captures, residual rows and typed attempted/committed work. Preserve local-reference-stage, authentication:none, consumption:untracked. False observations keep segment:null and the actual unchanged continuation. Successful waiting/complete exit0; stage rejection exit1. Ordinary simulation writes only stdout/stderr, no continuation file, ledger state or cache. Complete raw process captures are separate from semantic comparison. No new outcome-format field or fabricated semantics member is added to the stage envelope; Execution carries program semantics and check/inspect/capabilities identify the source profile.

`--trace`, signed intent/prepare/prove/verify/call routes are separate native consumers. They remain explicit unsupported for /3 until their respective contracts are implemented; do not reroute them to local stage output or accept an unsigned report as a replay/proof. Before enabling a /3 trace route, its complete replay record/witness and artifact bindings require designated-owner specification. No new security policy is authored here.

## 4. Public test manifests

Propose new profile `moriarty-tests/3`, retaining /1 and /2 exact behavior and refusal boundaries. Its CLOSED schema is exactly the existing /2 schema:

Manifest {profile:the literal above,cases:[Case]}; Case {name:String,source:Path,scenario:Path,action:String,logical_cap:UInt-string-or-null,expected:Path,events:[Event]}; Event {event:Path,expected:Path}. All keys are required, including logical_cap:null and events:[]; deny unknown fields. Retain existing nonempty <=256 cases, unique nonempty names, <=255 events and current path resolution/strict-reader limits. These are existing harness bounds, not new language/resource decisions.

Dispatch profile /3 in test.rs to stage_test::run with explicit source profile /3; /2 continues selecting load only, /3 selects load_v3 only. No manifest can change the source identity or reinterpret a /2 source. Reuse run_case's whole expected JSON equality, actual continuation chaining, actual event order, actual retry behavior after a rejection and individually bounded aggregate exporter. Read all expected reports before starting each case as today. Report exactly {status,cases:[{name,passed,stages:[{index,passed,actual}]}]}; keep every actual outcome even on an ordinary expectation mismatch. stderr counts and process exit match actual comparison results. This is D3: a new public manifest profile, not permission to derive goldens from the implementation.

The existing independent Core/3 financial/work freeze is NOT a whole outcome/hash/byte oracle. Positive /3 manifest fixtures cannot be completed honestly until independent whole reports have been frozen under adopted /3 premises. Financial projection tests can proceed using that freeze, with their narrower comparison scope named; they do not establish manifest whole equality. Do not fill the missing fields with captures or current producer output.

## 5. Real compiler journey and exact blockers

Keep existing backend compile/compile_signed (/1 Artifact/3), compile_staged (/2 StagedArtifact/4), their decoders and consumers unchanged. Source Core/3 is not artifact/3. Propose additive public signature:

`mori_backend::stage::compile_staged_v3(src:&str,target:&Target,out:&Path) -> Result<Core3StagedArtifact,Error>`.

The type/name is a proposed interface; exact artifact/ABI/relation/context version strings are intentionally NOT selected by this ordinary author. They are blocking decision B1 below. No actual implementation or signature approval is asserted. A new explicit type prevents accidental handoff to legacy /1 proving/replay or /2 staged consumers. Shared private lowering is reusable only after its complete declared input/registry/context premises are established; no cloned compiler or handwritten target IR is proposed.

Actual current blockers, in dependency order:

1. stage/lower.rs compile_staged always calls stage::load (/2). A new v3 entry must call real load_v3 and retain all actions/segments; no language substitution or synthetic liability statement. One circuit per successful source segment remains the candidate mapping, not a statement about false-observation proof/native behavior.
2. backend liabilities::registry currently derives identities only from Statement::Liability and refuses discharge/read without one. Core/3's defining positive discharge-only declared-duty program has no such statement. The designated owner must specify the exact Core/3 registry/admission/layout correspondence to the existing REG3 definition and complete financial frame. Ordinary code may then mechanically transport that approved registry into the real lowerer. This proposal supplies no new identity, completeness, consent or circuit-soundness rule.
3. stage/context.rs checks Core/2 semantics and encodes it in a 29-element context; stage/artifact.rs validates only Core/2 semantics with artifact/ABI/relation/4. A Core/3 artifact and context contract, decoder and correspondence obligations are needed before a /3 artifact can be returned. Simply changing SEMANTICS_2 or widening an OR would silently alter existing acceptance and is not this plan.
4. stage/frame.rs and lib.rs lower_action_segment currently accept a sealed StagedProgram and an independently built backend registry. Their registry consumers, pending layout, effects/balance/liability descriptors and every expression/statement match must be inspected against the approved /3 compiler contract; reuse is not established by Rust type compatibility. All source constructs stay in the coverage denominator. An unsupported lowerer construct must refuse explicitly and remain an open implementation obligation.
5. compile.rs must route /3 to the real additive entry, finish compilation and serialization before creating its new output directory, publish actual compiler-produced circuit files and the manifest last, and preserve no-overwrite behavior. File naming/version-specific artifact projection is B1. No host-produced success, fixture lookup, handwritten ZKIR or DTO substitute is permitted.
6. inspect --target needs a complete /3 artifact enforcement map. Proving/replay/key/verify/ledger consumers need explicit new typed admission and full owner contracts before they can consume the new artifact. Existing /4 is deliberately unable to enter those legacy paths; do not erase that boundary. Compiler-source structure, SDK import/build, SDK check/model, VM, native proof, key generation, ledger acceptance and Preview/Fenrir effects are distinct observations and resource decisions. This draft authorizes none.

This is a plan to finish these connections, not to declare the local half sufficient or remove the backend half. Core/3 duties/conditions must survive the compiler journey through the actual native target, under the owner's separately authored relation. Target phase, authority, consent, soundness, resource and proof semantics are designated-owner work; this document names the missing interfaces without answering them.

## 6. Required design dispositions

D1 (ordinary, two substantive agreeing votes): source-header selection and preservation of old unknown/malformed-header diagnostics; legacy library APIs remain exact. Alternative is intentional new diagnostic text naming /3; that would require a declared compatibility change and new independently frozen literals rather than silently rewriting old expected files.

D2 (ordinary, two substantive agreeing votes): /3 check adds its explicit semantics to the existing validity-summary shape; /3 inspect exposes the exact full source/plan object including mandatory duties; /3 simulate uses unchanged complete stage envelopes and local labels. No automatic Core/2 normative inheritance is claimed: adopt the exact /3 ordinary arena/report projection under its own semantics, with duty declaration metadata and unchanged inherited node/statement mappings explicitly specified.

D3 (ordinary, two substantive agreeing votes): additive moriarty-tests/3 closed schema, source-profile binding and whole-report comparison/aggregate behavior described above. /2 remains /2-only.

B1 (designated-owner-led plus applicable interface votes; unresolved): exact Core/3 staged compiler artifact type/schema/version identifiers, registry/context/lowering admission and downstream typed bindings. Request the concrete owner-authored contract; do not treat this document as a security vote. Ordinary version/type/publication mechanics can then receive their own two substantive interface votes where consequential.

B2 (designated-owner-owned; unresolved): native trace/proof/verification/key/ledger interfaces and target/phase/authority/resource predicates for the new artifact. No approving or rejecting judgment is supplied by Sol. Missing B2 does not block independent authorized ordinary source work, but it blocks native completion claims and dependent enabling.

Original parent requirements, all27 grammar obligations, all148 requirements/292 scenarios/20 families, U0–U7/M1–M17/MC/SP/G, full finance and all five composition operators remain. No milestone, scheduler, campaign or approval requirement is imposed on public developers. All choices above are maintainer design/adoption work only.

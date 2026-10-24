Ordinary canonical carrier implemented; owner V8 seam pending

Status: R-only source afterimages, uncompiled and unrun. This is a partial connected source prerequisite pending the designated-owner V8 patch, not a compilable/admitted candidate or a claim of generic runtime closure. W, Git, provider wires and independent expectation/assertion source were not written or executed.

The canonical runtime carrier is the existing `shared::adt::TypeIdC(pub usize)` itself. No second wrapper, dialect, enum or authoritative signature table is added. The original representation, equality/order derives and arbitrary IDs remain unchanged. Associated constants give the existing fixed builtin identities UInt=0, Bool=1, List=2 and Tree=3. `TypeIdC::from_source_sort` and `From<SortTag>` are total pure mappings from the unchanged four-variant source API. A nominal `TypeIdC(n)` for n>=4 is stored and copied exactly as n; no conversion to a fake source SortTag is present.

Reviewable artifacts

- `afterimages/<W-relative-path>`: full source for all eight changed files.
- `SOURCE.diff`: exact unified diff against the current installed W preimages.
- `SOURCE-MANIFEST.json`: exact preimage/afterimage byte counts and SHA256 for every changed file.
- `DECLARATIONS.rs.txt`: declaration-only public/internal API summary.
- `BODY-PRESERVATION.json`: exact predicate-body pins/comparisons and whole-file permitted-change inversion evidence.
- `OWNER-INTERFACE-REQUEST.json`, `OWNER-INTERFACE-REQUEST.md`, `OWNER-INTERFACE-check_program_view.current.rs.txt`: exact unresolved owner seam, full current guard function and SHA pins.

Producer and real consumer changes

1. `shared/adt.rs`: add builtin associated constants and total source-to-canonical conversion on the actual existing canonical TypeIdC. The type builder, constructor storage and T3Record already use this type; no builder restart, finish, second resolution pass or additional resolved-signature table is introduced.
2. `shared.rs`: reexport that exact TypeIdC. Change FunctionRowC.return_type, ParameterRowC.sort, PairTask.sort, ControlC::Bound.return_type, ReturnF.return_type and Scalar.expected to canonical TypeIdC. CoreProgram/CoreFunction/CoreParameter still use the unchanged source SortTag. PreImageView.checked_sort/scalar_test_type_ok use the canonical carrier. A local pure_leaves alias `TypeIdC as SortTag` retains sort_of_kind's complete match body exactly while returning canonical IDs.
3. `shared/owner_prepare.rs`: the current LIVE prepare_core C5 producer now maps parameter and return source SortTags into canonical TypeIds in its two existing row assignments. Capacity checks, source/function/parameter order, name binding, body IDs and all other function bytes remain unchanged. The whole existing check_program_view body is intentionally untouched; see the owner seam below.
4. `shared/owner_runtime.rs`: change only the checked_sort/scalar_test_type_ok signatures in both concrete PreImageView and read-ledger forwarding. The existing checked FunctionRowC/ParameterRowC readers automatically expose the canonical row fields, with no new read/order/ledger behavior. All runtime predicate bodies are exact.
5. `families/structural.rs`: change only bind_pair_payload's private checked-row tuple type to TypeIdC. The public wrapper still reads parameter before argument and passes pr.sort; the unchanged payload pushes the exact canonical ID into PairTask.
6. `shared/owner_runtime/owner_si21_bind_match.rs`: change only sort_accepts's wanted type signature to TypeIdC. Existing task/parameter equality and Bind denotation/continuity logic remain exact.
7. `shared/owner_runtime/owner_si21_closure.rs`: change only has_type's input signature and ScalarEntry.expected's declaration to canonical TypeIdC. Existing typed-return selection, Scalar continuity/staging/comparison and all other bodies remain exact.
8. `compile_artifact/wire.rs`: add ordinary OwnerWire rendering for TypeIdC. Builtins keep the old public strings "UInt", "Bool", "List", "Tree" in every preexisting row/control/frame/helper field. Other IDs render as `{"TypeId":"<decimal identity>"}` through the existing usize wire renderer. Existing SortTag OwnerWire and all old renderers remain exact.

Existing live copy paths that require no body change

`prepare_core -> ProgramView.parameters -> PreImageView.parameter -> structural plan_bind -> bind_pair_payload -> PairTask` now uses TypeIdC at every slot. The structural wrapper, read-ledger parameter forwarding, guard evaluator, SI21 pair-task matching and Bind ordering bodies remain exact.

`prepare_core -> ProgramView.functions -> PreImageView.function_row -> legacy KMachineEnterBind -> ControlC::Bound -> legacy KMachineBoundValid -> ReturnF -> Scalar RetCall entry -> Scalar.expected -> scalar_test_type_ok` now uses the same TypeIdC at every slot. Legacy and Scalar installed fragments/family wrappers remain byte-identical, including the installed word lowering. SI21 scalar_entry, scalar_den, has_type continuity and result checks consume the same canonical identity. Complete observations/comparisons clone and compare these same row/state records and need no alternate table.

The existing full-program prepared source owns ExtProgram, final interned names/declIDs, BuiltAdtTypeTables and T3Records. All seven installed ONE-builder production files were inspected against current W authority; five are unchanged and the two shared/owner_prepare files preserve that installed work. T3Record.type_id is already the same TypeIdC used by these runtime slots. Full generic C/P emission into them is a retained follow-on.

Byte preservation evidence

Data-only comparisons against REBASE-PREDICATE-BODY-PINS verify all 18 listed predicate bodies exact (including both concrete/read-ledger checked_sort and scalar_test_type_ok, sort_of_kind, owner eval_guard/select/validate/step/run/StepInv, SI21 sort_accepts/has_type/scalar paths). BODY-PRESERVATION also proves five complete runtime/SI21/structural/prepare files equal their current preimages after reversing only the specifically listed mechanical signatures/declaration or two allowed producer assignments. The complete check_program_view function is exact, SHA256 `b09ef3359df2e62b8bf4f4ca5536543c1eaef061efe6c540ecb3b713a3c17440`; its braces-through-closing-brace body SHA256 is `a2a1b05d6ec33b68448be3c0e17aff9e43dcd7dbf1e7f663ad0d423436376d90`.

The unchanged entire core_action_consumer_security.rs is pinned in the preservation artifact; check_generated compares the A SherlockType, so it receives no mechanical change. Raw R4/source SortTag, A/layout-A/reference/K/Quint/Clean, resource/security guards, public parser/CLI, budgets/defaults/native code are unchanged. Marker and reserved-constructor match/error behavior remain exact. No new nominal acceptance predicate is authored.

Exact unresolved owner-only seam

The existing V8 check_program_view reconstruction still assigns `v.parameters[pid].sort` and `f.return_type` directly into unchanged CoreParameter.sort/CoreFunction.return_type source SortTags. Those two assignments now require designated-owner partial canonical-to-source conversion: builtin IDs 0..3 preserve their prior source meanings; IDs >=4 cannot be represented by the Core source API. The owner chooses the precise ProgramViewViolation refusal/order/locator/detail. This author does not supply a fallback, coercion, refusal branch, TODO, panic or stub. Consequently the eight ordinary afterimages are intentionally not claimed to compile until that seam is incorporated.

Root must integrate the owner patch into the ordinary owner_prepare afterimage (SHA256 `9d9a59458439e09845858c80292bd12e5324d1953301ff70e7935f6432b9de80`), preserving its two prepare_core producer assignment edits. The owner request pins the current W whole file at `86008d494e7b160cea554fea1ba9e6c87f0c32a3b4b512ec3ce3b67ab3058036` and supplies its full current guard function.

Verification and acceptance

Performed only source reads, data-only transformations, diffs and SHA/byte comparisons; no provider calls, project imports, compiler, tests, candidate/model/native execution or Cargo. No protected R259/R278 runtime oracle/test bodies or populated independent expectation/assertion packets were read. Source author does not approve this candidate. Fresh designated non-author/security review and independently frozen expectations remain required before any execution.

Full generic C/P ProgramView emission; missing patternChildSlots/planCaseSlots/fieldPathSlots interface; V14 declaration templates; extended admission; constructor backing/ConstructorFields; ordered MatchPlan/Build dispatch; and same-owner-engine Schedule first-arm4/separate-sum9/repeated8 remain required follow-ons. This prerequisite claims neither generic runtime/admission nor original004/007/034 closure.

Retrospective narrow source-review delivery supplement for the unchanged Core369 candidate. Root reports PR273 already merged at fb49d0ee and exact19 ordinary QA independently passed. This is prepared only to qualify the prior review's documented API coverage, not to create a new merge/execution gate, reopen source design, replay tests or infer a defect.

Prior actual Gemini source opinion (33,525 bytes,SHA778d63ed1b0ba582652eb6f634bb8ff41b7c1b1bea92183a7f45a5fc3d828aae) SOURCE-APPROVED all20 changed/new files and33 literal operations. Actual tool metadata records successful whole-file requests for all20, each with exact local line/byte header; no StartLine/EndLine ranges were supplied. It also records86 DONE file views,11 failed views and one ls metadata command. This was a read-only tool-using review, not no-tools. The event log retains summary headers, not returned bodies; complete content delivery is unknown. Core inline truncation is NOT inferred from the separate newer K task. Original full opinion is preserved unchanged alongside this supplement.

The only three complete original public API contract documents lacking direct DONE views are inlined in full below: ABI-DECLARATIONS.json (sixteen selected/old354 declarations), ADDITIONAL-ADT-DECLARATIONS.json (sixteen ADT/plan/control declarations), ACTUAL-CONSUMER-WIRE.json (eleven serializer implementations). The last has a successful direct actual public wire.rs request as additional metadata evidence, but that does not turn into a contract-body delivery receipt. All other original API documents and all20 afterfiles have successful size-matching whole-file request metadata. The exact unchanged33-operation diff is also complete below. This packet contains no private expected/comparison cell arrays, provider thoughts, transcript bodies or source edits.

Review only these full API surfaces against the existing exact source delta: trait forwards and field/carrier types; Constructor snapshot identifiers/field slice; Match.plan/Construct/Wildcard/Constructor independent representations; ValueNode/InputPath additions; TypeIdC dedicated sort handling; and the restored owner test module's used APIs. Do not substitute independent serializers, flatten nominal identifiers, change assertions/fixtures or invent correspondence. The original source and later operations opinions retain their scope/limits. Draft-runtime Match.plan:null versus omitted fixture runtime disposition remains open; only the four owner tests ran, eleven external targets were compiled only. No native/target/generic B projection/EARS claim is under review.

Return a narrow API-coverage verdict or precise additional PUBLIC source needed. Do not claim complete prior source-body delivery, do not semantically approve the whole pipeline from file metadata, and do not dispatch tools/commands, inspect private goldens or execute any subject. Actual reviewer identity/terminal status and any limitation must be retained. All below bodies are exact, and the entire packet is below128KiB; that size is a delivery precaution, not proof of what the earlier Core run received.


===== Complete original public API contract lacking direct DONE view =====
/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/core-cfgtest-generic-abi-inventory-20261019/ABI-DECLARATIONS.json
Bytes 31126 SHA256 1957c8347e92bd1de4d5a9bc8035185886ee0888e0031444b0a4565b1cf185c2
```text
[
  {
    "name": "LayoutSchemaC",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 385,
      "declaration": "pub struct LayoutSchemaC { pub layout_version: (String, String),\n    pub field_id: Option<(String, String)>, pub widths: Option<WidthTable>,\n    pub scalar_layout_id: Option<(String, String)>, pub bounds: BoundsC,\n    pub capacities: LayoutCapacitiesC,\n    pub ext_capacities: Option<owner_ext_admit::ExtCapacitiesC>,\n    pub boundary_capacities_present: bool }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 347,
      "declaration": "pub struct LayoutSchemaC { pub layout_version: (String, String),\n    pub field_id: Option<(String, String)>, pub widths: Option<WidthTable>,\n    pub scalar_layout_id: Option<(String, String)>, pub bounds: BoundsC,\n    pub capacities: LayoutCapacitiesC,\n    pub boundary_capacities_present: bool }"
    }
  },
  {
    "name": "FamilyPlans",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 671,
      "declaration": "pub trait FamilyPlans {\n    fn plan_build(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_plan(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_legacy(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_scalar(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_env_get(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_env_put(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_find(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_bind(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_append(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_validate(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_equal(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_match(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_bookkeeping_release(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 608,
      "declaration": "pub trait FamilyPlans {\n    fn plan_legacy(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_scalar(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_env_get(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_env_put(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_find(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_bind(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_append(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_validate(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_equal(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_match(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n    fn plan_bookkeeping_release(&self, sel: &Selected, pre: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation>;\n}"
    }
  },
  {
    "name": "ExpressionRowC",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 312,
      "declaration": "pub enum ExpressionRowC { Literal { constant: usize }, Var { binder: usize },\n    Let { binder: usize, value: usize, body: usize },\n    Binary { op: BinaryOp, left: usize, right: usize },\n    Unary { op: UnaryOp, operand: usize },\n    If { cond: usize, then_expr: usize, else_expr: usize },\n    Call { function_name: usize, args: SliceC },\n    Match { scrutinee: usize, arms: SliceC, plan: Option<usize> },\n    Construct { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: SliceC,\n        constant: Option<usize> } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 295,
      "declaration": "pub enum ExpressionRowC { Literal { constant: usize }, Var { binder: usize },\n    Let { binder: usize, value: usize, body: usize },\n    Binary { op: BinaryOp, left: usize, right: usize },\n    Unary { op: UnaryOp, operand: usize },\n    If { cond: usize, then_expr: usize, else_expr: usize },\n    Call { function_name: usize, args: SliceC },\n    Match { scrutinee: usize, arms: SliceC } }"
    }
  },
  {
    "name": "PatternRowC",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 328,
      "declaration": "pub enum PatternRowC { PVar { x: usize }, PNil, PCons { x: usize, y: usize },\n    PLeaf { x: usize }, PBranch { x: usize, y: usize }, Wildcard,\n    Constructor { type_id: TypeIdC, ctor_id: adt::CtorIdC, children: SliceC } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 309,
      "declaration": "pub enum PatternRowC { PVar { x: usize }, PNil, PCons { x: usize, y: usize }, PLeaf { x: usize }, PBranch { x: usize, y: usize } }"
    }
  },
  {
    "name": "FunctionRowC",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": false,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 322,
      "declaration": "pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: TypeIdC, pub body: usize }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 303,
      "declaration": "pub struct FunctionRowC { pub function_name: usize, pub params: SliceC, pub return_type: TypeIdC, pub body: usize }"
    }
  },
  {
    "name": "ParameterRowC",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": false,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 324,
      "declaration": "pub struct ParameterRowC { pub binder: usize, pub sort: TypeIdC }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 305,
      "declaration": "pub struct ParameterRowC { pub binder: usize, pub sort: TypeIdC }"
    }
  },
  {
    "name": "SortTag",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": false,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 30,
      "declaration": "pub enum SortTag { UInt, Bool, List, Tree }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 30,
      "declaration": "pub enum SortTag { UInt, Bool, List, Tree }"
    }
  },
  {
    "name": "PreImageView",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 548,
      "declaration": "pub trait PreImageView {\n    // Concrete owner/recording implementations supply checked ADT projections.\n    fn adt_reads(&self) -> &dyn adt_runtime::AdtRuntimeReads;\n    fn control(&self) -> &ControlC;\n    fn control_origin(&self) -> usize;\n    fn env_root(&self) -> EnvRootC;\n    fn depth(&self) -> usize;\n    fn task_live(&self) -> usize;\n    fn stack_limit(&self) -> usize;\n    fn outcome(&self) -> &OutcomeC;\n    fn initial_fuel(&self) -> u128;\n    fn remaining_work(&self) -> u128;\n    fn consumed_work(&self) -> u128;\n    fn helper_work(&self) -> u128;\n    fn helper_visits(&self) -> u128;\n    fn reference_cursor(&self) -> u128;\n    fn helper(&self) -> Option<&HelperControlC>;\n    fn result(&self) -> Option<&HelperResultC>;\n    fn top_frame(&self) -> Option<&FrameRowC>;\n    fn top_task(&self) -> Option<&HelperTaskC>;\n    fn active(&self) -> ActiveLengths;\n    fn alloc(&self) -> AllocCounters;\n    // All keys by reference with C carriers; all reads checked, Err never panic. No view()/ProgramView escape.\n    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation>;\n    fn child(&self, pos: &usize) -> Result<usize, HostViolation>;\n    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation>;\n    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation>;\n    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation>;\n    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation>;\n    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation>;\n    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation>;\n    fn constant_borrowed(&self, id: &usize) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;\n    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation>;\n    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation>;\n    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;\n    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation>;\n    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation>;\n    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation>;\n    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation>;\n    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation>;\n    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation>;\n    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation>;\n    // pure arity_of None (Marker/ConstructorExtensionReserved) maps only here to ArithmeticUndefined.\n    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation>;\n    // pure sort_of_kind None maps only here to ArithmeticUndefined for structural sort consumers.\n    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation>;\n    // Sole Scalar TestType seam: kind Marker/ConstructorExtensionReserved yields Ok(false), UInt/Bool/Nil/Cons/Leaf/Branch compares checked_sort; families never map Option themselves.\n    fn admitted_function_count(&self) -> usize;\n    // Sole function-table length seam: owner returns admitted view functions.len(); no other implementation and no zero fallback is permitted. Other tables remain readable only through the checked row methods above.\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 495,
      "declaration": "pub trait PreImageView {\n    fn control(&self) -> &ControlC;\n    fn control_origin(&self) -> usize;\n    fn env_root(&self) -> EnvRootC;\n    fn depth(&self) -> usize;\n    fn task_live(&self) -> usize;\n    fn stack_limit(&self) -> usize;\n    fn outcome(&self) -> &OutcomeC;\n    fn initial_fuel(&self) -> u128;\n    fn remaining_work(&self) -> u128;\n    fn consumed_work(&self) -> u128;\n    fn helper_work(&self) -> u128;\n    fn helper_visits(&self) -> u128;\n    fn reference_cursor(&self) -> u128;\n    fn helper(&self) -> Option<&HelperControlC>;\n    fn result(&self) -> Option<&HelperResultC>;\n    fn top_frame(&self) -> Option<&FrameRowC>;\n    fn top_task(&self) -> Option<&HelperTaskC>;\n    fn active(&self) -> ActiveLengths;\n    fn alloc(&self) -> AllocCounters;\n    // All keys by reference with C carriers; all reads checked, Err never panic. No view()/ProgramView escape.\n    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation>;\n    fn child(&self, pos: &usize) -> Result<usize, HostViolation>;\n    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation>;\n    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation>;\n    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation>;\n    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation>;\n    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation>;\n    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation>;\n    fn constant_borrowed(&self, id: &usize) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;\n    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation>;\n    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation>;\n    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation>;\n    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation>;\n    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation>;\n    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation>;\n    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation>;\n    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation>;\n    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation>;\n    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation>;\n    // pure arity_of None (Marker/ConstructorExtensionReserved) maps only here to ArithmeticUndefined.\n    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation>;\n    // pure sort_of_kind None maps only here to ArithmeticUndefined for structural sort consumers.\n    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation>;\n    // Sole Scalar TestType seam: kind Marker/ConstructorExtensionReserved yields Ok(false), UInt/Bool/Nil/Cons/Leaf/Branch compares checked_sort; families never map Option themselves.\n    fn admitted_function_count(&self) -> usize;\n    // Sole function-table length seam: owner returns admitted view functions.len(); no other implementation and no zero fallback is permitted. Other tables remain readable only through the checked row methods above.\n}"
    }
  },
  {
    "name": "ProgramView",
    "relative": "src/sherlock/helper_machine/shared.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 342,
      "declaration": "pub struct ProgramView {\n    pub entry: usize,\n    pub expressions: Vec<ExpressionRowC>, pub occurrences: Vec<OccurrenceRowC>, pub occurrence_paths: Vec<usize>,\n    pub children: Vec<usize>, pub functions: Vec<FunctionRowC>,\n    pub parameters: Vec<ParameterRowC>, pub arms: Vec<ArmRowC>,\n    pub patterns: Vec<PatternRowC>, pub pattern_children: Vec<usize>,\n    pub types: Vec<adt::TypeRowC>, pub type_args: Vec<TypeIdC>,\n    pub constructors: Vec<adt_storage::CtorRowExt>,\n    pub ctor_fields: Vec<adt_storage::CtorFieldRowExt>,\n    pub decl_param_counts: Vec<usize>, pub decl_ctor_counts: Vec<usize>,\n    // Original source templates are required by the owner's V14 round trip;\n    // resolved monomorphic field IDs cannot recover parameterized templates.\n    pub decl_templates: Vec<owner_ext_type_builder::TypeDeclSrc>,\n    pub plan_nodes: Vec<PlanNodeC>, pub plan_cases: Vec<Option<usize>>,\n    pub field_paths: Vec<usize>,\n    pub constants: Vec<RawConstRow>, pub constant_fields: Vec<usize>,\n    pub names: Vec<String>,\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 313,
      "declaration": "pub struct ProgramView {\n    pub entry: usize,\n    pub expressions: Vec<ExpressionRowC>, pub occurrences: Vec<OccurrenceRowC>, pub occurrence_paths: Vec<usize>,\n    pub children: Vec<usize>, pub functions: Vec<FunctionRowC>,\n    pub parameters: Vec<ParameterRowC>, pub arms: Vec<ArmRowC>,\n    pub patterns: Vec<PatternRowC>,\n    pub constants: Vec<RawConstRow>, pub constant_fields: Vec<usize>,\n    pub names: Vec<String>,\n}"
    }
  },
  {
    "name": "DataRow",
    "relative": "src/sherlock/helper_machine/raw.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 19705,
        "sha256": "bb0a510b4142b46ae12f45e6cfdbbcebef95e04a4a1426fcd61e54c0a81778f4"
      },
      "line": 716,
      "declaration": "pub enum DataRow {\n    U128 { n: UInt128 },\n    Bool { b: Bool },\n    Nil,\n    Cons { fields: FieldSlice },\n    Leaf { fields: FieldSlice },\n    Branch { fields: FieldSlice },\n    Constructor { type_id: TypeId, ctor_id: CtorId, fields: FieldSlice },\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 18821,
        "sha256": "ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1"
      },
      "line": 688,
      "declaration": "pub enum DataRow {\n    U128 { n: UInt128 },\n    Bool { b: Bool },\n    Nil,\n    Cons { fields: FieldSlice },\n    Leaf { fields: FieldSlice },\n    Branch { fields: FieldSlice },\n}"
    }
  },
  {
    "name": "ConstRow",
    "relative": "src/sherlock/helper_machine/raw.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 19705,
        "sha256": "bb0a510b4142b46ae12f45e6cfdbbcebef95e04a4a1426fcd61e54c0a81778f4"
      },
      "line": 704,
      "declaration": "pub enum ConstRow {\n    U128 { marker_free: Bool, n: UInt128 },\n    Bool { marker_free: Bool, b: Bool },\n    Nil { marker_free: Bool },\n    Cons { marker_free: Bool, fields: ConstFieldSlice },\n    Leaf { marker_free: Bool, fields: ConstFieldSlice },\n    Branch { marker_free: Bool, fields: ConstFieldSlice },\n    Constructor { marker_free: Bool, type_id: TypeId, ctor_id: CtorId, fields: ConstFieldSlice },\n    U128OutOfRange { marker_free: Bool },\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 18821,
        "sha256": "ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1"
      },
      "line": 677,
      "declaration": "pub enum ConstRow {\n    U128 { marker_free: Bool, n: UInt128 },\n    Bool { marker_free: Bool, b: Bool },\n    Nil { marker_free: Bool },\n    Cons { marker_free: Bool, fields: ConstFieldSlice },\n    Leaf { marker_free: Bool, fields: ConstFieldSlice },\n    Branch { marker_free: Bool, fields: ConstFieldSlice },\n    U128OutOfRange { marker_free: Bool },\n}"
    }
  },
  {
    "name": "TypeId",
    "relative": "src/sherlock/helper_machine/raw.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 19705,
        "sha256": "bb0a510b4142b46ae12f45e6cfdbbcebef95e04a4a1426fcd61e54c0a81778f4"
      },
      "line": 698,
      "declaration": "pub struct TypeId(pub usize);"
    },
    "old354": null
  },
  {
    "name": "CtorId",
    "relative": "src/sherlock/helper_machine/raw.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/raw.rs",
        "bytes": 19705,
        "sha256": "bb0a510b4142b46ae12f45e6cfdbbcebef95e04a4a1426fcd61e54c0a81778f4"
      },
      "line": 701,
      "declaration": "pub struct CtorId(pub usize);"
    },
    "old354": null
  },
  {
    "name": "ValueNode",
    "relative": "src/sherlock/helper_machine/witness_plan.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/witness_plan.rs",
        "bytes": 13605,
        "sha256": "7328c960e7a7f98eca964511f62a2e4d0a9b7b664156c5e0cfc23a83a4571a92"
      },
      "line": 62,
      "declaration": "pub enum ValueNode {\n    UInt(u128), Bool(bool), Nil, U128OutOfRange,\n    Cons { head: usize, tail: usize }, Leaf { value: usize },\n    Branch { left: usize, right: usize },\n    /// Canonical nominal identity with ordered roots in this occurrence arena.\n    /// This diagnostic projection does not authenticate constructor membership.\n    Constructor { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: Vec<usize> },\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/witness_plan.rs",
        "bytes": 12255,
        "sha256": "069c78ce06e093409a038ae3e9b36d2fa76585f843aa9f733e128461a96932a9"
      },
      "line": 62,
      "declaration": "pub enum ValueNode {\n    UInt(u128), Bool(bool), Nil, U128OutOfRange,\n    Cons { head: usize, tail: usize }, Leaf { value: usize },\n    Branch { left: usize, right: usize },\n}"
    }
  },
  {
    "name": "InputPath",
    "relative": "src/sherlock/helper_machine/witness_plan.rs",
    "declarationChanged": true,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/witness_plan.rs",
        "bytes": 13605,
        "sha256": "7328c960e7a7f98eca964511f62a2e4d0a9b7b664156c5e0cfc23a83a4571a92"
      },
      "line": 104,
      "declaration": "pub enum InputPath {\n    Metadata, Constant(usize), ConstantField(usize), Data(usize), DataField(usize),\n    Environment(usize), Name(usize), Expression(usize), Function(usize),\n    Parameter(usize), Arm(usize), Pattern(usize), Occurrence(usize),\n    OccurrencePath(usize), Child(usize), Frame(usize), ScheduledFrame,\n    Control, Helper, Task(usize), Result, Outcome, ValueSlot(usize),\n    PatternChild(usize), Type(usize), TypeArg(usize), Constructor(usize),\n    ConstructorField(usize), DeclParamCount(usize), DeclCtorCount(usize),\n    DeclTemplate(usize), PlanNode(usize), PlanCase(usize), FieldPath(usize),\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/witness_plan.rs",
        "bytes": 12255,
        "sha256": "069c78ce06e093409a038ae3e9b36d2fa76585f843aa9f733e128461a96932a9"
      },
      "line": 101,
      "declaration": "pub enum InputPath {\n    Metadata, Constant(usize), ConstantField(usize), Data(usize), DataField(usize),\n    Environment(usize), Name(usize), Expression(usize), Function(usize),\n    Parameter(usize), Arm(usize), Pattern(usize), Occurrence(usize),\n    OccurrencePath(usize), Child(usize), Frame(usize), ScheduledFrame,\n    Control, Helper, Task(usize), Result, Outcome, ValueSlot(usize),\n}"
    }
  },
  {
    "name": "TypeIdC",
    "relative": "src/sherlock/helper_machine/shared/adt.rs",
    "declarationChanged": false,
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared/adt.rs",
        "bytes": 1139,
        "sha256": "bc4c239ec8258abd528a90a71c35bf02168fb716c4f4742d2fef7839fe8cdc69"
      },
      "line": 4,
      "declaration": "pub struct TypeIdC(pub usize);"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared/adt.rs",
        "bytes": 1139,
        "sha256": "bc4c239ec8258abd528a90a71c35bf02168fb716c4f4742d2fef7839fe8cdc69"
      },
      "line": 4,
      "declaration": "pub struct TypeIdC(pub usize);"
    }
  }
]

```


===== Complete original public API contract lacking direct DONE view =====
/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/core-cfgtest-generic-abi-inventory-20261019/task-B-opus-source-author/ADDITIONAL-ADT-DECLARATIONS.json
Bytes 26344 SHA256 5cf8807b67a12718c2c55c54bcaf30d2c56587b4b3ecf0c45f20a20414e2bd3d
```text
[
  {
    "name": "HelperControlC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 196,
      "declaration": "pub enum HelperControlC {\n    Scalar { common: HelperCommon, mode: ScalarMode, operand_a: RefC,\n        operand_b: Option<RefC>, expected: Option<TypeIdC>,\n        phase: ScalarPhase, reg_a: u128, staged: ScalarStaged },\n    EnvGet { common: HelperCommon, binder: usize, cursor: Option<usize>,\n        found: Option<RefC>, status: TriStatus },\n    EnvPut { common: HelperCommon, binder: usize, value: RefC, base: EnvRootC,\n        new_root: Option<usize>, status: TriStatus },\n    Find { common: HelperCommon, function_name: usize, row_cursor: usize,\n        found: Option<usize>, status: TriStatus },\n    Bind { common: HelperCommon, function: usize, params: SliceC, args: ValSliceC,\n        p_cursor: usize, a_cursor: usize, partial: EnvRootC,\n        dup_cursor: Option<usize>, phase: BindPhase, status: BindStatus },\n    Append { common: HelperCommon, src: ValSliceC, value: RefC,\n        src_cursor: usize, dest_start: usize, staged: Option<RefC>,\n        phase: AppendPhase },\n    Validate { common: HelperCommon, root: RefC, status: TriStatus },\n    Equal { common: HelperCommon, left: RefC, right: RefC, status: TriStatus },\n    Match { common: HelperCommon, scrutinee: RefC, arm: usize,\n        saved_env: EnvRootC, partial: EnvRootC, pat_tag: PatternTag,\n        x: usize, y: usize, field_idx: u8, staged: Option<RefC>,\n        phase: MatchPhase, status: TriStatus },\n    Plan { common: HelperCommon, source_match: usize, scrutinee: RefC,\n        saved_env: EnvRootC, partial: EnvRootC, node: usize, phase: PlanPhase,\n        after_walk: PlanAfterWalk, cur: RefC, path: SliceC, path_cursor: usize,\n        arm: Option<usize>, status: TriStatus },\n    Build { common: HelperCommon, type_id: TypeIdC, ctor_id: adt::CtorIdC,\n        src: ValSliceC, src_cursor: usize, field_start: usize,\n        staged: Option<RefC>, node: Option<RefC>, phase: BuildPhase },\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 190,
      "declaration": "pub enum HelperControlC {\n    Scalar { common: HelperCommon, mode: ScalarMode, operand_a: RefC,\n        operand_b: Option<RefC>, expected: Option<TypeIdC>,\n        phase: ScalarPhase, reg_a: u128, staged: ScalarStaged },\n    EnvGet { common: HelperCommon, binder: usize, cursor: Option<usize>,\n        found: Option<RefC>, status: TriStatus },\n    EnvPut { common: HelperCommon, binder: usize, value: RefC, base: EnvRootC,\n        new_root: Option<usize>, status: TriStatus },\n    Find { common: HelperCommon, function_name: usize, row_cursor: usize,\n        found: Option<usize>, status: TriStatus },\n    Bind { common: HelperCommon, function: usize, params: SliceC, args: ValSliceC,\n        p_cursor: usize, a_cursor: usize, partial: EnvRootC,\n        dup_cursor: Option<usize>, phase: BindPhase, status: BindStatus },\n    Append { common: HelperCommon, src: ValSliceC, value: RefC,\n        src_cursor: usize, dest_start: usize, staged: Option<RefC>,\n        phase: AppendPhase },\n    Validate { common: HelperCommon, root: RefC, status: TriStatus },\n    Equal { common: HelperCommon, left: RefC, right: RefC, status: TriStatus },\n    Match { common: HelperCommon, scrutinee: RefC, arm: usize,\n        saved_env: EnvRootC, partial: EnvRootC, pat_tag: PatternTag,\n        x: usize, y: usize, field_idx: u8, staged: Option<RefC>,\n        phase: MatchPhase, status: TriStatus },\n}"
    }
  },
  {
    "name": "FrameRowC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 229,
      "declaration": "pub enum FrameRowC {\n    LetF { binder: usize, body: usize, saved_env: EnvRootC, origin: usize },\n    LeftF { op: BinaryOp, right: usize, saved_env: EnvRootC, origin: usize },\n    RightF { op: BinaryOp, left: RefC, origin: usize },\n    UnaryF { op: UnaryOp, origin: usize },\n    IfF { then_expr: usize, else_expr: usize, saved_env: EnvRootC, origin: usize },\n    MatchF { arms: SliceC, saved_env: EnvRootC, origin: usize },\n    ArgsF { function_name: usize, completed: ValSliceC, rest: SliceC,\n        saved_env: EnvRootC, origin: usize },\n    ReturnF { return_type: TypeIdC, saved_env: EnvRootC, origin: usize },\n    ScopeF { saved_env: EnvRootC, origin: usize },\n    ConstructorFieldsF { type_id: TypeIdC, ctor_id: adt::CtorIdC, index: usize,\n        field_type: TypeIdC, checked: bool, completed: ValSliceC,\n        rest: SliceC, saved_env: EnvRootC, origin: usize },\n}"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 217,
      "declaration": "pub enum FrameRowC {\n    LetF { binder: usize, body: usize, saved_env: EnvRootC, origin: usize },\n    LeftF { op: BinaryOp, right: usize, saved_env: EnvRootC, origin: usize },\n    RightF { op: BinaryOp, left: RefC, origin: usize },\n    UnaryF { op: UnaryOp, origin: usize },\n    IfF { then_expr: usize, else_expr: usize, saved_env: EnvRootC, origin: usize },\n    MatchF { arms: SliceC, saved_env: EnvRootC, origin: usize },\n    ArgsF { function_name: usize, completed: ValSliceC, rest: SliceC,\n        saved_env: EnvRootC, origin: usize },\n    ReturnF { return_type: TypeIdC, saved_env: EnvRootC, origin: usize },\n    ScopeF { saved_env: EnvRootC, origin: usize },\n}"
    }
  },
  {
    "name": "ControlC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 251,
      "declaration": "pub enum ControlC { Eval { expr: usize }, Return { value: RefC },\n    ScheduleNext { frame: FrameRowC, next: usize, env: EnvRootC },\n    Invoke { function_name: usize, args: ValSliceC },\n    Enter { function: Option<usize>, args: ValSliceC },\n    Bound { binding: BoundBinding, return_type: TypeIdC, body: usize },\n    Cases { value: RefC, arms: SliceC, saved_env: EnvRootC },\n    CaseResult { matching: CaseMatching, value: RefC, body: usize,\n        rest: SliceC, saved_env: EnvRootC },\n    PrimResult { result: PrimOutcome }, Error { code: u8 },\n    Materialize { type_id: TypeIdC, ctor_id: adt::CtorIdC, fields: ValSliceC } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 236,
      "declaration": "pub enum ControlC { Eval { expr: usize }, Return { value: RefC },\n    ScheduleNext { frame: FrameRowC, next: usize, env: EnvRootC },\n    Invoke { function_name: usize, args: ValSliceC },\n    Enter { function: Option<usize>, args: ValSliceC },\n    Bound { binding: BoundBinding, return_type: TypeIdC, body: usize },\n    Cases { value: RefC, arms: SliceC, saved_env: EnvRootC },\n    CaseResult { matching: CaseMatching, value: RefC, body: usize,\n        rest: SliceC, saved_env: EnvRootC },\n    PrimResult { result: PrimOutcome }, Error { code: u8 } }"
    }
  },
  {
    "name": "HelperResultC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 268,
      "declaration": "pub enum HelperResultC { Validated { ok: bool }, EnvFound { value: RefC },\n    EnvMissing, EnvUpdated { cell: usize }, FindFound { function: usize },\n    FindMissing, BindOk { env: EnvRootC }, BindBad, AppendDone { seq: ValSliceC },\n    ScalarValue { value: RefC }, ScalarError { code: u8 },\n    ScalarSelector { sel: BoolSelector }, ScalarTypeTest { ok: bool },\n    EqualDone { value: RefC }, MatchMatched { env: EnvRootC }, MatchNoMatch,\n    PlanMatched { arm: usize, env: EnvRootC }, PlanNoMatch, BuildDone { value: RefC } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 252,
      "declaration": "pub enum HelperResultC { Validated { ok: bool }, EnvFound { value: RefC },\n    EnvMissing, EnvUpdated { cell: usize }, FindFound { function: usize },\n    FindMissing, BindOk { env: EnvRootC }, BindBad, AppendDone { seq: ValSliceC },\n    ScalarValue { value: RefC }, ScalarError { code: u8 },\n    ScalarSelector { sel: BoolSelector }, ScalarTypeTest { ok: bool },\n    EqualDone { value: RefC }, MatchMatched { env: EnvRootC }, MatchNoMatch }"
    }
  },
  {
    "name": "PlanNodeC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 332,
      "declaration": "pub enum PlanNodeC {\n    Switch { source_match: usize, path: SliceC, type_id: TypeIdC,\n        cases: SliceC, default: usize },\n    Bind { source_match: usize, path: SliceC, binder: usize, next: usize },\n    Select { source_match: usize, arm: usize },\n    NoMatch { source_match: usize },\n}"
    },
    "old354": null
  },
  {
    "name": "HelperKind",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 71,
      "declaration": "pub enum HelperKind { Scalar, EnvGet, EnvPut, Find, Bind, Append, Validate, Equal, Match, Plan, Build }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 65,
      "declaration": "pub enum HelperKind { Scalar, EnvGet, EnvPut, Find, Bind, Append, Validate, Equal, Match }"
    }
  },
  {
    "name": "FamilyId",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 73,
      "declaration": "pub enum FamilyId { Legacy, Scalar, EnvGet, EnvPut, Find, Bind, Append,\n    Validate, Equal, Match, BookkeepingRelease, Build, Plan }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 67,
      "declaration": "pub enum FamilyId { Legacy, Scalar, EnvGet, EnvPut, Find, Bind, Append,\n    Validate, Equal, Match, BookkeepingRelease }"
    }
  },
  {
    "name": "SiteTag",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 79,
      "declaration": "pub enum SiteTag { EvalLiteral, EvalVar, EvalLet, EvalBinary, EvalUnary, EvalIf,\n    EvalMatch, EvalCall, Push, RetLet, RetLeft, RetRight, RetUnary, RetIf,\n    RetMatch, RetArgs, RetCall, RetScope, Invoke, Enter, Bound, Cases,\n    CaseResult, Prim, EvalConstruct, RetCtorFields, Materialize }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 73,
      "declaration": "pub enum SiteTag { EvalLiteral, EvalVar, EvalLet, EvalBinary, EvalUnary, EvalIf,\n    EvalMatch, EvalCall, Push, RetLet, RetLeft, RetRight, RetUnary, RetIf,\n    RetMatch, RetArgs, RetCall, RetScope, Invoke, Enter, Bound, Cases,\n    CaseResult, Prim }"
    }
  },
  {
    "name": "BuildPhase",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 53,
      "declaration": "pub enum BuildPhase { Copy, Done }"
    },
    "old354": null
  },
  {
    "name": "PlanPhase",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 55,
      "declaration": "pub enum PlanPhase { Walk, Test, Bind, Ready, NoMatchReady, Done }"
    },
    "old354": null
  },
  {
    "name": "PlanAfterWalk",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 57,
      "declaration": "pub enum PlanAfterWalk { Test, Bind }"
    },
    "old354": null
  },
  {
    "name": "RuleIdC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 128,
      "declaration": "pub enum RuleIdC { KMachineEvalLiteralValid,KMachineEvalLiteralInvalid,KMachineEvalVariablePresent,KMachineEvalVariableAbsent,KMachineEvalLet,KMachineEvalBinary,KMachineEvalUnary,KMachineEvalIf,KMachineEvalMatch,KMachineEvalCallEmpty,KMachineEvalCallArguments,KMachinePushAccept,KMachinePushRefuse,KMachineReturnLet,KMachineReturnLeft,KMachineReturnRight,KMachineReturnUnary,KMachineReturnIfTrue,KMachineReturnIfFalse,KMachineReturnIfInvalid,KMachineReturnMatch,KMachineReturnArgumentsNext,KMachineReturnArgumentsFinal,KMachineReturnCallValid,KMachineReturnCallInvalid,KMachineReturnScope,KMachineInvokeFind,KMachineEnterMissing,KMachineEnterBind,KMachineBoundInvalid,KMachineBoundValid,KMachineCasesEmpty,KMachineCasesTry,KMachineCasesMiss,KMachineCasesHit,KMachinePrimitiveValue,KMachinePrimitiveError,BHelperScalarEntry,BHelperScalarTag,BHelperScalarArithmetic,BHelperScalarConstruct,BHelperScalarExit,BHelperScalarFailure,BHelperEnvGetEntry,BHelperEnvGetCell,BHelperEnvGetEmpty,BHelperEnvGetExit,BHelperEnvGetFailure,BHelperEnvPutEntry,BHelperEnvPutCell,BHelperEnvPutInsert,BHelperEnvPutEmpty,BHelperEnvPutExit,BHelperEnvPutFailure,BHelperFindEntry,BHelperFindRow,BHelperFindEmpty,BHelperFindExit,BHelperFindFailure,BHelperBindEntry,BHelperBindPair,BHelperBindEmpty,BHelperBindArity,BHelperBindType,BHelperBindDuplicate,BHelperBindInsert,BHelperBindPropagate,BHelperBindExit,BHelperBindFailure,BHelperAppendEntry,BHelperAppendCell,BHelperAppendEmpty,BHelperAppendTail,BHelperAppendCopy,BHelperAppendExit,BHelperAppendFailure,BHelperValidateEntry,BHelperValidateOccurrence,BHelperValidateField,BHelperValidateDeclaration,BHelperValidateEmpty,BHelperValidateInvalid,BHelperValidateExit,BHelperValidateFailure,BHelperEqualEntry,BHelperEqualPair,BHelperEqualField,BHelperEqualEmpty,BHelperEqualDifferent,BHelperEqualExit,BHelperEqualFailure,BHelperMatchEntry,BHelperMatchArm,BHelperMatchOccurrence,BHelperMatchField,BHelperMatchBinding,BHelperMatchDistinct,BHelperMatchDiscard,BHelperMatchNext,BHelperMatchSelected,BHelperMatchEmpty,BHelperMatchExit,BHelperMatchFailure,BBookkeepingTaskEnqueue,BBookkeepingTaskRelease,BBookkeepingResumeStore,BBookkeepingResumeRestore,KAdminStartupValid,KAdminStartupInvalid,KAdminPublishValue,KAdminPublishError,KAdminPublishExhausted,XAdtEvalConstructNullary,XAdtEvalConstructFields,XAdtReturnCtorFieldsTyped,XAdtReturnCtorFieldsMistyped,XAdtReturnCtorFieldsNext,XAdtReturnCtorFieldsFinal,XAdtMaterializeDone,XAdtCasesPlanHit,XAdtCasesPlanMiss,XAdtHelperBuildEntry,XAdtHelperBuildCell,XAdtHelperBuildField,XAdtHelperBuildExit,XAdtHelperPlanEntry,XAdtHelperPlanWalk,XAdtHelperPlanSwitch,XAdtHelperPlanBind,XAdtHelperPlanSelect,XAdtHelperPlanNoMatch,XAdtHelperPlanExit,XAdtHelperPlanFailure }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 122,
      "declaration": "pub enum RuleIdC { KMachineEvalLiteralValid,KMachineEvalLiteralInvalid,KMachineEvalVariablePresent,KMachineEvalVariableAbsent,KMachineEvalLet,KMachineEvalBinary,KMachineEvalUnary,KMachineEvalIf,KMachineEvalMatch,KMachineEvalCallEmpty,KMachineEvalCallArguments,KMachinePushAccept,KMachinePushRefuse,KMachineReturnLet,KMachineReturnLeft,KMachineReturnRight,KMachineReturnUnary,KMachineReturnIfTrue,KMachineReturnIfFalse,KMachineReturnIfInvalid,KMachineReturnMatch,KMachineReturnArgumentsNext,KMachineReturnArgumentsFinal,KMachineReturnCallValid,KMachineReturnCallInvalid,KMachineReturnScope,KMachineInvokeFind,KMachineEnterMissing,KMachineEnterBind,KMachineBoundInvalid,KMachineBoundValid,KMachineCasesEmpty,KMachineCasesTry,KMachineCasesMiss,KMachineCasesHit,KMachinePrimitiveValue,KMachinePrimitiveError,BHelperScalarEntry,BHelperScalarTag,BHelperScalarArithmetic,BHelperScalarConstruct,BHelperScalarExit,BHelperScalarFailure,BHelperEnvGetEntry,BHelperEnvGetCell,BHelperEnvGetEmpty,BHelperEnvGetExit,BHelperEnvGetFailure,BHelperEnvPutEntry,BHelperEnvPutCell,BHelperEnvPutInsert,BHelperEnvPutEmpty,BHelperEnvPutExit,BHelperEnvPutFailure,BHelperFindEntry,BHelperFindRow,BHelperFindEmpty,BHelperFindExit,BHelperFindFailure,BHelperBindEntry,BHelperBindPair,BHelperBindEmpty,BHelperBindArity,BHelperBindType,BHelperBindDuplicate,BHelperBindInsert,BHelperBindPropagate,BHelperBindExit,BHelperBindFailure,BHelperAppendEntry,BHelperAppendCell,BHelperAppendEmpty,BHelperAppendTail,BHelperAppendCopy,BHelperAppendExit,BHelperAppendFailure,BHelperValidateEntry,BHelperValidateOccurrence,BHelperValidateField,BHelperValidateDeclaration,BHelperValidateEmpty,BHelperValidateInvalid,BHelperValidateExit,BHelperValidateFailure,BHelperEqualEntry,BHelperEqualPair,BHelperEqualField,BHelperEqualEmpty,BHelperEqualDifferent,BHelperEqualExit,BHelperEqualFailure,BHelperMatchEntry,BHelperMatchArm,BHelperMatchOccurrence,BHelperMatchField,BHelperMatchBinding,BHelperMatchDistinct,BHelperMatchDiscard,BHelperMatchNext,BHelperMatchSelected,BHelperMatchEmpty,BHelperMatchExit,BHelperMatchFailure,BBookkeepingTaskEnqueue,BBookkeepingTaskRelease,BBookkeepingResumeStore,BBookkeepingResumeRestore,KAdminStartupValid,KAdminStartupInvalid,KAdminPublishValue,KAdminPublishError,KAdminPublishExhausted }"
    }
  },
  {
    "name": "BoundBinding",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 245,
      "declaration": "pub enum BoundBinding { Bad, Ok { env: EnvRootC } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 230,
      "declaration": "pub enum BoundBinding { Bad, Ok { env: EnvRootC } }"
    }
  },
  {
    "name": "PrimOutcome",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 249,
      "declaration": "pub enum PrimOutcome { Value { value: RefC }, Error { code: u8 } }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 234,
      "declaration": "pub enum PrimOutcome { Value { value: RefC }, Error { code: u8 } }"
    }
  },
  {
    "name": "RefC",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 169,
      "declaration": "pub enum RefC { Const(usize), Data(usize) }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 163,
      "declaration": "pub enum RefC { Const(usize), Data(usize) }"
    }
  },
  {
    "name": "HelperCommon",
    "selected": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 49413,
        "sha256": "a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2"
      },
      "line": 187,
      "declaration": "pub struct HelperCommon { pub site: SiteTag, pub origin_reference_cursor: u128,\n    pub parent_resume: Option<usize> }"
    },
    "old354": {
      "source": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-S1-canonical-Core-integration-publication-20261019/source-preimages/crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "bytes": 44525,
        "sha256": "8092ce1e67b5eaeca418a57418b8b8efbaf89d80e9a01626def7262f2fcf6df5"
      },
      "line": 181,
      "declaration": "pub struct HelperCommon { pub site: SiteTag, pub origin_reference_cursor: u128,\n    pub parent_resume: Option<usize> }"
    }
  }
]

```


===== Complete original public API contract lacking direct DONE view =====
/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/core-cfgtest-generic-abi-inventory-20261019/task-B-opus-source-author/ACTUAL-CONSUMER-WIRE.json
Bytes 8975 SHA256 8c30b9d26c9e97a62323265ea378ffcb0facdbe767c6bc626041596470c08c3f
```text
{
  "source": {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-adt-C6-ABI-F7-S1-canonical-union-source-20261019/snapshot/crates/mori-core/src/sherlock/compile_artifact/wire.rs",
    "bytes": 75164,
    "sha256": "ece132802f049552460d45a78cbf0d40251bd099739af28398e1cb2b7564bb85"
  },
  "implementations": [
    {
      "type": "ExpressionRowC",
      "line": 269,
      "implementation": "impl OwnerWire for ExpressionRowC {\n    fn owner_wire(&self) -> Value {\n        match self {\n            Self::Literal { constant } => json!({\"Literal\": {\"constant\": constant.owner_wire()}}),\n            Self::Var { binder } => json!({\"Var\": {\"binder\": binder.owner_wire()}}),\n            Self::Let { binder, value, body } => json!({\"Let\": {\"binder\": binder.owner_wire(), \"value\": value.owner_wire(), \"body\": body.owner_wire()}}),\n            Self::Binary { op, left, right } => json!({\"Binary\": {\"op\": op.owner_wire(), \"left\": left.owner_wire(), \"right\": right.owner_wire()}}),\n            Self::Unary { op, operand } => json!({\"Unary\": {\"op\": op.owner_wire(), \"operand\": operand.owner_wire()}}),\n            Self::If { cond, then_expr, else_expr } => json!({\"If\": {\"cond\": cond.owner_wire(), \"then_expr\": then_expr.owner_wire(), \"else_expr\": else_expr.owner_wire()}}),\n            Self::Call { function_name, args } => json!({\"Call\": {\"function_name\": function_name.owner_wire(), \"args\": args.owner_wire()}}),\n            Self::Match { scrutinee, arms, plan } => json!({\"Match\": {\"scrutinee\": scrutinee.owner_wire(), \"arms\": arms.owner_wire(), \"plan\": plan.owner_wire()}}),\n            Self::Construct { type_id, ctor_id, fields, constant } => json!({\"Construct\": {\"type_id\": type_id.owner_wire(), \"ctor_id\": ctor_id.owner_wire(), \"fields\": fields.owner_wire(), \"constant\": constant.owner_wire()}}),\n        }\n    }\n}"
    },
    {
      "type": "PatternRowC",
      "line": 602,
      "implementation": "impl OwnerWire for PatternRowC {\n    fn owner_wire(&self) -> Value {\n        match self {\n            Self::PVar { x } => json!({\"PVar\": {\"x\": x.owner_wire()}}),\n            Self::PNil => json!(\"PNil\"),\n            Self::PCons { x, y } => json!({\"PCons\": {\"x\": x.owner_wire(), \"y\": y.owner_wire()}}),\n            Self::PLeaf { x } => json!({\"PLeaf\": {\"x\": x.owner_wire()}}),\n            Self::PBranch { x, y } => json!({\"PBranch\": {\"x\": x.owner_wire(), \"y\": y.owner_wire()}}),\n            Self::Wildcard => json!(\"Wildcard\"),\n            Self::Constructor { type_id, ctor_id, children } => json!({\"Constructor\": {\"type_id\": type_id.owner_wire(), \"ctor_id\": ctor_id.owner_wire(), \"children\": children.owner_wire()}}),\n        }\n    }\n}"
    },
    {
      "type": "ProgramView",
      "line": 801,
      "implementation": "impl OwnerWire for ProgramView {\n    fn owner_wire(&self) -> Value {\n        let ProgramView {\n            entry, expressions, occurrences, occurrence_paths, children, functions, parameters,\n            arms, patterns, pattern_children, types, type_args, constructors, ctor_fields,\n            decl_param_counts, decl_ctor_counts, decl_templates, plan_nodes, plan_cases,\n            field_paths, constants, constant_fields, names,\n        } = self;\n        json!({\n            \"entry\": entry.owner_wire(), \"expressions\": expressions.owner_wire(),\n            \"occurrences\": occurrences.owner_wire(), \"occurrence_paths\": occurrence_paths.owner_wire(),\n            \"children\": children.owner_wire(), \"functions\": functions.owner_wire(),\n            \"parameters\": parameters.owner_wire(), \"arms\": arms.owner_wire(),\n            \"patterns\": patterns.owner_wire(), \"pattern_children\": pattern_children.owner_wire(),\n            \"types\": types.owner_wire(), \"type_args\": type_args.owner_wire(),\n            \"constructors\": constructors.owner_wire(), \"ctor_fields\": ctor_fields.owner_wire(),\n            \"decl_param_counts\": decl_param_counts.owner_wire(),\n            \"decl_ctor_counts\": decl_ctor_counts.owner_wire(),\n            \"decl_templates\": decl_templates.owner_wire(),\n            \"plan_nodes\": plan_nodes.owner_wire(), \"plan_cases\": plan_cases.owner_wire(),\n            \"field_paths\": field_paths.owner_wire(), \"constants\": constants.owner_wire(),\n            \"constant_fields\": constant_fields.owner_wire(), \"names\": names.owner_wire(),\n        })\n    }\n}"
    },
    {
      "type": "RawConstRow",
      "line": 827,
      "implementation": "impl OwnerWire for RawConstRow {\n    fn owner_wire(&self) -> Value {\n        match self {\n            Self::U128 { marker_free, n } => json!({\"U128\": {\"marker_free\": marker_free.owner_wire(), \"n\": n.owner_wire()}}),\n            Self::Bool { marker_free, b } => json!({\"Bool\": {\"marker_free\": marker_free.owner_wire(), \"b\": b.owner_wire()}}),\n            Self::Nil { marker_free } => json!({\"Nil\": {\"marker_free\": marker_free.owner_wire()}}),\n            Self::Cons { marker_free, fields } => json!({\"Cons\": {\"marker_free\": marker_free.owner_wire(), \"fields\": fields.owner_wire()}}),\n            Self::Leaf { marker_free, fields } => json!({\"Leaf\": {\"marker_free\": marker_free.owner_wire(), \"fields\": fields.owner_wire()}}),\n            Self::Branch { marker_free, fields } => json!({\"Branch\": {\"marker_free\": marker_free.owner_wire(), \"fields\": fields.owner_wire()}}),\n            Self::Constructor { marker_free, type_id, ctor_id, fields } => json!({\"Constructor\": {\"marker_free\": marker_free.owner_wire(), \"type_id\": TypeIdC(type_id.0).owner_wire(), \"ctor_id\": adt::CtorIdC(ctor_id.0).owner_wire(), \"fields\": fields.owner_wire()}}),\n            Self::U128OutOfRange { marker_free } => json!({\"U128OutOfRange\": {\"marker_free\": marker_free.owner_wire()}}),\n        }\n    }\n}"
    },
    {
      "type": "RawDataRow",
      "line": 842,
      "implementation": "impl OwnerWire for RawDataRow {\n    fn owner_wire(&self) -> Value {\n        match self {\n            Self::U128 { n } => json!({\"U128\": {\"n\": n.owner_wire()}}),\n            Self::Bool { b } => json!({\"Bool\": {\"b\": b.owner_wire()}}),\n            Self::Nil => json!(\"Nil\"),\n            Self::Cons { fields } => json!({\"Cons\": {\"fields\": fields.owner_wire()}}),\n            Self::Leaf { fields } => json!({\"Leaf\": {\"fields\": fields.owner_wire()}}),\n            Self::Branch { fields } => json!({\"Branch\": {\"fields\": fields.owner_wire()}}),\n            Self::Constructor { type_id, ctor_id, fields } => json!({\"Constructor\": {\"type_id\": TypeIdC(type_id.0).owner_wire(), \"ctor_id\": adt::CtorIdC(ctor_id.0).owner_wire(), \"fields\": fields.owner_wire()}}),\n        }\n    }\n}"
    },
    {
      "type": "FunctionRowC",
      "line": 336,
      "implementation": "impl OwnerWire for FunctionRowC {\n    fn owner_wire(&self) -> Value {\n        let FunctionRowC { function_name, params, return_type, body } = self;\n        json!({\"function_name\": function_name.owner_wire(), \"params\": params.owner_wire(), \"return_type\": return_type.owner_wire(), \"body\": body.owner_wire()})\n    }\n}"
    },
    {
      "type": "ParameterRowC",
      "line": 595,
      "implementation": "impl OwnerWire for ParameterRowC {\n    fn owner_wire(&self) -> Value {\n        let ParameterRowC { binder, sort } = self;\n        json!({\"binder\": binder.owner_wire(), \"sort\": sort.owner_wire()})\n    }\n}"
    },
    {
      "type": "TypeIdC",
      "line": 1118,
      "implementation": "impl OwnerWire for TypeIdC {\n    fn owner_wire(&self) -> Value {\n        match *self {\n            Self::UInt => json!(\"UInt\"),\n            Self::Bool => json!(\"Bool\"),\n            Self::List => json!(\"List\"),\n            Self::Tree => json!(\"Tree\"),\n            _ => json!({\"TypeId\": self.0.owner_wire()}),\n        }\n    }\n}"
    },
    {
      "type": "SortTag",
      "line": 1130,
      "implementation": "impl OwnerWire for SortTag {\n    fn owner_wire(&self) -> Value {\n        match self {\n            Self::UInt => json!(\"UInt\"),\n            Self::Bool => json!(\"Bool\"),\n            Self::List => json!(\"List\"),\n            Self::Tree => json!(\"Tree\"),\n        }\n    }\n}"
    },
    {
      "type": "adt::CtorIdC",
      "line": 617,
      "implementation": "impl OwnerWire for adt::CtorIdC {\n    fn owner_wire(&self) -> Value { self.0.owner_wire() }\n}"
    },
    {
      "type": "SliceC",
      "line": 1111,
      "implementation": "impl OwnerWire for SliceC {\n    fn owner_wire(&self) -> Value {\n        let SliceC { start, len } = self;\n        json!({\"start\": start.owner_wire(), \"len\": len.owner_wire()})\n    }\n}"
    }
  ],
  "use": "Review exact current output compatibility while preserving independent test serializers; no imported production serializer as expected oracle."
}

```


===== Exact original33-operation public source delta, unchanged =====
/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/core-cfgtest-generic-abi-exact-AB-candidate-20261019/EXACT-AB.diff
Bytes 34382 SHA256 c02a098ba28968e97caec389121dd8c71c508c7929c26867fc7e9e9655ccd1d7
```text
--- a/crates/mori-core/src/sherlock/helper_machine/core_action_consumer_security_operator_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/core_action_consumer_security_operator_tests.rs
@@ -64,6 +64,7 @@
         scalar_layout_id: None,
         bounds,
         capacities,
+        ext_capacities: None,
         boundary_capacities_present: false,
     };
     ConsumerConfig {
--- a/crates/mori-core/src/sherlock/helper_machine/core_action_consumer_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/core_action_consumer_tests.rs
@@ -40,7 +40,7 @@
             capacities: LayoutCapacitiesC {
                 name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 1024,
             },
-            boundary_capacities_present: false,
+            ext_capacities: None, boundary_capacities_present: false,
         },
         fuel: RunIntC::Nat(1000),
         stack: RunIntC::Nat(32),
--- a/crates/mori-core/src/sherlock/helper_machine/core_action_operator_pipeline_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/core_action_operator_pipeline_tests.rs
@@ -57,7 +57,7 @@
         scalar_layout_id: None,
         bounds,
         capacities,
-        boundary_capacities_present: false,
+        ext_capacities: None, boundary_capacities_present: false,
     };
     ConsumerConfig {
         bounds,
--- a/crates/mori-core/src/sherlock/helper_machine/core_action_resource_integration_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/core_action_resource_integration_tests.rs
@@ -29,7 +29,7 @@
             capacities: LayoutCapacitiesC {
                 name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 1024,
             },
-            boundary_capacities_present: false,
+            ext_capacities: None, boundary_capacities_present: false,
         },
         fuel: RunIntC::Nat(1000),
         stack: RunIntC::Nat(32),
--- a/crates/mori-core/src/sherlock/helper_machine/resource_report_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/resource_report_tests.rs
@@ -56,7 +56,7 @@
             capacities: LayoutCapacitiesC {
                 name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 1024,
             },
-            boundary_capacities_present: false,
+            ext_capacities: None, boundary_capacities_present: false,
         },
         fuel: RunIntC::Nat(1000), stack: RunIntC::Nat(32),
         profile: ConsumerProfile::SherlockHelperMicrostep1,
--- a/crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare/nominal_decl_producer_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare/nominal_decl_producer_tests.rs
@@ -27,7 +27,7 @@
             occurrence_path_slots: 32,
             value_slots: 32,
         },
-        boundary_capacities_present: false,
+        ext_capacities: None, boundary_capacities_present: false,
     }
 }
 
--- a/crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare/v4_empty_params_tests.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare/v4_empty_params_tests.rs
@@ -40,7 +40,7 @@
             occurrence_path_slots: 256,
             value_slots: 4096,
         },
-        boundary_capacities_present: false,
+        ext_capacities: None, boundary_capacities_present: false,
     };
     (b, l)
 }
--- a/crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs
+++ b/crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs
@@ -1883,6 +1883,7 @@
             field_id: None, widths: None, scalar_layout_id: None, bounds: *b,
             capacities: LayoutCapacitiesC { name_rows: 16, name_bytes: 128,
                 occurrence_path_slots: 128, value_slots: 1000 },
+            ext_capacities: None,
             boundary_capacities_present: false }
     }
     fn deps() -> PrepareDeps {
@@ -1939,7 +1940,8 @@
     // ---------- full MachineImage field snapshot for preservation ---------
     #[derive(Debug, PartialEq)]
     enum SnapDataRow { U128(u128), Bool(bool), Nil,
-        Cons(usize, usize), Leaf(usize, usize), Branch(usize, usize) }
+        Cons(usize, usize), Leaf(usize, usize), Branch(usize, usize),
+        Constructor(usize, usize, usize, usize) }
     fn snap_data_row(r: &crate::sherlock::helper_machine::raw::DataRow) -> SnapDataRow {
         match r {
             crate::sherlock::helper_machine::raw::DataRow::U128 { n } => SnapDataRow::U128(*n),
@@ -1948,6 +1950,7 @@
             crate::sherlock::helper_machine::raw::DataRow::Cons { fields } => SnapDataRow::Cons(fields.start.0, fields.len),
             crate::sherlock::helper_machine::raw::DataRow::Leaf { fields } => SnapDataRow::Leaf(fields.start.0, fields.len),
             crate::sherlock::helper_machine::raw::DataRow::Branch { fields } => SnapDataRow::Branch(fields.start.0, fields.len),
+            crate::sherlock::helper_machine::raw::DataRow::Constructor { type_id, ctor_id, fields } => SnapDataRow::Constructor(type_id.0, ctor_id.0, fields.start.0, fields.len),
         }
     }
     #[derive(Debug, PartialEq)]
@@ -2305,6 +2308,8 @@
         fn plan_validate(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_validate(s, p, g)) }
         fn plan_equal(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_equal(s, p, g)) }
         fn plan_match(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_match(s, p, g)) }
+        fn plan_build(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_build(s, p, g)) }
+        fn plan_plan(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_plan(s, p, g)) }
         fn plan_bookkeeping_release(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_bookkeeping_release(s, p, g)) }
     }
     #[test]
--- a/crates/mori-core/tests/sherlock_complete_comparison_positive.rs
+++ b/crates/mori-core/tests/sherlock_complete_comparison_positive.rs
@@ -34,7 +34,7 @@
         layout_version:("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
         field_id:None, widths:None, scalar_layout_id:None, bounds,
         capacities:LayoutCapacitiesC { name_rows:32, name_bytes:256, occurrence_path_slots:256, value_slots:1024 },
-        boundary_capacities_present:false,
+        ext_capacities: None, boundary_capacities_present:false,
     };
     let owner = host_owner_engine();
     let admitted = owner.admit(&source, &bounds, &layout)
--- a/crates/mori-core/tests/sherlock_complete_observation_positive.rs
+++ b/crates/mori-core/tests/sherlock_complete_observation_positive.rs
@@ -28,7 +28,7 @@
         layout_version:("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
         field_id:None, widths:None, scalar_layout_id:None, bounds,
         capacities:LayoutCapacitiesC { name_rows:32, name_bytes:256, occurrence_path_slots:256, value_slots:1024 },
-        boundary_capacities_present:false,
+        ext_capacities: None, boundary_capacities_present:false,
     };
     let owner = host_owner_engine();
     let admitted = owner.admit(&source, &bounds, &layout)
--- a/crates/mori-core/tests/sherlock_draft_runtime_independent.rs
+++ b/crates/mori-core/tests/sherlock_draft_runtime_independent.rs
@@ -6,7 +6,7 @@
 use mori_core::sherlock::helper_machine::shared::{
     BinaryOp, BoundaryKind, ControlC, EventKind, ExpressionRowC, FinanceInput,
     GrammarDomain, LocatorC, OutcomeC, OutcomeTagC, PatternRowC, ProgramView,
-    RefC, RuleIdC, RunIntC, SliceC, SortTag, StepDisposition, StepEventC, UnaryOp,
+    RefC, RuleIdC, RunIntC, SliceC, SortTag, StepDisposition, StepEventC, TypeIdC, UnaryOp,
 };
 use serde_json::{json, Value};
 
@@ -19,8 +19,8 @@
 
 fn dec(n: usize) -> String { n.to_string() }
 fn slice(s: SliceC) -> Value { json!({"start":dec(s.start),"len":dec(s.len)}) }
-fn sort(s: SortTag) -> &'static str {
-    match s { SortTag::UInt=>"UInt",SortTag::Bool=>"Bool",SortTag::List=>"List",SortTag::Tree=>"Tree" }
+fn sort(s: TypeIdC) -> &'static str {
+    match s { TypeIdC::UInt=>"UInt",TypeIdC::Bool=>"Bool",TypeIdC::List=>"List",TypeIdC::Tree=>"Tree", _=>panic!("unexpected generic type id in sealed dedicated program") }
 }
 fn binary(op: BinaryOp) -> &'static str {
     match op { BinaryOp::Add=>"Add",BinaryOp::Sub=>"Sub",BinaryOp::Mul=>"Mul",
@@ -38,7 +38,8 @@
         ExpressionRowC::Unary{op,operand}=>json!({"Unary":{"op":unary(*op),"operand":dec(*operand)}}),
         ExpressionRowC::If{cond,then_expr,else_expr}=>json!({"If":{"cond":dec(*cond),"then_expr":dec(*then_expr),"else_expr":dec(*else_expr)}}),
         ExpressionRowC::Call{function_name,args}=>json!({"Call":{"function_name":dec(*function_name),"args":slice(*args)}}),
-        ExpressionRowC::Match{scrutinee,arms}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms)}}),
+        ExpressionRowC::Match{scrutinee,arms,plan}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms),"plan":(*plan).map(dec)}}),
+        ExpressionRowC::Construct{type_id,ctor_id,fields,constant}=>json!({"Construct":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"fields":slice(*fields),"constant":(*constant).map(dec)}}),
     }).collect();
     let occurrences: Vec<Value> = view.occurrences.iter().map(|r|
         json!({"owner_is_function":r.owner_is_function,"function":dec(r.function),"path":slice(r.path)})).collect();
@@ -52,6 +53,8 @@
         PatternRowC::PCons{x,y}=>json!({"PCons":{"x":dec(*x),"y":dec(*y)}}),
         PatternRowC::PLeaf{x}=>json!({"PLeaf":{"x":dec(*x)}}),
         PatternRowC::PBranch{x,y}=>json!({"PBranch":{"x":dec(*x),"y":dec(*y)}}),
+        PatternRowC::Wildcard=>json!("Wildcard"),
+        PatternRowC::Constructor{type_id,ctor_id,children}=>json!({"Constructor":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"children":slice(*children)}}),
     }).collect();
     let constants: Vec<Value> = view.constants.iter().map(|r|match r {
         ConstRow::U128{marker_free,n}=>json!({"U128":{"marker_free":marker_free,"n":n.to_string()}}),
--- a/crates/mori-core/tests/sherlock_witness_baseline_independent.rs
+++ b/crates/mori-core/tests/sherlock_witness_baseline_independent.rs
@@ -25,6 +25,7 @@
         wp::ValueNode::Nil=>json!(["Nil"]),wp::ValueNode::U128OutOfRange=>json!(["U128OutOfRange"]),
         wp::ValueNode::Cons{head,tail}=>json!(["Cons",head,tail]),wp::ValueNode::Leaf{value}=>json!(["Leaf",value]),
         wp::ValueNode::Branch{left,right}=>json!(["Branch",left,right]),
+        wp::ValueNode::Constructor{type_id,ctor_id,fields}=>json!(["Constructor",type_id.0,ctor_id.0,fields]),
     }).collect();
     json!({"nodes":nodes,"root":v.root})
 }
@@ -47,6 +48,11 @@
         Occurrence(i)=>("occurrence",Some(*i)),OccurrencePath(i)=>("occurrence_path",Some(*i)),Child(i)=>("child",Some(*i)),
         Frame(i)=>("frame",Some(*i)),ScheduledFrame=>("scheduled_frame",None),Control=>("control",None),
         Helper=>("helper",None),Task(i)=>("task",Some(*i)),Result=>("result",None),Outcome=>("outcome",None),ValueSlot(i)=>("value_slot",Some(*i)),
+        PatternChild(i)=>("pattern_child",Some(*i)),Type(i)=>("type",Some(*i)),TypeArg(i)=>("type_arg",Some(*i)),
+        Constructor(i)=>("constructor",Some(*i)),ConstructorField(i)=>("constructor_field",Some(*i)),
+        DeclParamCount(i)=>("decl_param_count",Some(*i)),DeclCtorCount(i)=>("decl_ctor_count",Some(*i)),
+        DeclTemplate(i)=>("decl_template",Some(*i)),PlanNode(i)=>("plan_node",Some(*i)),
+        PlanCase(i)=>("plan_case",Some(*i)),FieldPath(i)=>("field_path",Some(*i)),
     }
 }
 fn check_plan(p:&wp::WitnessPlan){
@@ -128,7 +134,7 @@
     let n=|name:&str|b[name].as_str().unwrap().parse::<usize>().unwrap();
     let bounds=BoundsC{code_rows:n("code_rows"),function_rows:n("function_rows"),code_child_slots:n("code_child_slots"),parameter_slots:n("parameter_slots"),pattern_rows:n("pattern_rows"),match_plan_rows:n("match_plan_rows"),match_arm_rows:n("match_arm_rows"),type_rows:n("type_rows"),constructor_rows:n("constructor_rows"),constructor_field_slots:n("constructor_field_slots"),max_arity:n("max_arity"),constant_nodes:n("constant_nodes"),constant_field_slots:n("constant_field_slots"),data_nodes:n("data_nodes"),data_field_slots:n("data_field_slots"),environment_cells:n("environment_cells"),frames:n("frames"),helper_tasks:n("helper_tasks"),serialization_bytes:n("serialization_bytes"),max_reference_work:n("max_reference_work") as u128,max_helper_work:n("max_helper_work") as u128,step_cap:n("step_cap") as u128};
     let cap=&c["layout"]["capacities"];let k=|name:&str|cap[name].as_str().unwrap().parse::<usize>().unwrap();
-    let layout=LayoutSchemaC{layout_version:(c["layout"]["layout_version"][0].as_str().unwrap().into(),c["layout"]["layout_version"][1].as_str().unwrap().into()),field_id:None,widths:None,scalar_layout_id:None,bounds,capacities:LayoutCapacitiesC{name_rows:k("name_rows"),name_bytes:k("name_bytes"),occurrence_path_slots:k("occurrence_path_slots"),value_slots:k("value_slots")},boundary_capacities_present:false};(bounds,layout)
+    let layout=LayoutSchemaC{layout_version:(c["layout"]["layout_version"][0].as_str().unwrap().into(),c["layout"]["layout_version"][1].as_str().unwrap().into()),field_id:None,widths:None,scalar_layout_id:None,bounds,capacities:LayoutCapacitiesC{name_rows:k("name_rows"),name_bytes:k("name_bytes"),occurrence_path_slots:k("occurrence_path_slots"),value_slots:k("value_slots")},ext_capacities: None, boundary_capacities_present:false};(bounds,layout)
 }
 fn pair(p:&CoreProgram)->(Admitted,MachineImage){
     let (b,l)=config();let owner=host_owner_engine();let a=owner.admit(p,&b,&l).unwrap_or_else(|_|panic!("fixed positive admission"));
--- a/crates/mori-core/tests/support/generic_adt_one_builder_checks.rs
+++ b/crates/mori-core/tests/support/generic_adt_one_builder_checks.rs
@@ -208,7 +208,7 @@
             name_rows:usize_cell(c,"name_rows"),name_bytes:usize_cell(c,"name_bytes"),
             occurrence_path_slots:usize_cell(c,"occurrence_path_slots"),value_slots:usize_cell(c,"value_slots"),
         },
-        boundary_capacities_present:input["boundary_capacities_present"].as_bool().unwrap(),
+        ext_capacities: None, boundary_capacities_present:input["boundary_capacities_present"].as_bool().unwrap(),
     }
 }
 
--- a/crates/mori-core/tests/support/nominal_text_source_positive.rs
+++ b/crates/mori-core/tests/support/nominal_text_source_positive.rs
@@ -20,7 +20,7 @@
             constant_field_slots:128,data_nodes:128,data_field_slots:256,environment_cells:128,
             frames:64,helper_tasks:256,serialization_bytes:65536,max_reference_work:4096,
             max_helper_work:4096,step_cap:1},
-        capacities:LayoutCapacitiesC {name_rows:256,name_bytes:4096,occurrence_path_slots:1024,value_slots:256},boundary_capacities_present:false}
+        capacities:LayoutCapacitiesC {name_rows:256,name_bytes:4096,occurrence_path_slots:1024,value_slots:256},ext_capacities: None, boundary_capacities_present:false}
 }
 fn ty(t:&TypeExprSrc)->String {
     match t {
--- a/crates/mori-core/tests/support/sherlock_core1_provenance_independent.rs
+++ b/crates/mori-core/tests/support/sherlock_core1_provenance_independent.rs
@@ -11,7 +11,7 @@
 use mori_core::sherlock::helper_machine::shared::{
     ActiveLengths, AllocCounters, BinaryOp, ControlC, ExpressionRowC, FinanceInput,
     GrammarDomain, LocatorC, OutcomeC, PatternRowC, ProgramView, RefC, RuleIdC,
-    RunIntC, SliceC, SortTag, StepEventC, UnaryOp,
+    RunIntC, SliceC, SortTag, StepEventC, TypeIdC, UnaryOp,
 };
 use serde_json::{json, Value};
 use std::collections::BTreeMap;
@@ -24,8 +24,8 @@
 
 fn dec(n: usize) -> String { n.to_string() }
 fn slice(s: SliceC) -> Value { json!({"start":dec(s.start),"len":dec(s.len)}) }
-fn sort(s: SortTag) -> &'static str {
-    match s { SortTag::UInt=>"UInt",SortTag::Bool=>"Bool",SortTag::List=>"List",SortTag::Tree=>"Tree" }
+fn sort(s: TypeIdC) -> &'static str {
+    match s { TypeIdC::UInt=>"UInt",TypeIdC::Bool=>"Bool",TypeIdC::List=>"List",TypeIdC::Tree=>"Tree", _=>panic!("unexpected generic type id in sealed dedicated program") }
 }
 fn binary(op: BinaryOp) -> &'static str {
     match op { BinaryOp::Add=>"Add",BinaryOp::Sub=>"Sub",BinaryOp::Mul=>"Mul",
@@ -43,7 +43,8 @@
         ExpressionRowC::Unary{op,operand}=>json!({"Unary":{"op":unary(*op),"operand":dec(*operand)}}),
         ExpressionRowC::If{cond,then_expr,else_expr}=>json!({"If":{"cond":dec(*cond),"then_expr":dec(*then_expr),"else_expr":dec(*else_expr)}}),
         ExpressionRowC::Call{function_name,args}=>json!({"Call":{"function_name":dec(*function_name),"args":slice(*args)}}),
-        ExpressionRowC::Match{scrutinee,arms}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms)}}),
+        ExpressionRowC::Match{scrutinee,arms,plan}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms),"plan":(*plan).map(dec)}}),
+        ExpressionRowC::Construct{type_id,ctor_id,fields,constant}=>json!({"Construct":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"fields":slice(*fields),"constant":(*constant).map(dec)}}),
     }).collect();
     let occurrences: Vec<Value> = view.occurrences.iter().map(|r|
         json!({"owner_is_function":r.owner_is_function,"function":dec(r.function),"path":slice(r.path)})).collect();
@@ -57,6 +58,8 @@
         PatternRowC::PCons{x,y}=>json!({"PCons":{"x":dec(*x),"y":dec(*y)}}),
         PatternRowC::PLeaf{x}=>json!({"PLeaf":{"x":dec(*x)}}),
         PatternRowC::PBranch{x,y}=>json!({"PBranch":{"x":dec(*x),"y":dec(*y)}}),
+        PatternRowC::Wildcard=>json!("Wildcard"),
+        PatternRowC::Constructor{type_id,ctor_id,children}=>json!({"Constructor":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"children":slice(*children)}}),
     }).collect();
     let constants: Vec<Value> = view.constants.iter().map(|r|match r {
         ConstRow::U128{marker_free,n}=>json!({"U128":{"marker_free":marker_free,"n":n.to_string()}}),
--- a/crates/mori-core/tests/support/sherlock_nominal_declarations_independent.rs
+++ b/crates/mori-core/tests/support/sherlock_nominal_declarations_independent.rs
@@ -23,7 +23,7 @@
         field_id: None, widths: None, scalar_layout_id: None,
         bounds: BoundsC { type_rows:32, constructor_rows:32, constructor_field_slots:64, max_arity:4, ..BoundsC::default() },
         capacities: LayoutCapacitiesC { name_rows:32, name_bytes:256, occurrence_path_slots:0, value_slots:0 },
-        boundary_capacities_present:false,
+        ext_capacities: None, boundary_capacities_present:false,
     };
     let prepared = prepare_nominal_declarations(&decls, &layout, GrammarDomain::GenericNominalExtensionReserved, None)
         .unwrap_or_else(|_| panic!("normal declaration preparation returned an error"));
--- a/crates/mori-core/tests/support/sherlock_provenance_twins_independent.rs
+++ b/crates/mori-core/tests/support/sherlock_provenance_twins_independent.rs
@@ -6,7 +6,7 @@
 use mori_core::sherlock::helper_machine::shared::{
     BinaryOp, ControlC, ExpressionRowC, FinanceInput, GrammarDomain, LocatorC,
     OutcomeC, PatternRowC, ProgramView, RefC, RuleIdC, RunIntC, SliceC, SortTag,
-    StepEventC, UnaryOp,
+    StepEventC, TypeIdC, UnaryOp,
 };
 use serde_json::{json, Value};
 use std::io::Write;
@@ -18,8 +18,8 @@
 
 fn dec(n: usize) -> String { n.to_string() }
 fn slice(s: SliceC) -> Value { json!({"start":dec(s.start),"len":dec(s.len)}) }
-fn sort(s: SortTag) -> &'static str {
-    match s { SortTag::UInt=>"UInt",SortTag::Bool=>"Bool",SortTag::List=>"List",SortTag::Tree=>"Tree" }
+fn sort(s: TypeIdC) -> &'static str {
+    match s { TypeIdC::UInt=>"UInt",TypeIdC::Bool=>"Bool",TypeIdC::List=>"List",TypeIdC::Tree=>"Tree", _=>panic!("unexpected generic type id in sealed dedicated program") }
 }
 fn binary(op: BinaryOp) -> &'static str {
     match op { BinaryOp::Add=>"Add",BinaryOp::Sub=>"Sub",BinaryOp::Mul=>"Mul",
@@ -37,7 +37,8 @@
         ExpressionRowC::Unary{op,operand}=>json!({"Unary":{"op":unary(*op),"operand":dec(*operand)}}),
         ExpressionRowC::If{cond,then_expr,else_expr}=>json!({"If":{"cond":dec(*cond),"then_expr":dec(*then_expr),"else_expr":dec(*else_expr)}}),
         ExpressionRowC::Call{function_name,args}=>json!({"Call":{"function_name":dec(*function_name),"args":slice(*args)}}),
-        ExpressionRowC::Match{scrutinee,arms}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms)}}),
+        ExpressionRowC::Match{scrutinee,arms,plan}=>json!({"Match":{"scrutinee":dec(*scrutinee),"arms":slice(*arms),"plan":(*plan).map(dec)}}),
+        ExpressionRowC::Construct{type_id,ctor_id,fields,constant}=>json!({"Construct":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"fields":slice(*fields),"constant":(*constant).map(dec)}}),
     }).collect();
     let occurrences: Vec<Value> = view.occurrences.iter().map(|r|
         json!({"owner_is_function":r.owner_is_function,"function":dec(r.function),"path":slice(r.path)})).collect();
@@ -51,6 +52,8 @@
         PatternRowC::PCons{x,y}=>json!({"PCons":{"x":dec(*x),"y":dec(*y)}}),
         PatternRowC::PLeaf{x}=>json!({"PLeaf":{"x":dec(*x)}}),
         PatternRowC::PBranch{x,y}=>json!({"PBranch":{"x":dec(*x),"y":dec(*y)}}),
+        PatternRowC::Wildcard=>json!("Wildcard"),
+        PatternRowC::Constructor{type_id,ctor_id,children}=>json!({"Constructor":{"type_id":dec(type_id.0),"ctor_id":dec(ctor_id.0),"children":slice(*children)}}),
     }).collect();
     let constants: Vec<Value> = view.constants.iter().map(|r|match r {
         ConstRow::U128{marker_free,n}=>json!({"U128":{"marker_free":marker_free,"n":n.to_string()}}),
--- a/crates/mori-core/tests/support/sherlock_resolved_signature_positive.rs
+++ b/crates/mori-core/tests/support/sherlock_resolved_signature_positive.rs
@@ -44,7 +44,7 @@
         capacities: LayoutCapacitiesC {
             name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 4096,
         },
-        boundary_capacities_present: false,
+        ext_capacities: None, boundary_capacities_present: false,
     };
     (b, l)
 }
--- a/crates/mori-core/tests/support/sherlock_witness_cache_stream_positive.rs
+++ b/crates/mori-core/tests/support/sherlock_witness_cache_stream_positive.rs
@@ -38,7 +38,7 @@
         field_id: None, widths: None, scalar_layout_id: None, bounds,
         capacities: LayoutCapacitiesC {
             name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 4096,
-        }, boundary_capacities_present: false,
+        }, ext_capacities: None, boundary_capacities_present: false,
     };
     (bounds, schema)
 }
--- /dev/null
+++ b/crates/mori-core/src/sherlock/helper_machine/witness_plan/adt_projection_tests.rs
@@ -0,0 +1,169 @@
+//! Ordinary raw-observation projection tests; no admission or execution claim.
+use super::*;
+use super::super::complete_observation::{
+    ConstFieldsSnapshot, DataFieldsSnapshot, ImageSnapshot, LayoutSnapshot, ProgramSnapshot,
+};
+
+fn snapshot() -> HostSnapshot {
+    let bounds = BoundsC {
+        code_rows: 32, function_rows: 8, code_child_slots: 32, parameter_slots: 8,
+        pattern_rows: 32, match_plan_rows: 32, match_arm_rows: 8, type_rows: 16,
+        constructor_rows: 16, constructor_field_slots: 32, max_arity: 8,
+        constant_nodes: 32, constant_field_slots: 32, data_nodes: 32, data_field_slots: 32,
+        environment_cells: 16, frames: 16, helper_tasks: 16, serialization_bytes: 4096,
+        max_reference_work: 1024, max_helper_work: 1024, step_cap: 1024,
+    };
+    let identity = IdentityC {
+        profile_id: 2, grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
+        finance: FinanceInput::Absent, layout_version: ("projection".into(), "1".into()),
+        layout_digest: DigestC::Unselected, bounds_digest: DigestC::Unselected,
+        field_id: None, program_membership_digest: DigestC::Unselected,
+    };
+    let view = ProgramSnapshot {
+        entry: 0, expressions: vec![ExpressionSnapshot::Literal { constant: 0 }],
+        occurrences: vec![OccurrenceRowC { owner_is_function: false, function: 0, path: SliceC::empty() }],
+        occurrence_paths: vec![], children: vec![], functions: vec![], parameters: vec![],
+        arms: vec![], patterns: vec![], pattern_children: vec![], types: vec![], type_args: vec![],
+        constructors: vec![], ctor_fields: vec![], decl_param_counts: vec![], decl_ctor_counts: vec![],
+        decl_templates: vec![], plan_nodes: vec![], plan_cases: vec![], field_paths: vec![],
+        constants: vec![ConstantSnapshot::U128 { marker_free: true, n: 17 }],
+        constant_fields: vec![], names: vec!["x".into()],
+    };
+    HostSnapshot {
+        admission: AdmissionSnapshot {
+            identity: identity.clone(), bounds: bounds.clone(), view,
+            layout: LayoutSnapshot { layout_version: ("projection".into(), "1".into()),
+                field_id: None, widths: None, scalar_layout_id: None, bounds,
+                capacities: LayoutCapacitiesC::default(), boundary_capacities_present: false },
+        },
+        image: ImageSnapshot {
+            run_id: RunIdC { identity, fuel_input: RunIntC::Nat(1024), stack_limit_input: RunIntC::Nat(16) },
+            control: ControlC::Return { value: RefC::Const(0) }, control_origin: 0, env_root: None,
+            stack_limit: 16, outcome: OutcomeC::Running, initial_fuel: 1024, remaining_work: 1024,
+            consumed_work: 0, helper_work: 0, helper_visits: 0, reference_cursor: 0,
+            alloc: AllocCounters::default(), active: ActiveLengths::default(), helper: None, result: None,
+            seg_base: SegBase::genesis_uncut(), data: vec![], data_fields: vec![], env: vec![],
+            frames: vec![], tasks: vec![], value_slots: vec![],
+        },
+    }
+}
+fn sequence<'a>(decoded: &'a DecodedSnapshot, field: &str) -> &'a [SlotValue] {
+    let value = &decoded.fields.iter().find(|v| v.field == field).expect("output field").value;
+    let SlotValue::Sequence { items, .. } = value else { panic!("sequence output") };
+    items
+}
+fn value(item: &SlotValue) -> &DecodedValue {
+    let SlotValue::Value(value) = item else { panic!("decoded value") }; value
+}
+fn constructor(node: &ValueNode) -> (TypeIdC, adt::CtorIdC, &[usize]) {
+    let ValueNode::Constructor { type_id, ctor_id, fields } = node else { panic!("constructor") };
+    (*type_id, *ctor_id, fields)
+}
+#[test]
+fn nested_nominal_values_keep_ids_order_and_repeated_occurrences() {
+    let mut input = snapshot();
+    input.admission.view.constants.extend([
+        ConstantSnapshot::Constructor { marker_free: true, type_id: TypeIdC(4), ctor_id: adt::CtorIdC(8), fields: ConstFieldsSnapshot { start: usize::MAX, len: 0 } },
+        ConstantSnapshot::Constructor { marker_free: true, type_id: TypeIdC(5), ctor_id: adt::CtorIdC(9), fields: ConstFieldsSnapshot { start: 0, len: 3 } },
+    ]);
+    input.admission.view.constant_fields = vec![1, 0, 1];
+    input.image.data = vec![DataSnapshot::Constructor { type_id: TypeIdC(6), ctor_id: adt::CtorIdC(10), fields: DataFieldsSnapshot { start: 0, len: 2 } }];
+    input.image.data_fields = vec![RefC::Const(2), RefC::Const(0)];
+    let plan = construct_plan(&input).expect("raw nominal projection");
+    let result = evaluate_uncached(&plan, &input).expect("evaluate projection");
+    let v = value(&sequence(&result.decoded, "constants")[2]);
+    let (ty, ctor, fields) = constructor(&v.nodes[v.root]);
+    assert_eq!((ty, ctor), (TypeIdC(5), adt::CtorIdC(9)));
+    assert_eq!(fields, &[0, 1, 2]);
+    assert_ne!(fields[0], fields[2]);
+    assert_eq!(v.nodes[fields[0]], v.nodes[fields[2]]);
+    assert_eq!(v.nodes[fields[1]], ValueNode::UInt(17));
+    let data = value(&sequence(&result.decoded, "data")[0]);
+    let (_, _, fields) = constructor(&data.nodes[data.root]);
+    let (_, _, inner) = constructor(&data.nodes[fields[0]]);
+    assert_eq!(inner.len(), 3);
+    assert_eq!(data.nodes[fields[1]], ValueNode::UInt(17));
+}
+#[test]
+fn all_generic_tables_are_typed_immutable_inputs_and_outputs() {
+    let mut input = snapshot();
+    let p = &mut input.admission.view;
+    p.pattern_children = vec![3];
+    p.types = vec![adt::TypeRowC { name_id: 0, decl: Some(0), args: SliceC { start: 0, len: 1 }, ctor_start: adt::CtorIdC(0), ctor_len: 1 }];
+    p.type_args = vec![TypeIdC(19)];
+    p.constructors = vec![adt_storage::CtorRowExt { owning_type: TypeIdC(0), name_id: 0, source_order: 0, field_start: 0, field_len: 1 }];
+    p.ctor_fields = vec![adt_storage::CtorFieldRowExt { name_id: 0, type_id: TypeIdC(19) }];
+    p.decl_param_counts = vec![1]; p.decl_ctor_counts = vec![1];
+    p.decl_templates = vec![owner_ext_type_builder::TypeDeclSrc { name: "Box".into(), parameters: vec!["a".into()], ctors: vec![] }];
+    p.plan_nodes = vec![PlanNodeC::NoMatch { source_match: 0 }];
+    p.plan_cases = vec![Some(0), None]; p.field_paths = vec![2, 1];
+    let plan = construct_plan(&input).expect("table projection");
+    let result = evaluate_uncached(&plan, &input).expect("table evaluation");
+    for (name, len) in [("pattern_children",1),("types",1),("type_args",1),("constructors",1),
+        ("ctor_fields",1),("decl_param_counts",1),("decl_ctor_counts",1),("decl_templates",1),
+        ("plan_nodes",1),("plan_cases",2),("field_paths",2)] {
+        assert_eq!(sequence(&result.decoded, name).len(), len, "{name}");
+    }
+    assert_eq!(sequence(&result.decoded,"type_args"), &[SlotValue::Source(InputValue::TypeId(TypeIdC(19)))]);
+    input.admission.view.plan_cases[1] = Some(0);
+    assert!(matches!(evaluate_uncached(&plan, &input), Err(WitnessFailure { kind: FailureKind::InputChanged, .. })));
+}
+#[test]
+fn new_payload_variants_follow_existing_value_expression_and_environment_paths() {
+    let mut input = snapshot();
+    let ty = TypeIdC(0); let ctor = adt::CtorIdC(0);
+    input.admission.view.types = vec![adt::TypeRowC { name_id: 0, decl: None, args: SliceC::empty(), ctor_start: ctor, ctor_len: 1 }];
+    input.admission.view.constructors = vec![adt_storage::CtorRowExt { owning_type: ty, name_id: 0, source_order: 0, field_start: 0, field_len: 0 }];
+    input.admission.view.plan_nodes = vec![PlanNodeC::Select { source_match: 0, arm: 0 }];
+    input.admission.view.arms = vec![ArmRowC { pattern: 0, body: 0 }];
+    input.admission.view.children = vec![0];
+    input.image.value_slots = vec![RefC::Const(0)];
+    input.image.frames = vec![FrameRowC::ConstructorFieldsF { type_id: ty, ctor_id: ctor, index: 0,
+        field_type: ty, checked: true, completed: SliceC { start: 0, len: 1 }, rest: SliceC { start: 0, len: 1 }, saved_env: None, origin: 0 }];
+    input.image.control = ControlC::Materialize { type_id: ty, ctor_id: ctor, fields: SliceC { start: 0, len: 1 } };
+    let common = HelperCommon { site: SiteTag::Materialize, origin_reference_cursor: 0, parent_resume: None };
+    let helpers = [
+        HelperControlC::Build { common, type_id: ty, ctor_id: ctor, src: SliceC { start: 0, len: 1 }, src_cursor: 0, field_start: 0,
+            staged: Some(RefC::Const(0)), node: None, phase: BuildPhase::Copy },
+        HelperControlC::Plan { common, source_match: 0, scrutinee: RefC::Const(0), saved_env: None, partial: None,
+            node: 0, phase: PlanPhase::Ready, after_walk: PlanAfterWalk::Test, cur: RefC::Const(0),
+            path: SliceC { start: usize::MAX, len: 0 }, path_cursor: 0, arm: Some(0), status: TriStatus::Pending },
+    ];
+    for helper in helpers {
+        input.image.helper = Some(helper);
+        for result in [HelperResultC::BuildDone { value: RefC::Const(0) }, HelperResultC::PlanMatched { arm: 0, env: None }, HelperResultC::PlanNoMatch] {
+            input.image.result = Some(result);
+            let projection = materialize(&input).expect("generic payload projection");
+            assert_eq!(projection.raw, input);
+            let field = projection.evaluation.decoded.fields.iter().find(|f|f.field=="helper").unwrap();
+            let SlotValue::Payload(payload) = &field.value else { panic!("helper payload") };
+            assert!(!payload.fields.is_empty());
+        }
+    }
+}
+#[test]
+fn dedicated_values_and_nominal_children_share_existing_occurrence_reconstruction() {
+    let mut input = snapshot();
+    input.admission.view.constants.extend([
+        ConstantSnapshot::Nil { marker_free: true },
+        ConstantSnapshot::Cons { marker_free: true, fields: ConstFieldsSnapshot { start: 0, len: 2 } },
+        ConstantSnapshot::Leaf { marker_free: true, fields: ConstFieldsSnapshot { start: 2, len: 1 } },
+        ConstantSnapshot::Branch { marker_free: true, fields: ConstFieldsSnapshot { start: 3, len: 2 } },
+        ConstantSnapshot::Constructor { marker_free: true, type_id: TypeIdC(41), ctor_id: adt::CtorIdC(73), fields: ConstFieldsSnapshot { start: 5, len: 2 } },
+    ]);
+    input.admission.view.constant_fields = vec![0, 1, 0, 3, 3, 2, 4];
+    let plan = construct_plan(&input).expect("mixed dedicated and nominal rows");
+    let plain = evaluate_uncached(&plan, &input).expect("baseline DAG evaluation");
+    assert_eq!(value(&sequence(&plain.decoded,"constants")[2]).nodes,
+        vec![ValueNode::UInt(17), ValueNode::Nil, ValueNode::Cons { head: 0, tail: 1 }]);
+    let mixed = value(&sequence(&plain.decoded,"constants")[5]);
+    let (ty, ctor, fields) = constructor(&mixed.nodes[mixed.root]);
+    assert_eq!((ty,ctor),(TypeIdC(41),adt::CtorIdC(73)));
+    assert!(matches!(&mixed.nodes[fields[0]], ValueNode::Cons { .. }));
+    assert!(matches!(&mixed.nodes[fields[1]], ValueNode::Branch { .. }));
+    assert_eq!(evaluate_last_use(&plan,&input).unwrap().witness.decoded,plain.decoded);
+    let mut cache = WitnessCache::default();
+    for storage in [SlotStorage::RetainAll, SlotStorage::LastUse] {
+        assert_eq!(evaluate_memoized(&plan,&input,&mut cache,storage).unwrap().witness.decoded,plain.decoded);
+    }
+}

```

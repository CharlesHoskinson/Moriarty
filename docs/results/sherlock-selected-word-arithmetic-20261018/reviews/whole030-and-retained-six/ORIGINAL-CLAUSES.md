### Requirement: UIREQ-003 Lexical call scope
WHEN a named call enters its body, the interpreter SHALL evaluate it under the declared parameter bindings and global function table.

#### Scenario: Caller-local preservation
- **WHEN** An outer let binds x=7 while a called function binds its own x=2 and returns 2.
- **THEN** The caller still reads x=7 after return.

### Requirement: UIREQ-005 Named recursive calls
WHEN a named function calls itself or another named function, the interpreter SHALL apply the ordinary call/return relation.

#### Scenario: Self and mutual recursion
- **WHEN** factorial(3), even(4), and odd(5) execute within declared bounds.
- **THEN** They return 6, true, and true without adding program-specific rules.

### Requirement: UIREQ-006 Exact scalar arithmetic
WHEN a scalar primitive executes, the interpreter SHALL produce its declared checked UInt128 or Bool outcome.

#### Scenario: Normal arithmetic
- **WHEN** add(2,3), sub(7,2), mul(3,4), and lt(2,3) execute.
- **THEN** Their results are 5, 5, 12, and true under the selected primitive table.

### Requirement: UIREQ-022 Compiler semantic mapping
WHEN compilation claims support for a construct, its lowering SHALL have a named semantic correspondence obligation.

#### Scenario: New supported program
- **WHEN** A new nested-call/list program is compiled through the public pipeline.
- **THEN** The mapping covers returned values, defined failures, order, resource observations and applicable effects/duties; host evaluation does not supply missing circuit evidence.

### Requirement: UIREQ-025 Evidence-class report
WHEN an interpreter capability is reported, its evidence record SHALL identify predicate, source tree, profile, consumer, bounds, target and evidence class.

#### Scenario: Pure models only
- **WHEN** Unchanged Candidate A model observations accompany the new Candidate B design.
- **THEN** The report identifies exactly the source-specific K frontend-only check and repaired scalar15 Quint tests/samples, then labels every new B helper/image/arithmetic/plan/residual/ADT acceptance scenario unexecuted. Old observations do not validate changed B semantics; compiler/circuit correspondence, native proof acceptance and ledger qualification remain open.

### Requirement: UIREQ-027 Indexed program-image provenance
WHEN preparation indexes a supported source program, it SHALL produce a ProgramImage preserving source membership, original function-table order and SourceOccurrenceId provenance.

#### Scenario: Distinct occurrences and recursive references
- **WHEN** Two source sites contain the same expression and a named function recursively calls itself.
- **THEN** They retain distinct SourceOccurrenceId values while recursive calls remain function-table references; expanded implementation-step IDs map back to those sites. Ordered child/parameter slices and first-match lookup remain as in the source until a separately admitted unique-function profile applies; preparation neither evaluates the selected path nor unrolls recursion.

### Requirement: UIREQ-030 Typed arithmetic component API
WHEN arithmetic lowering selects a word layout, it SHALL expose named full-result components with the exact checked UInt128 semantic wrapper.

#### Scenario: Carry, borrow and full product
- **WHEN** b is 32 or 64 and checkedU128Add/Sub/Mul use little-endian limbs.
- **THEN** addCarry_b(a,bv,carryIn) returns named {low,carryOut}, subBorrow_b(a,bv,borrowIn) returns {low,borrowOut}, and mulWide_b(a,bv) returns {low,high}; carry/borrow bits and multiply-accumulate whole-word carries have distinct types. The wrappers retain K mathematical-result/error10 meaning, including the full up-to-256-bit product before checked UInt128 selection. Independently specified normal results are (2^32−3)+8 = 2^32+5, (2^64+9)−7 = 2^64+2, and (2^48+1)·3 = 3·2^48+3; no layout or row-cost winner is selected.

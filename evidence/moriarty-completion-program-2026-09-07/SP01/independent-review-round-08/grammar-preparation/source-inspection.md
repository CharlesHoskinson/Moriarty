Preparation only. The current42-production,53-keyword EBNF describes `moriarty-bounded-atomic/1`, not the full successor language. Its opening “no parser” comment is stale: the current frontend implements and pins the atomic profile. This inspection grants no SP02 entry, grammar acceptance or executable K acceptance.

The grammar/parser keyword sets match. A bounded symbol scan found no undefined productions. Nineteen read-only parser probes confirm atomic precedence, floor_div, keyword migration collisions, comment rejection and UTF-8/text boundaries. This is not a formal ISO14977 or ambiguity proof.

**G01 — Full grammar deliverable and status**

Current: 42 atomic EBNF productions and53 reserved keywords match parser inventory. grammar.ebnf:1 says no parser/freeze/audit, although parser.ts/frontend.ts implement the registered atomic profile.

Required: New complete successor lexical/EBNF/static specification covering every admitted DA01–DA24 source construct. Explain BNF/EBNF/ABNF and select EBNF as SP02.1 requires; full language coverage is the deliverable, not an atomic grammar relabel.

Disposition: Stale atomic status comment; successor specification absent. This inspection does not change either status.

**G02 — Lexical contract, comments and bounds**

Current: Well-formed UTF-8; scalar JS strings; BOM survives decoding then fails lexing. ASCII space/tab/CR/LF only. ASCII identifiers1..64. JSON strings decode to Unicode scalars, maximum256 decoded UTF-8 bytes. Source maximum65536 UTF-8 bytes. No source comments, floats, exponent notation, plain division or !=.

Required: Publish exact successor token rules, full reserved-word categories, Unicode/BOM/normalization policy, escapes, byte/depth limits and // plus nonnested block-comment disposition. Comments count in source bytes; signed Core treatment must be explicit.

Disposition: Atomic grammar comments omit implemented text256/source65536/BOM details. Grammar's json_string special sequence has no explicit decoded byte cap. These are contract-documentation gaps, not newly found parser failures.

**G03 — Precedence and lexical migration**

Current: Atomic not binds above multiplication/comparison: not true == false parses Eq(Not(true),false). Binary operators left-associate; comparison chains and unparenthesized not not reject. floor_div exists; ceil_div does not.

Required: Successor proposal makes comparisons bind above not, so the same text means Not(Eq(true,false)). Specify approved precedence, associativity, unary repetition, signed literal versus subtraction and floor/ceil calls. Preserve atomic behavior under its old profile.

Disposition: Intentional proposed cross-profile difference requiring an approved decision and profile-specific tests; not an atomic parser/grammar disagreement.

**G04 — Staged action/update grammar**

Current: guard/let/set/emit only; reads state./arg./obs./const. and previous locals. No requires, pre.field, next.field, ensures or post reads. next is currently a legal ordinary identifier.

Required: Productions and static rules for immutable pre, sequential immutable locals, single next write, no next reads, post only in an ensures suffix; exact requires/let/next/emit/ensures ordering, rollback and source spans.

Disposition: Inherited successor staging constraint; syntax/elaboration absent. Do not migrate atomic set/state by a filename change.

**G05 — Numeric and financial types**

Current: Stored UInt128, Text and Amount<Unit>; Bool expression literals only; inferred dimensional Quantity is an internal type, not a user-declared record type. Settlement links a text asset and unit quantum.

Required: Source type/literal forms for the approved UInt64/UInt128/SInt128/Bool/Time/Party/AssetId, asset-indexed Amount, nominal Debt, holder/vault Shares, dimensional Quantity, Rate and directional Price, including bounds and narrowing rules.

Disposition: No productions. Widths, scales, financial identities and several numeric choices remain proposed; old Amount<Unit> cannot silently become successor Amount<AssetId>.

**G06 — Structured/bounded data and expressions**

Current: No enum/option/record/finite collection declaration, construction, projection or indexing. No general calls; primary has only typed literals, floor_div, named refs and locals.

Required: Complete approved syntax and static constraints for finite enums/options/records/collections, field/index projection, constructors, capacity/depth accounting and recursion rejection; map each form to closed typed Core operands/results.

Disposition: Names in the successor expression list are not signatures or source productions.

**G07 — Operations and observable effects**

Current: Only Transfer/Fee/DueCreated/DueSettled effect schemas, exact ordered fields, and emit. An atomic action can contain multiple statements and emits.

Required: An approved source-to-Core-to-effect coverage matrix for all38 proposed CoreOp values; explicit actor/party/asset/identity operands and result types, distinction between source action names, Core permissions and observable effects.

Disposition: Do not invent sugar against blocked signatures. Genesis arity1 lists two parameters; ShareConvert has untyped in/out/rounding and inconsistent shares-versus-Amount effect identity. Candidate03 C03-F1 already blocks this interface.

**G08 — Obligations and asynchronous lifecycles**

Current: Atomic status has closed_when, remaining_notional/no_remaining_notional; DueCreated/DueSettled are effects. No source debt object, request, message, event claim, position or reward lifecycle.

Required: Admitted construction/identity/lifecycle syntax for debt create/accrue/repay/writeoff/capitalize, partial payments and residual duties, shares/positions, requests/claim/cancel/expiry, message send/receive/refund, event claims and rewards/slash/rebase.

Disposition: Full financial coverage absent. Source syntax must preserve residual liabilities and distinguish episode closure from agreement debt.

**G09 — Composition, administration and recovery**

Current: One action block with sequential statements. reserve action for closure is a narrow atomic reserve rule; no composition, recovery rights or lifecycle admin statements.

Required: Separate approved productions/typing for seq/par/interleave/atomic/message, branch naming and conflict rules, genesis/activate/pause/revoke/migrate and recovery authority/work. Reject recursion and unbounded loops explicitly.

Disposition: Composition spellings and charge/profile choices remain proposed. Boolean and is not composition; proof join is not source parallelism.

**G10 — Profiles, APIs, formatting and CLI**

Current: Public parse/check/elaborate require exact registered atomic bounds bytes; compile and parseSource throwing convenience APIs exist. No successor frontend/formatter/CLI files in sprint map.

Required: Separate successor API selected by explicit accepted profile hash; stable old signatures/bytes; canonical formatter and check/format CLI with exact stdout/stderr/exit contracts and profile/source hash reporting.

Disposition: Future successor bounds must be a new artifact. Existing npm build/test scripts do not establish successor readiness.

**G11 — Executable K handoff**

Current: This inspection found a source parser/checker; it did not inspect or execute K. Successor contract names a finite Core and Prepared/Reject interface.

Required: After rp01-full and a closed approved Core, SP02 specifies total elaboration and source-map contract; SP03 provides executable K rules, rejects, bounded state/effects and correspondence tests. Grammar completeness and executable K are separate required outcomes.

Disposition: Neither atomic grammar nor JSON signing examples provide full BNF coverage or executable successor K. No K acceptance claim.

The frozen successor candidate03 is `511b179e983daddea2dce93edc0ee62e8a60cf19086b9747d37f5e7857d5c45b`. It is a proposal, and its prior independent review is BLOCKED. In particular, Genesis arity and ShareConvert operand/effect identity must be resolved before frontend authors invent syntax. The eight frozen successor files were hash-checked; the changing author worktree was not read.

Future ownership must be fixed in `openspec/sprints/execution/SP02.md`. The sprint requires new successor lexical.md, grammar.ebnf, static-semantics.md, bounds.json, executable examples, src/successor/frontend.ts, src/successor/format.ts, src/cli.ts and tests/successor-frontend.test.mjs, plus package.json entry points. The JSON inventory gives exact paths and a proposed internal module split; none is authorized for modification by this preparation.

Conformance categories:

- Production/token coverage: one positive and one isolated negative per production and optional/repetition boundary; reserved/contextual words; EOF; undefined symbols; duplicate alternatives; formal ISO EBNF notation check.
- Lexical bytes: valid/invalid UTF-8, BOM, lone surrogate, ASCII/non-ASCII identifier,64/65 characters, JSON escapes/scalars, comments, nonnested block termination,256-style and selected successor byte boundaries, source spans over astral text.
- Operator ASTs: full precedence/associativity matrix, not/comparison profile difference, chained comparisons, unary nesting, signed literals, floor/ceil arity, no plain division.
- Staging: requires placement, prior locals, immutable pre, one next write, next read rejection, post only in ensures suffix, forward references and forbidden update ambiguity.
- Types and financial identities: every literal/type constructor; mixed asset/denomination/vault/holder failures; UInt/SInt edges; rate/price scale and direction; narrowing; Bool stored versus expression rules.
- Finite structures: enum tags, option branches, record unknown/missing fields, collection capacity/depth and projection/index bounds; recursion and unbounded-loop rejection.
- Core image matrix: each approved CoreOp arity/types and observable effect image; source action versus CoreOp/effect naming; no opaque generic effect escape.
- Financial programs: executable successor loan, swap, partial-payment and request positives; debt/accrual/remainder/fee/residual and lifecycle negatives derived independently.
- Composition and admin: five operators separately, branch read/write/identity/cap conflicts, atomic rollback, message async behavior, genesis/admin/recovery work conservation.
- Profile isolation: exact accepted hash, unknown/modified profile rejection, old atomic bytes/signatures unchanged, reserved-word migration collision such as next.
- Diagnostics: public closed shape, deterministic earliest stage and lexical source span; parser/shape/type/resource distinctions; invalid input produces a rejection rather than crash.
- Formatter/CLI: parse-meaning preservation, idempotence, comments and signed Core contract, UTF-8 spans, API/CLI agreement, exact stdout JSON/stderr and exit codes.
- SP03 handoff: total SourceAST-to-typed-Core mapping, deterministic hashes/source maps, independent expected Core fixtures and K simulation I/O contract; K execution/proofs deferred to its admitted scope.
- Coverage/usability: DA01–DA24 disposition matrix, three matched syntax specimens from the pinned dossier, formative developer tasks with retained observations; model opinion does not substitute.

Next actionable contract: obtain current `rp01-full` admission and exact accepted successor inputs, then prepare the owned SP02.1 specification packet with complete DA01–DA24 coverage, lexical rules, all EBNF productions, static staging/type/resource rules and independent expected examples. Keep old atomic signatures, source and registered bounds bytes stable. SP03 separately owns executable K; grammar alone does not satisfy that requirement.

Unchecked: full corpus and matched syntax specimens, empirical usability, formal ISO grammar/ambiguity validation, complete test suite/build, new syntax decisions, live admission and all K execution/proofs. Exact hashes, source evidence, parser probes and command outcomes are in source-inspection.json.

Elapsed: 376.537 seconds of300; all 22 tracked hashes unchanged.

# Opus brainstormer 1: language architecture and deterministic semantics audit design

**Scope.** This is an advisory design for the audit. It is not an audit result. Source is frozen at `cb8280ef`. I only read and searched files. I ran no tests and no native actions, and touched no private material. "Observed" below means I read it in source or docs. It never means I saw it run. Paths are relative to the worktree.

## 0. What I found while reading

1. **Several source profiles exist, and only Core reaches ZKIR.**
   - Rust `mori check` sends files starting with `language` to `mori_core::parse` and everything else to the beta parser (`crates/mori/src/commands/check.rs:29-53`).
   - `compile`, `test`, `compile-call` and the prove/call commands parse Core only (`commands/core_io.rs:242`, `compile_call.rs:201`, `test.rs:58`).
   - The Core identity is `moriarty/1` with semantics `moriarty-core/1` (`crates/mori-core/src/ir.rs:4-5`).
   - The TypeScript beta bridge produces Source/6 and feeds the *TypeScript* "Core/5" (`packages/moriarty-beta/src/bridge.ts:1,162`). That is a different "Core".
   - The Luna frontend map says Rust routes to "Core/5" (`luna-frontend.md:9`). That is wrong. Two unrelated "Core" names are being mixed up.
   - **Observed:** no beta, Source/5 or Source/6 program reaches ZKIR.
2. **Core has two independent ZKIR lowerings.**
   - `mori-backend` builds the ordinary relation (`moriarty-zkir-relation/3`, `crates/mori-backend/src/lib.rs:29-36`).
   - `mori-ledger` builds the native-call relation through `compile-call` (`crates/mori-ledger/src/relation/compiler.rs`).
   - The two accept different subsets of programs:
     - The ordinary relation allows at most 16 duty names. A discharge needs a duty declared in the source, and asset networks must equal the Target network (`lib.rs:387-396`, `liabilities.rs:8,89-97`).
     - The native-call relation needs a single payer, or the unauthenticated mapped-roles mode (`crates/mori-ledger/src/layout.rs:206-223`).
     - Under native-call, some event values cannot be proved. Docs say a branch the native VM would drop is constrained inactive (`docs/NATIVE-CORE.md:400-407`).
   - The interpreter accepts a superset of both. It accepts discharge of duties that exist only in the scenario (`crates/mori-core/src/runtime.rs:492-494`).
   - U2 requires that the "generality argument covers the admitted grammar" (`ROADMAP.md:47`). Nothing in the repo yet defines *which* admitted grammar that means.
3. **`ensures` means different things in different profiles.**
   - In Source/5, `ensures` is a trailing postcondition, and it is the only place post-state reads are allowed (`README.md:107,139-140,160`).
   - In Core it is positional and behaves exactly like `requires`, in both the interpreter (`runtime.rs:728-741`) and the lowering (`mori-backend/src/lib.rs:890-895`).
   - `crates/mori-core/README.md:25-26` documents the Core behaviour.
   - **Inferred risk:** if Source/5 contracts are ported or lowered to Core, they would silently weaken.
4. **A passing check does not mean a program will compile.**
   - For Core, `mori check` only parses and validates (`check.rs:29-51`; `parser.rs:147`). It reports no admission result from either lowering.
   - `mori test` compares only the post-state and effects of the reference interpreter. For rejections it compares only the error code, not the step (`commands/test.rs:58-65`).
   - So FeePool's 162 cases test the interpreter. They do not test the compiler.
5. **Arithmetic agrees on paper.**
   - The interpreter uses checked u128 (`runtime.rs:450-463`).
   - The relation range-checks to 128 bits, uses 64-bit limbs for multiplication, and makes subtraction underflow unsatisfiable (`mori-backend/src/lib.rs:586-619`).
   - Boolean operators evaluate both sides in both implementations (`runtime.rs:446-448`; `lib.rs:736-741`).
   - A self-transfer leaves the balance unchanged and still needs enough funds, in both (`runtime.rs:680-694`; `lib.rs:945-968`).
   - These agree as written. Nobody has run them to confirm.
6. **There is no division or rounding primitive.**
   - FeePool computes floors through untrusted quotient/remainder inputs (`examples/core/fee-pool/README.md:29-34`).
   - The U0 numeric decision covers only primitives in the *successor* implementation (`docs/decisions/u0-numeric-profile-decision.md:22,26`).
   - So the profile that actually compiles has no rounding policy it can certify.
7. **Public CLI stubs.**
   - `mori lsp` and `mori mcp` are `todo!()` (`crates/mori/src/commands/lsp.rs:9`, `mcp.rs:9`), but both appear in the CLI help (`cli.rs:150-154`).
   - **Inferred:** calling either panics.

## 1. Approach

**Recommended: an admitted-grammar map plus differential walks, recorded in a predicate-level obligation ledger.**

- Treat every Core production as a row: each statement, expression, scenario field and Target field.
- For each row, record what each consumer does with it:
  - parser/validate
  - reference interpreter
  - replay
  - ordinary relation (unsigned, and signed JubJub)
  - native-call relation (single-payer, mapped-roles, signed-v2)
  - intent preparation
- Each cell is one of: accept with the same meaning, accept with a different meaning, refuse, or unknown.
- From those cells, derive the differential cases and the two new programs. Attach every cell to the U/M/MC/SP/G children it supports.
- **Tradeoff:** it produces the generality denominator U2 asks for, and it finds semantic holes. It needs careful static reading first, and the execution half waits on an authorization ruling.

**Alternative A: walk the obligations top-down.** Go through the traceability crosswalk and `legacy-release-gates.json`, and mark each row with its evidence.
- Good at preserving obligations and cheap.
- Bad at finding semantic gaps. It would miss items 2–4 above, because no obligation row names them.

**Alternative B: one golden pipeline trace.** Push a single program through every stage as far as it can go.
- Concrete and easy to demonstrate.
- Overfits to its fixture and gives no generality argument. It would repeat the Oct 11 single-case pattern.

## 2. Outputs the audit must produce

1. `profile-register.json`: every profile ID, its parser and consumers, what it can reach (reference, relation, native-call, nothing), and its version strings.
2. `admission-lattice.jsonl`: one row per (production, consumer), with verdict, meaning note, source citation and the witnessing case IDs.
3. `obligation-ledger.jsonl`: one row per child predicate (schema below). It covers U0–U7, M1–M17, MC01–08, SP01–12 and G01–24, with no row dropped.
4. `divergence-register.jsonl`: each difference between interpreter and relation, or between profiles. Each is classified intended, defect or unknown, with an owner and a test that discriminates it.
5. `chain-walk/<program>/stages.json`: for each new program and stage, the predicate, input digests, oracle provenance, result class and stop reason.
6. `ROADMAP-AUDIT-REPORT.md`, for people to read:
   - what exists, by evidence class
   - which profiles reach which stage
   - the top risks
   - minimal roadmap edits
   - what was deferred
   - dissent
   - an explicit "claims not supported" section

Candidate ledger row:

```json
{"schema":"moriarty-audit-predicate/1",
 "id":"U2.core.ensures-positional","obligations":["U0","U2","MC01","M6"],
 "child_capability":"core-postcondition-semantics","profile":"moriarty/1+moriarty-core/1",
 "stages":["validate","reference","ordinary-relation","native-call-relation"],
 "predicate":"ensures is checked at its position, the same as requires, in interpreter and both lowerings",
 "finding_class":"observed","status":"source-observed",
 "evidence":[{"path":"crates/mori-core/src/runtime.rs","lines":"728-741","class":"source-inspected"},
             {"path":"crates/mori-backend/src/lib.rs","lines":"890-895","class":"source-inspected"}],
 "oracle":"none-yet","contrast":"Source/5 trailing postcondition README.md:139-160",
 "unknowns":["native-call lowering of ensures not inspected line-level"],
 "blocked_by":[],"native_authority":0,"claim_ceiling":"no-runtime-or-acceptance-claim"}
```

The status vocabulary is closed: `specified-only | source-observed | ordinary-run-current | historical-run | native-run-current | externally-accepted | blocked | unknown | refuted`. Finding class is one of `observed | inferred | unknown`.

## 3. End-to-end chain and test programs

The new programs must be written by someone who is not maintaining the audit oracle. Expected results must be worked out by hand before anything runs.

**P1 `DutyLedger`** (duty-centric, one asset):
- Creates a duty, partly discharges it, then fully discharges it to a present-zero tombstone.
- Reads `liability_present` and `liability_amount` after the discharge.
- Includes a self-transfer and a fee whose recipient differs from the transfer recipient.
- Places `ensures` *before* a `next` on the same state.
- Shares one expression ID between two actions in which `x` has different types (UInt in one, Bool in the other). That is legal, because typing is checked per use (`runtime.rs:51-118`).

**P2 `WitnessSplit`** (structurally different):
- No duties, two assets on the same network.
- A floor split and a ceiling split, each using q/r inputs.
- A disjunction whose unneeded right-hand side underflows. Both implementations should refuse, because Boolean operators evaluate both sides.
- Emits a Bool event and a near-maximum UInt event.

**P3 boundary programs (refusal checks):**
- 17 duties
- a discharge of a duty that exists only in the scenario
- an asset whose network differs from the Target network
- two payers under legacy `compile-call` (should give `one-public-payer`)

Each must be accepted by the interpreter and refused by the matching lowering.

| Stage | Predicate | Oracle and mutation | Status now |
|---|---|---|---|
| S0 author/parse | Bytes parse to the canonical IR. Reserved words and escapes behave as specified. | Hand-written IR; Unicode or confusable account strings | Can run once ordinary runs are authorized |
| S1 validate/type | Lattice verdicts hold | Expected refusal code **and** step | Same |
| S2 reference | Full steps, effects, pre and post match the hand oracle | Reorder or drop effects; tamper with `replay_detailed` | Same |
| S3 intent | owner-intent/2 bytes and the JubJub prepared intent bind the network, Anchor and caps | Wrong domain, nonce or cap. The Target network and Anchor are free strings (`lib.rs:95-104`). | Same; no real network ID |
| S4 ZKIR | Ordinary-relation artifact plus native-call static bundle. Public ABI equals the S2 projection. | Compare ABI positions with the effect frame; tamper with context | Ordinary; needs a ruling on `compile-call` |
| S5 relation check | Preimage satisfies the relation exactly when S2 accepts | Witness mutations: wrong q/r, underflow, presence/amount mismatch | Ordinary (`relation::check`) |
| S6–S7 proof/verify | Real proof; hostile-witness rejection | Built-in negative cases | **Blocked** (native authority 0) |
| S8 deploy/call | Pinned Fenrir target | None possible | **Blocked** (no network ID, Anchor or registry) |
| S9 effects, history, recovery | Authenticated readback, nonce use, tombstone persistence across calls | None possible | **Blocked** |

A positive control must pass at every stage before its refusals count as evidence (`ROADMAP.md:64`).

## 4. Highest-risk gaps

| # | Gap | Class | Evidence |
|---|---|---|---|
| R1 | No defined admitted grammar for U2; two lowerings accept different subsets | observed | §0.2 citations |
| R2 | Same keyword, different meaning (`ensures`) across profiles | observed; impact inferred | §0.3 |
| R3 | The authoring path that has users (beta) cannot reach ZKIR | observed | `bridge.ts:1`; `check.rs:53` |
| R4 | `check`/`test` pass while compile refuses; tests cover only the interpreter | observed | `check.rs:29-51`; `test.rs:58-65` |
| R5 | No rounding primitive; the U0 numeric profile is tied to a profile that does not compile | observed | numeric decision `:22,26`; FeePool README `:29-34` |
| R6 | Interpreter accepts event values that native-call may make unprovable | observed in docs; behaviour unknown | `NATIVE-CORE.md:400-407` |
| R7 | Network and Anchor are arbitrary strings; time is a plain input (`observedAt`) | observed | `lib.rs:95-104`; FeePool README `:162-165` |
| R8 | Whether a present-zero tombstone is the intended "residual duty" semantics | observed behaviour; intent unknown | `crates/mori-core/README.md:57,67`; `NATIVE-CORE.md:187-190` |
| R9 | `lsp`/`mcp` `todo!()` in the public CLI | observed; panic inferred | `lsp.rs:9`; `mcp.rs:9` |
| R10 | String identity for accounts and events (byte equality, no normalization) | unknown | `runtime.rs:276-277` |
| R11 | Interpreter and lowerings agree on paper; nobody has run the comparison | inferred; unverified | §0.5 citations |

## 5. Preserving every obligation

Copy every U0–U7, MC, SP and G row unchanged, including G18's block-and-preserve wording. Add children under them; never merge or replace a row. Treat each M row as a language-capability requirement, and keep its external Fenrir target dependency in a separate field.

Example children for M1 (general Moriarty-source compile to ledger-9 ZKIR):

| Child | Status |
|---|---|
| a. Core parse/validate | source-observed |
| b. Ordinary-relation lowering | source-observed; ordinary-run at this tree unknown |
| c. Native-call single-payer lowering | source-observed |
| d. Mapped-roles static export | source-observed; unauthenticated by its own label |
| e. Beta, Source/5 and Source/6 to ZKIR | **no path observed** |
| f. General correspondence | specified-only |

Other rows:
- **M6 (deterministic semantics and reference step replay):** reference/replay is source-observed. Interpreter–relation equivalence is unknown. Fraud-game replay is specified-only.
- **M10 (simulate, test, verify-intent):** simulate, test and verify-intent are source-observed. LSP and MCP are observed stubs. Recheck the historical "all stubs" wording rather than inheriting it.
- **M11 (FeePool and Capacity Exchange):** the FeePool finite component is source-observed with interpreter-only cases. Capacity Exchange is specified-only.

No percentage scores. Report denominators as (child predicates in each status) / (total child predicates in that row), and keep unknowns separate.

## 6. Workstreams

| WS | Content | Depends on | Can run now? |
|---|---|---|---|
| W1 | Profile register and lattice, from source | — | Yes, static |
| W2 | Rule-by-rule semantic extraction for each consumer, plus the divergence register | W1 | Yes, static |
| W3 | Write P1–P3 and their hand oracles (separate author) | W2 | Yes, authoring only |
| W4 | Ordinary runs: `cargo test`, check/simulate/test/replay/compile, `compile-call --inspect-static`, `relation::check` | W3; root authorization | **Blocked pending authorization** |
| W5 | Obligation ledger, including `legacy-release-gates.json` | — | Yes, alongside W1 |
| W6 | Write native experiment packets for S6–S9 (no execution) | W2 | Spec only |
| W7 | Roadmap report | all | Last |

- W1, W5 and the W3 authoring write to separate outputs, so they can run in parallel.
- W4 is the only step that executes anything, and it uses existing consumers. No new runner.

**Done when:**
- every Core production has a verdict for every consumer
- every divergence is classified
- P1 and P2 have reached their furthest unblocked stage and recorded why they stopped
- every obligation row has children with evidence or an explicit `specified-only` or `unknown`
- nothing claims closure

## 7. Roadmap changes

**Now (minimal and factual):**
1. Add a dated PR49 entry under "What exists" (`ROADMAP.md:7-9`), qualified as a source-static, unauthenticated, interpreter-tested finite component. It must not count as M11 or U6 progress.
2. Under U0, add children:
   - profile register
   - admitted-grammar lattice
   - `ensures` meaning across profiles
   - numeric-profile scope against the compiling profile
3. Under U2, state that "admitted grammar" means the lattice for each lowering, and that R4 tests must exercise the compiler path.
4. Record that M10's LSP/MCP are Rust stubs. Point users to the Node servers, or hide the commands.
5. Remove the subjective 60–70% / 30–40% figures in favour of ledger denominators.

**Deferred until there is evidence:**
- any change in U or M status
- the choice of canonical public authoring profile (needs a user decision)
- unifying or certifying the two lowerings
- retargeting the numeric profile
- tombstone semantics
- anything about S6–S9

## 8. Dissent, open questions and claims this review cannot support

**Dissent:**
- I disagree with treating FeePool's 162 cases as evidence of compiler correctness. `mori test` never runs the compiler.
- I disagree with the Luna frontend map's "Core/5" label.
- U2's generality exit can't be judged until one lattice is chosen as the reference.

**Open questions:**
- Is positional `ensures` intended in Core?
- Is a present-zero tombstone the intended meaning of a residual duty?
- Which profile is the public authoring path?
- Is `compile-call` ordinary work or native work for authorization purposes? It uses SDK ZKIR compilers; `prepare-call` runs the native transcript.
- Do the M1–M17 exits on Oct 10 have a user-approved controlling record?

**Cannot support:**
- that any test passes at `cb8280ef`
- that FeePool mapped export succeeds at this tree (its README defers this to current results, `:85-86`)
- interpreter–relation equivalence
- native soundness or proof validity
- any network ID, Anchor or clock
- any completion of U, M, MC, SP or G
- any percentage score

I couldn't save this to the plan file at `/home/charl/.claude/plans/you-are-independent-opus-adaptive-phoenix.md`, because this session only had read and search tools. This message is the full report.

# Documentation Review — Moriarty README

**Verdict: NEEDS_REVISION**

The README is unusually honest and well-scoped: no test counting, no chronology, no inflated claims. I found **no overclaim** relative to the reference material — every capability statement in the README ("restricted kernels", "no native recursive proof yet", "rejects without a backend") is matched or under-claimed against `MAPPING.md`, the package README and `simulate.mjs`. The revisions below are onboarding gaps, not integrity problems.

The core problem: the stated audience is a Compact developer, and the README never shows them (a) a complete Moriarty program, (b) how to compile or read the generated Compact, or (c) how to run their own file. A reader finishes it understanding the *thesis* but unable to *use* the project beyond one canned demo.

---

## High severity

**H1. No way to author or check your own agreement.** The opening claims "You can currently author and simulate contracts," but every command shown is hardcoded to `loan.moriarty` and `swap.moriarty` (confirmed in `simulate.mjs`: the file list and call sequences are literals). There is no CLI, and the README never mentions `src/frontend.ts`.
*Fix:* after the demo commands, add:
> To check your own agreement, call the frontend directly against the registered profile:
> ```js
> import {check} from './experiments/moriarty-language/src/frontend.ts';
> const result = check(readFileSync('my.moriarty'), readFileSync('experiments/moriarty-language/spec/bounds.json'));
> ```
> `parse`, `check` and `elaborate` return either a typed result or one closed diagnostic. There is no CLI wrapper yet.

**H2. Compact output is invisible to a Compact audience.** The bold summary promises "generate restricted Compact execution kernels," yet the README gives no command, no output path, and no toolchain requirement. A reader must open `MAPPING.md` to find `node compact/materialize-mapping.mjs`, `compact/verify-mapping.py`, `--runtime-node-modules`, python3, and the pinned Compact 0.31.1 / language 0.23.0 / runtime 0.16.0.
*Fix:* add a short "Inspect the generated Compact" subsection with both commands, the pinned versions, the note that compilation always uses `--skip-zk` (no keys, no proving), and the artifact path `experiments/moriarty-language/compact/generated/{loan,swap}/kernel.compact`.

**H3. The shape of the generated Compact is never stated.** For this audience it is the first question: does Moriarty emit a contract with `ledger` state and witnesses, or bare circuits? Per `MAPPING.md` it is the latter — pure transitions taking every state field, argument, observation, allowance, revision and arithmetic hint as *inputs* and returning all outputs; the only ledger write is a test-only `harness.compact` snapshot cell.
*Fix:* add to the "From source to settlement" section:
> Generated kernels are pure Compact circuits. They declare no ledger state and no witnesses: current state is passed in as circuit arguments and the next state is returned. Persistence, custody and witness privacy are the settlement adapter's job and are not yet generated.

**H4. The pinned-bounds constraint is omitted entirely.** The frontend admits exactly one bounds document (fixed digest); a matching registry name or reformatted JSON is rejected, and there is no public override. A developer will assume the "registered profile" limits are tunable and will be confused by a `PROGRAM_ENCODING` rejection.
*Fix:* one sentence in **Finite execution**:
> Bounds are supplied as a registered profile document. The frontend currently admits exactly one pinned `spec/bounds.json` by content digest — reformatted, extended or relaxed documents are rejected. Limits are not a configurable input today.

**H5. No expected output, so a designed rejection reads as a failure.** The demo deliberately calls acceptance without a backend and asserts `PROOF_INVALID`, and deliberately feeds an adverse input. A new reader will see rejection codes and assume the demo broke.
*Fix:* include 5–6 lines of representative output and one sentence:
> Two lines per transition are expected rejections, not errors: `adverse input:` shows the tampered case being refused, and `acceptance without proof: PROOF_INVALID` shows the acceptance path failing closed because no proof backend is supplied.

---

## Medium severity

**M1. "The DeFi kernel study" is an undefined proper noun.** ACTUS, Marlowe and PCD are all defined; this one is introduced with a definite article, no expansion and no link, while the other two bullets in the same list carry explanations.
*Fix:* expand and link it to the same `deliverables/…` study, or describe it as "an internal catalogue of decentralized-finance behaviors (swaps, liquidity, lending, composition) used as a conformance checklist."

**M2. "Obligation" is used four times and never defined**, and it is never distinguished from "effect" — despite being a load-bearing distinction ("which obligations survive a transaction", "obligation changes", "residual obligations").
*Fix:* in "What a developer writes": "An *effect* is an ordered instruction to move value at settlement. An *obligation* is a recorded future duty carried in state — remaining principal, an accrued but unsettled due — which persists across transactions."

**M3. "Observations" are named but never explained**, though they appear in the diagram, the PCD acceptance rule and the demo (`now`, bound at genesis to a provider with an authentication policy).
*Fix:* "Observations are externally supplied inputs — a clock, a price — bound at genesis to a named provider and authentication policy. The demo binds `now` to a simulation-only clock; authenticating real observations is unfinished."

**M4. The `--json` output is advertised without a map of its objects.** A reader opening it meets `genesis`, `instanceId`, `principalBindings`, `nonce`, `revision`, `predecessors`, `requiredClaimRoot`, `authority.tag` — none of which appear in the README.
*Fix:* a five-line "Key objects" list: **genesis** (domain, instance, principal and observation bindings), **state** (revision + `stateHash`), **action** (name, actor, arguments), **authority** (`ExactPlan` or `IntentRefinement`), **candidate transition** (writes + ordered effects).

**M5. The Mermaid diagram does not distinguish existing from intended.** It is the README's central figure, and the README's central claim is precisely that distinction. Today `Core → Compact` and `Core → Simulation` exist; `Proof → Acceptance → Settlement` does not.
*Fix:* dash the unimplemented edges and add a legend line: "Solid: implemented today. Dashed: intended."

**M6. "A smaller fixed-scenario encoding is prepared for experimentation" is too vague to act on.** The reader cannot tell what exists.
*Fix:* name the proof system/backend, say what relation it encodes, state that it is *not* the Moriarty transition relation, and point at `experiments/moriarty-native-ivc-r3/`.

**M7. Toolchain prerequisites are split across files.** The README says Node 24 and "no npm runtime dependencies"; the package README additionally requires an installed `tsc` for `build`/`typecheck`, and `MAPPING.md` requires python3 plus a Compact runtime `node_modules` path.
*Fix:* state in "Try the local developer workflow": "The demo needs only Node 24 — no `npm install`, no `tsc`. `npm run typecheck`/`build` additionally require a `tsc` executable; regenerating Compact additionally requires python3 and the pinned Compact toolchain."

**M8. Unit algebra is asserted but not specified.** "An amount of one asset cannot be added to another" is fine, but the swap snippet multiplies an amount by a reserve and by a unitless constant, producing a composite unit, then `floor_div`s it back. `MAPPING.md` reveals units are tracked as vectors.
*Fix:* "Units are tracked as exponent vectors: multiplication and division combine them, and only values with identical units may be added or compared. `floor_div` reduces a composite product back to a single asset unit."

**M9. `MC04`, `MC02–MC05` appear in a link path and in referenced docs with no gloss.** Add one clause: "the completion program tracks work items MC01–MC0n (acceptance, settlement, proof, history)."

---

## Low severity

**L1.** The code snippets use `arg.`, `state.`, `const.` and `let` with no explanation of the namespaces. Add one sentence naming them (arguments, contract state, declared constants, action-local bindings).

**L2.** "no unbounded loops" implies bounded loops exist. State plainly whether the language has any iteration construct at all.

**L3.** "Successive actions consume the contract's remaining execution allowance" — say what happens at exhaustion (contract expires / no further actions accepted).

**L4.** "agreement" and "contract" are used interchangeably ("agreement source", "contract transition", "contract lifetime"). Define once: an *agreement* is the source program; a *contract* is a deployed instance of it.

**L5.** "short-circuit Boolean lowering" — name the surface syntax (`&&`/`||`, i.e. `And`/`Or`) so a reader can recognize the rejection.

**L6.** Dated deep links (`deliverables/moriarty-design-sprint-2026-09-06/`, `evidence/moriarty-completion-program-2026-09-07/MC04/wrapper-interface-source-02/`) leak chronology into a document that otherwise avoids it, and are fragile. Prefer a stable index (`wiki/index.md`, `docs/`) with descriptive link text.

**L7.** The "Network integration" table row mentions a local Docker environment that appears nowhere else and has no link. Point it at `experiments/moriarty-midnight-network/`.

**L8.** `moriarty-native-ivc-r3` — expand "IVC" (incrementally verifiable computation) and drop or explain "r3" in the repository guide.

**L9.** Worth one sentence for this audience: wide arithmetic is not native to Compact, so generated circuits take an explicit hints struct (multiplication limbs, quotient/remainder) and independently constrain it. It is a genuinely interesting design point and explains why kernel signatures look unusual.

**L10.** The exact-plan workflow is more interesting than described: the demo simulates first, freezes the resulting writes and effects into the plan, then re-runs for exact comparison. One sentence — "simulate → freeze → sign → compare" — would make "exact plan" concrete.

---

## What is already good (keep)

- The negative claims are the strongest part of the document: "a valid history proof does not establish that an external price is true", "a host-computed verification flag cannot substitute", "comparing a final balance alone is insufficient", "a named proof claim is a requirement to discharge". Do not soften these.
- The "What remains to build" table is the correct structure for this reader and is accurate against the references.
- ACTUS, Marlowe and PCD are each introduced with enough definition for a cold reader.
- The loan-vs-interest framing in "Why a financial language above Compact?" lands the motivation in one paragraph.

---

**NEEDS_REVISION** — clearing H1–H5 and M1–M3 is sufficient for approval; the medium and low items are quality improvements rather than blockers.
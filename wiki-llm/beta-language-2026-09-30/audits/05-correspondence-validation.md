# PL audit 5: correspondence and adversarial validation

**Verdict: revise candidate v1 before implementation freeze.** I agree with the bounded local S0 authoring product and its explicit qualification limits. I disagree with treating the current numeric, identity and expansion contracts as sufficiently precise to implement correspondence-preserving sugar. The defects below are design findings, not allegations of ledger vulnerability or evidence that the proposed beta already exists.

## Identity, scope and evidence

- Requested reviewer routing: GPT-6.1 Sol, medium effort, fifth independent PL audit. This child cannot independently attest a returned provider/model identity or effective effort; no provider receipt is exposed here. Record the parent dispatch metadata separately. This report does not supply unavailable provider attestation.
- Reviewed design SHA-256: `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`, matching `candidate-v1.sha256`.
- Supplementary `PROGRAMMER-MOCKUP.md` SHA-256: `5ef082e9369506bcd4a7a5f1b1e8ec7a5b1ee8e9ba6f95c2ac233fd4736a7bfb`.
- Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`; design baseline records `870f998b36ecda04622fa4274132e74902942d0b`.
- Read AGENTS.md, the checked-in Moriarty development skill, guarded status, the programmer mockup requirements, Source/6 parser/lowering/wrapper, Core/5 preparation, provisional S0 implementation contract, and relevant surface/types/toolchain/editor/AI research passages. No peer audit was read. No production change, commit, campaign or network transaction was performed.
- Guarded status reported unresolved operational/admission evidence and no pending transactions. That blocks dependent campaign dispatch, not this review.
- Repository observations below come from `experiments/moriarty-language/src/successor/{financial-agreement-source-v6-frontend,mil4-s0-source-v6,mil4-s0-core-v5}.ts` and `formal/mil4/s0-implementation-contract.md`. No additional remote source was necessary for these implementation discriminators.

## Substantive choices

| Choice | Vote and reason |
| --- | --- |
| External brace syntax and one TypeScript analysis service | Agree. A distinct versioned surface can express exact fixed intent without host coercions; shared checking is appropriate. This is a design preference, not a usability measurement. |
| Source-fixed transfer/repayment with a separately stipulated scenario | Agree. Completion must never substitute endpoints, caps, nonce, head or policy claims. The repayment accounting cell can supply its creditor only under the existing declared obligation binding. |
| Reuse strict Source/6 and Core/5 | Agree, subject to findings 1–3. Preserve the wrapper, not merely a direct call to Core, because the wrapper checks signed/submitted action equality and returns all four unverified bindings. |
| Nominal domain/asset types, exact atoms and explicit scale | Agree with the principle; disagree that the wire mapping is yet specified. Agreement IDs, selected action IDs and metadata claims need distinct names and explicit retention. |
| Existing Base-per-Quote price convention | Agree. Preserve it and require displayed numerator/denominator. The mockup correctly leaves price construction open; there is no price-arithmetic implementation to approve. |
| Eight families as specified authoring examples | Agree with unsupported execution and truthful status. Parsing a profile call or checking its quantities establishes no profile relation. |
| Native authentication, numeric target correspondence and ledger consumption remain open | Agree. `PreparedUnqualified` is the correct local result. Successful preparation does not close W-D0–W-D4, M4-C1–C5 or native U obligations. |
| First failure and complete-effect claims | Agree only on an explicitly enumerated comparison domain. Automatic generation, source formation and a supplied hostile vector currently have different observable boundaries. |

## High findings

### H1 — Specify nominal S0 admission widths separately from UInt128 computation

**Repository observation:** Source/6 `Parser.action()` and cap parsing use `S128 = 2^127−1`; liability fields also use S128. Balances, allowance/work counters and effect line amounts use UInt128. Core repeats the S128 action/cap restrictions. The provisional implementation contract explicitly requires this split.

**Design gap:** The candidate gives Qty/scalar expressions checked UInt128 intermediates and rejects out-of-range atom values, without stating that supported S0 action amounts and bounds have the narrower nominal admission range. A programmer can read the design as admitting `atoms(asset: USD, value: 2^127)` to executable intents. Lowering that amount to Source/6 rejects before Core. An identity between the beta type domain and S0 admission domain has not been defined.

**Experiment observation:** A Source/6 transfer with value/gross cap `170141183460469231731687303715884105728` returns `SourceRejected/SOURCE6_RANGE`, with null published effects.

**Repair:** Define UInt128 pure value evaluation and a separate S0 nominal field admission check, with a named authoring diagnostic and origin. State whether constants above S128 remain usable for authoring-only profiles. Do not change existing Source/6 widths to make the sugar pass.

**Required discriminator:** Exercise S128, S128+1 and U128 for literal/atoms quantities, action amount and each cap; U128 balances/counters; intermediate overflow followed by subtraction; negative subtraction. Compare exact status/code/boundary as well as atom value. These beta tests are specified-only.

### H2 — Freeze how declared IDs become Source/6 identifiers and distinguish agreement from selected program

**Repository observation:** Beta declarations use ASCII variable names plus string-valued `id`. Source/6 does not accept arbitrary string IDs for domain, accounts, settlement or obligation: its identifier grammar starts with an ASCII letter, uses letters/digits/underscore, and rejects a reserved-word set. Core's direct identifier predicate accepts dot/hyphen but rejects underscore. Source/6's `ast.programId` is the agreement name; Core `intent.programId` is `selected.actionId`, such as `TransferLiteralFee`. Chain, network and representation metadata do not enter current Core state. Agreement name, scale and predecessor survive in the AST, not qualified bindings.

**Inference:** The candidate's “exact” mapping is ambiguous between authored declaration name and `id` string. Using names silently changes the declared economic identity; substituting arbitrary strings can fail Source/6 parsing or direct Core nominal checks. Distinct source declarations could also collapse to the same wire ID if an encoder sanitizes them. Returning bare Core data would lose the retained contextual claims needed to explain this gap.

**Experiment observation:** Replacing Owner with `Owner.1` rejects as `UNEXPECTED_CHAR`; replacing it with reserved `transfer` rejects as `SOURCE6_SHAPE`. These are existing Source/6 limitations, not proof that opaque blockchain address strings are supported.

**Repair:** Define a resolution table with declaration category, authored identity tuple, Source/6 spelling, supported-profile restrictions and origin. For beta/1, the smallest compatible choice is explicit authoring rejection of executable IDs outside the intersection of Source/6 and Core rules, including reserved words. A reversible opaque-ID encoding would be a new semantic decision requiring a mapping contract. State that agreement, selected action/program, representation/scale, chain/network, predecessor and source bytes remain retained claims; do not rename these all `programId`. Restrict alias creation or define identity-preserving explicit aliases.

**Required discriminator:** Same-symbol/different-ID assets; identical ID with conflicting domain/representation/scale; category reuse of the same bytes; two aliases of one account used as distinct transfer endpoints; underscore, dot, hyphen, reserved words and unsafe string contents; renaming only a declaration accessor; changing only representation, network, scale or agreement. The checker must expose what changes in emitted fields versus retained unverified metadata. Price tests remain deferred until an actual constructor exists.

### H3 — Automatic effect generation needs a total contract on financially invalid requests

**Repository observation:** Core checks intent and numeric effect range before comparing the complete submitted vector. Source/6 requires unsigned post-liability values and a closed effect skeleton. A repayment above outstanding can reach Core when supplied with an otherwise syntactically valid vector. Naively deriving its alleged post-principal may produce a negative number which Source/6 cannot parse.

**Design gap:** The scenario may omit effects, requiring expansion to construct a complete candidate from request/pre-state. The candidate does not specify what expansion does when validly typed inputs imply underflow, over-repayment or an unrepresentable derived post-liability. Throwing an elaboration error or pre-validating economic success can mask Core's named first failure and introduce a second acceptance predicate. Requiring successful derivation would exclude precisely the ordinary negative simulations the beta promises.

**Experiment observations:** With principal 20, accrued 10, outstanding 30 and repayment 31, an explicit unsigned candidate vector reaches `CoreRejected/effect/S0_EFFECT_RANGE`. A naively computed candidate containing post-principal `-1` instead produces `SourceRejected/UNEXPECTED_CHAR`. Combined missing-fee-credit, allowance/work exhaustion, stale head and replay produces `effect/S0_EFFECT_MISMATCH` when the vector is formable.

**Repair:** Define a total preparation adapter and an explicit comparison domain. One bounded option is deterministic unsigned diagnostic placeholders when derivation cannot produce a legal vector, with a proof/test that Core rejects at its earlier numeric range check and no placeholder can enable success. Another is a distinct documented expansion failure, narrowing the promised Core correspondence domain. Do not accidentally choose through implementation order. Preserve Stage-before-Intent and Intent-before-Effect under simultaneous faults.

**Required discriminator:** Automatic versus supplied vector for over-repayment, insufficient payer balance, recipient overflow, invalid cap plus over-repayment, stale head plus bad credit, zero work plus bad vector, and malformed obligation plus stale head. Assert the exact earliest boundary/code and null published state/effects. The beta adapter experiment is specified-only.

## Medium findings

### M1 — State the correspondence observation and hostile-vector coverage precisely

The candidate reasonably allows source formation to fail before Core, but needs a concrete theorem/test statement. Compare beta source plus scenario to the independently specified Source/6 wrapper result, including status, first judgment/code, ordered effects, complete post-state, replay/work, required premises, unverified bindings and field origins. Exclude runtime cost and cryptographic truth; `diagnosticWork=1` is an abstract Core observation, not a runtime bound.

**Experiment observations:** Removing the only recipient credit in a zero-fee transfer, or the creditor credit in repayment, returns `SourceRejected/SOURCE6_SHAPE`. Removing the primary credit from a positive-fee transfer leaves another credit slot and reaches `effect/S0_EFFECT_MISMATCH`. The existing suite's missing repayment credit test invokes direct Core, bypassing Source/6. These distinctions must remain visible in release claims. Supply a scenario-origin map for each explicit hostile effect and a typed outcome for vectors outside Source/6's grammar; never claim the public Source/6 path reproduced a direct-Core-only failure.

### M2 — Numeric resource budgets are incomplete for expansion, scenarios and services

The source byte/token/node/depth limits are concrete. “Finite expression/expansion work counters” and “bounded JSON” have no selected numeric work, output-size, array-length or service-aggregate caps. A declaration DAG with repeated aliases can be exponentially expanded by a tree serializer even with 256 declarations. Source limits do not bound generated Source/6 text/maps or retained LSP documents and request queues.

Select evaluator/expansion counters, output bytes/nodes, scenario bytes/depth/vector length and frame/document/queue limits. Evaluate constants once and retain values/shared origins rather than recursively copying alias trees. Abort with a deterministic named resource error and no partial financial artifact. Test counter boundary and boundary+1, repeated-doubling aliases, large escaped JSON, many small concurrent documents and cancellation. The measured runtime/memory result is still open.

### M3 — Origins need more than one copied byte span, and editor versioning needs request-level tests

A generated field may depend on a quantity literal, the asset's scale/identity and a chain of constants; repayment creditor/accounting depends on scenario cells. A single “source origin” can conceal these dependencies. Define source/scenario identity, immutable document version or content digest, half-open UTF-8 spans, and derived-field dependencies. Keep the original user text/map when reparsing generated Source/6; generated offsets are not user offsets. Treat source_hash as an authored claim, distinct from the computed content digest. Formatter equivalence should compare nominal/financial observations while explicitly allowing commitment bytes to change.

Test an astral character and CRLF before a diagnostic, escaped Unicode, alias-origin navigation, and a slow request on version N followed by N+1 and close/reopen. No N result may become the displayed current analysis or an edit targeting N+1. Partial editor recovery must never produce a strict expandable artifact.

### M4 — Strengthen independent oracles and packed-path acceptance

The design already calls for independent explicit fixtures; I agree. Make the oracle independence enforceable: author fixtures and expected post-state/effects without importing the beta expander or Core's derivation helper. Calling the same analysis service from CLI, MCP and tests checks adapter parity, not independent economics. Keep a separately reviewed literal transfer, partial/full repayment and near-bound fixture, then mutate every signed/economic field and require the predetermined result. Mutation must include source fields, scenario cells and generated fields so a jointly wrong source compiler/oracle cannot pass.

Exercise these fixtures through the packed public CLI and imported public API outside the checkout, with repository access absent and network disabled. A clean temporary install alone could still accidentally resolve `.ts` sources or maintainer files from the workspace. Import compiled JS only; inspect tarball contents, runtime dependency closure and Node version. LSP and MCP need real protocol clients; Neovim needs an actual installed client; VS Code activation remains untested until observed. AI instruction files are documentation artifacts, not vendor activation evidence.

For MCP specifically, assert every tool returns the same typed qualification/status, never treats caller AST/evidence flags as trusted, and accepts bounded inert text. Test source/scenario text containing instructions, path-looking strings and shell text without filesystem traversal or execution. Read-only tools must not expose a policy-changing “repair” that silently widens financial bounds.

## Executed checks and disposition

**Experiment observation:** `node --test experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs` passed 28/28 on this checkout. Additional in-memory Source/6 probes reproduced the nominal-width rejection, reserved/dotted identifier rejection, over-debt automatic-vector hazard, complete-vector first-failure ordering and source-only missing-credit boundaries described above. Probes reused the existing test fixture builders; they are targeted discriminators, not an independently authored financial oracle.

**Recommendation:** Accept the substantive architecture and scope votes above. Resolve H1–H3 before freezing implementation contracts; give M1–M4 explicit implementation/test dispositions. Maintain the larger mockup horizon and its local/specified/open labels. No general elaboration proof, K/Quint correspondence theorem, native numerical/proof result, provider authentication, editor activation or ledger settlement is established by this audit. All proposed beta tests remain specified-only because the beta implementation is absent.

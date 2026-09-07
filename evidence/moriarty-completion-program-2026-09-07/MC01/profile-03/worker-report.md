# Moriarty MC01 source-profile correction report

Status: correction candidate complete for independent review; not frozen,
approved, implemented, or decision-grade.

## Outcome

The bounded source profile is now internally specified as one finite,
straight-line `moriarty-bounded-atomic/1` language. The correction preserves the
UInt128/floor_div arithmetic, nonresetting genesis lifetime and horizon, loan and
swap values, mandatory proof obligations, and the unchanged 32/72 target
crosswalk. It does not add a parser, compiler, evaluator, proof, PCD, or ledger
acceptance path.

The principal replacement is `experiments/moriarty-language/spec/typed-schemas.md`:
it defines one canonical JSON codec and exact closed SourceAST, TypedProgram,
CoreProgram, SemanticManifest, BoundProgram, ProgramRef, genesis, state, typed
value, unit vector, authority, observation, effect, obligation, status, and result
records with literal versions. The full manifest is a separately bounded
program-hash preimage; signed statements contain only the compact ProgramRef.
Every hash preimage is noncircular.

`semantics.md` now fixes deterministic policy identities/provenance, settlement
quantum conversion, obligation creation/settlement and adverse cases, status
derivation, actor/principal and observation bindings, exact-plan/outcome checks,
and explicit external cryptographic/oracle/PCD/ledger assumptions. Complete
cannot imply discharge: the loan closes its episode with 4,500,000,000 micro-USD
remaining notional and AgreementStatus Outstanding.

## Audit finding dispositions

All statuses below mean “source correction supplied for re-review”, not reviewer
acceptance.

| Finding | Disposition and exact corrected locator |
|---|---|
| MC01-R1 | Resolved: occurrence identities, exhaustive coverage, exact unit checks, and singleton FloorDiv provenance are in `semantics.md:72-116`; all loan/swap targets are explicit at `examples/loan.moriarty:28-51` and `examples/swap.moriarty:26-57`. |
| MC01-R2 | Resolved: `grammar.ebnf:6-8,101-106,149`; `numeric-profile.json:14-17,94,105`; and `profile-proposal.md:44-46,185` use only `floor_div(n,d)`. Policy-only `floor(action,local)` is not an expression. |
| MC01-R3 | Resolved: the accepted distinct limits and joint rule are reconciled in `profile-proposal.md:111-137`, `bounds.json:5-36`, and `semantics.md:311-320`; no rejected depth-40 signing rule remains. |
| MC01-R4 | Resolved: full SemanticManifest/BoundProgram and compact ProgramRef are exact and separate at `typed-schemas.md:282-328`; their simultaneous aggregate limits are at `bounds.json:5-36`. |
| MC01-R5 | Resolved: the sole manifest bounds representation is BoundsRef at `typed-schemas.md:37-41,306-310` and `bounds.json:8-15`; there is no alternate embedded bounds object. |
| MC01-R6 | Resolved: Bool/Quantity lets versus stored/public values are explicit at `typed-schemas.md:51-83` and `semantics.md:10-18`; the verification model checks the swap A*B/A derivation. |
| MC01-R7 | Resolved: exact obligation/status/state schemas are at `typed-schemas.md:338-365,437-496`; transition and rejection rules are at `semantics.md:164-207`; the two-step vector is `profile-03/canonical-vectors.json:/loan`. |
| MC01-R8 | Resolved: Amount is always `{tag,unit,value}` at `typed-schemas.md:65-81`; effect and settlement records are exact at `typed-schemas.md:437-470`. |
| MC01-R9 | Resolved: unique asset/unit lookup, quantum semantics, result resolution, denomination checks, and diagnostics are at `semantics.md:120-158`; result schemas retain the resolution at `typed-schemas.md:437-470`. |
| MC01-N1 | Resolved: `field_label` is restricted and the keyword list is unique at `grammar.ebnf:81-85,130-140`; exact word classification is at `grammar.ebnf:5-6`. |
| MC01-N2 | Resolved: pre-consumption `remaining` is stated at `semantics.md:43-44`; the distinct resource-return closure rule and loan/swap distinction are at `semantics.md:238-243`. |
| MC01-N3 | Resolved: documentary strings are explicitly opaque and no stale-text predicate exists at `semantics.md:105-110` and `typed-schemas.md:267-275`. |
| MC01-N4 | Resolved: surprising `not` precedence is explicit at `grammar.ebnf:153` and `semantics.md:32-35`. |
| MC01-N5 | Resolved: mandatory genesis horizon checking and the examples’ intentionally redundant guards are stated at `semantics.md:230-236`. |
| MC01-N6 | Resolved: all actions require `actor: Text`, and the authenticated principal binding is mandatory at `typed-schemas.md:208-231,369-373,421-435` and `semantics.md:246-253`. |
| MC01-N7 | Resolved without changing the crosswalk: `numeric-profile.json:105-116`, `semantics.md:321-326`, and existing `target-crosswalk.json:844-870` explicitly limit the swap to one fixed vector and deny protocol conformance. |
| MC01-N8 | Disposition: files intentionally remain an S2 corrected candidate. A freeze is reserved to root after both reviewers accept; that atomic step must update statuses, hashes, and the candidate manifest together. No worker self-approval was fabricated. |
| G6-MC01-001 | Resolved: the legal `settlement_asset` parameter and references are at `examples/loan.moriarty:89,93,101-103`; keyword rules remain strict at `grammar.ebnf:5-6,83-85`. |
| G6-MC01-002 | Resolved with the same source as R2/R3: sole floor_div at `grammar.ebnf:101-106`, distinct encoding limits at `bounds.json:5-36`, and reconciled proposal text at `profile-proposal.md:102-137`. |
| G6-MC01-003 | Resolved with exact target/node schemas at `typed-schemas.md:129-140,247-275`, rules at `semantics.md:72-116`, and conforming policies in both examples. Documentary strings remain non-executable. |
| G6-MC01-004 | Resolved with fixed Due schemas at `typed-schemas.md:233-246`, bounded obligation/status records at `typed-schemas.md:338-365`, and exact transitions/status derivation at `semantics.md:164-207`. |
| G6-MC01-005 | Resolved: one-to-one binding and exact `nominal/quantum` conversion, remainder, reverse-product and overflow rules are at `semantics.md:120-158`; the 20/10 success and 15/10 rejection are in `profile-03/canonical-vectors.json:/quantum`. |
| G6-MC01-006 | Resolved: canonical codec/scalars at `typed-schemas.md:8-48`; exact AST/typed/Core/program at `typed-schemas.md:86-328`; noncircular hashes at `typed-schemas.md:315-331`; genesis/actor/observation at `typed-schemas.md:333-373`; authority at `typed-schemas.md:375-435`; results at `typed-schemas.md:437-496`. |

## Checks actually run

Command:

```text
python3 evidence/moriarty-completion-program-2026-09-07/MC01/profile-03/verify_profile.py
```

It passed and reported:

- exact JSON decoding for every JSON artifact;
- byte-level token coverage/balanced source structure and declared-reference
  checks for both full examples;
- fixed effect schemas and exhaustive policy targets: loan 14, swap 12;
- targeted unit derivations and checked UInt128 sample arithmetic;
- loan interest 33,972,602, remainder 54/73, two obligation transitions,
  adverse unknown/partial/excess/duplicate cases, and 4,500,000,000 remaining;
- swap output 19,743 and reserves 1,010,000 / 1,980,257, plus closure trace;
- quantum 20/10 -> 2 and 15/10 -> SETTLEMENT_NON_DIVISIBLE;
- canonical codec digest `5d14dd3ffb4fd1504ff5d543d5a1b5197daff5f4b3adedfb6fab767254d8d40a`;
- canonical ActionCall digest `a23fecd50e4031491603645da50faab1a4aa8730a74b6b801ab3070d6d3414e8`;
- target crosswalk exact SHA-256
  `0cc5bddf6a10ea9a2df7b7b5686c148314eb8f9b48f97c54ba02ab32154de884`,
  32 ACTUS / 72 DeFi rows, 104 distinct IDs, all MC07-mandatory;
- `python3 -m json.tool` for all owned JSON and `git diff --check`.

Manual source comparison against the EBNF found both examples well-formed. The
verification script is deliberately a bounded review model, not a reusable
parser/typechecker. Exact outputs and file digests are retained in
`evidence/moriarty-completion-program-2026-09-07/MC01/profile-03/`.

## Unresolved and absent work

Independent Fable and GPT-6 verdicts are absent and mandatory before freeze.
There is no implemented parser, typechecker, elaborator, compiler mapping,
evaluator, canonical production codec, signature verifier, durable nonce or
ledger-consumption mechanism, oracle authentication, native proof, recursive
PCD, private witness handoff, split/join, Preview financial comparison, or
compiler-to-ledger correspondence. ACTUS fixture and DeFi conformance remain
mandatory MC07 extensions; the fixed swap and first-period loan are specified
vectors only. Pending remains unsupported. No audit, conformance, proof, or
financial-operation result was fabricated.

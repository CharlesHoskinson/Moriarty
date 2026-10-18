# CORE-REQUEST-WIRE-CONTRACT Addendum 1 (supersession)

Status: AUTHOR FOR proposal.

- Author: claude-opus-5-5. Tools were off and nothing was run.
- Review: a fresh, focused non-author Opus review is required (same provider, separate context, disclosed).
- Frozen and unchanged: the contract (11656 B, sha 5c426d65), the test (36315 B, sha cd09dcd1), the author packet, the review and both RESULT records.
- Where this addendum conflicts with the contract, the addendum governs.
- It describes proposed behaviour and is not an implementation claim.

## 1 R-01 corrected clauses

These replace section 3 'Common rules' and rules A5, A12, A14 and A15.

- **C1 Exact nodes.**
  - Accept only exact builtin JSON nodes, checked with `type(x) is dict|list|str|int|bool`. int excludes bool, and bool is accepted only where the schema says Bool.
  - `None` is accepted only at Option positions: `vkDigest`, `expectedSourceSha256` and `expectedProgramHash`.
  - Any other object, including subclasses, is refused with ValueError.
  - No caller method (`__eq__`, `__hash__`, `__deepcopy__`, `__reduce_ex__`, `__repr__`, `__str__`, `__iter__`, `__len__`, `__bool__`, or overridden str/dict methods) is invoked before refusal.
  - Error messages name only the path and the expected type.
- **C2 Keys first.**
  - Every key is checked as `type(k) is str` before any membership test, hash, set operation or comparison.
  - Unknown keys of closed objects are refused before any value under them is visited.
  - `state` and `inputs` are maps, so their keys may be any exact str.
- **C3 Typed walk and copy.**
  - Validation is a fixed-depth, schema-directed walk of the typed wire, not a generic JSON normalizer.
  - The builder copies validated exact nodes into new dict and list objects. The copy preserves field presence, null, dict iteration order and array order, and is detached from the caller.
  - `copy.deepcopy` is never applied to unvalidated input, unknown keys or foreign objects.
  - The result is then validated.
- **C4 Cheap bounds.**
  - UInt: an exact str of 1..39 characters, ASCII 0-9 only, with no leading zero except `0`, checked BEFORE `int()`; then at most 2^128-1. This matches the hexutil `parse_u128_decimal` precheck and has the same acceptance set as the ir.rs UInt.
  - action: an exact str of at most 1024 characters, checked BEFORE encoding; then 1..1024 UTF-8 bytes.
  - Lone surrogates and over-long strings raise an ordinary ValueError.
- **C5 Scope of the ValueError guarantee.**
  - The guarantee covers input validation only.
  - There is no time, memory, MemoryError or process guarantee.
  - Section 6 stands: duplicate keys, depth, nodes and size belong to the byte parser.
- **C6 Owner of semantics.**
  - Core and Rust decide semantics and the admitted program.
  - Context stays CALLER_SUPPLIED_UNAUTHENTICATED.
  - No financial evaluation, fallback, mirrored module or stubs.
  - The future full host_codec includes the separately defined `encode_data` alongside these three APIs. This is not a permanent ban on other APIs.

## 2 R-02 Supersession

- **F-01 (closed):** lib.rs confirms `ADAPTER = "SettlementHashV2"`.
- **F-02 (closed):** hexutil `decode_exact` requires length 2N, and `nibble` accepts only [0-9a-f]. A4 is therefore a driver use-rule (enforced in `context_from_input` and `compile_selected`), not an invented host rule.
- **F-03 (narrowed):** witness.rs and plan.rs confirm that a missing registry duty maps to (false, 0), that rows are looked up by name, and the `frame_unchanged` and effect-shape checks. Native `from_records` acceptance is still unexecuted; these are source facts, not compiled correspondence.
- **F-04 (closed):** root's `json.loads` equality with the 3008 B request is true.
- **F-05 (narrowed):**
  - The reserve request's inputs, state, liabilities and ten balances equal the fixture's pre values as multisets. This includes A.observer 17, B.beta 4, B.fees 0 and B.observer 9.
  - Only array order differs.
  - The original reserve CoreReference request is not transcribed in the tests.
- **Ordinary RED run:**
  - 24 tests ran: 6 gold passes, 18 missing-implementation failures, 0 errors.
  - 69 label lookups ran and 6 accounting rows were deferred. These are not 75 compiler checks.
  - RESULT.json's counter of 0 came from a prefix parse error ('False is not true') and is preserved as is.
  - RESULT-QUALIFIED (18) is the cited count. No rerun or rewrite was made.
- **Pins:** all 18 source pins matched (ROOT-SOURCE-FACTS).
- **Stale wording:** the test header's 'never run' and the reserve docstring's 'unread' are author-time statements.

## 3 R-03 Order rationale

This replaces the rationale in P11, F-07 and section 7.

- Verbatim order is kept for a specific reason. intent.rs `prepare_intent` sets `scenario_hash = hash_bytes(canonical(scenario))`, and `canonical` sorts object keys but keeps array order.
- A balances or liabilities permutation therefore changes `scenario_hash`, and with it the intents a caller binds through `intentsDigest` and the context. The derivation of intentsDigest is not in the supplied sources.
- The statement body does not directly commit a balance-array permutation:
  - witness `body()` finds rows by name;
  - `scenario_snapshot` sorts;
  - `body_bytes` follows plan order.
- A derived scenario needs a fresh context-bound proof. K6 is never relabelled, and there is no claim that every proof-body byte changes because of array order.
- Class 0 setup forgeability and ledger authentication are unchanged.

## 4 R-04

- TEST-REPLACEMENTS.json applies to a NEW copy only.
- The 24 base methods and the GOLD rows are unchanged.
- 7 methods and their private helpers are added.
- Predicted, not observed: 31 run, 6 passes, 25 RED failures.

## 5 Flags and open items

- Operator, native, authority and publication flags are all false.
- Compile, prove and deploy stay permissionless.
- These items remain open: the general compiler/VK relation, 20 UTxO families, ACTUS 277/18/32, DS01-07, DeFi 72, DA01-24, held-outs, intents, regressions, recursion, private handoff, multi-parent, universal, and real Preprod.

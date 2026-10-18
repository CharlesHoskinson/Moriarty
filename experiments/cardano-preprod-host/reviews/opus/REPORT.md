DISCLOSURE. Reviewer: claude-opus-5-5 (Opus 5.5), HIGH, fresh context, acting as the non-author security and actual-result reviewer. I wrote neither the combined candidate (GPT-6.1-Sol HIGH) nor the DTO contract, addendum or tests. The DTO contract author, the R2 amendment author and the earlier encoder and DTO reviewers are also claude-opus-5-5 from the same provider, so independence comes only from the separate context. Tools were disabled: I imported, executed, hashed, diffed and wrote nothing, and I added no source. Every SHA, byte count and run result is copied from the supplied index and RESULT files. This review does not stand in for the required Muse Wonder HIGH non-security implementation audit, which is unavailable. All native authority remains ZERO.

SCOPE. One full combined review of the actual host_codec.py (16134 B, 8e15bc78...). It covers (a) the unchanged 5634-byte encoder prefix (0de63bea...), and (b) the appended `_wire_*` helpers and three DTO APIs: build_prove_request, validate_prove_request and validate_context_input. They are checked against the nine full canonical clauses A1-A9, the frozen Rust wire sources (input.rs, ir.rs, relation.rs check_selector, claim.rs context_from_input, hexutil.rs, witness.rs, plan.rs, abi2.rs, statement.rs, adapter.rs, runtime.rs, json.rs, intent.rs, lib.rs), both frozen test files and the actual runs.

ENCODER PREFIX. Reading the supplied text, the first 5634 bytes of the combined file match the standalone encoder line for line: _append, _head, _write_bytes, _encode and encode_data, including the MemoryError comment and the absence of any handler. Root records encoderPrefixByteIdentical=true in both runs. The prior R2 trace therefore carries forward unchanged: an iterative stack, exact-type checks before any operation, B1/B2/B3/B4 enforced incrementally, fixed messages, and no partial bytes. The appended code never calls or rebinds any encoder name. The actual 19/19 encoder behavior run (0.161 s, test sha 33326559...) retains every byte, admission, bound and rejection row. One method is excluded: TestInterfaceAndScope.test_phase1_scope_gate_only_encode_data_public, whose I1-SCOPE-R2 clause explicitly limits it to the encoder-only increment.

REPLACEMENT GATE FOR THE EXCLUDED METHOD (source audit):
- Top-level public callables are exactly encode_data, build_prove_request, validate_prove_request and validate_context_input. Every helper starts with `_wire_` or `_`.
- None of the other seven FUTURE_APIS names (build_value_data, ..., statement_public_inputs) is defined. There are no stubs and no mirrored module.
- There are zero Import/ImportFrom nodes. None of eval, exec, compile, open, __import__, breakpoint or input appears as a name.
- There is no getattr, globals, vars, __builtins__ or dunder attribute access. Nothing is I/O, entropy, environment or global mutation.
- At top level there is only the docstring, the five constants and function definitions.
- The encoder test's own AST import audit runs in setUp on the combined file, and all 19 methods passed, so that audit accepted the combined bytes. The audit is best-effort, as E1-AUDIT-R2 discloses, and root reports its own human inspection.

DTO TRACE (A1/A2: exact nodes, keys before values, no hooks):
- Every validator starts with a `type(x) is` check, or with `x is None` for Option, and short-circuits before len, iteration, comparison, int() or encode. Validators checked: _wire_text, _wire_action, _wire_uint, _wire_bool, _wire_integer (excludes bool because type(True) is bool), _wire_hex and _wire_optional_hex32.
- _wire_keys requires an exact dict, then iterates it. Dict iteration neither hashes nor compares stored keys. It checks each key with `type(key) is str` before anything else.
- _wire_record calls _wire_keys, then refuses any key not in the schema before reading a single value, then checks required keys. Every membership test hashes an exact str, and any collision compares only exact str objects, so the builtin str.__eq__ is used.
- _wire_tagged decides admissible variants from the key set alone, using equal length plus every key in the schema, which implies set equality. Only then does it read the tag value, and it checks that value as an exact str before testing it against the candidate names. So unknown keys and a mismatched shape are refused before any value is visited, the tag value included.
- _wire_map checks key exactness and UTF-8 for every key before visiting any value. Keys may be any exact str, empty included.
- The _Trap, _KeyTrap, _Str, _Int, _List and _Dict cases all reach a type check or a key-set refusal before any caller method could run.
- Every message is built from the path plus a literal. The path contains only schema field names, '.*' or '[]', never caller content. There are no f-strings or formatting, and `from None` suppresses chaining.

A3 (typed copy):
- Each container is copied by a dict comprehension over value.items() or a list comprehension over an exact list, and every child passes through its schema validator. The result is new dict and list objects with immutable exact str/int/bool/None leaves.
- The copy preserves presence: optional keys that are absent stay absent and are never invented. It also preserves null, dict iteration order and array order.
- There is no import, so there is no copy.deepcopy.
- The schema is non-recursive with a fixed depth of about 8 (request > transition > pre > balances > [] > asset > scale), well under Python's recursion limit and json.rs depth 32. No cycle can satisfy it, so RecursionError cannot occur.
- build_prove_request revalidates the assembled request (the C3 'then validated' step) and returns the first copy. validate_context_input appends authentication=CALLER_SUPPLIED_UNAUTHENTICATED to a detached copy. Passing that result back into the wire is refused as an unknown key.

A4 (cheap bounds):
- UInt requires an exact str of 1..39 characters, each between '0' and '9' by code point, so Arabic-Indic, fullwidth and superscript digits are refused. It also requires no leading zero except '0'. All of this runs before int(), and the value must then be at most 2^128-1. The acceptance set equals hexutil parse_u128_decimal and the ir.rs UInt deserializer.
- action requires an exact str of 1..1024 characters before encode('utf-8'), then a byte length of at most 1024. A lone surrogate raises UnicodeEncodeError, which is re-raised as a plain ValueError. This matches check_selector's byte bound.

RUST WIRE BOUNDARY:
- Closed field sets and camelCase match input.rs and ir.rs: ProveRequest 4, RelationSelector 8 plus 2 Option, ContextInput 9 plus Option vkDigest, Scenario 4 required, Snapshot 3, Balance 3, Liability 5, AssetId 4 with u8 scale.
- TransitionInput is CoreReference{scenario} or Records{inputs,pre,post,effects}.
- Value is adjacently tagged: type/value, UInt or Bool.
- Effect has five kinds with exact fields: State before/after as Value; Transfer and Fee as movement with a UInt amount; Liability before/after as UInt; Emit as Value. Emit is wire-valid and is refused only at compile time by the SettlementHashV2 adapter.
- Context ranges are u8, u16 and u64.
- The host refuses earlier than Rust only for driver use-rules that Rust also enforces: fixed selector and format strings (parse_request, check_selector), lowercase exact hex (hexutil decode_exact in context_from_input and compile_selected), and the action byte bound. I found no Rust-acceptable DTO that the host refuses, and no host-accepted DTO that serde would refuse at the type level.
- Native serde, from_records and compile acceptance remain unexecuted.

A5/A6. No MemoryError handler exists in either part. There are no time or memory claims and no byte-parser claims; width, string length and total size are unbounded on the host, so json.rs MAX_JSON_BYTES and the 65,536-node limit remain a separate byte stage. There is no Core, runtime or financial evaluation and no fallback, and the selector checks are literal string equality only. Context stays CALLER_SUPPLIED_UNAUTHENTICATED and only labels caller data, never ledger authentication.

ACTUAL RESULTS. Missing-implementation RED: 31 run, 6 gold passes, 25 missing-implementation failures, 0 errors. That matches the A9 prediction, with base24TestPrefixUnchanged=true and test sha ef343bbe... The first combined run was 31/31 OK (0.097 s), then 19/19 encoder behavior OK (0.161 s), each under timeout 10 s and prlimit (as 512 MiB, cpu 10, fsize 2 MiB, core 0) with python3 -I -B. Both isolated directories were unchanged afterwards, and no network was requested. The traces above predict that every method passes, and I found no test that passes only because of a weakened assertion. The six gold checks are fixture cross-consistency only: 69 label lookups with 6 accounting rows deferred. That is not native compiler coverage. Modes 555/444 bound the run but are not an instrumented sandbox.

POSITIONS. AGREE on all nine full canonical clauses. A7 is qualified: the historical 24-run RESULT.json and RESULT-QUALIFIED were not themselves in this packet, and I checked them through the addendum and the prior review. My own comparison of the literals agrees with F-04 and F-05.

NON-BLOCKING ITEMS:
- C-N01 (concurrency nonclaim extends to the DTO APIs).
- C-N02 (host-valid is not parse admission).
- C-N03 (validate-then-copy is fused, plus a second full pass).
- C-N04 (the frozen prefix docstring describes only the encoder).
- C-N05 (refusals do not name the unknown key, by design).
- C-N06 (a future combined-scope gate test needs fresh review).
- Carried from the encoder review: R2-N01 (peak memory is about 2*B4 plus a leaf temporary), R2-N02 (input is assumed not to be mutated concurrently) and R2-N03 (two stale contract status lines).
There is no blocking defect.

STILL OPEN, all of it:
- native from_records and parse acceptance, and any current target transaction;
- compiled-VM encoder roundtrip and source-to-VK matrix correspondence;
- general UTxO compatibility and financial conformance;
- all 20 UTxO families, ACTUS 277/18/32, DS01-07, DeFi 72, DA01-24 and held-outs;
- intents and regressions, recursion, private handoff, multi-parent and universal;
- K/Quint, and actual Preprod end-to-end.
sourceReady=true is limited to this narrow combined security-source and actual-result scope. It is not product adoption, not a Muse audit, not authority, and not execution acceptance of anything beyond the recorded ordinary runs.

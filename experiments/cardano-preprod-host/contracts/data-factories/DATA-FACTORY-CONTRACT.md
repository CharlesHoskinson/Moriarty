# DATA-FACTORY-CONTRACT

Source contract for four pure Plutus Data factories appended to `host_codec.py`. Companions: `DATA-FACTORY-API.json` (signatures, schemas, examples) and `test_data_factories.py` (13 independent literal tests).

Author: claude-opus-5-5 HIGH, sole security source-contract and test author. Tools off; nothing executed. This is an author proposal only: not implemented, run, reviewed or adopted. Clauses DF-01..DF-15 are canonical, and the same literal text is returned once as AUTHOR_FOR positions in the review output.

Labels: SOURCE = read in a pinned input below. DESIGN = API convention chosen here. UNVERIFIED = not established from the inputs read.

## 1. Inputs read (pins as supplied, not recomputed)

- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-combined-statement-host-candidate-root-20261016/host_codec.py` 19096 B `8f59b28f8307c789dc1f4abe7251b0de661156af130650e9b358bceb668df377`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-serialise-data-r2-source-candidate-20261016/PLUTUS-DATA-SOURCE-CONTRACT.md` 18012 B `c9827e0b0e58762c3a767a880f620c133478d2d3e2be9b243224c7d983728bc7`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k11-r4a-semantic-source-candidate-20261016/runner/build_inputs.py` 35091 B `a24ad8c054b4cbf0b1010a687fd772dbad61d568307b9acd8375999b1bbb4edc`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-vi-abi-and-core-program-source-data-sol-20261016/source/consumers/lib/pvc2/types.ak` 5681 B `84284d308e9cd15b4227f005e36c56f34b565a45e4c1e083717d049d4ec51645`
- `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-vi-abi-and-core-program-source-data-sol-20261016/source/stdlib/lib/cardano/transaction.ak` 7796 B `3d579be63806c418b1f2442dee21caf0975b320bbf99cda56173fe436540e4db`
- `/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement/lib/pvc2/core.ak` 19555 B `2f4fbf3b5a9c6f09afaa0bf9342a36e0d547ee993e82b854682d05c148f60cfa`
- `/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement/lib/pvc2/claim.ak` 4954 B `d304fe59e15fdfe61df532dd0a05cb4a12a5de953f411896492de2ea0d8bb882`

## 2. Source laws used (SOURCE)

- L1 `types.ak`: the constructor index is the declaration order, and record fields are positional.
  - `EnvV2 { version, claim_kind, family_id, predicate_digest, vk_digest, claimant_vkh, settle_outref: OutputReference, statement_digest, expiry }`.
  - `C2Mint { Issue { vk, proof }, Burn }`.
- L2 `transaction.ak`: `OutputReference { transaction_id, output_index }` has one constructor. `Transaction.mint` and `Output.value` are `Value`.
- L3 `claim.ak` `well_formed` requires:
  - version 2, claim_kind 0 and `family_id == core.family_v2`;
  - predicate_digest, vk_digest and statement_digest of 32 bytes, and claimant_vkh of 28 bytes;
  - a settle transaction_id of 32 bytes and `0 <= output_index < 65536`.
  - `issue` also requires `env.vk_digest == blake2b_256(vk)` and `groth16.verify_digest`. `envelope_name` is `blake2b_256(serialise_data(raw))`. The factories perform none of these (DF-11).
- L4 `core.ak`: `family_v2` is the 32 ASCII bytes `MRG16-PVC-FAM2-G16-BLS-ARK04-HL2`. This equals host `_FAMILY_V2` and the statement bytes 16..48.
- L5 `build_inputs.py` (consumer evidence only):
  - `value(pairs)` is a Map of policy to a Map of name to Int.
  - `thread_value` places ada first, as `(b'', [(b'', n)])`.
  - The empty mint is `value([])`, and `outref` is Constr 0 [B, I].
  - Not adopted: its uppercase tuples, the `C(i, f)` coercion via `int(i)`/`list(f)`, and its missing length and order checks.
- L6 Plutus Data profile:
  - an empty array is `80`; a nonempty array is `9f..ff`; maps are definite;
  - bytes longer than 64 are chunked as `5f`, 64-byte chunks, then `ff`;
  - integer heads are minimal;
  - constructors 0/1 use tags 121/122;
  - Burn is `d87a80`, and the outref vector is `d8799f5820..03ff`.

## 3. Clauses

**DF-01.** host_codec.py keeps its current 19096 bytes (SHA-256 8f59b28f8307c789dc1f4abe7251b0de661156af130650e9b358bceb668df377) as an exact byte prefix, and the existing first-16134-byte preservation pin stays authoritative. build_value_data, build_output_reference, build_env_data and build_c2_mint_data are added only in an appended suffix, giving eleven public APIs. The suffix must not rebind, wrap, shadow or alter any top-level name of the prefix; the seven existing APIs keep their code and behaviour.

**DF-02.** Each factory is pure and deterministic: no I/O, environment, clock, randomness, global or cached mutable state, argument mutation, subprocess, network, SDK or native call, and no call to encode_data, hashing or the statement helpers. In this increment the suffix adds no import and uses no eval, exec, compile, open, __import__, breakpoint, input, globals, locals, vars, getattr, setattr, delattr, __builtins__, global, nonlocal or dunder attribute access. A later increment may replace this phase gate only through a fresh audit of the actual combined candidate.

**DF-03.** Each factory returns a Data node in the lowercase tagged-tuple algebra admitted by encode_data: ('int', int), ('bytes', bytes), ('list', list), ('map', list of 2-tuples), ('constr', index, list). Every node and map pair is an exact tuple; every container is an exact list created by that call and shared with no argument and no other call; every integer is an exact int; every bytes leaf is exact bytes equal to the caller's value. The build_inputs.py uppercase tuples ('I', 'B', 'Constr', 'List', 'Map') and UPLC text are not outputs.

**DF-04.** Exact type means type(x) is T. An int parameter refuses bool and int subclasses; a bytes parameter refuses bytearray, memoryview, str (including hex text) and bytes subclasses; list, tuple and str parameters refuse subclasses. None is accepted only where a clause names it. Nothing is coerced, sorted, truncated, padded or defaulted.

**DF-05.** Every refusal raises ValueError (a private subclass is allowed) and returns nothing. The message is fixed text beginning with the refusal prefix of the failing check (a parameter or element path followed by a colon) and never contains or formats an argument value. Checks run in parameter order and, within a parameter, in the listed order; the first failing check decides the error. No argument value may cause TypeError, OverflowError, RecursionError or any other exception class; Python call-protocol errors (missing or unknown parameters) are outside this clause.

**DF-06.** build_output_reference(transaction_id, output_index) returns ('constr', 0, [('bytes', transaction_id), ('int', output_index)]). transaction_id is exact bytes of length 32, else transaction_id:; output_index is an exact int in 0..65535, else output_index:. SOURCE: transaction.ak OutputReference single constructor and field order; claim.ak well_formed 32-byte id and 0 <= index < 65536 for settle_outref. DESIGN: applying the 16-bit bound to every reference this factory builds.

**DF-07.** build_value_data(lovelace, tokens) returns ('map', entries). If lovelace is an int, the first entry is (('bytes', b''), ('map', [(('bytes', b''), ('int', lovelace))])), also when lovelace is 0; if lovelace is None there is no ada entry. Each maximal run of consecutive tokens with the same policy then yields one entry (('bytes', policy), ('map', [(('bytes', name), ('int', quantity)), ...])) in input order. None with [] yields ('map', []). SOURCE (consumer only): the map-of-maps shape, ada under the empty policy and empty name, and the empty mint map follow build_inputs.py value(), thread_value and value([]). DESIGN: the None/0 lovelace convention.

**DF-08.** lovelace is None or an exact int in 0..2^64-1, else lovelace:. tokens is an exact list of at most 256 elements, else tokens:. Elements are then checked one at a time in index order, each completely before the next: an exact tuple of length 3, else tokens[]:; policy exact bytes of length 28, else tokens[].policy:; name exact bytes of length 0..32, else tokens[].name:; quantity an exact nonzero int in -(2^64-1)..2^64-1, else tokens[].quantity:; and, after the first element, (policy, name) strictly greater than the previous pair under bytewise unsigned lexicographic order, else tokens.order:. The factory never sorts, merges or drops entries. These bounds and the ordering rule are DESIGN, not ledger or Aiken law.

**DF-09.** build_env_data(predicate_digest, vk_digest, claimant_vkh, settle_transaction_id, settle_output_index, statement_digest, expiry) returns ('constr', 0, [('int', 2), ('int', 0), ('bytes', b'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2'), ('bytes', predicate_digest), ('bytes', vk_digest), ('bytes', claimant_vkh), ('constr', 0, [('bytes', settle_transaction_id), ('int', settle_output_index)]), ('bytes', statement_digest), ('int', expiry)]). predicate_digest, vk_digest, settle_transaction_id and statement_digest are exact bytes of length 32; claimant_vkh is exact bytes of length 28; settle_output_index is an exact int in 0..65535; expiry is an exact int in 0..2^64-1. Each refusal prefix is the parameter name followed by a colon. SOURCE: types.ak EnvV2 field order; claim.ak well_formed version 2, claim_kind 0, family_id core.family_v2, lengths and index bounds; core.ak family_v2. DESIGN: fixing the three constants and the expiry range.

**DF-10.** build_c2_mint_data(kind, vk=None, proof=None): kind is the exact str 'Issue' or 'Burn', else kind:. For 'Issue', vk and then proof must each be exact bytes of length 1..65536, else vk: or proof:, and the result is ('constr', 0, [('bytes', vk), ('bytes', proof)]). For 'Burn', vk and then proof must each be None, else vk: or proof:, and the result is ('constr', 1, []). SOURCE: types.ak C2Mint { Issue { vk, proof }, Burn } declaration and field order. DESIGN: the selector strings, the length bounds and refusing payloads for Burn.

**DF-11.** Factories check structure only. They do not check that vk_digest equals blake2b_256(vk), that statement_digest matches any statement, that a VK or proof is a valid Groth16 or curve encoding, or that any key hash, policy, transaction id, index or expiry exists, is authentic or is current. They compute no envelope token name, build no transaction and evaluate no validator. All arguments are caller-supplied and unauthenticated; outputs are local Data fixtures, not ledger evidence.

**DF-12.** Every factory output is admitted by the prefix encode_data: depth at most 3, at most 1029 expanded nodes, bytes leaves of at most 65536 bytes and integers within CBOR major types 0 and 1, so no bignum or tag-102 form arises; constructor indices are only 0 and 1 (tags 121 and 122).

**DF-13.** UNVERIFIED and not claimed: stdlib cardano/assets.ak and aiken/collection/dict.ak (Value representation, key order, zero-quantity and ada-entry conventions), pvc2/groth16.ak (VK and proof sizes), the ledger ScriptContext builder, and equality of the Sol-capture types.ak with the utxo-worktree types.ak used by core.ak and claim.ak. The Value shape rests only on the build_inputs.py consumer; DF-07 and DF-08 choices are API conventions. No compiled-validator, VM, ledger or Preprod correspondence is claimed.

**DF-14.** test_data_factories.py has 13 test methods: test_00 pins the 19096-byte prefix and its SHA-256 and inspects the suffix best-effort; test_01..test_12 compare factory outputs with literal Data and literal CBOR bytes and check refusals. Before the factories exist test_00 passes and each of test_01..test_12 fails with an AssertionError containing 'missing implementation', with no error, skip or expected failure. The test reads only the regular non-symlink sibling host_codec.py (lstat, O_NOFOLLOW, 2 MiB cap), executes exactly the pinned bytes it inspected, writes nothing and uses no network, subprocess, RNG, VM, proof, SDK or private material. Root runs it only after full human source and effect inspection of both files, as python3 -I -B /abs/dir/test_data_factories.py in a dedicated directory holding only the two files, with wall 10 s, CPU 10 s, address space 512 MiB, file size 2 MiB, core 0 and a minimal secret-free environment.

**DF-15.** This is an author proposal by claude-opus-5-5 with tools off and nothing executed. Sol HIGH implements; a fresh Muse audits that implementation; a fresh non-author Opus later audits the actual combined host plus statement increment, preserving these literal clauses (same provider; independence is context only). Passing tests are narrow local host Data checks, not Rust, VM, ledger, security, financial or Preprod acceptance, and every operational and native authority stays closed.

## 4. Refusal prefixes (DF-05)

- build_output_reference: `transaction_id:`, `output_index:`
- build_value_data:
  - `lovelace:`, `tokens:` (type and count), `tokens[]:` (element shape)
  - `tokens[].policy:`, `tokens[].name:`, `tokens[].quantity:`
  - `tokens.order:`
- build_env_data: `predicate_digest:`, `vk_digest:`, `claimant_vkh:`, `settle_transaction_id:`, `settle_output_index:`, `statement_digest:`, `expiry:`
- build_c2_mint_data: `kind:`, `vk:`, `proof:`

## 5. Status and open items

Publication baseline:
- The four APIs at commit 5931a4833ea915f613f5e21d0aa7924d0a807b4f on utxo.
- The seven-API host is provisional. Its 19 statement, 31 request and 19 encoder passes are narrow local host data and hash checks.

Still open:
- the Sol implementation, the Muse audit, the root E1 run and the combined Opus audit;
- all 20 UTxO families and compiler/VK correspondence;
- authenticated context;
- ACTUS 277/18/32 with DS01-07, and DeFi 72 with DA01-24;
- held-outs, intents and regressions; K and Quint;
- recursion, private handoff, multi-parent, universal and PCD;
- real T0/T1/T2/T3 Preprod.

No financial or native claim is made.

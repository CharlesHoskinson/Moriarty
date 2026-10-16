# VI ABI and Core Program declarations — source data, UNRUN

This packet pins the already acquired declarations and callers needed to compare the K11 draft with the stdlib and published invoice evidence. **No adapter, helper, compiler, test, cryptography or validator was executed.** Observations below describe source, not security, cryptographic correspondence or target acceptance. Existing candidate/source comments are copied unchanged and are not adopted judgments.

The [manifest](SOURCE-MANIFEST.json) lists 25 selected payload bodies, conservatively including candidate/fixture JSON, plus two licenses and six provenance records. All available input size/SHA pins match. The five current Aiken consumer files match their named FX1 R2 freeze. Core files match the earlier tracked four-crate snapshot allowlist. [Read coverage](READ-COVERAGE.json) identifies semantic windows and omitted dependencies; this is not a full audit of those sources.

## ScriptContext, ScriptInfo and ScriptPurpose

The pinned stdlib is `aiken-lang/stdlib v2.2.0`, commit `779e9025356a85484475e06d5a14acb032bcdcc9`. Its acquired aiken.toml declares compiler v1.1.9 and Plutus v3; those two metadata files were read/pinned, not copied or executed. No assertion is made that a separately compiled wrapper uses these exact declaration bytes.

[script_context.ak](source/stdlib/lib/cardano/script_context.ak), 27–31, declares the record fields in this order: **transaction: Transaction; redeemer: Redeemer; info: ScriptInfo**. ScriptInfo and ScriptPurpose are distinct types:

| Zero-based declaration position | ScriptInfo, script_context.ak 34–62 | ScriptPurpose, transaction.ak 19–47 |
| --- | --- | --- |
| 0 | `Minting(PolicyId)` | `Mint(PolicyId)` |
| 1 | `Spending { output: OutputReference, datum: Option<Data> }` | `Spend(OutputReference)` |
| 2 | `Withdrawing(Credential)` | `Withdraw(Credential)` |
| 3 | `Publishing { at: Index, certificate: Certificate }` | `Publish { at: Index, certificate: Certificate }` |
| 4 | `Voting(Voter)` | `Vote(Voter)` |
| 5 | `Proposing { at: Index, proposal_procedure: ProposalProcedure }` | `Propose { at: Index, proposal_procedure: ProposalProcedure }` |

The candidate [build_inputs.py](candidate/runner/build_inputs.py), 152–157, places `C(2,[vi_cred])` both in the redeemer-purpose map and as ScriptContext info; its outer context is `C(0,[tx,auth_shard,info])`. Those choices align with the respective declaration positions and context field order. Its comment calls ScriptInfo “Rewarding”; the pinned source name is **Withdrawing**. This is a terminology difference, not an observed constructor-order failure. Spending illustrates why the two types cannot be generally interchanged: their fields differ.

[script_purpose.ak](source/stdlib/lib/cardano/transaction/script_purpose.ak), 13–55, defines comparison order Mint < Spend < Withdraw < Publish < Vote < Propose. [address.ak](source/stdlib/lib/cardano/address.ak), 9–12, declares Credential as VerificationKey then Script, whereas [credential.compare](source/stdlib/lib/cardano/address/credential.ak), 4–17, orders Script before VerificationKey. Transaction withdrawal ordering explicitly follows that comparison, rather than the Credential declaration order.

## Transaction and nested declarations

[transaction.ak](source/stdlib/lib/cardano/transaction.ak), 55–78, declares these 16 positional fields:

| Position | Field | Type / declaration line |
| --- | --- | --- |
| 0 | inputs | List<Input>, 56 |
| 1 | reference_inputs | List<Input>, 57 |
| 2 | outputs | List<Output>, 58 |
| 3 | fee | Lovelace, 59 |
| 4 | mint | Value, 60 |
| 5 | certificates | List<Certificate>, 61 |
| 6 | withdrawals | Pairs<Credential, Lovelace>, 64 |
| 7 | validity_range | ValidityRange, 65 |
| 8 | extra_signatories | List<VerificationKeyHash>, 66 |
| 9 | redeemers | Pairs<ScriptPurpose, Redeemer>, 69 |
| 10 | datums | Dict<DataHash, Data>, 70 |
| 11 | id | TransactionId, 71 |
| 12 | votes | Pairs<Voter, Pairs<GovernanceActionId, Vote>>, 74 |
| 13 | proposal_procedures | List<ProposalProcedure>, 75 |
| 14 | current_treasury_amount | Option<Lovelace>, 76 |
| 15 | treasury_donation | Option<Lovelace>, 77 |

K11's 16-element transaction at build_inputs.py 153 follows this field sequence. It supplies `M(...)` for withdrawals/redeemers/datums/votes and nested `M(...)` for Value; it does not substitute a list for those map choices. `M` and `L` are distinct handwritten tagged tuples, emitted as `Map` and `List` by `uplc` (37–50). This packet has not run that renderer or a wrapper decoder.

Other source declaration orders:

| Type | Ordered fields / variants |
| --- | --- |
| Input (transaction.ak 85–88) | output_reference, output |
| OutputReference (93–96) | transaction_id, output_index |
| Output (99–104) | address, value, datum, reference_script |
| Datum (107–113) | NoDatum; DatumHash(DataHash); InlineDatum(Data) |
| Address (address.ak 22–25) | payment_credential, stake_credential |
| Referenced (address.ak 64–67) | Inline(a); Pointer(slot_number, transaction_index, certificate_index) |
| Interval (interval.ak 42–45) | lower_bound, upper_bound |
| IntervalBound (48–51) | bound_type, is_inclusive |
| IntervalBoundType (56–60) | NegativeInfinity; Finite(a); PositiveInfinity |

ValidityRange aliases Interval<Int> and is documented in **POSIX milliseconds** (transaction.ak 80–82). This is not a slot-to-time conversion or evidence that a fabricated interval corresponds to any ledger slot. [assets.ak](source/stdlib/lib/cardano/assets.ak), 7–39, aliases Lovelace to Int and declares opaque Value with inner `Dict<PolicyId, Dict<AssetName,Int>>`; Ada policy/name are empty byte arrays by convention.

**Raw Data limit:** these declarations locate names, types and source order. They do not supply the compiler implementation that lowers records, Pairs, Dict, opaque Value, Bool or Option to Plutus Data, nor the compiled validator's ScriptContext decoder. `Pairs` is not an ordinary two-field record declaration in this packet. The candidate writes raw `Constr`/`Map`/`List` terms directly; comments or positional agreement cannot establish that a compiled wrapper decodes them. The first missing ABI dependency is that compiler/wrapper lowering/decoding source or independently observed exact decode behavior. No new source was acquired to fill it.

## Current VI caller and custom records

[vi.ak](source/consumers/validators/vi.ak), 13–20, has `withdraw(redeemer: AuthShardV1, _account: Credential, self: Transaction)` and delegates to `shard.authorize`; else fails. [shard.ak](source/consumers/lib/pvc2/shard.ak), 41–50, declares ThreadDigestV1(outref, intents_digest) and AuthShardV1(thread_policy, threads). Its source comment explicitly calls these constructor 0 with positional fields. `covered` reads a spend redeemer as raw Data, expects SettleRedeemerV2, then expects an inline raw datum as ThreadDatumV2 (127–147). `shard_redeemer` consumes Pairs using `Pair(purpose,d)` and compares against `Withdraw(cred)` (82–104). Thus an integer-purpose label alone does not replace the typed decoding/caller path.

The [pvc2 types source](source/consumers/lib/pvc2/types.ak) states its own ABI convention at 3–5. Relevant ordered fields are:

| Type and lines | Ordered fields / variants |
| --- | --- |
| ValueV2 11–14; IntentKeyV2 38–41 | VUInt(Int), VBool(Bool); SchnorrKey(ByteArray), EcdsaKey(ByteArray) |
| AssetIdV2 17–22; AssetMapV2 31–35 | network, issuer, reference, scale; id, unit, exp |
| AccountV2 59–65 | name, kind, key, native |
| ActionEntryV2 73–82 | name, index, predicate_digest, vk_digest, shape_digest, advancers |
| CatalogueV2 84–95 | network_id, moriarty_network, program_hash, source_hash, assets, accounts, states, actions, reserve_owner, reserve_floor |
| ThreadDatumV2 113–127 | version, thread_id, claim_hash, catalogue, state, balances, liabilities, reserve, seq, event_chain |
| ShapeV2 150–158 | inputs, cells, duties, effects, debit_cells, statement_count, liability_statements |
| LimitV2 174–181 | asset, beneficiary, gross_cap, fee_cap, net_floor, liability_cap |
| SignedIntentV2 183–189 | signer, limits, not_before, not_after, signature |
| Settle variant 204–217 | action, shape, inputs, post_states, post_cells, post_duties, effects, intents, payouts, continuation, topup |

`authorize` calls its gate then `list.all(sigs,intent.verify)` (shard.ak 157–235). The gate's source chain passes through thread.settle_base, settle_effects, authority shape, window, signers, native co-authority, scenario hash and message construction. [thread.ak](source/consumers/lib/pvc2/thread.ak), 651–729, constructs the intent domain from the action entry/thread/outref and sets SigCheckV2.digest to `builtin.sha2_256(message)`. [intent.ak](source/consumers/lib/pvc2/intent.ak), 327–376, defines message framing, whole-interval window checks and scheme-selected builtin calls. These are located call sites, not an execution or cryptographic correspondence result. Imported catalogue/core/bytes and collection implementations are outside this selected caller subset.

## Core structures versus the handwritten K11 Program

[Core ir.rs](source/core/ir.rs) declares:

| Type and lines | Ordered Rust fields / wire rule |
| --- | --- |
| Program 179–187 | language, semantics, name, states, assets, expressions, actions |
| Action 172–176 | name, inputs, statements |
| AssetDecl 114–117 | name, identity |
| AssetId 106–111 | network, issuer, reference, scale |
| StateDecl 93–97; InputDecl 100–103 | name, ty, initial; name, ty |
| Scenario 206–211 | state map, inputs map, balances array, liabilities array |
| Value 38–43; UInt 11–31 | tagged JSON `{type,value}`; UInt is a canonical decimal string, Bool is a boolean |

The shared Program/Action/Statement serialize directly; deserialization routes through the frozen `v1` wire at ir.rs 271–465, which excludes Await. That is a wire/profile distinction, not legacy Source/5. [lib.rs](source/core/lib.rs), 2–16, exports intent and re-exports IR/validate. [intent.rs](source/core/intent.rs), 30–67, declares Authorization(network, anchor, program_address, target_hash, signer, public_key_hex, scheme, nonce, predecessor, not_before, not_after, limits), OwnerIntent and prepared/signed outputs.

`prepare_intent(&Program,&str,&str,&Scenario,&Authorization)` (128–193) validates the supplied Program and action name, accepts a caller-supplied 64-character source-hash string, then sets `program_hash = SHA256(canonical(program))` and `scenario_hash = SHA256(canonical(scenario))`. It does not receive or parse the source file. Canonicalization sorts object keys but preserves arrays (72–88); Rust declaration field order is not a CBOR/Data order. [runtime.validate](source/core/runtime.rs), 120–322, is the located program validator, not a source compiler or observed validation result.

The K11 [helper source](candidate/signer/src/main.rs), 63–85, manually constructs Program with empty expressions and exactly one `partial` action with empty statements. Its prepare call at 242–246 uses that object. The published [installment-invoice.mori](published/fixtures/installment-invoice.mori) has open, partial and finish actions; partial includes requirements, liability read, fee 2, transfer, discharge, next paid and ensures (21–30). K11's objective says no handwritten program IR, while its typedGaps acknowledges the empty statement template. These are concrete source differences, not an observed compile/validate/hash result. No replacement Program was authored here.

## Source, request and GOLD identity comparison

The helper's SOURCE_HASH literal equals the actual 1,497-byte published fixture SHA-256: `105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9`. That equality does not make the separately supplied empty Program a parsed representation of the fixture. The published [request](published/fixtures/invoice-partial.request.json) is `moriarty-k2-prove-request/1`, selects partial / SettlementHashV2 / NotBound / GrossDebitAndFees / TouchedOnly. K11's scenario source (87–107) reproduces its amount 35, state and ordered balances escrow109/spectator41/fees0/seller0 and liabilities invoice90/reserve14. This is manual source comparison; the helper was not run.

[GOLD](published/gold/K4H-GOLD.json), `/relations/invoice~1partial`, records programHash `d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9`, predicate `0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd`, shape `36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969`. The K11 helper instead obtains its program hash dynamically from the manually constructed object; its numerical hash has not been computed here. Its target literal uses the same predicate, d19d99a4… VK digest and an all-zero shape digest. GOLD's declared aade9e4e… VK digest is explicitly historical **synthetic**, not the later K6 setup VK. These identities must retain their separate origins.

The request's context has fabricated a2… settlement hash, 22… outref and 55… claimant. K11 substitutes its e315… thread-policy settlement domain and its declared compiled C2 hash while retaining fabricated local outref/claimant and the thread id. No live-ledger fact is established by either context. No assertion is made that a new catalogue equals the request's recorded catalogue/intents digests.

GOLD records fee2, transfer35, discharge90→55, paid0→35 and gross debit37; spectator41 is explicitly outside its TouchedOnly statement. K11 builder 145–146 instead supplies a single-cell `(asset0, signer)` shape, `ETransfer(0,0)`, one amount35 effect and no duty rows. Its helper limits are gross35/fee0 (129–136). Those are different source payloads; this packet does not evaluate their acceptance or interpret authorization safety.

Finally, candidate `value()` (build_inputs.py 117) calls `B(p)`/`B(n)`, and `B` (39) calls `bytes(b)` without an encoding. The call at 140 supplies Python string literals `""` and the string THREAD_POLICY. This is the source-level string/bytes mismatch identified by root; no Python adapter was imported or called and no TypeError was reproduced here.

The next interpretation belongs to fresh Muse/Gemini review. This packet supplies declarations, source differences and typed missing dependencies only; it grants no build, key, signing, query or ledger authority.

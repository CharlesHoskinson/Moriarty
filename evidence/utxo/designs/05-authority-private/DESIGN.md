# Authority, replay and certificate experiments

Designer5 of5; checked-out base84f64b30df53ff0b8ccaf475f57324d3d7c6e594. Repository startup, development skill, active routing, guarded status and wiki index inspected. Old campaign dispatch remains stale; these source experiments are independently authorized. No repository implementation, wallet or public transaction was changed.

**Vote B:** use a source-derived bounded public backend, compile and execute its predicates in Aiken/UPLC, then consume/recreate authenticated state in an isolated real Cardano ledger. A source evaluator alone cannot establish script signatories, head identity or freshness. A full private backend immediately would add an unmeasured proof relation, setup/key governance and witness recovery to the public-state adaptation. Keep those separate acceptance gates. This vote is a design recommendation, not native acceptance.

## Actual source experiment

Run `node verify.mjs` here. Result:15 controls checked, including complete independent ordinary/financial poststates and all effect records for two positives. All amounts are public test quanta and all commitments are generated from labeled public fixture inputs. `authority.mori` uses current source /6, funded Core /5 and lifecycle state /2. The factory derives Core afresh. The /6 source compiler requires all five operation declarations even when the action uses only Transfer/Fee; the unused schemas retain the existing example definitions. No language change was necessary to run these source cases.

Route economics: Lender100 -> Borrower100 with Borrower5 -> Lender5 Fee. Balances become Lender5/Borrower105; allowances become Lender remaining0/spent100 and Borrower remaining105/spent5. Borrower net credit95. The unrelated Other/Token balance7 and allowance remaining3/spent1 remain byte-for-byte unchanged. Obligations and all nontransfer identifier sets remain empty. Guard4 + nonce-write4 + Transfer8 + Fee7 + ensure6 + two kernel effects2 =31 units: remaining512-31=481; spent17+31=48; closureReserve16 remains.

Consume economics: Borrower pays Fee1 to Lender. Balances become Lender101/Borrower9; Borrower allowance remaining109/spent1; Lender allowance unchanged. Used transfer IDs contain Nonce1. Guard4 + nonce-write4 + Fee7 + ensure6 + one kernel effect1 =22 units: remaining490/spent39. Ordinary nonce0 becomes1 in both positives. Complete expected states were defined before evaluation and compared with `deepEqual`; postconditions alone were not the oracle.

## AP01: bounded outcome authority and exact identity

Source-reproduced positives: the route exactly meets minimumNetCredits95 and grossCaps Lender100/Borrower5. Source-reproduced negatives: minimum96 rejects INTENT_NET_GOAL; Lender gross cap99 rejects INTENT_DEBIT_CAP without financial publication.

Measured limits: replacing networkTag or programDigest with another well-formed 32-byte digest still prepares the source transition. /6 validates shape but does not authenticate a deployment/genesis domain or compare programDigest with the source hash. This is retained counterevidence. These marker values are not signatures and the source example is not an exact-head authorization envelope.

Native predicate: verify exact canonical signed envelope, expected genesis/network, instance identity, immutable program digest, action+arguments, predecessor out-ref/head revision, complete gross debits, fees and outcome goals. Exact-head authority binds the selected head; outcome authority also needs durable per-instance nonce consumption and allowed-program policy. Whole spent values count against native gross caps; source semantic100 must not be mislabeled as the complete wallet input amount when an input also supplies change. Native cases: changed program/domain/action/head/fee, valid envelope but substituted body, and lender100000 input with90000 change against10000 cap all reject. Collateral exposure is a separate final-body wallet policy because V3 context excludes it.

## AP02: durable per-instance replay

Source-reproduced: first Nonce1 Fee prepares; evaluating the same transfer ID against its complete accepted financial successor rejects DUPLICATE. An ordinary expectedNonce0 against predecessor nonce1 rejects GUARD_FAILED. A fresh independent instance may reuse the ID. Reusing old ordinary and financial snapshots also prepares, measuring that evaluation is not authenticated ledger freshness.

Native predicate: consume exactly the authenticated instance head; enforce new nonce=old+1; retain used/cancelled outcome IDs in successor state and across migrations/split/join. Test same signed outcome on next head, restored old nonce/identifier set, concurrent fill/fill and cancel/fill, and migration that erases nonce history. Exactly one competing spend may settle. A signature intentionally permitting several instances requires a shared consumed authority for globally-once behavior; an ordinary per-instance nonce does not supply global uniqueness.

## AP03: role binding versus native consent

Source-reproduced: malformed capability and equal role parties reject. Substituting a different well-formed debtor capability still prepares; all positives supply no signatures. This is structural admission, not cryptographic consent.

Native predicate: map each declared role to its immutable key/token policy; require necessary `transaction.extra_signatories` and actual matching witnesses (the ledger validates witnesses). A key witness used only to spend a funding input does not by itself appear in the V3 required-signer list. Negative controls: omit required signer while including its funding witness; replace role key; reference rather than spend a role token; forged same-name token under a different policy; extra signer without an authorized role. Native multisig must require both selected keys, not a host boolean or a reference to someone else's token.

## AP04: certificate/private boundary fails closed

Source-reproduced: an emitted unknown VerifyCertificate operation rejects OPERATION_BINDING; `private state` rejects UNKNOWN_DECLARATION. Retained failing sources prove current profiles cannot silently treat an opaque certificate or private datum as accepted proof. No proof, key, private witness or native privacy result was produced.

Public native backend must explicitly reject private/certificate mode until a complete verifier is compiled and measured. Tests: arbitrary certificate bytes, host `verified=true`, substituted verification key, missing opening, changed public head/effect/nonce/program, surplus/short public inputs, field aliases, infinity or malformed points. A future approved key must bind the whole transition+authority+effects+intent+invariant relation; successful unrelated arithmetic proof does not suffice. A plain public datum may confirm eUTXO execution but cannot confirm Moriarty private execution or general off-ledger PCD. Retain this family in the result matrix as unsupported with rejection evidence.

## Tiers and evidence

Source tier executed:15 controls in observations.json, canonical inputs, independently specified complete expected states and verifier. Compiled UPLC tier:specified-only; acceptance requires running the complete source-derived validator on native contexts, not a constant Bool fixture. Real-ledger tier:specified-only; require transaction IDs, ledger readbacks and negative conflicts against an actual isolated node. Private/certificate correspondence:not implemented; failclosed source controls only. No source success is a Cardano settlement, formal correspondence or Preview result.

## Provenance

Repository observations: `experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md`; `src/successor/financial-outcome-intent.ts`, validLifecycleAuthority/checkOutcomeIntent; `src/successor/financial-expression-v5.ts`, createFinancialExpressionContractV5; current `openspec/PCD-INTEGRATION-2026-09-11.md` and `openspec/changes/pcd-ledger-anchored-acceptance/specs/pcd-mandatory-acceptance/spec.md` digest-v2 exact/outcome distinction.

Retained source facts: research `09-authority-intents/REPORT.md`, sections1–3/5, cites pinned Cardano Ledger56ddee12ec768b85d14c62da049a33fc05114323 Conway/TxInfo.hs:497 and Plutus0849ed6a3385d17ca7dacc075b29f15999d490e9 V3/Contexts.hs:436/573. CIP8/30 specify message/transaction signing, not a universal nonce registry. `08-private-proofs/REPORT.md`, Groth16 and slim-Nova sections, identifies whole-relation/arity/key/opening obligations and warns that slim sumcheck without commitment openings is not complete certificate verification. Primary source snapshots and receipts are retained in the research collection. This design consulted those reports and exact local Moriarty code; it did not rerun downloaded verifiers.

## Minimal additions if required

Keep frozen /6 bytes unchanged. Add versioned backend admission data for authenticated deployment/program/instance/head, native role credentials and strict native-context projection; thread ordinary state and complete lifecycle state through the authenticated successor. Declare explicit supported intent mode; reject unsupported public/native features and private mode. Only introduce a new source profile if applications need source-level authority/reference/validity/proof declarations. Never implement VerifyCertificate as trust in a caller flag. Native projection and signature policy are required capabilities, not syntactic shortcuts.

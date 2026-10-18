# Moriarty Webs: from a financial calculation to a Cardano payment

Moriarty is a language for describing financial agreements and the actions that change them. Webs is its proposed Cardano integration: an application would bring evidence that an action follows the agreement, then connect that evidence to the transaction that pays people and carries the agreement forward. This guide follows one partial invoice payment, including the money that leaves escrow, the fee and the obligations that remain. [N01]

## Why prove a calculation?

A financial action usually involves more than choosing a destination and an amount. A repayment may include interest and a fee. A trade may combine pool arithmetic with a minimum amount each trader must receive. A partial payment must update the outstanding balance while preserving other promises. An application can calculate all of this away from the ledger, but the receiving contract still needs a reason to accept the result. Trusting the application’s reported answer gives the application a different job from proving that its calculation follows agreed rules. [N02]

The proposed proof path lets a producer perform a bounded calculation and supply cryptographic evidence for the compiled rules and a particular statement. A verifier checks that evidence rather than treating the producer’s answer as an assertion. This can separate the work of producing an answer from the work of checking it. Whether it reduces total cost depends on the circuit, proof production and the transactions required on the chosen ledger. The useful starting point is an independently checkable result; cost savings need measurements. [N03]

There are two connections to establish. The compiled calculation must express the financial program faithfully. The ledger consumer must connect the statement to the state and payments in the actual transaction. Proving that 100 minus 35 equals 65 does not itself pay the seller, identify the asset being paid or authorize spending escrow. Those responsibilities explain why a proof belongs inside a larger transaction workflow. [N02, N04]

## Follow 35 CASH through an invoice

The invoice records escrow owing the seller 100 CASH. Escrow also has a separate obligation to return 10 CASH to the buyer. For this worked example, assume escrow holds 112 CASH: 100 for the invoice, 10 for the reserve obligation and 2 for one partial-payment fee. This is a proposed funding policy for the illustration. The source program’s opening action instead funds 110; the comparison below shows why those two extra units matter. CASH is the fixture’s named asset, with its complete identifier available in the source notes. [N05]

The selected action is `partial(35)`. Its rules require an open agreement and a payment greater than zero but less than the outstanding invoice. Escrow pays the seller 35 CASH and pays the program’s fee recipient another 2 CASH. Only the seller’s payment discharges invoice debt. The next state records an invoice obligation of 65 and preserves the buyer’s reserve obligation of 10. The source also increases `paid` from 0 to 35, keeps `opened` true and leaves the unrelated `marker` value at 7. [N06]

| Position | Before this action | Change | After this action |
| --- | ---: | ---: | ---: |
| Escrow custody | 112 CASH | −37 CASH | 75 CASH |
| Seller’s receipts for this step | 0 CASH | +35 CASH | 35 CASH |
| Fee recipient’s receipts for this step | 0 CASH | +2 CASH | 2 CASH |
| Invoice owed by escrow to seller | 100 CASH | −35 CASH | 65 CASH |
| Reserve owed by escrow to buyer | 10 CASH | 0 CASH | 10 CASH |

The cash account balances: **112 = 35 + 2 + 75**. The debt account balances separately: **100 − 35 = 65**, while the reserve remains 10. Escrow therefore holds 75 against 75 still owed. Adding the remaining obligations to the outputs would count promises as if they were additional coins. Treating the fee as repayment would erase two units of debt without paying the seller. Keeping custody and obligations in separate rows makes both mistakes visible. [N05, N06]

Now compare the literal opening source. Calling `open(90,10)` creates an invoice face amount of 100 by adding 10 to the principal, funds escrow with 110 and charges the buyer a separate opening fee of 3. The buyer’s opening gross debit is therefore 113. The same later partial payment leaves escrow with **110 − 35 − 2 = 73**, but the continuing obligations still total 75. Non-negative custody and full backing are different policies; this source does not require the latter. Funding 112 changes that one-step arithmetic, but the supplied program contains no action making the extra transfer. [N07]

## What the proof covers

The source expresses the action as one ordered calculation. It reads the opening invoice duty, checks the amount, charges the fee, transfers the payment, discharges that payment from the duty and checks the resulting amount. A whole-execution proof can cover these conditions together. For this explanation, the proof design is **one whole-execution Strand covering the partial action**. Payment, fee and continuing obligations are three things to inspect within that action; they are not three independently produced Strands. [N06, N08]

The statement identifies the particular action and values to which the evidence applies. At the level of this example, its financial meaning is: start with this escrow state, execute `partial(35)` under the selected program, produce these payments and retain these duties. The integration also needs identities for the program and compiled relation, the predecessor state and the intended consumer. The witness supplies the other values needed by the compiled constraints, including intermediate calculations. A proof of this particular execution is different from a certificate about every possible execution of the program. [N02, N08]

A **Strand** packages an individual proof with its claim, statement and binding metadata. A **Web** is the proposed collection of Strands required by the consuming contract for an action. A claim specification describes a reusable requirement; the proof instance concerns particular inputs and state. In this one-proof teaching design, the Web contains one Strand. Splitting the calculation later would need actual separately supported relations and a rule explaining how their statements jointly cover the action. Renaming three checks does not perform that work. [N01, N08]

## What still belongs to the ledger

Cardano represents spendable value in transaction outputs. Spending the escrow output consumes that piece of state; a continuing agreement needs a successor output carrying the remaining value and state. In the example’s intended settlement, the financial inputs and outputs are an escrow input holding 112 CASH, a seller output of 35, a fee output of 2 and continuing escrow custody of 75. The continuing state records the 65-CASH invoice and 10-CASH reserve obligations. These are explanatory values, not a serialized transaction or deployed datum schema. [N04, N05]

The consuming validator has to connect the claimed calculation to those actual outputs. It must identify the predecessor, select the intended asset and recipients, apply the authorization rules and require the correct continuation. A proof whose supplied calculation says “seller receives 35” is useful to settlement only through that connection. Likewise, the ledger still applies its own transaction-validity rules. Program fees in CASH do not quote Cardano fees, collateral or the minimum ADA required in outputs. [N02, N04]

The reviewed source describes an issuance route with two distinct ledger stages. An **Issue transaction** checks one proof and creates a **claim output**, representing checked evidence under the claim policy. A later **settlement transaction** consumes that output, burns its claim token and applies the financial transition. Settlement reconstructs the statement from its transaction and spent state and compares it with the statement checked at issuance. It does not check the Groth16 proof again. The claim output is an intermediary ledger object; it is neither the proof nor a payment to the seller. [N09]

For the one-Strand illustration, that route has **one Issue transaction and one settlement transaction**, excluding opening and any later recovery. If an application genuinely required four separate Strands on this source route, it would need four Issue transactions followed by settlement. Each issued claim holds minimum ADA until release. A number of financial conditions therefore tells you neither the proof count nor the transaction count. Those counts follow the selected relations and consumer rule. [N09]

## What a developer writes, builds and sends

The developer writes the agreement’s state and actions in Moriarty. Here is the actual partial-payment action from the supplied invoice source:

```text
action partial(amount: UInt) {
  requires opened and amount > 0;
  let opening = liability_amount("invoice");
  requires amount < opening;
  fee(CASH, "escrow", "fees", 2);
  transfer(CASH, "escrow", "seller", amount);
  discharge("invoice", amount);
  next paid = paid + amount;
  ensures liability_amount("invoice") == opening - amount;
}
```

The experimental compiler selects an action rather than compiling three screen labels into three proofs. Its selector names the language, semantics, action and supported accounting profile. A transition request supplies state, balances, duties, action inputs and context. Compilation produces a relation descriptor; inspection exposes the statement, decoded financial slots and a fixed-matrix result. A useful inspection compares those results with independently derived expectations, such as the accounts above. An exit code reporting command completion is insufficient: a receipt can contain violated rows. [N10]

Beyond those local artifacts, an integrated application needs an authorized intent, a proof for the selected statement and a transaction builder that follows the consumer’s rules. The owner’s intent describes permitted effects before proving; transaction signatures authorize later ledger transactions. The claimant associated with a claim output has a separate role in its release. Buyer, seller, prover, builder and claimant describe different responsibilities even when one operator performs several. The supplied material does not assign all those roles to named people in this invoice, so a working integration must resolve them rather than infer them from the word “escrow.” [N09, N11]

After submission, the application observes inclusion, waits for the finality it requires and reads back the payments and continuing state. The result to reconcile is the seller’s 35, the fee recipient’s 2, escrow’s 75 and duties of 65 plus 10. A submitted transaction or an issued claim cannot substitute for that observation. If observation times out, the outcome remains unknown until reconciled. The remaining invoice is intentional partial progress; every proof required for the chosen step still needs to be present. [N02, N09]

## Implementation status and reference boundaries

This guide explains a proposed Web integration. The supplied records describe one local current-Rust invoice-partial Groth16 roundtrip and seven finite local compiled-wrapper cases, using a single-party test setup and caller-supplied contexts. The V2 consumer descriptions come from Opus’s reading of unadopted, uncompiled source; those exact bytes are not established as the source of the recorded runs. General Web assembly, genuine intent authorization and onchain settlement remain unestablished in this packet. The 112-CASH calculation above is display data, not an executed source program, proof or transaction. [N12]

The complete invoice lifecycle also needs more work. Another partial payment charges another 2 CASH. After `finish()`, the invoice is present with amount zero, but the 10-CASH reserve obligation remains. The supplied source has no reserve-return or discharge action; paying the invoice does not finish every obligation. Exact statement encoding, transaction signers, fee funding and a concrete proof-to-consumer interface belong in the integration reference. Unresolved details are listed there explicitly. [N07, N11]

Moriarty’s public tooling is intended to remain permissionless: developers may author, compile, prove, publish and deploy supported programs without maintainer, campaign, council or registry approval. Asset-owner authorization and objective ledger rules still apply. Optional proving or relay services remain optional. The broader Moriarty product contract targets Midnight ZKIRv3 and its native proof infrastructure; the Cardano experiment has its own artifacts and qualification work. [N13]

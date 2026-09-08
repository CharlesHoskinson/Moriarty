# SP01.3 independent proposal review

Verdict: **BLOCKED**. Candidate `2f2d4206b37f6991a3ff6741e4bd29479795de42f1e33514134f5831962bb24a`.

Reviewer: fresh independent GPT-6 Astra, high effort. Read-only review of the six frozen candidate files.

This review checks proposal coherence and completeness. It does not accept a successor semantic freeze, runtime implementation, proof, or ledger behavior.

The candidate cannot close SP01.3 as a complete reviewable contract. Archival publication must retain proposed/blocked status and this review.

All six owned hashes and all 30 input hashes matched before and after review. The candidate digest also matched. Three old profile files matched their recorded hashes.

Independent host checks reproduced sourceHash, both claim roots, executionBodyHash, exactPlanDigest and outcomeIntentDigest. Other advertised DAG preimages were absent.

Installed jsonschema 4.19.2 reproduced the malformed-scalar schema passes below. No context evaluator, proof, build, network or install ran.

## R1: The closed wire contract cannot represent the declared Core or lifecycle states

ExactEffect admits only Transfer, Fee, DueCreated and DueSettled. Core also declares Accrual, Capitalize, WriteOff, Mint/Burn, locks, shares, positions, requests, messages and administrative effects. StoredValue admits only UInt128, SInt128, Text and Amount. It cannot carry Bool, Debt, Shares, records, enums, options or collections. ExecutionBody requires at least one effect and swap-specific dust, rateOrPrice and outPredicate for every action. Both signing modes require at least one predecessor. No separate typed genesis signing case is defined. This contradicts the declared complete effect contract and explicit genesis/admin coverage.

References: `semantic-contract.md:295-360`, `semantic-contract.md:378-394`, `signing-display-schema.json#/$defs/ExactEffect`, `signing-display-schema.json#/$defs/StoredValue`, `signing-display-schema.json#/$defs/ExecutionBody`, `signing-display-schema.json#/$defs/ExactPlanDocument/properties/predecessors`.

Required disposition: Define the complete finite typed contract for the declared cases, including genesis and non-exchange actions. Alternatively identify a precise narrower supported proposal domain and preserve every excluded required behavior as an owned obligation. Do not call the current schema a complete successor interface.

## R2: Malformed scalar strings pass the canonical schema

Using installed Draft202012Validator, the complete OutcomeIntent document still validates after principal is changed to "trader\n", or grossDebitCaps[0].maximum to "1\n". The dollar anchor permits a final newline. Canonical JSON re-encoding preserves the escaped newline, so byte identity does not remove the defect. UIntText also accepts 2^128 and Validity.notBefore accepts 2^64. The contract specifies widths, but the companion list does not identify field-specific numeric-domain validation as a required pre-hash check. RequiredClaims also accepts four copies of ContractInvariant. The prose correctly mandates all four kinds, but that check is absent from the schema and explicit contextual-check list.

References: `signing-display-schema.json#/$defs/Identifier/pattern`, `signing-display-schema.json#/$defs/UIntText/pattern`, `signing-display-schema.json#/$defs/Party/pattern`, `signing-display-schema.json#/$defs/Validity`, `semantic-contract.md:54-69`, `semantic-contract.md:515-522`.

Required disposition: Make lexical rejection exact. Enforce each numeric width and all mandatory claim kinds in the schema or in an explicit typed validation stage before hashing. Preserve the distinction between schema validity and contextual acceptance.

## R3: The signed-to-display projection is incomplete

The contract promises display metadata for every authority-impacting field and a complete visible projection. The exact example has 12 entries and the outcome example has seven. The outcome projection omits network, deployment, principal, nonce, validity, program/profile/policy, partial-fill rules, residual permissions, work budgets and assumptions. It shows only the first gross cap, fee cap and net goal. It omits the borrower debit and lender net goal. The debt display omits debtor and creditor. The exact projection omits deployment, validity, transfer recipients and residual allowedEffectSet. Many corresponding schema properties have no x-display. Existing metadata uses relative paths and generic units without a complete deterministic projection rule.

References: `semantic-contract.md:505-507`, `semantic-contract.md:554-560`, `signing-display-schema.json#/$defs/Price`, `signing-display-schema.json#/$defs/Authority`, `signing-display-schema.json#/$defs/OutcomeIntentDocument`, `signing-examples.json#/valid/0/displayProjection`, `signing-examples.json#/valid/1/displayProjection`.

Required disposition: Provide a complete projection for every signed authority field, including each indexed entry, identity and unit source. Specify formatting for compound records and empty prohibitions. Validate projection completeness as well as matching displayed values.

## R4: The acceptance hash construction lacks typed preimages and reproducible examples

GenesisBody, StateBody, ObservationSet, the authenticated Authority wrapper, ProofContext and Prepared body have no complete finite record definitions. The schema Authority is residual capability data, not the authorization digest/signature wrapper named in the DAG. acceptanceBind names a prose concatenation of intent, plan, observations, effects, liabilities, work and predecessors without an exact object or framed byte encoding. The preimages object supplies only hashes for profile, program, genesis, state and observations. The advertised acceptanceBind also has no retained preimage. Neither example supplies a complete concrete acceptance body. Independent sourceHash, claimRoot, executionBodyHash and intent digests reproduce. The other DAG hashes cannot be independently reproduced from the retained artifacts.

References: `semantic-contract.md:492-499`, `semantic-contract.md:531-552`, `signing-examples.json#/preimages`, `signing-examples.json#/valid/0/hashes/acceptanceBind`, `signing-examples.json#/valid/0/hashDag`.

Required disposition: Define closed typed bodies and exact domain-separated preimages for the full DAG. Include the concrete preimages and complete selected-plan output needed to reproduce each advertised hash. Mark omitted hashes as opaque placeholders if they are not evidence.

## R5: Nominal debt, accrual and exchange semantics remain underspecified

Debt contains one outstanding amount and a status, but no typed principal/accrual fields, allocation rule, payment dates or controller. DebtCapitalize returns principal+ although principal is undefined. DebtRepay has no explicit denomination-to-asset settlement mapping or typed allocation rule. DueSettledEffect carries an asset amount only. DebtAccrue accepts signed Rate yet specifies outstanding+ with no negative-rate disposition, accrual period rule or explicit liability-cap check. The Price record does not define which direction its ratio converts. The example signs base=AssetB and quote=AssetA but displays AssetB per AssetA. Output-denominated dust is said to remain in the source reserve, without a typed remainder conversion or disposition. These gaps prevent an independent financial interpretation of the proposal.

References: `semantic-contract.md:81-102`, `semantic-contract.md:333-365`, `semantic-contract.md:378-394`, `signing-display-schema.json#/$defs/Debt`, `signing-display-schema.json#/$defs/DueSettledEffect`, `signing-examples.json#/valid/0/canonicalUtf8/executionBody/rateOrPrice`, `signing-examples.json#/valid/0/displayProjection/8`.

Required disposition: Define the finite liability components, allocation and capitalization equations, negative accrual behavior, cap checks and settlement conversion. Define Price direction and dust accounting explicitly, then make the example and display agree.

## R6: Residual authorization and cumulative history cannot carry all protected obligations

Authority carries allowed effects/assets, gross caps and debt caps, but no fee caps, net goals, recipients, calls, validity, partial-fill rule or originating intent digest. History omits residual authority and uses Amount arrays without actor keys even though signed caps and goals are actor-indexed. It is not referenced by either document. No closed successor record defines how all original constraints remain bound across partial fills, cancellation or migration. The rule child cap <= parent cap does not itself charge already-consumed allowance. Pinned versus AdmittedContext has the same predecessor fields and no distinct interpretation. The contract does not state whether net goals apply per fill, cumulatively at each prefix, proportionally, or only at completion, nor how cancellation preserves an incomplete goal.

References: `semantic-contract.md:396-410`, `semantic-contract.md:474-481`, `signing-display-schema.json#/$defs/Authority`, `signing-display-schema.json#/$defs/History`, `signing-display-schema.json#/$defs/PartialFill`, `signing-display-schema.json#/$defs/PredecessorConstraint`.

Required disposition: Specify exact cumulative equations and successor bindings for every protected field, including actor-indexed accounting and the original signed intent. Define goal checks at partial prefixes, completion and cancellation. Define the two predecessor modes and attenuation of all authority dimensions.

## R7: Work conservation and composition have no complete checking rule

The proposal declares conserved ordinary and recovery budgets but no per-constructor charge, well-founded decrement or exact split/join accounting equation. RecoveryRight embeds a second Work record without defining whether it aliases or partitions remainingWork. The examples repeat the same recovery balance in both locations. The admission table omits numeric maxima for the required node, sidecar-byte and total verification-work checks. Interleave states conflict rule named but supplies no rule or typed policy field, and its decision explicitly leaves exact conflict rules open. par provides no concrete combined prefix or financial framing rule. These are unresolved contract decisions, not only unimplemented proofs.

References: `semantic-contract.md:269-293`, `semantic-contract.md:401-424`, `semantic-decisions.json#/entries/8/openChecks`, `signing-display-schema.json#/$defs/Work`, `signing-display-schema.json#/$defs/RecoveryRight`.

Required disposition: Define the work unit, charges, reserve ownership and conservation equations, including split, join, cancel and migration. Supply every admission maximum or a bounded profile parameter with explicit constraints. Resolve composition policy fields or give explicit unsupported dispositions before claiming a complete operator contract.

## R8: Several negative examples do not establish their stated financial or history failure

inv-net-after-fees-shortfall only raises a fee cap to 30. It adds no Fee effect or selected plan, so the original zero-fee execution can still satisfy the goal. inv-cloned-residual-work changes the controller but keeps the original work value and supplies no parent charge, sibling or partition. A single child inheriting a whole remaining budget is not itself cloning. inv-cancellation-race supplies no accepted winning fill or stale residual context. inv-replay-migration changes the action name without recomputing executionBodyHash and supplies no consumed predecessor context, so its stated history rejection is not isolated. These are schema mutations with narratives, not complete specified contextual rejection fixtures.

References: `signing-examples.json#/invalid/9`, `signing-examples.json#/invalid/10`, `signing-examples.json#/invalid/11`, `signing-examples.json#/invalid/14`.

Required disposition: Provide concrete pre-state, original authorization, selected execution, cumulative history and competing or consumed context for each claimed failure. Ensure all earlier validation stages pass where the intended rejection is contextual. Keep execution status specified-only until a context evaluator exists.

## Scope and remaining gates

The proposal correctly preserves staged updates, four mandatory claims, ledger currentness responsibility and explicit proposal status. Those strengths do not resolve the contradictions above.

I did not inspect live SP01.2, other worktrees, runtime behavior, formal proofs, native cryptography or Preview acceptance. I did not review all primary financial fixtures.

SP01.2 reconciliation and consequential cross-provider majority remain required. Native correspondence, usability and formal evidence remain separate freeze gates. This review grants no broad SP01-SP12 acceptance.

Additional contract gaps: the finite Core table has no literal/read/projection or bounded record/enum/option/collection constructors and accessors. Positions, requests, messages, duties and successors lack complete bounded records. DA13 event claims and DA14 reward/slash accounting lack explicit Core dispositions. Unit-vector arithmetic has no concrete composite quantity result type.

Minimal schema reproduction, with no writes:

```python
import json
from pathlib import Path
from jsonschema import Draft202012Validator
p = Path("/home/charl/Moriarty/.worktrees/sp01-successor-contract-grok/experiments/moriarty-language/spec/successor")
v = Draft202012Validator(json.loads((p / "signing-display-schema.json").read_text()))
e = json.loads((p / "signing-examples.json").read_text())["valid"][1]
d = json.loads(e["canonicalUtf8"])
d["principal"] = "trader\n"
print(v.is_valid(d))  # True
d = json.loads(e["canonicalUtf8"])
d["grossDebitCaps"][0]["maximum"] = "1\n"
print(v.is_valid(d))  # True
```

Schema-valid does not mean accepted. Width bounds and all four claim kinds appear in prose. Their pre-hash typed validation stage needs an explicit contract. Contextual arithmetic, cap-sum, history and currentness rejection was not executed.

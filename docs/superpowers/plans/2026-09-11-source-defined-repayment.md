# Source-defined repayment implementation plan

> Execute the four user-approved AFK steps as one product slice. Grok 4.6 high implements; fresh GPT-6 Astra medium audits the complete final candidate.

**Goal:** Declare the bounded repayment contract in one .mori file and run it without an external schema file.
**Architecture:** Extend the existing bounded parser and formatter behind a new explicit profile. Compile declarations into the existing validated schema, then reuse source lowering, expression execution, and the existing funded adapter/kernel.
**Tech stack:** Existing TypeScript, Node.js and node:test. No dependency or infrastructure additions.

## User authority and completion

The user approved all four steps and armed the native goal while AFK. Do not ask routine permission questions.
1. Declare units, assets, record types and ordinary state fields in source.
2. Elaborate those declarations into the existing checked schema.
3. Check/format/simulate through the normal CLI without --schema, with snapshots supplying runtime values.
4. Reject duplicate declarations, unknown types, mixed units and protected operation redefinition.

All four require actual executable behavior and final independent audit. Passing a parser alone is insufficient.
The existing reviewed funded adapter is the baseline commit e6dc9f68bdb30adaed5adea6838549e7b457dc28.
One action is in scope. Multiple actions, source initialization, native/K/proving/Preview, plugin/infrastructure and other roadmap work are excluded.
The new source profile is authorized by this declaration-language extension; it does not promote a global semantic freeze.
Existing admission/accounting stops remain unchanged and do not block authorized local implementation.

## Design and independent review

Root and independent Astra medium agreed on a distinct `moriarty-financial-agreement-source/1` profile.
The reviewer inspected the parser, source compiler and funded adapter and specified the failure matrix below.
Alternatives rejected: source-string stripping breaks source offsets; a second interpreter duplicates semantics; extending old profiles silently changes their contracts.

Example declarations before the existing repayment action:
```mori
profile "moriarty-financial-agreement-source/1";
agreement FundedPayment {
  unit Cash;
  asset Cash: Asset;
  record TransferFields {
    id: Text;
    from: Text;
    to: Text;
    settlementAsset: Text;
    transferAmount: Amount<Cash>;
  }
  record RepayFields {
    allocationId: Text;
    transferId: Text;
    obligationId: Text;
    payer: Text;
    nominalAmount: Quantity<Units<Cash,1>,0>;
  }
  operation Transfer: TransferFields;
  operation Repay: RepayFields;
  state due: UInt128;
  state paid: UInt128;
  action pay(first: Quantity<Units<Cash,1>,0>, second: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text) {
    requires pre.due > 0;
    let nominal = first + second;
    requires is_negative(nominal) == false;
    let payment = magnitude(nominal);
    requires payment > 0;
    next.paid = pre.paid + payment;
    emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };
    emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };
    ensures post.paid == pre.paid + payment;
  }
}
```

The snapshot and repayment projection from expression-funded-payment remain valid byte-for-byte.
Unit and asset use separate namespaces, so Cash may occur once in each. Same-kind duplicates reject.
All other top-level declaration names must not collide with another top-level name, including unit/asset names.
Record fields have record-local namespaces. Duplicate fields and duplicate action parameters reject.
Keep existing source identifier and reserved-name checks. New record/operation keywords must be contextual or profile-gated.
Collect before resolving to allow forward record references. Reject record cycles and unknown references.
All state fields compile to ordinary writeClass. State declarations have no initializer; every Pre value remains required from the snapshot.
Source fields named like financial fields remain ordinary bookkeeping; they cannot replace or alter the separate kernel projection.
Only the required unit/party/asset, record, ordinary state, operation and single action declaration forms are enabled.
Enum/variant/observation/vault declaration syntax is not required for this slice; unsupported names must reject rather than silently inventing declarations.

## Protected financial contracts

Require exactly Transfer and Repay operations during check/elaborate AND evaluation.
Records may have chosen type names, but their exact fields and value types must satisfy the existing funded operation binding.
Transfer: id/from/to/settlementAsset Text and transferAmount Amount<declared asset>.
Repay: allocationId/transferId/obligationId/payer Text and nominalAmount Quantity<Units<declared denomination,1>,0>.
Reject missing/extra operations, changed fields, scalar substitutions, quantity powers/scales, unknown record references or reclassification.
Reuse one binding validator; do not maintain two divergent definitions.
Runtime must retain settlement-unit and nominal-unit witnesses, nonnegative signed128 range, kernel identifier/relationship checks and funding rules.
Use one ordered kernel invocation. Preserve complete residual projection, histories, allowance spent and work.
Work remains actual expression reductions E plus action count N; closureReserve stays separate and unchanged.
Check/format do not authenticate snapshots or establish financial authority. Source ensures still observes ordinary post only.

## Files and interfaces

Modify `experiments/moriarty-language/src/successor/frontend.ts` and `format.ts` for gated AST/parser/formatter entries.
Use distinct AST variants for uninitialized state fields, records and operation declarations; do not weaken old initialized StateDecl.
Update exhaustive consumers only as necessary to retain rejection/behavior of older profiles.
Create `src/successor/financial-agreement-source-v1.ts` within the language package.
Export `createFinancialAgreementSourceV1()` with `elaborate(source)`, `check(source)` and `evaluate(source,snapshotCanonicalJSON,repaymentStateJSON)`.
Elaboration returns the source-derived canonical schema and checked Core, with original source/profile and spans. These outputs are inspectable, not executable input.
Evaluation accepts primitive source/snapshot/state strings only and compiles internally each time.
Expose the profile constant and parser/formatter through clear named exports.
Refactor only the needed shared source compile/evaluate seam in `financial-expression-source-v1.ts` (or a small internal module).
The internal seam consumes original source, parsed action, and validated schema. Do not regenerate/strip/replace source text to fit the older factory.
Public APIs never accept caller-supplied AST/Core/schema for the new profile. Do not allow an injected evaluator callback to bypass checking.
Reuse the funded stage in `funded-expression-source-v1.ts`; factor shared protected binding and finalization if needed.
Keep the repayment kernel and numeric rules unchanged.

Modify `src/cli.ts` for these exact public forms from repository root:
```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/1 SOURCE
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/1 SOURCE
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/1 --snapshots SNAPSHOTS --repayment-state STATE SOURCE
```
The new profile rejects --schema. Simulate requires both snapshots and repayment state; check and format require neither.
Preserve old profiles and CLI argument forms exactly. Preserve bounded UTF-8 regular-file reading and exit0/1/2 conventions.
Use `SourceSimulated` with new sourceProfile and existing pre/financialPre/initialWork/result envelope for success.
Source rejection uses original diagnostic spans and exposes no tentative result. CLI semantic rejection has empty stdout and JSON stderr.

Create tests `tests/financial-agreement-source.test.mjs` and `tests/financial-agreement-cli.test.mjs`.
Create `spec/successor/financial-agreement-source.md`, `financial-agreement-source-grammar.ebnf`, and `examples/source-defined-payment.mori`.
Reuse the existing snapshot/state fixture files instead of duplicating them. Update package README and CLI documentation with exact runnable commands.

## Execution steps

- [ ] Write focused tests and retain a failing run before implementation.
- [ ] Implement bounded declarations, compilation, original-span diagnostics and format roundtrip.
- [ ] Implement protected binding checks and the normal CLI consumer.
- [ ] Add the self-contained example and documented commands.
- [ ] Verify generated schema equals the existing example schema canonically.
- [ ] Verify the same action/snapshots/state produce identical ordinary post, financial post, effects and work as the existing schema-based path.
- [ ] Verify due100/pay30, interest-first P100/I10/pay7, continuation, and failed funding/guard/ensure/work cases.
- [ ] Verify duplicate declarations/fields/parameters, forward and unknown references, cycles, mixed units and altered protected operations.
- [ ] Verify initialized state and attempted financial reclassification reject.
- [ ] Verify comments, Unicode and declarations before an error preserve original source offsets.
- [ ] Verify format(format(source))==format(source), generated schema stable, and formatted simulation equivalent.
- [ ] Verify transport bounds and old-profile source/CLI regressions.
- [ ] Run all package tests and typecheck; independently run actual CLI cases.
- [ ] Freeze full candidate hashes and obtain fresh Astra medium audit; repair through the same Grok author if needed.
- [ ] Integrate only baseline-checked task changes; publish reviewed branch and result evidence.

## Verification commands

From implementation worktree root:
```sh
node --test experiments/moriarty-language/tests/financial-agreement-source.test.mjs experiments/moriarty-language/tests/financial-agreement-cli.test.mjs
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run typecheck
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/1 experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/1 --snapshots experiments/moriarty-language/spec/successor/examples/expression-funded-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/expression-funded-payment.state.json experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori
```
Successful funded simulation must equal the schema-based baseline result; source declarations add no expression work.
Static source failures must reject before snapshot-dependent work. Do not replace failing cases with weaker expectations.

## Write scope and stop rules

Author writes only `experiments/moriarty-language/` and this task's deliverable directory.
Root owns plan/evidence/integration. Preserve user changes and existing failed receipts.
No installation, network research, transaction, proving, service control, git commit/push or new subagents from the Grok author.
After two same-class repair failures, reproduce the exact failure and change the approach before another retry.
Missing author/auditor is not approval; no silent model substitution.

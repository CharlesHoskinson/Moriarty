# Early SP03 boundary and resource design review

Reviewer: fresh independent GPT-6 Astra delegated reviewer (`sp03_boundary_review`). Date: 2026-09-09. Scope: proposals and retained kernel only. No product source edits, K execution, installs, proof attempts, candidate acceptance, or execution admission occurred.

Decision: **approve the bounded source/reference and independent K projection design subject to the conditions below; approve the proposed resource ceiling as a design vote only.** This is one substantive vote, not majority approval or dispatch authority. Reject any claim that this review approves candidate bytes or completes SP03.

Repository observations: startup AGENTS.md and checked-in develop skill were read; guarded CLI status returned an old blocked SP01 campaign with stale bindings, missing current accounting and unavailable resource live state. No pending transactions were reported. This does not block this authorized read-only review and does not admit dependent campaign execution. The proposed execution note `openspec/sprints/execution/SP03.md` is absent; the SP03 plan itself permits reuse of sufficient task records.

Reviewed inputs (SHA-256):

- source-direct-prompt.md: `95f5b56aaf2e1bd2fa0edfcc94c7cceaf37ab764ced75ded2e430933977247d3`
- k-task.md: `2990978c6f23443d1c5ad75cc169cef9645880bb06ed5e6b7ff8d10b637710ad`
- openspec/sprints/sp03-executable-bounded-semantics-in-k.md: `d79249fb580a0434ded931eacdac284ef738e9729b1cc8876388de33c29ff3b0`
- experiments/moriarty-language/src/successor/repayment.ts: `e1be96f067ddb2c5166342012388e0f612871705cd6aa74279a45ef9b288c808`

Also inspected: repayment-kernel.md and repository orchestration stop rules. Findings below are design risks and acceptance conditions, not assertions that unpublished implementation contains these defects.

## Semantic approval conditions

1. **Preserve the source trust boundary.** Execute the actual `.mori` parser, elaborator and selected action through the public CLI. A fixture-selected action list, unconditional canned repayment, or two calls to the retained evaluator cannot establish source or K agreement. Lower exactly the typed emissions in their source order. A standalone `next.principal` subtraction must reject. Perturb a source argument and observe both cash and debt change. Reject unsupported syntax, expressions, declarations, field types, duplicate names and unknown fields rather than erase them.

2. **Prevent type erasure at runtime.** The retained kernel accepts numeric nominal amounts without a source type witness. The adapter must bind `Debt<Unit>` to the selected obligation's actual denomination and `Amount<Asset>` to the actual transfer asset before lowering. A caller must not execute a forged or mutated serialized Core merely by presenting a matching profile string. A nominal unit and settlement asset with different names must remain distinct even under conversion 1. Changed obligation conversion must be honored by TypeScript, and explicitly unsupported by this K projection; it must never be overwritten with identity conversion to obtain agreement. Bound input before parsing/coercion; reject hostile object/prototype/accessor entry paths if public APIs accept objects.

3. **Keep the complete supplied projection.** Preserve unrelated obligations, balances, allowances, terms, conversion, array order and tombstones in the source/reference path. Do not rebuild state from the one fixture obligation or source declarations. Unauthenticated supplied state supports local Prepared candidates only. K's narrower one-obligation domain cannot establish preservation of omitted duties, prior replay history, third-party repayment, ProRata, rounding, refunds, or richer successor records.

4. **K must decide admitted financial failures.** Separate closed-schema/projection admission from financial rules. Once an input has the supported two-action structural shape, do not classify wrong transferId, obligationId, payer, recipient, asset, insufficient funding, invalid outstanding/status, zero amount, excessive payment, cash, allowance or work failures as unsupported merely because the desired successful relation does not hold. Pass the distinguishing values/relations to K. The restriction to debtor-funded successful cases must not make all mismatches disappear in Python. If a case cannot be represented, explicitly narrow the tested domain and make no claim that its financial predicate ran in K.

5. **Cash transferred is not nominal debt discharged.** Include an independently expected supported trace transferring 40 and repaying 30: payer cash -40, creditor cash +40, gross allowance -40/+40, debt -30, and repayment settlementAmount 30 under identity conversion. The unallocated transfer remainder is internal and must not be refunded, consumed as additional debt discharge, or emitted as a new public field. Include the inverse insufficient-funded case. A fixture where transfer and repayment always equal cannot distinguish these defects.

6. **Compare complete results and rejection order.** Positive independent expectations include due100/pay30 -> P70, TX02 AccrualFirst P100/I10/pay7 -> P100/I3 and PrincipalFirst -> P93/I10; verify cash, allowance, work, reserve, retained metadata, tombstone appends and ordered effects. Late Repay failure must expose neither Transfer effects nor post-state. The retained kernel validates the entire state/actions before work, then Transfer, then Repay. Supported rejection comparison must include exact code and actionIndex, including multiply-invalid precedence; unsupported projection rejection is a separate observation, not equivalent to a kernel rejection. Work reserve cannot pay ordinary action cost.

7. **Constrain the trusted codec honestly.** K output must carry changed numeric values, financial success/rejection, status and discharge components. Python may decode and reconstruct unchanged metadata, ordered effects and identified tombstone appends, but must not independently determine financial success, derive changed balances/work/debt, or replace malformed output with expected JSON. Bind K tuple fields to the exact input and check arity, tags, types, ranges and full output consumption. A stale compiled definition, wrong fixture, fallback evaluator, empty trace set, or subprocess error must fail closed. Fixture expectations must be independently written, not generated by the tested implementation.

8. **Keep the claim domain narrow.** K currently proposed consumes a projection of lowered RepaymentInput, not the full typed Core or source AST. Actual three-way fixtures would support finite observations on the intersection of the source and K domains. They do not establish parser-to-Core preservation, elaboration/evaluator equivalence, determinism, termination, type preservation, full residual-duty conservation, a successor freeze, financial ledger acceptance, or SP03 completion. SP03 explicitly keeps those proof and full-domain gates open. Missing Transfer as UNSUPPORTED_PROJECTION is useful admission coverage, but not a K execution of unfunded Repay rejection.

## Resource design vote

Approve the ceiling: one LLVM optimization-0 compile attempt with a 180-second timeout, at most 16 total krun invocations with at most 20 seconds each, aggregate elapsed execution at most 512 seconds, peak aggregate memory at most 4 GiB, one heavy command tree at a time, and no proof, acquisition, installation, network or native financial execution. This is a reasonable bounded feasibility experiment, not an assurance that K will fit.

Conditions for later execution admission:

- Review actual source, codec, fixture/expected bytes, exact compile/trace commands, compiler executable/version/backend and current admission inputs. This proposal is not a commitment to nonexistent candidate bytes. Preserve unresolved acquisition digests explicitly; a version string alone is not pinned provenance.
- Enforce memory on the complete compile/krun process tree, not 4 GiB per child; ensure timed-out descendants are terminated and accounted for before any next command. Define one heavy process as one command tree, with no overlapping compile/krun/native workloads.
- Apply one cumulative deadline across compile, fixtures and wrapper overhead. The maxima consume 180 + 16*20 = 500 seconds, leaving only 12 seconds overhead. Clamp later child timeouts to remaining aggregate budget; do not restart the clock for each command or after failure.
- Count actual subprocess invocations, including accidental duplicate fixture execution. No automatic compile fallback, compile retry, proof command, increased memory/time ceiling or hidden extra trace is covered by this vote. Stop and retain measured failure evidence on exhaustion.
- Retain source/profile and compiled-artifact binding, exact commands, exit codes, raw output, elapsed time and resource observations. The later fresh exact-byte audit remains required even if all finite traces pass.

Reject execution under this vote if any condition is absent. Continue ordinary authorized source work independently while candidate/resource admission remains unresolved.

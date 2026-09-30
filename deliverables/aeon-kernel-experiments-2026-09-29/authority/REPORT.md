# Aeon authority-envelope experiment

**Status:** experiment observation and scoped inference, not an adopted MIL/4 decision or acceptance proof. **Date:** 2026-09-29.

## Source and command

Primary source: [Aeon](https://github.com/alcides/aeon), remote `https://github.com/alcides/aeon.git`, branch `master`, pinned commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f` (2026-09-17 commit timestamp). The local editable checkout is `/home/charl/.graphify/repos/alcides/aeon`. Its tracked source was unchanged for this experiment; a pre-existing untracked `graphify-out/` directory was present. `python -m aeon --version` returned `aeon 4.9.0`; the interpreter returned Python 3.13.15. Public repository documentation describes Aeon as a language with liquid refinements and SMT validation. The earlier Moriarty [Aeon study](../../aeon-study-2026-09-19/RESEARCH.md) established the driver usage and trust-reporting caveat.

Run from the Moriarty worktree root:

```bash
/home/charl/.graphify/repos/alcides/aeon/.venv/bin/python deliverables/aeon-kernel-experiments-2026-09-29/authority/run.py > deliverables/aeon-kernel-experiments-2026-09-29/authority/run.log 2>&1
```

The script uses `AeonDriver.parse` with `no_main=True`, `strict_decidable=True`, an SMT synthesizer setting and a zero synthesis budget. It writes exact `.ae` inputs and [machine-readable results](results.json). [CLI-RESULTS.md](CLI-RESULTS.md) records fresh-process CLI confirmations and the exit codes. `reduce.py` and `reduce.log` preserve the bounded reduction sequence. This is a source-level experiment on integer stand-ins for domain and recipient identifiers. No signature, canonical byte encoding, adapter, ledger or Midnight transaction was checked.

## Tested predicate

The intended fixed envelope specifies recipient ID 42, destination domain ID 10, maximum gross debit 100 and maximum fee 5. A solver proposal gives recipient, domain, principal, fee and gross. The intended static predicate is:

```text
recipient = 42 ∧ domain = 10 ∧ 0 ≤ fee ≤ 5
∧ gross = principal + fee ∧ gross ≤ 100
```

The input *assumes* the envelope constants already came from an authenticated user signature. Aeon does not establish that assumption. The probe asks whether this pinned Aeon path rejects candidate substitutions encoded as refinements.

## Results

| Case | AeonDriver result | Interpretation |
| --- | --- | --- |
| Authorized plan, including combined gross constraint | Accepted | Positive control, but the combined cap is unreliable given the breach result. |
| Authorized plan, separate gross and fee cap obligations | Accepted | Positive control for the split encoding. |
| Recipient 43 instead of 42 | Rejected: `LiquidTypeCheckingFailedRelation` | The specific equality obligation was enforced. |
| Domain 11 instead of 10 | Rejected: same error class | The specific domain equality was enforced. |
| Fee 6 against cap 5 | Rejected: same error class | The specific fee obligation was enforced. |
| Gross 101 against cap 100 in `g = principal + fee && g <= maxGross` | **Accepted** | Counterexample to treating this Aeon encoding as an authority gate. |
| Same gross 101 with a separate `grossWithinCap` declaration | Rejected: same error class | Splitting the obligation changed the result. |
| Gross 97 when principal 94 plus fee 4 requires 98 | Rejected: same error class | The specific equality obligation was enforced. |
| Native value `43` asserted to have recipient 42's refinement | Accepted; trust report names `planRecipient` as `native` | A native annotation is a trust premise, not a verified implementation. |

Fresh-process CLI confirms the unusual result: `minimal_breach.ae` (`v = 101 && v <= 100`) and even `minimal_false.ae` (`false`) returned exit code 0, while `minimal_cap.ae` (`v <= 100`) returned exit code 13 and a type error. [CLI-RESULTS.md](CLI-RESULTS.md) lists the exact commands and logs. This rules out a mere same-process cache collision in the probe script. It does **not** establish the compiler's internal root cause. We did not modify Aeon or run its full test suite.

## Disposition for the kernel proposal

**Experiment observation:** Aeon can express selected simple authorization relations and reject some substitutions. **Experiment observation:** this pinned path also accepts an impossible refinement, including a gross cap breach. **Inference:** it cannot be used as a sound admission or settlement checker for the proposed DeFi kernel. A split encoding is useful for investigating pure candidate plans, but its observed rejection of one breach is no proof for all candidates or compiler paths. A signed intent still needs exact-byte authentication; any dispatched adapter payload must be independently matched to the authorized leg; acceptance must evaluate actual effects and financial limits in Moriarty/Midnight's specified path. A successful Aeon type check cannot authorize a signature request, bridge call, refund or final settlement.

**Open question:** identify the pinned Aeon implementation defect behind impossible refinements being accepted and determine whether a later upstream commit changes it. That is separate from the present design conclusion: even a repaired Aeon checker would not supply the cryptographic and ledger correspondence that Moriarty requires.

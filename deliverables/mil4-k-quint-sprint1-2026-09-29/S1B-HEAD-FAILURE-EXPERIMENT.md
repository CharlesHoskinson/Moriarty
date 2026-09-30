# MIL/4 S0 next finite experiment: head, failure and premise observations

**Status:** initial expectation matrix and executed finite experiment. The [result](S1B-FINITE-RESULT.md) records later controls and observations. W-D3 and Sprint 1 remain open. The executions do not establish a verified head extension, signature, native proof or ledger admission.

## Comparison boundary

Use one exact transfer pre-state, signed intent, round and complete effect vector from T-10-1. Name the predecessor `h0`, the stipulated expected successor `h1`, and an alternate token `h9`. K and TypeScript use opaque head tokens; Quint may use distinct abstract IDs, but it must compare identity and must not infer validity from integer addition. The executed stipulated external tuple is `(intent, pre-state, round, expected successor, requested outcome)`. It is an assumption in this experiment, not an authentication result.

Keep the six-judgment order `Stage → Intent → Effect → Authority → History → Failure`. Within History, check stale predecessor, then consumed replay, then successor mismatch or self-successor. Reject with one abstract diagnostic-work unit and no published post-state or effects. A rejected attempt leaves financial state, allowance, replay, head and work counters unchanged.

| Case | Modification to otherwise valid T-10-1 | Expected first observation |
| --- | --- | --- |
| H1 | Post-head and `AdvanceHead` line are `h1`; terminal outcome and empty retained vectors | Local success under the stipulated tuple |
| H2 | Post-head and complete effect line are `h9`; tuple still expects `h1` | History / `S0_HISTORY_SUCCESSOR` |
| H3 | Post-head and complete effect line are `h0` | History / `S0_HISTORY_SUCCESSOR` |
| H4 | Post-head is `h1`; effect line says `h9` | Effect / `S0_EFFECT_MISMATCH` |
| H5 | Signed predecessor is stale and successor is wrong | History / `S0_HISTORY_STALE` |
| H6 | Composite replay key was consumed and successor is wrong | History / `S0_HISTORY_REPLAY` |
| F1 | Requested phase is failure, with all earlier judgments valid | Failure / `S0_FAILURE_UNSUPPORTED` |
| F2 | Terminal success retains one effect | Failure / `S0_FAILURE_UNSUPPORTED` |
| F3 | Terminal success retains one duty | Failure / `S0_FAILURE_UNSUPPORTED` |
| P1 | Required premise is unavailable | Stage / `S0_STAGE_PREMISE` |
| P2 | Stipulated tuple changes the intent, pre-state or round, one at a time | Stage / `S0_STAGE_PREMISE` |
| P3 | A required typed cell is malformed and the premise is unavailable | Stage / `S0_STAGE_UNSUPPORTED` |

The first S1B audit found that the initial cases did not isolate all stated precedence claims. The post-audit corpus adds stale+replay, effect mismatch+replay, effect mismatch+wrong successor, and effect mismatch+requested failure. It also adds an explicit self-successor case where the tuple itself expects the self head, an outcome-mismatch tuple case, and a Quint case where a locally committed head lacks a fresh authenticated snapshot premise. These cases are in the [post-audit result](S1B-FINITE-RESULT.md); the original matrix above remains the initial independent expectation set.

The finite relation compares complete observations, including first code, diagnostic work, state preservation and the exact accepted effects. A separate qualified consumer would have to establish that `h1` actually extends the authenticated pre-state. The TypeScript preparer may expose a comparison entry point that accepts a stipulated tuple, but its ordinary result remains `PreparedUnqualified`.

## Original implementation and review sequence

1. Preserve the current frozen packet and its reviewer receipts. Fix any findings from both independent reviews before freezing the next candidate.
2. Add submitted successor and typed requested outcome to the abstract comparison inputs. The outcome contains phase, retained effects and retained duties. Make unsupported forms reach Failure after the earlier judgments pass.
3. Execute H1–H6, F1–F3 and P1–P3 in K, Quint and the local TypeScript preparer. Require literal complete outputs; a judgment-only witness does not support code agreement.
4. Publish a field-by-field finite map and a new packet hash. Obtain independent Grok 4.7 xhigh and GPT-6.1 Sol high reviews on those same bytes. Keep W-D1/W-D2 and external qualification open.

This experiment can establish only local behavior under stipulated premises. It cannot establish the premise or turn a caller token into a valid successor.

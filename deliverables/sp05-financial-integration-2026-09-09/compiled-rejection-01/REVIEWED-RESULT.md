# Reviewed compiled rejection checks

GPT-6 Astra and Grok 4.6 independently returned `PASS_SCOPED` for the same candidate. The [result receipt](reviewed-result.json) binds the candidate and both reviews.

The three actual compiled swap calls reject the wrong revision, program or network before transaction creation. All 11 checker tests and 18 loader regressions passed, with no skips. GPT-6 reran both suites; Grok reviewed the supplied source and retained results. Grok completed in 376.034 seconds with no caller timeout.

This establishes rejection during SDK call preparation on a retained historical public state with synthetic private inputs. It does not establish rejection by a node, financial state preservation after submission, Preview settlement or PCD. SP05.2 remains open.

The original candidate README, manifests and failing test logs are preserved. Their pending-review labels describe the earlier capture; this record supplies the later review outcome.

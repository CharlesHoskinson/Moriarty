

### DW-64 — Preserve review coverage through compaction and rerun only the changed case

A compressed handoff or a continued conversation is not a fresh full-source
review. Preserve the exact candidate, complete relevant bodies, independent
expectations, actual results and delivered membership. When replacing a large
packet, show how any lossless edit map reconstructs the omitted source; do not
ask a fresh reviewer to infer code from earlier conversational summaries. The
[paired-checker review](../docs/results/sherlock-paired-model-qa-20261018/review/r253-revision02/REPORT.md)
used a fresh context with the complete current checker/test/driver and exact
old-checker reconstruction, then returned a new source/result/reader judgment.
A previous review's missed `uiReturn` versus canonical `uiRet` fixture literal
remains part of the record; compaction does not erase that mistake or promote a
prior summary into coverage of changed source.

The first actual ten-case QA run had nine passes and one failure. Correct only
the two mistaken fixture literals against unchanged K/oracle declarations.
Execute the formerly failing method once, and retain the other nine historical
passes with the fresh source-compatibility judgment. The corrected case passed;
there was no fresh ten-case run and no rerun of the unchanged 254-test cohort.
Keep the original failing output, correction, exact selector and resulting pass
separate. Never fill an absent producer result from matching supplied fixtures.

Preserve review findings through integration: missing-field error typing,
unavailable top-level raw context on a program-identity difference, and the
unexercised F03 loading branch remain NB-01/02/03. Actual F03 K/Quint execution
and original UIREQ-021 stay open. The two current checker-hash cells maintain
an existing reporting closure; they add no new interpreter or formal closure.
Root also restored the ledger's original byte encoding after a JSON serializer
changed unrelated Unicode escapes: equivalent decoded values do not satisfy an
exact two-literal byte-edit claim. Use the guarded pointer patch and additive
correction, not a whole-ledger replacement that could overwrite another lane.

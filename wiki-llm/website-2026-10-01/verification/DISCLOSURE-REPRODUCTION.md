# Dynamic context reproduction before reader-pin repair

Read-only browser reproduction against `http://127.0.0.1:8896/Moriarty/tutorial.html`, desktop 1440 × 1000, using the actual Chrome binary and repaired anchor build before the subsequent S1 reader-pin correction. The failed full interaction record is preserved at `browser-check-20261001T130658058078Z/results.json`.

Sequence and observed values:

1. Fresh `#transfer`: scrollY 6273; transfer top 76.42px; repayment top 2613.06px; current Transfer.
2. Click the first transfer disclosure, scenario.json: scrollY 7324; repayment top 2436.56px; current Transfer.
3. Use the harness's original repayment heading `scroll_into_view_if_needed`, then repayment `scrollIntoView({behavior:'instant'})`: scrollY 9685; repayment top 75.56px; current Transfer.
4. After 2.2 seconds and 5.7 seconds: geometry and stale current label unchanged.
5. Send a real 1px wheel event: scrollY 9686; repayment top 74.56px; current Repayment.

The programmed hold did not restore the transfer scroll position. Source inspection showed that reader input canceled the hold, while the active-place pin was separately cleared for wheel/touch. A scroll update during the transient pin window could retain the old label without a later event to recompute it. Root identified keyboard as a related product risk and requested S1 clear the pin on all reader inputs, while preserving internal hold setup.

The temporary wheel substitution in the external interaction harness has been reverted. Its original disclosure → programmatic repayment scroll → current label assertion is retained to test the source repair. The external all-anchor check now also uses actual PageDown input to enter another lesson and compares its current label with rendered geometry. These latest script changes have not been executed by D1; root runs the final candidate. No GuideApp edits were made by D1.

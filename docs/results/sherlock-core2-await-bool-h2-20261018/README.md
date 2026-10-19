# Core Bool scalar representation: H2 slice

The supplied-observation consumer now requires an actual bool payload for the Core Bool scalar variant. The only consumer change is the two-line Opus guard in `_scalar`; canonical UInt conversion and every other caller/function are unchanged.

A [fresh Opus source review](independent-source-review/FINAL-RESPONSE.txt) accepted this slice and found one new two-test operation eligible. Root ran only those two new methods once: exit 0 in 0.085584539 seconds, two named `ok` results, `Ran 2 tests`, `OK`. Full [actual receipts and streams](actual-new-two-01/RESULT.json) retain the exact two-key environment, three-entry directory and before/after source/Python pins. H1's two tests and the original six tests were not replayed.

The source author supplied the guard and assertions, with an explicit instruction to adapt the assumed test package import to the actual standalone QA form. The original response and one-line import adaptation are retained. The independent review accepts the selected import; its limited abstentions on unsupplied historical files remain disclosed in the full opinion and [qualified record](QUALIFIED-RESULT.json).

This is only the Core Bool scalar part of H2. Other advanced/numeric/Error/Quint fields and H3–H6 remain open. Full consumer/caller integration, CLI and real Core2/B06 common-input captures are unestablished. These two same-author representation assertions establish no independent financial trace, program/effects pairing, admission, native, B-charge or EARS closure.

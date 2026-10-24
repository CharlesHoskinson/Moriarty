# Canonical Core UInt representation: H1 slice

The Core2 supplied-observation consumer now accepts canonical ASCII UInt decimal strings through the u128 maximum and rejects noncanonical or coercible representations. The only consumer change is the Opus-authored `_uint` parser; the existing decoder is unchanged.

Root ran exactly the two new Opus-authored primitive tests once: exit 0 in 0.076251225 seconds, two `ok`, `Ran 2 tests`, `OK`. Complete streams and postchecks are in [actual-new-two-01](actual-new-two-01/RESULT.json). This is direct private-helper evidence; full consumer integration, caller compatibility, CLI and real Core2/B06 common-input captures remain unestablished. The old six passing tests were not replayed.

The [fresh source review](independent-source-review/FINAL-RESPONSE.txt) found the parser/tests semantically acceptable but identified F1: initial materialization dropped one blank line outside the author hunk. Prior exactness claims for that 12,300-byte artifact were false. The selected 12,301-byte consumer restores the newline, preserves all outside-hunk bytes and meets the review's explicit conditional ACCEPT/ELIGIBLE terms. Root verified the corrected pin in the original three-entry cwd before running. The full preimage, selected diff/inverse, repair diff and Root receipts retain that distinction; an obsolete freeze is not presented as current.

[Qualified result](QUALIFIED-RESULT.json) retains H2–H6 and review limitations. These two same-author representation assertions and the independent source audit establish no independent financial trace, program/call pairing, model/runtime/native/B-charge or EARS closure.

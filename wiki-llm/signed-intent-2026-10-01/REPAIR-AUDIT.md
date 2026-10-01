# Review repair: supplementary rejection detail

Candidate12358d37 emitted a literal terminal escape/bidi control when a signature
artifact contained an unknown statement key and --review showed the first mismatch
pointer. The pointer was interpolated into an extra detail line; renderErrorReview
escaped code/message but appended extra lines verbatim. Source/schema rejection
and ledger_accepted:false remained intact, but the human screen was unsafe.

The exact hostile case and two failing regression tests are retained in evidence.
The renderer now escapes every supplementary line at the display boundary. The
portable permanent tests exercise ESC, bidi, newline and tab in both the pure
renderer and actual native-backed CLI. Review is printable ASCII/newlines, while
JSON preserves the original pointer after decoding. Both regression tests pass.
No signature, range, source-match, Core or authority validator was weakened.

The parser-limit diagnostic now reports the actual configured byte limit instead
of always65536. Local store assertions now name the real scheme/framing unions;
actual runtime values are unchanged. Existing ECDSA atomic coverage and the full
runner's Schnorr coverage retain their scope; no new atomic-Schnorr claim is made.
The changed full candidate needs both fresh independent result audits.

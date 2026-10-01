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

## Native response classification

The second candidate2dfcc570 preserved parser LocalError codes from a native
child response. A duplicate response key therefore surfaced as BETA_JSON_DUPLICATE,
CLI exit1 and a rejection judgment. The correct response is exit2 with unknown
signature validity. The exact reproduction and three failing regression checks
remain; the malformed caller-artifact control passed even before the repair.

Only the native response boundary now normalizes parser errors to BETA_CRYPTO_RESPONSE.
The caller signature parser retains its BETA_JSON_* input errors and exit1. Four
permanent tests exercise duplicate/numeric/Unicode/depth/string native failures,
actual CLI JSON and review exit2, and unchanged caller duplicate-key exit1.
All four pass. The prior test expecting a raw parser code from a native response
was corrected to the contract. No acceptance validator is relaxed.

The unfinished second Grok review was interrupted after the confirmed blocking
finding; it has no approval and no structured provider receipt. SIGINT/process130
and the explicit interruption record remain. Both fresh full result reviews are
required on the repaired candidate; the first Grok approval is historical only.

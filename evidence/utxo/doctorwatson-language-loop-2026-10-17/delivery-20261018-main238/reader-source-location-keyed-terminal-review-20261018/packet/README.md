# Keyed coverage review-format candidate

This packet changes the requested response format for the unchanged candidate
`b3b4f9166832d3905492654edbc8f720cef8642b`, tree
`4dbe42ccf802c17e5a4dd4e49c9f37df38b480c1`.

Use `ROOT-PROMPT.md` with separately structured `OUTPUT-SCHEMA.json`. The required
direct root `reviewToken` and task prefix are both
`reader-location-keyed-b3b4f916-20261018-v2`. `SHORT-PROMPT.md` contains the short
instructions; it must accompany the complete `REVIEW-INPUT.json` if used instead
of the self-contained root prompt.

`pathCoverage` is an object keyed by all four exact required file paths. Every
original per-path hash, same-path exact-quote condition and read-scope condition
is retained unchanged. Canonical JSON coverage remains the complete EU-NAT-028
row at physical lines 13597–13891 plus its exact rebinding; all 147 other parsed
rows are verified unchanged. The full raw file remains physically supplied,
without a whole-file model-read claim. The other three changed file bodies are
complete. Source/result axes remain separate within one complete opinion, and
identity limits are required.

The candidate/source/result bodies and every historical failure are unchanged
from the public original packet. The root-reported prior review ended in
`NO_EXPORT`; its underlying reason remains unknown. The keyed format is an
unconfirmed compatibility hypothesis. This packet makes no claim that the prior
tuple schema caused that outcome. No private capture was reread.

`FORMAT-VALIDATION.json` records the offline schema/body checks. The manifest
and full root prompt together are below 125,000 bytes. No source/result text was
truncated; only the duplicate embedded schema was removed because the complete
schema is supplied separately. `MANIFEST.json` pins both original public inputs
and the new prompt/input/schema.

No provider, subject runtime, shared source or Git mutation occurred. Original
records remain immutable. This is a response-format candidate for a fresh
ordinary source/result review, not a review verdict or product acceptance.

---
title: "Learn why a correct witness is not a sound circuit"
diataxis: tutorial
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# A correct witness is not a sound circuit

This conceptual exercise needs paper and pencil. It does not claim runnable Moriarty or ZKIR syntax.

Specify a Boolean assertion that succeeds only when its input is exactly `1`. Let an honest witness generator always supply `1` for a successful assertion. Test it several times: all tests pass.

Now suppose the target constraint merely says the input is nonzero. Supply `2`. It satisfies the target relation but not the Boolean source judgment. The honest generator was correct; the constraint relation was too weak for that source claim.

Add a Boolean constraint and recheck both values. The pair of conditions now accepts `1` and rejects `2`. State the field assumptions and canonical encoding rather than relying on a source type annotation alone.

Finally place the assertion inside a larger program. Check that the caller constrains the supplied value and that the result participates in the relevant accepted relation. An unused check output cannot enforce a contract condition.

This mirrors the kind of witness-side premise examined in [PR17's applicability audit](../../../../deliverables/aeon-study-2026-09-19/PR17-APPLICABILITY.md). It demonstrates an abstract proof obligation, not a discovered deployed Midnight exploit.

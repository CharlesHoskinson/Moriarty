# Persona 06 — Transaction-builder engineer

I followed the staged-invoice example through the Web explorer, inspected Fee accounting, and read the disclosures that describe Issue, settlement, funding, release, and current evidence. The frozen HTML SHA-256 matched the supplied manifest.

The page gives a comprehensible *proposed* route: each required Strand is issued as a claim output, then settlement consumes those claims, recomputes statements, and requires claimants’ signatures and token burns. It separately says owner intents are signed before proving. These details are labeled V2 source readings of uncompiled files; the page reports no on-chain transaction and says build/submit/observe interfaces are not runnable here.

The main builder question left open is the transaction-by-transaction signer set, especially where owner-intent signatures are carried relative to Issue. The page also does not show an observed acceptance or effect-readback signal that lets a builder report settlement success. A compact lifecycle map with signer roles and evidence status would make the intended handoff easier to implement without implying that an unobserved stage exists.

I inspected the supplied desktop-top, mobile-top and explorer screenshots and my own captured Fee accounting inspector state. The own screenshot shows the selected role, explanatory boundary and case arithmetic.

See [FEEDBACK.json](FEEDBACK.json) for typed findings, question dispositions, teach-back, limits, and full field contract; [navigation.json](navigation.json) records the browser actions and results.

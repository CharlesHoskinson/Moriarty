# Arithmetic and fuel witness source design

This folder publishes the exact `sherlock-k-a-aux/2` source proposal and the
conditional Root adoption. It specifies checked UInt128 Add/Sub/Mul/Lt byte
relations and fuel probe, decrement and preservation relations. It preserves
the four-record aux/1 prefix. It implements no circuit or native witness ABI.

The source manifest's UNADOPTED status is its authoring snapshot. The later
[Root record](ROOT-SOURCE-ADOPTION.json) adopts these unchanged bytes only under
the complete fresh Opus and Muse reviews. The author is not a review vote.
Each compressed review is an exact public capture, mapped by SHA-256 in
[capture-mapping.json](capture-mapping.json).

Owner errata E1–E7 and conditions 1–7 govern. Muse E-FUEL and conditions 1–6 also
govern. In particular, the illustrative fuel list omits charged A108. The
normative `en_chg` relation governs. The corrected charged partition is
24 + 8 + 4 = 36, with free A080, A082 and A084. No consumer may derive charges
from the incomplete inventory. The original proposal is retained verbatim.

The 232 added lanes are logical supplied columns. They are not measured circuit
rows, gates, degree, k, proof size, DUST or fees. The local per-limb requirement
`p > 2^20` is conditional. The actual native field and ISA behavior are unknown
here. It is not a sufficiency statement for a composed step circuit.

G4-FIELD, G4-ISA, G4-FETCH, G4-BYTE and G4-APPEND remain requirements. IO1–IO8
remain open, including the exact P3 errata reconciliation, A102 fuel-chain
equivalence, full P4 spine, P5 dispatch and L04 layout/statement binding. The
reviewers did not receive the full P3 errata texts. This publication closes
none of those obligations. Refer to the prior P3 source adoption and its full
reviews under ../arena-policy-source-20261016/.

All new arithmetic/fuel K, Quint and Clean correspondence checks are UNRUN.
No new Rust compilation, SDK check, proof or stagenet test was performed.
Existing append Quint QA concerns its separate finite append projection.
C4 still requires admitted A102 compilation before dependent circuit edits.
Compile failure requires registration revert. Native authority remains zero.
All historical limits, counters and failures and full roadmap exits remain.

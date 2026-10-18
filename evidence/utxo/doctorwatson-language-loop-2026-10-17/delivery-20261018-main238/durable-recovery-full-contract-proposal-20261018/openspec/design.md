# Recommended decision

Use explicit complete record A as the public lifecycle boundary; B may own the same state internally but must persist/reopen the same complete wire. See ../DESIGN.md for the complete recommendation, grammar, field/control/history/work semantics and reasoned alternatives, ../PUBLIC-LIFECYCLE.md for actual library/reader/writer/CLI/test links, and ../NORMAL-CASES.md for complete financial trajectories and expectation custody. No narrow JSON viewer or library-only first-delivery alternative is adopted.

# Mandatory unresolved owner interfaces

../OWNER-RULES.json enumerates the exact missing identity/admission/ceiling/failure/IO outputs. Their absence is retained explicitly and cannot be hidden by a prototype, fake token, generic error or completion claim. They may force a revised exact candidate before adoption/implementation. This design does not author those rules.

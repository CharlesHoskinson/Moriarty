Author revision 4 (claude-opus-5-5, tools off). It repairs immutable revision 3 against the exact S32 requiredFix for each of N01-N11. Nothing was executed, imported, hashed or compiled, and no test or implementation was authored.

Clause text changed in two places:
- CP04 (N06): the checks for deferred fields are now named exactly. STATEMENT and PROOF use length-first hex checks; DIGEST uses exact-string equality. The scan makes no input-proportional copy, never calls str.encode, and may use an isascii fast path.
- CP06 (N02): subset and byte-equality evidence is now attributed to T02 and T17 only. T16 shows strictness only.

The literal text of CP01-CP03, CP05, CP07 and CP08 is unchanged.

Test-policy repairs:
- T16 now lists exactly which cases v1 admits and r2 refuses (N01).
- There are 18 methods. T05a/b/c are subTests of T05, and any run count other than 18 is NOT RED/STOP (N03).
- T07's list-valued cases cover only the six nested objects; a list-valued export itself is ARG (N04).
- The T04 and T14 cells are fixed (N05). Section 3 wording is check-level only (N07). S31 is cited as scoped history (N08).
- hashlib token expectations are added and labelled non-oracle (N09). K2 is recorded as 3926 B, root-observed (N10). The T06 extra-key case is marked redundant (N11).

Stage expectations, labels, Rust identities, v1 history, the 20 families and the permissionless model are unchanged.

Schema conflict: `files` allows 0 items (only SECURITY-REVIEW.md/ERRATA paths), and `positions` also allows 0 items. So the full verbatim file bodies and the eight author-FOR literal clauses are in crosschecks: FILE-C2-PROFILE-R2-CONTRACT.md, FILE-C2-PROFILE-R2-API.json and POSITION-CP01..CP08. Root should extract them, compute pins and check that each clause appears exactly once. Every readiness flag is false until a fresh non-author vote.

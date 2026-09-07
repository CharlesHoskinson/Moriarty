# MC01 corrected source-profile evidence

Status: source-only correction candidate; not frozen, independently approved, or implemented.

This directory records the bounded local checks for the corrected
`moriarty-bounded-atomic/1` specification. `canonical-vectors.json` contains
canonical JSON/action hash vectors, exact source artifact digests, non-unit
quantum acceptance/rejection, both loan obligation/status transitions, and the
swap/closure trace. `verify_profile.py` checks them with the Python standard
library and prints a compact pass record. `verification.json` preserves the
observed result and limitations.

The checker lexes every byte of both complete examples, checks delimiter/source
structure, keywords, declared references, the fixed effect schemas, exact policy
target coverage (loan 14; swap 12), rounding-local roots, selected dimensional
derivations, checked sample arithmetic, obligation adverse cases, and quantum
conversion. Manual comparison against `grammar.ebnf` also found both examples
well-formed under the source productions. This is a review aid, not a reusable
parser or frontend.

The target crosswalk remained byte-for-byte unchanged at SHA-256
`0cc5bddf6a10ea9a2df7b7b5686c148314eb8f9b48f97c54ba02ab32154de884`.
The checker observed exactly 32 ACTUS and 72 DeFi rows, 104 distinct IDs, and all
rows still mandatory for MC07. That is inventory preservation, not conformance.

No compiler, evaluator, native proof, PCD, cryptographic wrapper, oracle, wallet,
network, ledger settlement, ACTUS fixture run, or DeFi conformance run was used.
Independent Fable and GPT-6 review remains required before a root freeze decision.

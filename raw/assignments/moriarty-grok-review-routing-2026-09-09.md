# Reviewer routing update — September 9, 2026

User instruction: “Switch the review of opus to grok 4.6”.

For pending and future Moriarty reviews, Grok 4.6 replaces Claude Opus. GPT-6 remains the implementer, and a fresh independent GPT-6 Astra reviewer remains required. Use the installed Grok CLI with explicit `--model grok-4.6` and high reasoning effort, consistent with the user's prior Grok preference. Record the actual returned model identity and terminal status; a CLI alias or process exit code alone is not review evidence.

This supersedes the Opus reviewer selection in `moriarty-opus-review-routing-2026-09-09.md`. Do not wait for Opus availability for pending result reviews. Do not rewrite prior Opus/Fable/Grok/GPT review records or treat routing changes as an approval. Bind each new verdict to the exact reviewed candidate and scope. Fresh GPT-6 and Grok reviews are both required; unavailable or failed reviewers do not approve work or silently trigger substitution.

Retain all MC/RP gates, two agreeing substantive votes for consequential decisions, cumulative resource charges, actual tests/proofs and Midnight Preview evidence requirements. User authority changes the reviewer, not the acceptance predicate.

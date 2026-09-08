# Ledger complete-effects diagnostic

The current finalized-transaction comparison accepts six changes that its contract must reject: a changed fallible recipient or amount, a changed balancing recipient, an additional intent, a missing balancing intent and a missing expiration. The unchanged control passes. Root and a fresh independent GPT-6 reviewer reproduced one pass and six failures.

These controlled objects exercise the existing public helper contract. They have no signed inputs and do not pass upstream signature validation. This is evidence of missing comparisons, not evidence of a reachable signed or proven transaction exploit. The actual SDK merge representation still needs a fixture built with the pinned SDK.

The immutable test imports the candidate worktree recorded in its verification receipt. Its source hash matches candidate01 in [the blocked source result](../ledger-source-result-01/). All original acceptance requirements remain open. This diagnostic invokes no wallet, compiler, prover or network transport and produces no transaction ID.

Use these failures to guide the approved source repair. Retain the public driver, receipt-decoding, provider wiring, gross debit, deadline and builder findings in the existing result review. No new design packet is required for those corrections.

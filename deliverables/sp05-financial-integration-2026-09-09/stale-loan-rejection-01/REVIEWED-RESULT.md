# Reviewed stale-loan rejection source

GPT-6 Astra and Grok 4.6 both approved candidate02 for source and controlled
checks. This is preparation for SP05.2's live negative test; no adverse
transaction, wallet operation, installed proof or node service ran.

The code prepares a fresh accrue call against the retained initialized loan
state, using the current queried Zswap state and ledger parameters. It uses
the existing proof, balance and one-use submission providers. Public finalized
candidate bytes and classified rejection evidence must be retained before a
strictly later finalized state is compared, including zero balance entries.

The original candidate missed the wallet's extra submission-service error
wrapper. GPT-6 reproduced that failure; the original Grok pass is retained.
The correction recognizes exactly the installed wrapper and matching native
transaction bytes. Unknown errors remain unknown. Twelve tests passed,
including actual default submission-service and node-client code behind a
controlled transport. GPT-6 also ran nine independent wrapper controls.

The positive native-fee test rejects a debit one unit above its allowance
before wallet access and reaches the inert wallet boundary at equality. Its
native container is synthetic and its transplanted public proof is not
verified or submitted.

Operational integration, original-store preservation, fresh bounded resource
admission, actual rejection/financial nonmutation, paid-fee observation and
Midnight Preview testing remain open. RPC1010 is retained as an unknown
outcome in this candidate. No stale-revision cause or included rollback is
inferred. All historical reservations and service/review charges remain.

Reproduction limit: the default-service test imports this reviewed worktree's
helper by absolute path. The independent audits checked those exact bytes.
The forthcoming integration change must bind that test to its local helper.

See `CORRECTION-02.md`, `source-review-gpt6-02.json`,
`source-review-grok-02.json`, and `reviewed-result.json` for exact bindings.
Original source and audit records remain unchanged.

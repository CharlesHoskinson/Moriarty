# Current signed compiler boundary for FeePool

Repository observation at main dc31345850b5800687b16b26bfa20f1d66d94211, source tree f6248b06c486b4bc008a2a9e40cf206fc1a71424. No native or compiler execution was required for this observation.

The frozen FeePool interface preserves funded intake from `funding-source` into `fee-pool` and subsequent claims out of `fee-pool`. These are distinct debit owners and must remain distinct. Current SignedStaticBindings::from_source in crates/mori-ledger/src/signed/mod.rs:217 requires bindings.payer == policy.owner, :256 requires every bound duty debtor == policy.owner, and :263-295 scans every source action and rejects any transfer/fee from another owner (sole-payer) or liability with another debtor (sole-debtor).

Therefore no one FixedSignerPolicy can positively admit the entire current FeePool program. Unsigned all-actions static export/reopen remains a valid source-static consumer. Consuming signed-v2 compilation must be an explicit negative control until general scoped multi-owner authorization is implemented. Do not rewrite intake as a self-transfer, omit funding, fake policy ownership, remove source-wide admission or silently fall back to unsigned to claim signed support.

Further, signed/nonce.rs:16-31 binds the shared authority-domain sentinel to the whole fixed signer policy. Merely swapping per-action policies would fail continuity against that domain, and is not a complete multi-owner design. Per-action authority, debtor/recipient consent and shared durable replay need an explicit versioned relation/storage design and independent current native qualification.

This supplements the original frozen design/interface; it does not rewrite them or add authority. Local Core owner-intent/2 can still authenticate distinct caller-trusted owners per action through its existing interpreter, without claiming native account authority or nonce consumption. Full M11, M7/M8/M13/M17, Capacity Exchange and native acceptance remain open. Native authority remains zero.

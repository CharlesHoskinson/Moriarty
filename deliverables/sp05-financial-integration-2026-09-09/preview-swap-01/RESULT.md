# Moriarty swap execution on Midnight Preview

The bounded Moriarty constant-product swap completed deployment, initialization,
trade and close on Midnight Preview on September 10, 2026. Independent GPT-6
Astra and Grok 4.6 audits approve this specific financial result.

The trader exchanged **10,000 A for 19,743 B**. The native transaction spent the
100,000-A initialization output and returned 90,000 A change. Closing paid
**1,010,000 A and 1,980,257 B** to the provider and left zero contract reserves.
Both assets are native minted Preview test tokens. The epoch is closed at
revision 2 with remaining lifetime 6, as specified by the swap source.

| Stage | Preview block | Observed transaction ID |
| --- | ---: | --- |
| Deploy | 807510 | `00822f8fbfb42b8c670da99cc6343478e5f38ee52f1018e6664014dfe51dd272a6` |
| Initialize | 807515 | `003db2f55f44b828b9dbd17e12c3d5f81230bbde4174b215654ca74408e22839a9` |
| Swap | 807526 | `0055d11b5259c654aae885de96b95870e6b63f712c3f72ecd60b1ef6dd14ff488a` |
| Close | 807530 | `00f39c96c5a9c8bafb42bef579a854c9db7a8d5ea3c347bd3859ce507e1c8c8560` |

Contract: `87affdd94943d844667cd6978d223f112c1d3e7c17b42f092323d4078e792b86`.
Each transaction has two SDK identifiers; all eight were posted as confirmed
and remain in the public native payload sidecars and audit records.

[The scoped approval](scoped-approval-01.json) binds both independent audits:
[GPT-6](actual-result-review-gpt6-01.json) and
[Grok](actual-result-review-grok-01.json).
[The actual candidate](actual-result-candidate-01.json) binds the original
native bytes, stage observations, limits and shutdown records. GPT-6 separately
verified native signatures and the initialization-output origin, per-asset
conservation, all historical state fields and balances, deployed verifier keys,
plan normalization and resource accounting. Its
[verification manifest](gpt6-actual-verification-manifest-01.json) retains the
checks and 15 bounded public RPC requests with exact responses. Map iteration
order and swap lifetime assumptions corrected in the reviewer harness are
preserved with their dispositions; neither was a producer defect.

## Scope and remaining gates

All four financial comparisons passed. The raw integration remains `FAILED`,
the driver remains `INCOMPLETE`, and the launcher exited 1: its in-process
provider cannot attest external process containment. A separate actual
observation established process/cgroup absence and stopped containers before
timer cancellation. Original statuses and false acceptance flags stay intact.

The single attempt reserved four submissions and 1,200,000,000,000,004 SPECK of
native DUST debit. The whole 100,000-A input consumes the gross asset allowance;
90,000-A change does not refund it. Indexer fee units remain unresolved relative
to native debit; reservations are not asserted to be economic fees paid. Both
loan and swap wallet backups remain private and the pending marker is absent.

Together with the [reviewed Preview loan](../preview-loan-01/RESULT.md), this
supplies real financial examples for both fixed language programs. **Preview
remains a hard release gate.** Complete SP05 reconciliation, admission-bound
fee comparison, mandatory PCD, SP09 verification, SP11 coverage and SP12 release
remain open. Trusted node/indexer observations do not establish an authenticated
state proof or mandatory proof-carrying acceptance.

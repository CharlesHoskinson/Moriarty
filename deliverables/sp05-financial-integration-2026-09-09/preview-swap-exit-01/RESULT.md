# Preview swap: reviewed financial command result

GPT-6 Astra and Grok 4.6 both returned **PASS_SCOPED** for the actual four-stage Preview swap, the retained command exit zero and separate outer containment. This closes this invocation's command-result gap. Full SP05 and mandatory PCD remain open; the previous loan's missing raw exit remains unresolved.

The run used contract `a1bc37f889ca6a2cb17f6425a43e236a46446f8bdfd5b0dae9f5f73734682c96`:

| Stage | Finalized Preview height | Financial result |
| --- | ---: | --- |
| Deploy | 808320 | Empty initial state |
| Initialize | 808325 | Pool receives 1,000,000 A and 2,000,000 B; trader receives 100,000 A |
| Swap | 808337 | Trader spends 10,000 A for 19,743 B, retaining 90,000 A change |
| Close | 808341 | Provider receives 1,010,000 A and 1,980,257 B; both native pool reserves end at zero |

Independent verification checked native transaction bytes, the spending signature and UTXO linkage, complete historical states, balances and all three verifier keys against the retained build. Fifteen fresh public RPC reads observed canonical finality through height 808391. These trusted RPC observations are not authenticated state proofs or a new recursive proof.

The launcher retained `ExecMainCode=1`, `ExecMainStatus=0`, `MainPID=0` and `Result=success` while loaded and active/exited. Its InvocationID matched startup. That observation was saved before explicit stop, which returned zero. The exact final stdout was `FINANCIAL_COMPLETE` with all four booleans false. Root then observed the startup cgroup absent, the proof server exited, and the local node/indexer still stopped while the forced timer remained active. Timer cancellation followed and returned zero. Main exit, financial result, containment and acceptance remain distinct records.

Four native DUST reservations totaled **1,200,000,000,000,004 SPECK**, within the admitted 2,000,000,000,000,000 cap. Gross wallet input was 100,000 A. These native debits are not a verified economic fee total; the indexer's fee encoding remains unresolved. Known cumulative reservations are 27 and 8,100,000,000,000,027 SPECK. Listed admitted ceilings total 12,000,000,000,000,010 SPECK; all older unquantified charges remain consumed.

Both loan and swap have consumed their two public attempts. No retry, additional attempt, wallet reset or rollback follows from this approval. The swap finishes at revision 2 with six episode steps remaining and its closed-epoch marker set; this does not demonstrate general financial coverage.

Evidence: [result candidate](actual-result-candidate-01.json), [GPT-6 audit](actual-result-review-gpt6-01.json), [Grok audit](actual-result-review-grok-01.json), [verification manifest](gpt6-actual-verification-01/verification-manifest.json), [main exit](terminal-observation-01.json), [containment](outer-containment-observation-01.json), [scoped approval](scoped-result-approval-01.json). All eight SDK identifiers and native payloads are retained under [actual-run](actual-run/); each identifier was posted with its confirmed status.

**Midnight remains the hard release gate.** This result does not certify mandatory proof/history acceptance, private handoff, full language coverage or release. All original raw acceptance flags stay false.

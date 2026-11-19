# Signed order acceptance

An owner authorizes one O1 deployment on one named Midnight ledger. The full identifier consists of the network, contract address and 32-byte order ID. Two deployments may use the same order ID. A grant for one deployment cannot settle the other.

## Authority and funding

The owner has a native unshielded wallet key and a separate Jubjub application key. The wallet signs an enrollment containing the deployment address, network, all three verifier-key hashes, complete order terms and coordinates of the application key. The adapter derives the account address from the wallet verification key, checks it against the accepted account field and verifies the enrollment signature. The application signature covers the contract address, action, revision, key, network, order ID, program, both assets, all three wallet addresses, clock origin, expiry, exact input, minimum output and zero fee.

The Compact constructor stores the key and account supplied at deployment. The enrollment check takes place in the adapter. A funder must verify the deployment, installation and wallet enrollment before transferring assets to this contract. The public funding entry point accepts a contribution of the stipulated A amount from any sender. A contribution gives the sender no cancellation or payout right. In the funded evidence, the A inputs belong to the enrolled owner, the executor signs its B inputs, and a separate DUST key pays network fees. All fixture keys derive from public test constants.

An order pays 100 A to the named executor and at least 200 B to the named recipient. The executor's transaction supplies the actual B and pays exactly that amount to the recipient. The current circuit checks balance sufficiency and the lowered kernel's typed transfer operands. It records the new kernel state, remaining allowance and revision in the same transcript as the transfer claims. The wallet enrollment permits an improvement in B output without requiring a second application signature.

## State, time and custody

The supported local reader gets ContractState from the accepted LedgerState at the pinned address. It checks the program, network domain and installed verifier keys. A receipt contains the complete pre-state, proved transaction, controlled block timestamp and post-state. The proof-verifying Rust validator applies each transaction with all strictness flags enabled. Readback of serialized wallet UTXOs and contract balances accounts for A and B. DUST spend events identify network charges separately. Preparation refusals have no transaction or charge. A previously proved call can still fail at application when its head or time condition changes.

The signed epoch maps the source's order-relative seconds to the accepted ledger clock. A fill must be included before epoch plus 1400. The test ledger configures zero time uncertainty. Deployed networks supply their own block time and uncertainty. The local runs fork the funded test ledger for independent exact and improved orders. They establish no external inclusion or finality.

An owner-signed cancellation spends only the residual A recorded by the order. The adapter calls the time-independent cancellation branch with a constant source observation inside the kernel horizon. This implements the custody policy permitting refund after the original execution horizon. A fill and refund compete for the same revision. Extra unsolicited deposits leave the recorded entitlement intact and can remain locked beyond the terminal transition.

## Verifiers and upgrades

The replay pins the Moriarty source, lowered kernel, Compact compiler, actual ZKIRv3 entries and the tagged verifier installed for each operation. The verifier enforces the compiled relation. Correspondence between the source and that relation still depends on the compiler and runtime. Key generation and proof verification depend on the pinned Midnight implementation and its public proving parameters. The fee payer and proof provider can withhold service. They cannot create the owner's wallet enrollment or application signature.

The local deployment gives the owner wallet a one-member maintenance committee with threshold one. A maintenance action can replace a verifier or add another entry point. Custody depends on that authority for the lifetime of any outstanding balance. Only the pinned, original installation is supported by this adapter. A verifier or domain migration requires the owner's evaluation of the new program and a new enrollment. The proof server used for this test only receives synthetic inputs. An independent live deployment and operator have separate trust assumptions.

Named feed controller, clock and outage assumptions are documented in [FEED-TRUST.md](FEED-TRUST.md).

## Partial fills and refunds

The separate recovery policy funds 100 A and permits one 40 A fill for at least 80 B. Each deployment stores its own remaining principal. Only the signed owner can cancel at the current revision. Cancellation returns that principal to the enrolled account, preserves the completed trade and closes the order. Expiry and the execution horizon restrict trading. Owner withdrawal remains available afterward. The withdrawal circuit uses the cancellation transition with a fixed in-range clock because that transition has no time-dependent terms.

Additional A received by the surplus entry point stays in that deployment without creating a claim or reopening an order. Contributors should expect no refund of surplus. The order has no sweep operation. A verifier change requires owners and clients to decide whether to enroll a new deployment and withdraw outstanding claims under the existing keys. These receipts apply only to the recorded verifier identities and state.

The in-memory harness seals each transaction block with the native ledger's post-block update. That update records the DUST commitment and generation roots used by the next fee proof. The strict replay checks both the immediate post-transaction digest and the finalized block digest. Each subsequent branch names its finalized predecessor.

## Staged exchange and continuation storage

O9 records the result of the first trade at the deployed custody address. Its pending record commits to the predecessor, order terms, program and revision. It binds one terminal action to the order's actual B proceeds, custody address and owner-selected event. The event becomes true when the enrolled owner signs its activation. The executor supplies the B and C required by each individual trade. Each owner client reads the accepted ledger and checks the enrolled verifier keys before issuing a new grant. Clients can disconnect between actions because the custody contract stores the pending record and its consumed state.

The first trade must settle before the signed epoch plus 1200 seconds. The second trade must settle before epoch plus 1400 seconds. From that deadline the owner may recover all captured B. For refund, the adapter projects the accepted native clock to the deadline within the source horizon. The refund branch uses only that lower bound. The receiver's original 100 A transfer remains settled in either branch. Separate trial branches share the accepted first trade and activation. Each trial retains its own finalized transaction history. These local trials assume the enrolled keys remain installed, the owner retains its keys, and the client can obtain the current accepted ledger.

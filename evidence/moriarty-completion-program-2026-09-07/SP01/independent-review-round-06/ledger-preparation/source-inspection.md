# SP05 local ledger source inspection

Source inspection complete; no result acceptance or submission. System-described reviewer GPT-6; exact serving identifier unavailable.

Docker reports node1.0.0, indexer4.3.3 and proof-server8.1.0 healthy. No service was probed or changed. All frozen custody owned-file hashes match. Source SHA256 manifest and full dependency versions are in source-inspection.json.

## Existing APIs and exact source references

### Load compiled deployment and providers

CompiledContract.make(name, Generated.Contract).pipe(CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(zkConfigPath)); NodeZkConfigProvider; httpClientProofProvider; indexerPublicDataProvider; levelPrivateStateProvider. deployContract(providers,{compiledContract,args,privateStateId,initialPrivateState}) returns deployTxData.public.contractAddress. Treat current scripts as source examples: deploy.ts calls getOrCreateWallet at module scope and main() automatically; do not import it for helpers.

Sources: [deploy.ts:90](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/deploy.ts:90), [deploy.ts:121](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/deploy.ts:121), [deploy.ts:322](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/deploy.ts:322).

### Call and read finalized state

findDeployedContract and submitCallTx are real pinned APIs. Calls supply compiledContract, contractAddress, privateStateId, circuitId and args. Read queryContractState(address,{blockHash}) or blockHeight configuration; decode Generated.ledger(contractState.data). queryUnshieldedBalances(address,block config) retrieves contract holdings. Bind readbacks to receipt block, not latest unrelated state.

Sources: [cli.ts:160](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/cli.ts:160), [cli.ts:185](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/cli.ts:185), [index.d.mts:1151](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-contracts/dist/index.d.mts:1151), [index.d.mts:925](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts:925).

### Unshielded balancing MUST sign before binding

Available chain: balanceUnboundTransaction(tx,{shieldedSecretKeys,dustSecretKey},{ttl}) -> signRecipe(recipe,payload=>unshieldedKeystore.signData(payload)) -> finalizeRecipe(signedRecipe) -> submitTransaction(finalized). Source finalizeRecipe binds/merges and records pending tx; it does not sign inputs. Existing hello-world provider omits signRecipe and wrongly describes it as removed. This is a concrete incompatibility when receiveUnshielded balancing adds token inputs. DUST registration already has a dedicated signing callback: do not blindly double-sign that separate path.

Sources: [index.d.ts:261](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.d.ts:261), [index.js:357](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.js:357), [index.js:414](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.js:414), [index.js:439](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.js:439), [deploy.ts:108](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/deploy.ts:108).

### Wallet setup and funding APIs

createWallet accepts selected network/config/seed and returns WalletContext with wallet, unshieldedKeystore and secret-key handles. Unshielded child starts/restores from public keystore identity. waitForSyncedState, state(), registerNightUtxosForDustGeneration, finalizeRecipe, submitTransaction exist. transferTransaction(CombinedTokenTransfer[],keys,{ttl,payFees?}) returns UnprovenTransactionRecipe for separately admitted funding transfers. Existing NIGHT balance uses unshieldedToken().raw; custom custody colors are different token types. No actual wallet state, seed, address, balance, funding or recovery was read or verified.

Sources: [wallet.ts:64](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/wallet.ts:64), [wallet.ts:99](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/wallet.ts:99), [wallet.ts:134](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/wallet.ts:134), [deploy.ts:239](/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/src/deploy.ts:239), [index.d.ts:287](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.d.ts:287).

### Mint/funding sources already in custody candidate

Loan initialize mints20,000,000,000 custom USD units to sealed borrower. Swap initialize mints1,000,000 A and2,000,000 B to contract and100,000 A to sealed trader. rawTokenType(domainSeparator,contractAddress) exists to independently derive each color. Initialization is test issuance, not evidence of external loan funding or provider liquidity deposit. Need actual on-ledger initialization receipts and wallet synchronization before funded settle/swap.

Sources: [loan.compact:54](/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/candidate-01/experiments/moriarty-midnight-financial/custody/loan.compact:54), [swap.compact:57](/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/candidate-01/experiments/moriarty-midnight-financial/custody/swap.compact:57), [ledger-v8.d.ts:522](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:522).

### Payer ownership and signed offers

Each Intent exposes guaranteedUnshieldedOffer/fallibleUnshieldedOffer and signatureData(segmentId). Offers expose inputs/outputs/signatures. UtxoSpend.owner is SignatureVerifyingKey, whereas Utxo/UtxoOutput.owner is UserAddress; convert with addressFromKey and compare exact sealed payer address. Verify signatures using verifySignature(vk,intent.signatureData(segment),signature), retaining original input order and (intentHash,outputNo,type,value). TransactionOps fills missing signature slots across all offer inputs with the supplied signature; do not assume a multi-payer signing combiner. Start with one selected owner per funded call; make third-party funding an explicit negative attribution control.

Sources: [ledger-v8.d.ts:435](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:435), [ledger-v8.d.ts:1769](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:1769), [ledger-v8.d.ts:1813](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:1813), [ledger-v8.d.ts:1818](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:1818), [ledger-v8.d.ts:2038](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:2038), [ledger-v8.d.ts:2076](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:2076), [TransactionOps.js:69](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-unshielded-wallet/dist/v1/TransactionOps.js:69).

### Receipt decoder and full outputs

FinalizedTxData includes public tx, status, txId, identifiers, txHash, blockHash/height/timestamp, protocolVersion, fees, segmentStatusMap and unshielded.created/spent. Public tx offers retain input references and signatures; the SDK summary omits outputNo and labels owner ContractAddress, so it is insufficient alone for complete UTXO identity/payer attribution. Serialize only the public finalized transaction via tx.serialize(), plus an explicitly selected public receipt schema. Never stringify FinalizedCallTxData wholesale: its private field contains proof inputs/private transcript/private state. Decode every kernelState field and lastAccrue/lastSettle/lastSwap/lastClose record using exact generated decoder and pinned metadata; compare every token output, change, fee and residual duty against independent fixtures.

Sources: [index.d.mts:186](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts:186), [index.d.mts:128](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts:128), [index.d.mts:582](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-contracts/dist/index.d.mts:582), [ledger-v8.d.ts:2420](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts:2420), [loan.compact:23](/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/candidate-01/experiments/moriarty-midnight-financial/custody/loan.compact:23), [swap.compact:25](/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/candidate-01/experiments/moriarty-midnight-financial/custody/swap.compact:25).

### Failure/finality evidence

Statuses are FailEntirely, FailFallible and SucceedEntirely. FailFallible preserves guaranteed effects while discarding fallible effects, so do not claim all ledger/DUST effects roll back. Capture CallTxFailedError.finalizedTxData when present, compare full financial state and token holdings to appropriate pre-call block, and separately record fees/change/guaranteed outputs. Existing verification source checks indexed SUCCESS, chain_getBlockHash at receipt height and chain_getFinalizedHead/header coverage. It proves only existing hello-world cases; reuse the algorithm with exact financial receipt binding and bounded waits.

Sources: [index.d.mts:105](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts:105), [index.d.mts:925](/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-contracts/dist/index.d.mts:925), [verify-local.mjs:39](/home/charl/Moriarty/experiments/moriarty-midnight-network/verify-local.mjs:39), [verify-local.mjs:74](/home/charl/Moriarty/experiments/moriarty-midnight-network/verify-local.mjs:74), [preview-verify.mjs:30](/home/charl/Moriarty/experiments/moriarty-midnight-network/preview-verify.mjs:30).

## Blockers and packet questions

- L1 (requires new proven build packet): Custody build.mjs:18 explicitly requires --skip-zk and :101 compiles with it. Current artifacts supply runtime/ZKIR, not deployable proving/verifying assets. Add separately bounded full compile/proof-service phase; do not edit frozen build or infer keys exist.
- L2 (source-confirmed adapter gap): Existing hello-world WalletProvider balanceTx omits signRecipe. New provider must sign selected owner inputs after balancing and before bind; inspect final offers and test signature count/owner correctness.
- L3 (requires independent result dependencies): Custody candidate is inspected, not accepted by this task. Consume its terminal independent result review before deployment. Current packet says full financial comparison utility is blocked; do not import it or silently reuse its output until corrected and independently accepted.
- L4 (identity/funding not inspected): Choose existing funded local payer plus distinct recoverable sealed recipients and isolated private state store. Identity/capability handles must be supplied by separately admitted recovery workflow. No wallet seed/key/state read here; DUST/NIGHT/custom balances and destination ownership remain unverified.
- L5 (actual ledger evidence absent): Need real initialize/accrue/settle and initialize/swap/close receipts, input ownership, all outputs/change, contract balances, finalized block and full ledger decoder readback. Runtime query contexts with synthetic entry balances cannot supply this.
- L6 (bounded operational readiness unverified): Docker ps health is only container metadata. Endpoint responsiveness, actual protocol compatibility, proof key readiness and cost/duration are not probed. Existing proof readiness returns true on almost any HTTP response and several sync/indexer waits are unbounded; new harness must impose explicit deadlines and no implicit retries.
- L7 (receipt semantics questions): Choose and freeze normalization of user addresses versus verifying keys, guaranteed/fallible sections, UTXO references and fee/DUST records. Treat public tx offers as authoritative signed material and indexer summary as cross-check; prove same ledger network/deployment instead of trusting constructor networkTag.

## Minimal proposed ownership

- `experiments/moriarty-midnight-financial/ledger/README.md`
- `experiments/moriarty-midnight-financial/ledger/build-proven.mjs`
- `experiments/moriarty-midnight-financial/ledger/providers.mjs`
- `experiments/moriarty-midnight-financial/ledger/decode-receipt.mjs`
- `experiments/moriarty-midnight-financial/ledger/run-local.mjs`
- `experiments/moriarty-midnight-financial/ledger/receipt.test.mjs`

No shared package or existing runtime/network/custody/differential edit is needed in this proposed scope.

- Proposed build-proven.mjs: Explicit frozen custody/source/SDK manifest plus untracked build directory -> hash manifest of full compiler outputs and proof assets. Separate full-compile resource admission. No installs; same pinned generated source/kernel/arithmetic.
- Proposed providers.mjs: Explicit WalletContext handle + network configuration + proven asset directory + isolated private-state configuration -> MidnightProviders. Balance/sign/finalize chain above; no getOrCreateWallet side effects or secrets in returned receipt.
- Proposed decode-receipt.mjs: FinalizedTxData public fields + block-pinned ContractState and contract token balances + pinned role/token metadata -> closed public financial receipt with all signed input refs/owners, outputs/change, DUST/fees, full kernel results/dues, status/finality/network/deployment bindings. Reject incomplete/unknown fields in this receipt schema.
- Proposed run-local.mjs: Explicit case loan|swap, identity handles, frozen input manifest, output directory, time/attempt/spend limits and local network selection -> deploy/init/financial-call receipts and independent comparison. One deployment per case, one initialization, two ordered financial calls; negative attempts separately reserved, no implicit retry. Never select Preview in this Docker-only packet.
- Proposed receipt.test.mjs: Offline receipt mutation tests: wrong key-to-address attribution, duplicate/out-of-order UTXO refs, signature mismatch, wrong color/payee/change/fee, missing output, changed residual due/lifetime, failed/fallible status, unfinalized block. Existing generated runtime tests remain separate.

## Pinned versions

{"compact-runtime": "0.16.0", "ledger-v8": "8.1.0", "midnight-js-contracts": "4.1.1", "midnight-js-http-client-proof-provider": "4.1.1", "midnight-js-indexer-public-data-provider": "4.1.1", "midnight-js-level-private-state-provider": "4.1.1", "midnight-js-node-zk-config-provider": "4.1.1", "midnight-js-protocol": "4.1.1", "midnight-js-types": "4.1.1", "wallet-sdk": "1.2.0", "wallet-sdk-address-format": "3.1.2", "wallet-sdk-dust-wallet": "4.2.0", "wallet-sdk-facade": "4.1.0", "wallet-sdk-shielded": "3.0.2", "wallet-sdk-unshielded-wallet": "3.1.0"}

Actual local Docker financial settlement first; separately admitted Preview campaign afterward with existing identities and live attempt/spend reservation. I2 financial transactions remain uncertified until mandatory proof-bearing acceptance qualification. No full SP05 or twelve-sprint acceptance follows from this inspection.

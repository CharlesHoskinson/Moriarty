# Pinned implementation source excerpts

These are retained primary SDK/accepted source, not prior reviewer advice. Review the proposed source-only implementation packet, not a future ledger result. An empty Git diff identifies the packet baseline only; it does not prove all source files are intact. Parent verified every sources.json hash independently.


## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-facade/dist/index.js
SHA256 bb63f609499432f54b7db424d124d7dc80ad7a466af3c39cf7033f52b33f7562
350:         }
351:         return {
352:             type: 'FINALIZED_TRANSACTION',
353:             originalTransaction: tx,
354:             balancingTransaction: balancingTx,
355:         };
356:     }
357:     async balanceUnboundTransaction(tx, secretKeys, options) {
358:         const { shieldedSecretKeys, dustSecretKey } = secretKeys;
359:         const { ttl, tokenKindsToBalance = 'all' } = options;
360:         const { shouldBalanceDust, shouldBalanceShielded, shouldBalanceUnshielded } = TokenKindsToBalance.toFlags(tokenKindsToBalance);
361:         // Step 1: Run unshielded and shielded balancing
362:         const shieldedBalancingTx = shouldBalanceShielded
363:             ? await this.shielded.balanceTransaction(shieldedSecretKeys, tx)
364:             : undefined;
365:         // For unbound transactions, unshielded balancing happens in place not with a balancing transaction
366:         const balancedUnshieldedTx = shouldBalanceUnshielded
367:             ? await this.unshielded.balanceUnboundTransaction(tx)
368:             : undefined;
369:         // Step 2: Unbound unshielded tx are balanced in place, use it as base tx if present
370:         const baseTx = balancedUnshieldedTx ?? tx;
371:         // Step 3: Conditionally add dust/fee balancing
372:         const feeBalancingTransaction = shouldBalanceDust
373:             ? await this.dust.balanceTransactions(dustSecretKey, shieldedBalancingTx ? [baseTx, shieldedBalancingTx] : [baseTx], ttl)
374:             : undefined;
375:         // Step 4: Create the final balancing transaction
376:         const balancingTransaction = this.mergeUnprovenTransactions(shieldedBalancingTx, feeBalancingTransaction);
377:         // if there is no balancingTransaction and there was no unshielded tx balancing (in place) throw an error.
378:         if (!balancingTransaction && !balancedUnshieldedTx) {
379:             throw new Error('No balancing transaction was created. Please check your transaction.');
380:         }
381:         return {
382:             type: 'UNBOUND_TRANSACTION',
383:             baseTransaction: baseTx,
384:             balancingTransaction: balancingTransaction ?? undefined,
385:         };
386:     }
387:     async balanceUnprovenTransaction(tx, secretKeys, options) {
388:         const { shieldedSecretKeys, dustSecretKey } = secretKeys;
389:         const { ttl, tokenKindsToBalance = 'all' } = options;
390:         const { shouldBalanceDust, shouldBalanceShielded, shouldBalanceUnshielded } = TokenKindsToBalance.toFlags(tokenKindsToBalance);
391:         // Step 1: Run unshielded and shielded balancing
392:         const shieldedBalancingTx = shouldBalanceShielded
393:             ? await this.shielded.balanceTransaction(shieldedSecretKeys, tx)
394:             : undefined;
395:         // For unproven transactions, unshielded balancing happens in place
396:         const balancedUnshieldedTx = shouldBalanceUnshielded
397:             ? await this.unshielded.balanceUnprovenTransaction(tx)
398:             : undefined;
399:         // Step 2: Use the balanced unshielded tx if present, otherwise use the original tx
400:         const baseTx = balancedUnshieldedTx ?? tx;
401:         // Step 3: Merge shielded balancing into base tx if present
402:         const mergedTx = this.mergeUnprovenTransactions(baseTx, shieldedBalancingTx);
403:         // Step 4: Conditionally add dust/fee balancing
404:         const feeBalancingTx = shouldBalanceDust
405:             ? await this.dust.balanceTransactions(dustSecretKey, [mergedTx], ttl)
406:             : undefined;
407:         // Step 5: Merge fee balancing if present
408:         const balancedTx = this.mergeUnprovenTransactions(mergedTx, feeBalancingTx);
409:         return {
410:             type: 'UNPROVEN_TRANSACTION',
411:             transaction: balancedTx,
412:         };
413:     }
414:     async finalizeRecipe(recipe) {
415:         return Promise.resolve(recipe)
416:             .then(async (recipe) => {
417:             switch (recipe.type) {
418:                 case 'FINALIZED_TRANSACTION': {
419:                     const finalizedBalancing = await this.finalizeTransaction(recipe.balancingTransaction);
420:                     return recipe.originalTransaction.merge(finalizedBalancing);
421:                 }
422:                 case 'UNBOUND_TRANSACTION': {
423:                     const finalizedBalancingTx = recipe.balancingTransaction
424:                         ? await this.finalizeTransaction(recipe.balancingTransaction)
425:                         : undefined;
426:                     const finalizedTransaction = recipe.baseTransaction.bind();
427:                     return finalizedBalancingTx ? finalizedTransaction.merge(finalizedBalancingTx) : finalizedTransaction;
428:                 }
429:                 case 'UNPROVEN_TRANSACTION': {
430:                     return await this.finalizeTransaction(recipe.transaction);
431:                 }
432:             }
433:         })
434:             .then(async (finalizedTx) => {
435:             await this.pendingTransactionsService.addPendingTransaction(finalizedTx);
436:             return finalizedTx;
437:         });
438:     }
439:     async signRecipe(recipe, signSegment) {
440:         switch (recipe.type) {
441:             case 'FINALIZED_TRANSACTION': {
442:                 const signedBalancingTx = await this.signUnprovenTransaction(recipe.balancingTransaction, signSegment);
443:                 const withDustSig = await this.#signDustRegistrationIfPresent(signedBalancingTx, signSegment);
444:                 return {
445:                     type: 'FINALIZED_TRANSACTION',
446:                     originalTransaction: recipe.originalTransaction,
447:                     balancingTransaction: withDustSig,
448:                 };
449:             }
450:             case 'UNBOUND_TRANSACTION': {
451:                 const signedBalancingTx = recipe.balancingTransaction
452:                     ? await this.signUnprovenTransaction(recipe.balancingTransaction, signSegment).then((tx) => this.#signDustRegistrationIfPresent(tx, signSegment))
453:                     : undefined;
454:                 const signedBaseTx = await this.signUnboundTransaction(recipe.baseTransaction, signSegment);
455:                 return {


## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts
SHA256 4a6eaccd531f0bb711dd8a8926d14beedd2ab8345deff378f86068608d67dc36
1769: export type Utxo = {
1770:   /**
1771:    * The amount of tokens this UTXO represents
1772:    */
1773:   value: bigint,
1774:   /**
1775:    * The address owning these tokens.
1776:    */
1777:   owner: UserAddress,
1778:   /**
1779:    * The token type of this UTXO
1780:    */
1781:   type: RawTokenType,
1782:   /**
1783:    * The hash of the intent outputting this UTXO
1784:    */
1785:   intentHash: IntentHash,
1786:   /**
1787:    * The output number of this UTXO in its parent {@link Intent}.
1788:    */
1789:   outputNo: number,
1790: };
1791: 
1792: /**
1793:  * An output appearing in an {@link Intent}.
1794:  */
1795: export type UtxoOutput = {
1796:   /**
1797:    * The amount of tokens this UTXO represents
1798:    */
1799:   value: bigint,
1800:   /**
1801:    * The address owning these tokens.
1802:    */
1803:   owner: UserAddress,
1804:   /**
1805:    * The token type of this UTXO
1806:    */
1807:   type: RawTokenType,
1808: };
1809: 
1810: /**
1811:  * Converts a bare signature public key to its corresponding address.
1812:  */
1813: export function addressFromKey(key: SignatureVerifyingKey): UserAddress;
1814: 
1815: /**
1816:  * An input appearing in an {@link Intent}, or a user's local book-keeping.
1817:  */
1818: export type UtxoSpend = {
1819:   /**
1820:    * The amount of tokens this UTXO represents
1821:    */
1822:   value: bigint,
1823:   /**
1824:    * The signing key owning these tokens.
1825:    */
1826:   owner: SignatureVerifyingKey,
1827:   /**
1828:    * The token type of this UTXO
1829:    */
1830:   type: RawTokenType,
1831:   /**
1832:    * The hash of the intent outputting this UTXO
1833:    */
1834:   intentHash: IntentHash,
1835:   /**
1836:    * The output number of this UTXO in its parent {@link Intent}.
1837:    */
1838:   outputNo: number,
1839: };
1840: 


## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts
SHA256 4a6eaccd531f0bb711dd8a8926d14beedd2ab8345deff378f86068608d67dc36
2030: 
2031:   /**
2032:    * Removes signatures from this intent.
2033:    */
2034:   eraseSignatures(): Intent<SignatureErased, P, B>;
2035: 
2036:   /**
2037:    * The raw data that is signed for unshielded inputs in this intent.
2038:    */
2039:   signatureData(segmentId: number): Uint8Array;
2040: 
2041:   /**
2042:    * The UTXO inputs and outputs in the guaranteed section of this intent.
2043:    * @throws Writing throws if `B` is {@link Binding}, unless the only change
2044:    * is in the signature set.
2045:    */
2046:   guaranteedUnshieldedOffer: UnshieldedOffer<S> | undefined;
2047:   /**
2048:    * The UTXO inputs and outputs in the fallible section of this intent.
2049:    * @throws Writing throws if `B` is {@link Binding}, unless the only change
2050:    * is in the signature set.
2051:    */
2052:   fallibleUnshieldedOffer: UnshieldedOffer<S> | undefined;
2053:   /**
2054:    * The action sequence of this intent.
2055:    * @throws Writing throws if `B` is {@link Binding}.
2056:    */
2057:   actions: ContractAction<P>[];
2058:   /**
2059:    * The DUST interactions made by this intent
2060:    * @throws Writing throws if `B` is {@link Binding}.
2061:    */
2062:   dustActions: DustActions<S, P> | undefined;
2063:   /**
2064:    * The time this intent expires.
2065:    * @throws Writing throws if `B` is {@link Binding}.
2066:    */
2067:   ttl: Date;
2068:   readonly binding: B;
2069: }
2070: 
2071: /**
2072:  * An unshielded offer consists of inputs, outputs, and signatures that
2073:  * authorize the inputs. The data the signatures sign is provided by {@link
2074:  * Intent.signatureData}.
2075:  */
2076: export class UnshieldedOffer<S extends Signaturish> {
2077:   private constructor();
2078: 
2079:   static new(inputs: UtxoSpend[], outputs: UtxoOutput[], signatures: Signature[]): UnshieldedOffer<SignatureEnabled>;
2080: 
2081:   addSignatures(signatures: Signature[]): UnshieldedOffer<S>;
2082: 
2083:   eraseSignatures(): UnshieldedOffer<SignatureErased>;
2084: 
2085:   toString(compact?: boolean): string;
2086: 
2087:   readonly inputs: UtxoSpend[];
2088:   readonly outputs: UtxoOutput[];
2089:   readonly signatures: Signature[];
2090: }


## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts
SHA256 c6b9fb7c40e4cbe00a9054ad99026912df4358a3a450a90ede912a87929efb30
186: interface FinalizedTxData {
187:     /**
188:      * The transaction that was finalized.
189:      */
190:     readonly tx: Transaction<SignatureEnabled, Proof, Binding>;
191:     /**
192:      * The status of a submitted transaction.
193:      */
194:     readonly status: TxStatus;
195:     /**
196:      * One of the transaction ID of the submitted transaction.
197:      */
198:     readonly txId: TransactionId;
199:     /**
200:      * All transaction IDs of the submitted transaction.
201:      */
202:     readonly identifiers: readonly TransactionId[];
203:     /**
204:      * The transaction hash of the transaction in which the original transaction was included.
205:      */
206:     readonly txHash: TransactionHash;
207:     /**
208:      * The block hash of the block in which the transaction was included.
209:      */
210:     readonly blockHash: BlockHash;
211:     /**
212:      * The block height of the block in which the transaction was included.
213:      */
214:     readonly blockHeight: number;
215:     /**
216:      * The timestamp of the block in which the transaction was included.
217:      */
218:     readonly blockTimestamp: number;
219:     /**
220:      * The author of the block in which the transaction was included.
221:      */
222:     readonly blockAuthor: string | null;
223:     /**
224:      * The indexer internal db ID.
225:      */
226:     readonly indexerId: number;
227:     /**
228:      * The protocol version of the transaction.
229:      */
230:     readonly protocolVersion: number;
231:     /**
232:      * The fees associated with the transaction, including both paid and estimated fees.
233:      */
234:     readonly fees: Fees;
235:     /**
236:      * The map that associates segment identifiers (numbers) with their corresponding status {@link SegmentStatus}.
237:      * The segment identifier is represented as a number (key in the map), and the status indicates the success or failure of the transaction update.
238:      */
239:     readonly segmentStatusMap: Map<number, SegmentStatus> | undefined;
240:     /**
241:      * Represents the unshielded outputs, typically used for transactions or operations
242:      * involving data or values that are not encrypted or concealed.
243:      */
244:     readonly unshielded: UnshieldedUtxos;
245: }
246: /**
247:  * Represents an unshielded balance, which is a balance that is not shielded or encrypted.
248:  * This type is used to track the available funds in an account that are visible on the public ledger.
249:  */
250: type UnshieldedBalance = {
251:     /**
252:      * Represents the current number of funds available or held in an account.
253:      */
254:     readonly balance: bigint;
255:     /**
256:      * Represents the type of token in the system.
257:      */
258:     readonly tokenType: RawTokenType;
259: };
260: /**
261:  * Represents a collection of unshielded balances, which are balances that are not shielded or encrypted.


## /home/charl/Moriarty/experiments/moriarty-midnight-financial/src/differential.mjs
SHA256 4a99290b40d89a28b363fa1c58ee43c6922ed73e86d5079e43ec27fdcb2c53fe
600: function validateRecord(record, label, errors) {
601:   if (!isPlain(record)) {
602:     push(errors, 'MALFORMED_STRUCTURE', label, 'financial record object required');
603:     return false;
604:   }
605:   expectKeys(record, label, RECORD_KEYS, errors);
606:   let ready = hasRequired(record, RECORD_KEYS);
607:   if (record.schemaVersion !== SCHEMA_VERSION) {
608:     push(errors, 'VALUE_MISMATCH', `${label}.schemaVersion`, 'unsupported schema');
609:   }
610:   if (record.observationKind !== 'synthetic-local') {
611:     push(errors, 'VALUE_MISMATCH', `${label}.observationKind`, 'observationKind must be synthetic-local');
612:   }
613:   if ('networkAcceptance' in record) {
614:     expectBool(record.networkAcceptance, `${label}.networkAcceptance`, errors, false);
615:   }
616:   if (record.networkEvidence !== 'incompleteNetworkEvidence') {
617:     push(errors, 'VALUE_MISMATCH', `${label}.networkEvidence`, 'network evidence is incomplete');
618:   }
619:   if ('derivationNote' in record) expectString(record.derivationNote, `${label}.derivationNote`, errors);
620:   if (isPlain(record.sourcePins)) {
621:     expectKeys(record.sourcePins, `${label}.sourcePins`, SOURCE_PIN_KEYS, errors);
622:     for (const key of ['acceptedSourcePath', 'acceptedSourceSha256', 'metadataInspectedPath', 'metadataInspectedSha256']) {
623:       if (key in record.sourcePins) expectString(record.sourcePins[key], `${label}.sourcePins.${key}`, errors);
624:     }
625:     if ('metadataUsedForExpectations' in record.sourcePins) {
626:       expectBool(record.sourcePins.metadataUsedForExpectations, `${label}.sourcePins.metadataUsedForExpectations`, errors, false);
627:     }


## /home/charl/Moriarty/experiments/moriarty-midnight-financial/src/differential.mjs
SHA256 4a99290b40d89a28b363fa1c58ee43c6922ed73e86d5079e43ec27fdcb2c53fe
1795:       push(errors, 'UNKNOWN_FIELD', `observed.stages[${name}]`, `extra stage ${name}`);
1796:     }
1797:   }
1798: }
1799: 
1800: function emptyResult(errors) {
1801:   sortErrors(errors);
1802:   return {
1803:     ok: errors.length === 0,
1804:     errors,
1805:     networkAcceptance: false,
1806:     networkEvidence: 'incompleteNetworkEvidence'
1807:   };
1808: }
1809: 
1810: export function compareFinancialEffects(expected, observed) {
1811:   const errors = [];
1812:   const snapExpected = snapshot(expected);
1813:   const snapObserved = snapshot(observed);
1814:   try {
1815:     const expectedReady = validateRecord(expected, 'expected', errors);
1816:     const observedReady = validateRecord(observed, 'observed', errors);
1817:     if (expectedReady) replayRecord(expected, 'expected', errors);
1818:     if (observedReady) replayRecord(observed, 'observed', errors);
1819:     if (expectedReady && observedReady) compareRecords(expected, observed, errors);
1820:   } catch {
1821:     if (errors.length === 0) {
1822:       push(errors, 'MALFORMED_STRUCTURE', '', 'comparison aborted on malformed input');
1823:     }
1824:   }
1825:   if (snapshot(expected) !== snapExpected || snapshot(observed) !== snapObserved) {
1826:     push(errors, 'INPUT_MUTATED', '', 'compareFinancialEffects mutated an input');
1827:   }
1828:   return emptyResult(errors);
1829: }


## /home/charl/Moriarty/experiments/moriarty-midnight-financial/custody/loan.compact
SHA256 c1485f2915cedf173b858dfdbfdb9cf135915e109dea18bf00c8d237f302759e
40: 
41: constructor(borrowerSecret: Bytes<32>, lenderSecret: Bytes<32>, borrowerPayout: UserAddress, lenderPayout: UserAddress, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>) {
42:   const pinnedProgram: Bytes<32> = Bytes[149, 180, 110, 57, 169, 3, 158, 25, 6, 59, 179, 214, 24, 18, 138, 236, 108, 189, 158, 230, 86, 182, 230, 53, 179, 88, 126, 127, 63, 82, 53, 178];
43:   const publicProgram: Bytes<32> = disclose(expectedProgram);
44:   assert(publicProgram == pinnedProgram, "PROGRAM_MISMATCH");
45:   programDigest = pinnedProgram;
46:   networkTag = disclose(expectedNetwork);
47:   borrowerAddress = disclose(borrowerPayout);
48:   lenderAddress = disclose(lenderPayout);
49:   borrowerCapability = disclose(capabilityHash(pad(32, "moriarty:sp05:loan:borrower"), expectedNetwork, pinnedProgram, borrowerSecret));
50:   lenderCapability = disclose(capabilityHash(pad(32, "moriarty:sp05:loan:lender"), expectedNetwork, pinnedProgram, lenderSecret));
51:   usdDomain = pad(32, "moriarty:sp05:usd:v1");
52:   initialized = false;
53:   remaining = 0;
54:   revision = 0;
55: }
56: 
57: export circuit initialize(borrowerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedActor: Uint<32>, now: Uint<64>): [] {
58:   assert(expectedProgram == programDigest, "PROGRAM_MISMATCH");
59:   assert(expectedNetwork == networkTag, "NETWORK_MISMATCH");
60:   assert(capabilityHash(pad(32, "moriarty:sp05:loan:borrower"), networkTag, programDigest, borrowerSecret) == borrowerCapability, "BORROWER_CAPABILITY");
61:   assert(expectedActor == 2, "ACTOR_MAPPING");
62:   assert(!initialized, "ALREADY_INITIALIZED");
63:   constrainNow(now);
64:   kernelState = KernelState { f0: 5000000000, f1: 0, f2: 0, f3: 0, f4: 0, f5: 20000000000, f6: 0, f7: 0, f8: 0 };
65:   remaining = 2;
66:   revision = 0;
67:   usdColor = mintUnshieldedToken(usdDomain, 20000000000, right<ContractAddress, UserAddress>(borrowerAddress));
68:   initialized = true;
69: }
70: 
71: export circuit accrue(borrowerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedRevision: Uint<128>, expectedActor: Uint<32>, now: Uint<64>, hints: Hints0): Result0 {
72:   assert(expectedProgram == programDigest, "PROGRAM_MISMATCH");
73:   assert(expectedNetwork == networkTag, "NETWORK_MISMATCH");
74:   assert(expectedRevision == revision, "REVISION_MISMATCH");
75:   assert(initialized, "NOT_INITIALIZED");
76:   assert(capabilityHash(pad(32, "moriarty:sp05:loan:borrower"), networkTag, programDigest, borrowerSecret) == borrowerCapability, "BORROWER_CAPABILITY");
77:   assert(expectedActor == 2, "ACTOR_MAPPING");
78:   const t: Uint<128> = constrainNow(now);
79:   const result: Result0 = disclose(transition0(kernelState, Arguments0 { a0: expectedActor }, KernelObservations { o0: t }, remaining, revision, hints));
80:   assert(result.effect0.v0 == 4, "DUE_ID_PR");
81:   assert(result.effect0.v1 == 2, "DUE_PR_DEBTOR");


## /home/charl/Moriarty/experiments/moriarty-midnight-financial/custody/build.mjs
SHA256 a8e14954c77588d47360f01cb0604c49d5eec5c004b0d28f7a84f930935b8b95
15:   return argv[i + 1];
16: }
17: 
18: export function buildCustody(options = {}) {
19:   const hereDir = options.hereDir ?? here;
20:   const wt = options.worktree ?? worktree;
21:   const rf = options.readFileSync ?? readFileSync;
22:   const wf = options.writeFileSync ?? writeFileSync;
23:   const mk = options.mkdirSync ?? mkdirSync;
24:   const rm = options.rmSync ?? rmSync;
25:   const exists = options.existsSync ?? existsSync;
26:   const lstat = options.lstatSync ?? lstatSync;
27:   const symlink = options.symlinkSync ?? symlinkSync;
28:   const skipZk = options.skipZk ?? false;
29:   const skipToolchain = options.skipToolchain ?? false;
30:   const compile = options.compile;
31:   const argv = options.argv ?? process.argv;
32:   const cwd = options.cwd ?? process.cwd();
33:   const env = options.env ?? process.env;
34:   if (!skipZk) throw new Error('build requires --skip-zk');
35:   const outputDir = options.outputDir;
36:   if (!outputDir) throw new Error('missing --output-dir');
37:   const b = options.bindings ?? JSON.parse(asBuffer(rf(join(hereDir, 'bindings.json'))).toString('utf8'));
38:   const runtimeModules = b.toolchain.runtimeNodeModules;
39:   const commands = [];
40: 
41:   function run(command, opts = {}) {
42:     const result = spawnSync(command[0], command.slice(1), {
43:       cwd: opts.cwd ?? cwd,
44:       encoding: 'utf8',
45:       timeout: opts.timeout ?? 90000,
46:       env: opts.env ?? env,
47:     });
48:     commands.push({
49:       argv: command,
50:       cwd: opts.cwd ?? cwd,
51:       envNames: Object.keys(opts.env ?? env).sort(),
52:       exit: result.status,
53:       stdout: result.stdout,
54:       stderr: result.stderr,

## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts
SHA256 4a6eaccd531f0bb711dd8a8926d14beedd2ab8345deff378f86068608d67dc36
425: export function signData(key: SigningKey, data: Uint8Array): Signature;
426: 
427: /**
428:  * Returns the verifying key for a given signing key
429:  */
430: export function signatureVerifyingKey(sk: SigningKey): SignatureVerifyingKey;
431: 
432: /**
433:  * Verifies if a signature is correct
434:  */
435: export function verifySignature(vk: SignatureVerifyingKey, data: Uint8Array, signature: Signature): boolean;
436: 
437: /**
438:  * Encode a raw {@link RawTokenType} into a `Uint8Array` for use in Compact's
439:  * `RawTokenType` type

## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts
SHA256 4a6eaccd531f0bb711dd8a8926d14beedd2ab8345deff378f86068608d67dc36
2416:    * same coins
2417:    */
2418:   merge(other: Transaction<S, P, B>): Transaction<S, P, B>;
2419: 
2420:   serialize(): Uint8Array;
2421: 
2422:   static deserialize<S extends Signaturish, P extends Proofish, B extends Bindingish>(
2423:     markerS: S['instance'],
2424:     markerP: P['instance'],
2425:     markerB: B['instance'],
2426:     raw: Uint8Array,
2427: 
2428:   ): Transaction<S, P, B>;
2429: 
2430:   /**
2431:    * For given fees, and a given section (guaranteed/fallible), what the
2432:    * surplus or deficit of this transaction in any token type is.
2433:    *

## /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-types/dist/index.d.mts
SHA256 c6b9fb7c40e4cbe00a9054ad99026912df4358a3a450a90ede912a87929efb30
890:  * Starts a contract state stream at the given block hash.
891:  */
892: type BlockHashConfig = {
893:     readonly type: 'blockHash';
894:     /**
895:      * The block height indicating where to begin the state stream.
896:      */
897:     readonly blockHash: string;
898: };
899: /**
900:  * The configuration for a contract state observable. The corresponding observables may begin at different
901:  * places (e.g. after a specific transaction identifier / block height) depending on the configuration, but
902:  * all state updates after the beginning are always included.
903:  */
904: type ContractStateObservableConfig = ((TxIdConfig | BlockHashConfig | BlockHeightConfig) & {
905:     /**

## /home/charl/Moriarty/experiments/moriarty-language/spec/examples/loan.mori
SHA256 1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49
1: agreement LoanFirstPeriod profile "moriarty-bounded-atomic/1" {
2:   lifetime 2;
3:   horizon 2000000000;
4:   unit USD_micro;
5:   const borrower: Text = text("borrower");
6:   const lender: Text = text("lender");
7:   const annual_rate_numerator: UInt128 = uint(8);
8:   const annual_rate_denominator: UInt128 = uint(100);
9:   const day_count_numerator: UInt128 = uint(31);
10:   const day_count_denominator: UInt128 = uint(365);
11:   const principal_installment: Amount<USD_micro> = amount(500000000, USD_micro);
12:   const expected_interest: Amount<USD_micro> = amount(33972602, USD_micro);
13:   const expected_total_due: Amount<USD_micro> = amount(533972602, USD_micro);
14:   const expected_outstanding_notional: Amount<USD_micro> = amount(4500000000, USD_micro);
15:   state notional: Amount<USD_micro> = amount(5000000000, USD_micro);
16:   state principal_due: Amount<USD_micro> = amount(0, USD_micro);
17:   state interest_due: Amount<USD_micro> = amount(0, USD_micro);
18:   state principal_paid: Amount<USD_micro> = amount(0, USD_micro);
19:   state interest_paid: Amount<USD_micro> = amount(0, USD_micro);
20:   state borrower_cash: Amount<USD_micro> = amount(20000000000, USD_micro);
21:   state lender_cash: Amount<USD_micro> = amount(0, USD_micro);
22:   state cursor: UInt128 = uint(0);
23:   state episode_closed: UInt128 = uint(0);
24:   observation now: UInt128;
25:   settlement USD_micro_asset asset text("USD_TEST_ASSET") quantum amount(1, USD_micro);
26:   status episode closed_when episode_closed == uint(1);
27:   status agreement remaining_notional notional;
28:   policy accrued_interest targets write(accrue, interest_due), effect(accrue, 1, amount) {
29:     unit USD_micro;
30:     derivation "notional*8*31/(100*365)";
31:     rounding floor(accrue, interest_calculated);
32:     remainder "discard 54/73 micro-USD for this sample only";
33:     comparison "exact integer sample value; no ACTUS tolerance claim";
34:     proof "loan_first_period_interest_floor_v1";
35:   }
36:   policy accrued_principal targets write(accrue, principal_due), write(accrue, notional), effect(accrue, 0, amount) {
37:     unit USD_micro;
38:     derivation "fixed first-period principal installment";
39:     rounding none;
40:     remainder "none";

## /home/charl/Moriarty/experiments/moriarty-midnight-financial/src/differential.mjs
SHA256 4a99290b40d89a28b363fa1c58ee43c6922ed73e86d5079e43ec27fdcb2c53fe
39: 
40: const LOAN_PINS = {
41:   acceptedSourcePath: 'experiments/moriarty-language/spec/examples/loan.mori',
42:   acceptedSourceSha256: '1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49',
43:   metadataInspectedPath: 'experiments/moriarty-language/compact/generated/loan/metadata.json',
44:   metadataInspectedSha256: '9e13a2677797e6328ad6b7b8d1a34a447d34a9ed9828e1c80867618040648ab5',
45:   metadataUsedForExpectations: false
46: };
47: const SWAP_PINS = {
48:   acceptedSourcePath: 'experiments/moriarty-language/spec/examples/swap.mori',
49:   acceptedSourceSha256: '0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e',
50:   metadataInspectedPath: 'experiments/moriarty-language/compact/generated/swap/metadata.json',
51:   metadataInspectedSha256: 'e0b96181aad293cac32668cfaa3152c5c7cf38dca55e5f567616d17511fe7a9e',
52:   metadataUsedForExpectations: false
53: };
54: 
55: function gcd(a, b) {
56:   let x = a < 0n ? -a : a;
57:   let y = b < 0n ? -b : b;
58:   while (y !== 0n) {
59:     const t = x % y;
60:     x = y;
61:     y = t;
62:   }
63:   return x === 0n ? 1n : x;
64: }
65: 
66: function loanEconomics() {
67:   const notional = 5000000000n;
68:   const installment = 500000000n;
69:   const rateN = 8n;
70:   const rateD = 100n;
71:   const perN = 31n;
72:   const perD = 365n;
73:   const num = notional * rateN * perN;
74:   const den = rateD * perD;
75:   const interest = num / den;
76:   const remainder = num % den;
77:   const g = gcd(remainder, den);
78:   const total = installment + interest;
79:   const cash = 20000000000n;
80:   return {
81:     name: 'LoanFirstPeriod',
82:     profile: 'moriarty-bounded-atomic/1',
83:     lifetime: 2n,
84:     horizon: 2000000000n,
85:     stages: ['setup', 'accrue', 'settle'],
86:     roles: {
87:       borrower: { status: 'symbolic-not-address', logicalId: 'borrower' },
88:       lender: { status: 'symbolic-not-address', logicalId: 'lender' },
89:       'setup-mint': { status: 'synthetic-local-setup-source', logicalId: 'setup-mint' }
90:     },
91:     actors: ['borrower', 'lender'],
92:     roleNames: ['borrower', 'lender', 'setup-mint'],
93:     assets: [{
94:       id: 'USD_TEST_ASSET',
95:       logicalId: 'USD_TEST_ASSET',
96:       denomination: 'USD_micro',
97:       quantum: 1n,
98:       color: 'USD_TEST_ASSET',
99:       status: 'unresolved',
100:       unit: 'USD_micro'
101:     }],
102:     pins: LOAN_PINS,
103:     notional,
104:     installment,
105:     interest,
106:     remainderN: remainder / g,
107:     remainderD: den / g,
108:     rateN,
109:     rateD,
110:     perN,
111:     perD,
112:     total,
113:     remaining: notional - installment,
114:     cash,
115:     borrowerAfter: cash - total,
116:     debtId: 'LoanFirstPeriod:remaining-notional',
117:     duePR: 'lam01:period1:PR',
118:     dueIP: 'lam01:period1:IP'
119:   };
120: }
121: 
122: function swapEconomics() {
123:   const amountIn = 10000n;
124:   const reserveA = 1000000n;
125:   const reserveB = 2000000n;
126:   const traderA = 100000n;
127:   const feeN = 997n;
128:   const feeD = 1000n;
129:   const fee = amountIn - (amountIn * feeN) / feeD;
130:   const effective = amountIn * feeN;

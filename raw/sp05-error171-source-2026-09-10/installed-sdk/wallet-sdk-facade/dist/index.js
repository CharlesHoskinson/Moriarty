// This file is part of MIDNIGHT-WALLET-SDK.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
import * as ledger from '@midnight-ntwrk/ledger-v8';
import { makeDefaultSubmissionService, } from '@midnight-ntwrk/wallet-sdk-capabilities';
import { makeDefaultProvingService, } from '@midnight-ntwrk/wallet-sdk-capabilities/proving';
import { ShieldedSectionSchema, mergeShieldedSections, } from '@midnight-ntwrk/wallet-sdk-shielded';
import { UnshieldedSectionSchema } from '@midnight-ntwrk/wallet-sdk-unshielded-wallet';
import { DustSectionSchema, mergeDustSections } from '@midnight-ntwrk/wallet-sdk-dust-wallet';
import { Clock } from '@midnight-ntwrk/wallet-sdk-utilities';
import { FetchTermsAndConditions as FetchTermsAndConditionsQuery } from '@midnight-ntwrk/wallet-sdk-indexer-client';
import { QueryRunner } from '@midnight-ntwrk/wallet-sdk-indexer-client/effect';
import { Array as Arr, pipe, Schema } from 'effect';
import { TransactionHistoryStorage } from '@midnight-ntwrk/wallet-sdk-abstractions';
import { combineLatest, map, firstValueFrom, concatMap } from 'rxjs';
import { PendingTransactions, PendingTransactionsServiceImpl, } from '@midnight-ntwrk/wallet-sdk-capabilities';
import { finalizedTransactionTrait } from './transaction.js';
/**
 * Full entry schema for transaction history — common fields + all wallet sections. Pass this to
 * `InMemoryTransactionHistoryStorage` to enable serialize/restore.
 */
export const WalletEntrySchema = Schema.Struct({
    ...TransactionHistoryStorage.TransactionHistoryCommonSchema.fields,
    shielded: Schema.optional(ShieldedSectionSchema),
    unshielded: Schema.optional(UnshieldedSectionSchema),
    dust: Schema.optional(DustSectionSchema),
});
export const mergeWalletEntries = (existing, incoming) => ({
    ...existing,
    ...incoming,
    ...(existing.shielded !== undefined && incoming.shielded !== undefined
        ? { shielded: mergeShieldedSections(existing.shielded, incoming.shielded) }
        : {}),
    ...(existing.unshielded !== undefined && incoming.unshielded !== undefined
        ? { unshielded: { ...existing.unshielded, ...incoming.unshielded } }
        : {}),
    ...(existing.dust !== undefined && incoming.dust !== undefined
        ? { dust: mergeDustSections(existing.dust, incoming.dust) }
        : {}),
});
const isWalletEntry = Schema.is(WalletEntrySchema);
const TokenKindsToBalance = new (class {
    allTokenKinds = ['shielded', 'unshielded', 'dust'];
    toFlags = (tokenKinds) => {
        return pipe(tokenKinds, (kinds) => (kinds === 'all' ? this.allTokenKinds : kinds), (kinds) => ({
            shouldBalanceUnshielded: kinds.includes('unshielded'),
            shouldBalanceShielded: kinds.includes('shielded'),
            shouldBalanceDust: kinds.includes('dust'),
        }));
    };
})();
export const BalancingRecipe = {
    isRecipe: (value) => {
        return (typeof value === 'object' &&
            value !== null &&
            'type' in value &&
            typeof value.type === 'string' &&
            ['FINALIZED_TRANSACTION', 'UNBOUND_TRANSACTION', 'UNPROVEN_TRANSACTION'].includes(value.type));
    },
    getTransactions: (recipe) => {
        switch (recipe.type) {
            case 'FINALIZED_TRANSACTION': {
                return [recipe.originalTransaction, recipe.balancingTransaction];
            }
            case 'UNBOUND_TRANSACTION': {
                const balancingPart = recipe.balancingTransaction ? [recipe.balancingTransaction] : [];
                return [recipe.baseTransaction, ...balancingPart];
            }
            case 'UNPROVEN_TRANSACTION': {
                return [recipe.transaction];
            }
        }
    },
};
export class FacadeState {
    shielded;
    unshielded;
    dust;
    pending;
    get isSynced() {
        return (this.shielded.state.progress.isStrictlyComplete() &&
            this.dust.state.progress.isStrictlyComplete() &&
            this.unshielded.progress.isStrictlyComplete());
    }
    constructor(shielded, unshielded, dust, pending) {
        this.shielded = shielded;
        this.unshielded = unshielded;
        this.dust = dust;
        this.pending = pending;
    }
}
const DEFAULT_TTL_MS = 60 * 60 * 1000; // 1 hour
/**
 * Clock abstraction for obtaining the current time. By default, the facade uses the system clock
 * ({@link Clock.systemClock}); for testing with a simulator, inject a custom clock (e.g. one backed by the simulator's
 * time).
 *
 * Re-exported from `@midnight-ntwrk/wallet-sdk-utilities` as a namespace so the type is `Clock.Clock` and the default is
 * `Clock.systemClock`. Forwarding the same symbol — rather than re-declaring its members individually — keeps the
 * umbrella `wallet-sdk` package's star-exports unambiguous and lets lower-level packages (e.g. dust-wallet) share it
 * without a circular dependency.
 */
export { Clock };
export class WalletFacade {
    static makeDefaultSubmissionService(config) {
        return makeDefaultSubmissionService(config);
    }
    static makeDefaultPendingTransactionsService(config) {
        return PendingTransactionsServiceImpl.init({
            configuration: config,
            txTrait: finalizedTransactionTrait,
        });
    }
    static makeDefaultProvingService(config) {
        if (config.provingServerUrl) {
            return makeDefaultProvingService({
                provingServerUrl: config.provingServerUrl,
            });
        }
        else {
            throw new Error("Missing required configuration: 'provingServerUrl' must be set in config, or provide a custom provingService in init parameters.");
        }
    }
    /**
     * Fetches the current Terms and Conditions from the network indexer.
     *
     * This is a static, pre-initialization utility — no wallet instance is required. Wallet builders should call this
     * before or independently of wallet initialization to display the current T&C to end users and obtain the hash for
     * content verification.
     *
     * The returned `hash` is the hex-encoded SHA-256 hash of the document at `url`. Wallet builders are responsible for
     * fetching and rendering the document content via `url` in whatever manner suits their application.
     *
     * @param configuration - An object with an `indexerClientConnection.indexerHttpUrl`. Any wallet configuration that
     *   satisfies {@link FetchTermsAndConditionsConfiguration} can be passed directly.
     * @returns A promise resolving to the current {@link TermsAndConditions}, or rejecting if no Terms and Conditions have
     *   been set on the network yet.
     */
    static async fetchTermsAndConditions(configuration) {
        const result = await QueryRunner.runPromise(FetchTermsAndConditionsQuery, {}, {
            url: configuration.indexerClientConnection.indexerHttpUrl,
        });
        const tc = result.block?.systemParameters?.termsAndConditions;
        if (!tc) {
            throw new Error('Terms and Conditions are not currently set on the network.');
        }
        return tc;
    }
    /**
     * Default initialization for {@link WalletFacade}. It is a static method, which takes an object holding configuration
     * and initialization of necessary components. Specifically - it requires following fields:
     *
     * - `configuration` - holding a configuration, which needs to extend {@link DefaultConfiguration} - this way allows to
     *   convey use-case-specific settings in the same way, as the SDK works by default
     * - `shielded` - a function taking the configuration and returning shielded wallet (or a promise with such)
     *   implementing {@link ShieldedWalletAPI}
     * - `unshielded` - a function taking the configuration and returning unshielded wallet (or a promise with such)
     *   implementing {@link UnshieldedWalletAPI}
     * - `dust` - a function taking the configuration and returning Dust wallet (or a promise with such) implementing
     *   {@link DustWalletAPI} There are some optional services/abstractions to provide, too. If not provided - default
     *   implementations will be used, each of them is initialized by a function taking the configuration and returning
     *   proper implementation (wrapped in a {@link Promise} or not).
     * - `submissionService` - needs to implement {@link SubmissionService} for a {@link ledger.FinalizedTransaction} to
     *   submit transactions to the network, default uses Node RPC connection
     * - `pendingTransactionsService` - needs to implement {@link PendingTransactionsService} for a
     *   {@link ledger.FinalizedTransaction} to keep track of pending transactions, default uses in-memory implementation
     * - `provingService` - needs to implement {@link ProvingService} to prove it, default uses proving server
     * - `clock` - needs to implement {@link Clock.Clock} for getting current time, default uses system clock
     */
    static async init(initParams) {
        const submissionService = await Promise.resolve(initParams.submissionService
            ? initParams.submissionService(initParams.configuration)
            : WalletFacade.makeDefaultSubmissionService(initParams.configuration));
        const pendingTransactionsService = await Promise.resolve(initParams.pendingTransactionsService
            ? initParams.pendingTransactionsService(initParams.configuration)
            : WalletFacade.makeDefaultPendingTransactionsService(initParams.configuration));
        const provingService = await Promise.resolve(initParams.provingService
            ? initParams.provingService(initParams.configuration)
            : WalletFacade.makeDefaultProvingService(initParams.configuration));
        const shielded = await Promise.resolve(initParams.shielded(initParams.configuration));
        const unshielded = await Promise.resolve(initParams.unshielded(initParams.configuration));
        const dust = await Promise.resolve(initParams.dust(initParams.configuration));
        const clock = await Promise.resolve(initParams.clock ? initParams.clock(initParams.configuration) : Clock.systemClock);
        return new WalletFacade(shielded, unshielded, dust, submissionService, pendingTransactionsService, provingService, initParams.configuration.txHistoryStorage, clock);
    }
    shielded;
    unshielded;
    dust;
    submissionService;
    pendingTransactionsService;
    provingService;
    #txHistoryStorage;
    clock;
    #pendingSubscription;
    /**
     * Constructor is private on purpose - much of initialization of the facade is potentially asynchronous, and adding
     * new parameters is a breaking change to the users Use {@link WalletFacade.init} instead
     *
     * @private
     */
    constructor(shieldedWallet, unshieldedWallet, dustWallet, submissionService, pendingTransactionsService, provingService, txHistoryStorage, clock = Clock.systemClock) {
        this.shielded = shieldedWallet;
        this.unshielded = unshieldedWallet;
        this.dust = dustWallet;
        this.submissionService = submissionService;
        this.pendingTransactionsService = pendingTransactionsService;
        this.provingService = provingService;
        this.#txHistoryStorage = txHistoryStorage;
        this.clock = clock;
        this.#pendingSubscription = this.pendingTransactionsService
            .state()
            .pipe(concatMap((pending) => PendingTransactions.allFailed(pending)), concatMap((item) => this.revert(item.tx)))
            .subscribe();
    }
    defaultTtl() {
        return new Date(this.clock.now().getTime() + DEFAULT_TTL_MS);
    }
    mergeUnprovenTransactions(a, b) {
        if (a && b)
            return a.merge(b);
        return a ?? b;
    }
    async createDustActionTransaction(action, nightUtxos, nightVerifyingKey, signDustRegistration) {
        const ttl = this.defaultTtl();
        const now = this.clock.now();
        const isRegistration = action.type === 'registration';
        const dustReceiverAddress = isRegistration ? action.dustReceiverAddress : undefined;
        // Step 1 — Dust decides which Night UTxO belongs in the guaranteed slot (the one whose dust
        // generation can pay the fee) and computes the fee-payment allowance.
        const split = await this.dust.splitNightUtxosForDustRegistration(now, nightUtxos.map(({ utxo, meta }) => ({
            ...utxo,
            ctime: meta.ctime,
            registeredForDustGeneration: meta.registeredForDustGeneration,
        })), isRegistration);
        const toUnshieldedUtxoWithMeta = (u) => ({
            utxo: {
                value: u.utxo.value,
                type: u.utxo.type,
                owner: u.utxo.owner,
                intentHash: u.utxo.intentHash,
                outputNo: u.utxo.outputNo,
            },
            meta: {
                ctime: u.utxo.ctime,
                registeredForDustGeneration: u.utxo.registeredForDustGeneration,
            },
        });
        const guaranteedForUnshielded = split.guaranteedUtxos.map(toUnshieldedUtxoWithMeta);
        const fallibleForUnshielded = split.fallibleUtxos.map(toUnshieldedUtxoWithMeta);
        // Step 2 — Unshielded books the Night UTxOs (move available -> pending) and builds the intent
        // with the two offers. After this point, a concurrent build call that wants any of these UTxOs
        // will fail fast.
        const txWithOffers = await this.unshielded.rotateUtxos(guaranteedForUnshielded, fallibleForUnshielded, nightVerifyingKey, ttl);
        // Step 3 — Dust attaches its DustActions onto the intent the unshielded wallet just built.
        // If this fails we must unbook the UTxOs so the caller can retry.
        let txWithDustActions;
        try {
            txWithDustActions = await this.dust.attachDustRegistration(txWithOffers, now, nightVerifyingKey, dustReceiverAddress, split.feePayment);
        }
        catch (error) {
            await this.unshielded.revertTransaction(txWithOffers);
            throw error;
        }
        // Step 4 (first-time registration only) — Fail fast if the dust generated so far by the
        // unregistered guaranteed UTxOs is below the registration's own fee. Submitting would fail
        // on-chain with BalanceCheckOverspend. Skip for re-registration (all guaranteed UTxOs already
        // registered) since `feePayment` is 0 by design and the caller is expected to balance the fee
        // externally via `balanceUnprovenTransaction({ tokenKindsToBalance: ['dust'] })`.
        const hasUnregisteredGuaranteed = split.guaranteedUtxos.some((u) => !u.utxo.registeredForDustGeneration);
        if (isRegistration && hasUnregisteredGuaranteed) {
            const fee = await this.dust.calculateFee([txWithDustActions]);
            if (split.feePayment < fee) {
                await this.unshielded.revertTransaction(txWithOffers);
                throw Error(`Insufficient generated dust to cover registration fee (have ${split.feePayment}, need ${fee}). ` +
                    `Use WalletFacade.waitForGeneratedDust(utxos, ${fee}) before retrying.`);
            }
        }
        // Step 5 — Sign via the standard signRecipe pathway, which now stamps both the unshielded
        // offers and the dust registration. Signing failures also need to release the booking.
        try {
            const signedRecipe = await this.signRecipe({ type: 'UNPROVEN_TRANSACTION', transaction: txWithDustActions }, signDustRegistration);
            if (signedRecipe.type !== 'UNPROVEN_TRANSACTION') {
                throw Error('signRecipe returned unexpected recipe type for dust action transaction.');
            }
            return signedRecipe.transaction;
        }
        catch (error) {
            await this.unshielded.revertTransaction(txWithOffers);
            throw error;
        }
    }
    state() {
        return combineLatest([
            this.shielded.state,
            this.unshielded.state,
            this.dust.state,
            this.pendingTransactionsService.state(),
        ]).pipe(map(([shieldedState, unshieldedState, dustState, pending]) => new FacadeState(shieldedState, unshieldedState, dustState, pending)));
    }
    async waitForSyncedState() {
        const [shieldedState, unshieldedState, dustState, pending] = await Promise.all([
            this.shielded.waitForSyncedState(),
            this.unshielded.waitForSyncedState(),
            this.dust.waitForSyncedState(),
            firstValueFrom(this.pendingTransactionsService.state()),
        ]);
        return new FacadeState(shieldedState, unshieldedState, dustState, pending);
    }
    async submitTransaction(tx) {
        try {
            await this.pendingTransactionsService.addPendingTransaction(tx);
            await this.submissionService.submitTransaction(tx, 'Finalized');
            return tx.identifiers().at(-1);
        }
        catch (error) {
            await this.revert(tx);
            throw error;
        }
    }
    async balanceFinalizedTransaction(tx, secretKeys, options) {
        const { shieldedSecretKeys, dustSecretKey } = secretKeys;
        const { ttl, tokenKindsToBalance = 'all' } = options;
        const { shouldBalanceDust, shouldBalanceShielded, shouldBalanceUnshielded } = TokenKindsToBalance.toFlags(tokenKindsToBalance);
        // Step 1: Run unshielded and shielded balancing
        const unshieldedBalancingTx = shouldBalanceUnshielded
            ? await this.unshielded.balanceFinalizedTransaction(tx)
            : undefined;
        const shieldedBalancingTx = shouldBalanceShielded
            ? await this.shielded.balanceTransaction(shieldedSecretKeys, tx)
            : undefined;
        // Step 2: Merge unshielded and shielded balancing
        const mergedBalancingTx = this.mergeUnprovenTransactions(shieldedBalancingTx, unshieldedBalancingTx);
        // Step 3: Conditionally add dust/fee balancing
        const feeBalancingTx = shouldBalanceDust
            ? await this.dust.balanceTransactions(dustSecretKey, mergedBalancingTx ? [tx, mergedBalancingTx] : [tx], ttl)
            : undefined;
        // Step 4: Merge fee balancing and create final recipe
        const balancingTx = this.mergeUnprovenTransactions(mergedBalancingTx, feeBalancingTx);
        if (!balancingTx) {
            throw new Error('No balancing transaction was created. Please check your transaction.');
        }
        return {
            type: 'FINALIZED_TRANSACTION',
            originalTransaction: tx,
            balancingTransaction: balancingTx,
        };
    }
    async balanceUnboundTransaction(tx, secretKeys, options) {
        const { shieldedSecretKeys, dustSecretKey } = secretKeys;
        const { ttl, tokenKindsToBalance = 'all' } = options;
        const { shouldBalanceDust, shouldBalanceShielded, shouldBalanceUnshielded } = TokenKindsToBalance.toFlags(tokenKindsToBalance);
        // Step 1: Run unshielded and shielded balancing
        const shieldedBalancingTx = shouldBalanceShielded
            ? await this.shielded.balanceTransaction(shieldedSecretKeys, tx)
            : undefined;
        // For unbound transactions, unshielded balancing happens in place not with a balancing transaction
        const balancedUnshieldedTx = shouldBalanceUnshielded
            ? await this.unshielded.balanceUnboundTransaction(tx)
            : undefined;
        // Step 2: Unbound unshielded tx are balanced in place, use it as base tx if present
        const baseTx = balancedUnshieldedTx ?? tx;
        // Step 3: Conditionally add dust/fee balancing
        const feeBalancingTransaction = shouldBalanceDust
            ? await this.dust.balanceTransactions(dustSecretKey, shieldedBalancingTx ? [baseTx, shieldedBalancingTx] : [baseTx], ttl)
            : undefined;
        // Step 4: Create the final balancing transaction
        const balancingTransaction = this.mergeUnprovenTransactions(shieldedBalancingTx, feeBalancingTransaction);
        // if there is no balancingTransaction and there was no unshielded tx balancing (in place) throw an error.
        if (!balancingTransaction && !balancedUnshieldedTx) {
            throw new Error('No balancing transaction was created. Please check your transaction.');
        }
        return {
            type: 'UNBOUND_TRANSACTION',
            baseTransaction: baseTx,
            balancingTransaction: balancingTransaction ?? undefined,
        };
    }
    async balanceUnprovenTransaction(tx, secretKeys, options) {
        const { shieldedSecretKeys, dustSecretKey } = secretKeys;
        const { ttl, tokenKindsToBalance = 'all' } = options;
        const { shouldBalanceDust, shouldBalanceShielded, shouldBalanceUnshielded } = TokenKindsToBalance.toFlags(tokenKindsToBalance);
        // Step 1: Run unshielded and shielded balancing
        const shieldedBalancingTx = shouldBalanceShielded
            ? await this.shielded.balanceTransaction(shieldedSecretKeys, tx)
            : undefined;
        // For unproven transactions, unshielded balancing happens in place
        const balancedUnshieldedTx = shouldBalanceUnshielded
            ? await this.unshielded.balanceUnprovenTransaction(tx)
            : undefined;
        // Step 2: Use the balanced unshielded tx if present, otherwise use the original tx
        const baseTx = balancedUnshieldedTx ?? tx;
        // Step 3: Merge shielded balancing into base tx if present
        const mergedTx = this.mergeUnprovenTransactions(baseTx, shieldedBalancingTx);
        // Step 4: Conditionally add dust/fee balancing
        const feeBalancingTx = shouldBalanceDust
            ? await this.dust.balanceTransactions(dustSecretKey, [mergedTx], ttl)
            : undefined;
        // Step 5: Merge fee balancing if present
        const balancedTx = this.mergeUnprovenTransactions(mergedTx, feeBalancingTx);
        return {
            type: 'UNPROVEN_TRANSACTION',
            transaction: balancedTx,
        };
    }
    async finalizeRecipe(recipe) {
        return Promise.resolve(recipe)
            .then(async (recipe) => {
            switch (recipe.type) {
                case 'FINALIZED_TRANSACTION': {
                    const finalizedBalancing = await this.finalizeTransaction(recipe.balancingTransaction);
                    return recipe.originalTransaction.merge(finalizedBalancing);
                }
                case 'UNBOUND_TRANSACTION': {
                    const finalizedBalancingTx = recipe.balancingTransaction
                        ? await this.finalizeTransaction(recipe.balancingTransaction)
                        : undefined;
                    const finalizedTransaction = recipe.baseTransaction.bind();
                    return finalizedBalancingTx ? finalizedTransaction.merge(finalizedBalancingTx) : finalizedTransaction;
                }
                case 'UNPROVEN_TRANSACTION': {
                    return await this.finalizeTransaction(recipe.transaction);
                }
            }
        })
            .then(async (finalizedTx) => {
            await this.pendingTransactionsService.addPendingTransaction(finalizedTx);
            return finalizedTx;
        });
    }
    async signRecipe(recipe, signSegment) {
        switch (recipe.type) {
            case 'FINALIZED_TRANSACTION': {
                const signedBalancingTx = await this.signUnprovenTransaction(recipe.balancingTransaction, signSegment);
                const withDustSig = await this.#signDustRegistrationIfPresent(signedBalancingTx, signSegment);
                return {
                    type: 'FINALIZED_TRANSACTION',
                    originalTransaction: recipe.originalTransaction,
                    balancingTransaction: withDustSig,
                };
            }
            case 'UNBOUND_TRANSACTION': {
                const signedBalancingTx = recipe.balancingTransaction
                    ? await this.signUnprovenTransaction(recipe.balancingTransaction, signSegment).then((tx) => this.#signDustRegistrationIfPresent(tx, signSegment))
                    : undefined;
                const signedBaseTx = await this.signUnboundTransaction(recipe.baseTransaction, signSegment);
                return {
                    type: 'UNBOUND_TRANSACTION',
                    baseTransaction: signedBaseTx,
                    balancingTransaction: signedBalancingTx,
                };
            }
            case 'UNPROVEN_TRANSACTION': {
                const signedTx = await this.signUnprovenTransaction(recipe.transaction, signSegment);
                const withDustSig = await this.#signDustRegistrationIfPresent(signedTx, signSegment);
                return {
                    type: 'UNPROVEN_TRANSACTION',
                    transaction: withDustSig,
                };
            }
        }
    }
    async #signDustRegistrationIfPresent(tx, signSegment) {
        const intent = tx.intents?.get(1);
        const registrations = intent?.dustActions?.registrations ?? [];
        if (!intent || registrations.length === 0) {
            return tx;
        }
        const signature = signSegment(intent.signatureData(1));
        return await this.dust.addDustRegistrationSignature(tx, signature);
    }
    async signUnprovenTransaction(tx, signSegment) {
        return await this.unshielded.signUnprovenTransaction(tx, signSegment);
    }
    async signUnboundTransaction(tx, signSegment) {
        return await this.unshielded.signUnboundTransaction(tx, signSegment);
    }
    async finalizeTransaction(tx) {
        try {
            const unboundTx = await this.provingService.prove(tx);
            const finalizedTx = unboundTx.bind();
            await this.pendingTransactionsService.addPendingTransaction(finalizedTx);
            return finalizedTx;
        }
        catch (error) {
            await Promise.allSettled([
                this.shielded.revertTransaction(tx),
                this.unshielded.revertTransaction(tx),
                this.dust.revertTransaction(tx),
            ]);
            throw error;
        }
    }
    /** Estimates the fee for the given transaction only. This lacks the fees of the balancing transaction. */
    async calculateTransactionFee(tx) {
        return await this.dust.calculateFee([tx]);
    }
    /** Calculates the total fee for the given transaction plus the fee of the balancing transaction. */
    async estimateTransactionFee(tx, secretKey, options) {
        const ttl = options?.ttl ?? this.defaultTtl();
        return await this.dust.estimateFee(secretKey, [tx], ttl, options?.currentTime);
    }
    async transferTransaction(outputs, secretKeys, options) {
        const { shieldedSecretKeys, dustSecretKey } = secretKeys;
        const { ttl, payFees = true } = options;
        const unshieldedOutputs = outputs
            .filter((output) => output.type === 'unshielded')
            .flatMap((output) => output.outputs);
        const shieldedOutputs = outputs.filter((output) => output.type === 'shielded').flatMap((output) => output.outputs);
        if (unshieldedOutputs.length === 0 && shieldedOutputs.length === 0) {
            throw Error('At least one shielded or unshielded output is required.');
        }
        const shieldedTx = shieldedOutputs.length > 0
            ? await this.shielded.transferTransaction(shieldedSecretKeys, shieldedOutputs)
            : undefined;
        const unshieldedTx = unshieldedOutputs.length > 0 ? await this.unshielded.transferTransaction(unshieldedOutputs, ttl) : undefined;
        const mergedTxs = this.mergeUnprovenTransactions(shieldedTx, unshieldedTx);
        // Add fee payment
        const feeBalancingTx = payFees ? await this.dust.balanceTransactions(dustSecretKey, [mergedTxs], ttl) : undefined;
        const finalTx = this.mergeUnprovenTransactions(mergedTxs, feeBalancingTx);
        return {
            type: 'UNPROVEN_TRANSACTION',
            transaction: finalTx,
        };
    }
    /**
     * Provides estimate of the fee of issuing registration transaction with provided UTxOs
     *
     * @param nightUtxos - Night UTxOs to use for the registration
     * @returns And object informing about fee at the moment, as well as estimation of dust generation of the UTxO(s),
     *   that would be used for paying the fee. These include data that allows to compute when the fee could be paid
     */
    async estimateRegistration(nightUtxos) {
        const now = this.clock.now();
        const dustState = await this.dust.waitForSyncedState();
        const dustGenerationEstimations = pipe(nightUtxos, Arr.map(({ utxo, meta }) => ({
            ...utxo,
            ctime: meta.ctime,
            registeredForDustGeneration: meta.registeredForDustGeneration,
        })), (utxosWithMeta) => dustState.estimateDustGeneration(utxosWithMeta, now), (estimatedUtxos) => dustState.capabilities.coinsAndBalances.splitNightUtxos(estimatedUtxos), (split) => split.guaranteed);
        const fakeSigningKey = ledger.sampleSigningKey();
        const fakeVerifyingKey = ledger.signatureVerifyingKey(fakeSigningKey);
        // Use the legacy dust-only construction path here so estimation does NOT book real UTxOs in the
        // unshielded wallet state. (The race-fix path in createDustActionTransaction books on purpose;
        // estimation is meant to be observation-only.)
        const ttl = this.defaultTtl();
        const fakeUnsignedTx = await this.dust.createDustGenerationTransaction(undefined, ttl, nightUtxos.map(({ utxo, meta }) => ({
            ...utxo,
            ctime: meta.ctime,
            registeredForDustGeneration: meta.registeredForDustGeneration,
        })), fakeVerifyingKey, dustState.address);
        const intent = fakeUnsignedTx.intents?.get(1);
        if (!intent) {
            throw Error('Dust generation transaction is missing intent segment 1.');
        }
        const signatureData = intent.signatureData(1);
        const signature = ledger.signData(fakeSigningKey, signatureData);
        const fakeSignedTx = await this.dust.addDustGenerationSignature(fakeUnsignedTx, signature);
        const finalizedFakeTx = fakeSignedTx.mockProve().bind();
        const fee = await this.calculateTransactionFee(finalizedFakeTx);
        return {
            fee,
            dustGenerationEstimations,
        };
    }
    async initSwap(desiredInputs, desiredOutputs, secretKeys, options) {
        const { shieldedSecretKeys, dustSecretKey } = secretKeys;
        const { ttl, payFees = false } = options;
        const { shielded: shieldedInputs, unshielded: unshieldedInputs } = desiredInputs;
        const shieldedOutputs = desiredOutputs
            .filter((output) => output.type === 'shielded')
            .flatMap((output) => output.outputs);
        const unshieldedOutputs = desiredOutputs
            .filter((output) => output.type === 'unshielded')
            .flatMap((output) => output.outputs);
        const hasShieldedPart = (shieldedInputs && Object.keys(shieldedInputs).length > 0) || shieldedOutputs.length > 0;
        const hasUnshieldedPart = (unshieldedInputs && Object.keys(unshieldedInputs).length > 0) || unshieldedOutputs.length > 0;
        if (!hasShieldedPart && !hasUnshieldedPart) {
            throw Error('At least one shielded or unshielded swap is required.');
        }
        const shieldedTx = hasShieldedPart && shieldedInputs !== undefined
            ? await this.shielded.initSwap(shieldedSecretKeys, shieldedInputs, shieldedOutputs)
            : undefined;
        const unshieldedTx = hasUnshieldedPart && unshieldedInputs !== undefined
            ? await this.unshielded.initSwap(unshieldedInputs, unshieldedOutputs, ttl)
            : undefined;
        const combinedTx = this.mergeUnprovenTransactions(shieldedTx, unshieldedTx);
        if (!combinedTx) {
            throw Error('Unexpected transaction state.');
        }
        const feeBalancingTx = payFees ? await this.dust.balanceTransactions(dustSecretKey, [combinedTx], ttl) : undefined;
        const finalTx = this.mergeUnprovenTransactions(combinedTx, feeBalancingTx);
        return {
            type: 'UNPROVEN_TRANSACTION',
            transaction: finalTx,
        };
    }
    async registerNightUtxosForDustGeneration(nightUtxos, nightVerifyingKey, signDustRegistration, dustReceiverAddress) {
        if (nightUtxos.length === 0) {
            throw Error('At least one Night UTXO is required.');
        }
        const receiverAddress = dustReceiverAddress ?? (await this.dust.getAddress());
        const dustRegistrationTx = await this.createDustActionTransaction({ type: 'registration', dustReceiverAddress: receiverAddress }, nightUtxos, nightVerifyingKey, signDustRegistration);
        return {
            type: 'UNPROVEN_TRANSACTION',
            transaction: dustRegistrationTx,
        };
    }
    /**
     * Waits until the dust projected to be generated by the given Night UTxOs reaches `requiredAmount`, re-checking every
     * second. Pair with {@link estimateRegistration} to pick `requiredAmount`, then call before
     * {@link registerNightUtxosForDustGeneration} so the registration covers its own fee.
     *
     * @param nightUtxos - Night UTxOs to project generation for; the same set passed to the registration.
     * @param requiredAmount - Dust threshold to wait for. Resolves immediately if `<= 0n`.
     * @param opts.timeoutMs - Deadline, in ms, for the threshold to be reached. Rejects otherwise. Default `300_000`.
     * @throws If `nightUtxos` is empty, or if `requiredAmount` is not reached within `opts.timeoutMs`.
     */
    async waitForGeneratedDust(nightUtxos, requiredAmount, opts) {
        await this.dust.waitForGeneratedDust(nightUtxos.map(({ utxo, meta }) => ({
            ...utxo,
            ctime: meta.ctime,
            registeredForDustGeneration: meta.registeredForDustGeneration,
        })), requiredAmount, this.clock, opts);
    }
    async deregisterFromDustGeneration(nightUtxos, nightVerifyingKey, signDustRegistration) {
        const dustDeregistrationTx = await this.createDustActionTransaction({ type: 'deregistration' }, nightUtxos, nightVerifyingKey, signDustRegistration);
        return {
            type: 'UNPROVEN_TRANSACTION',
            transaction: dustDeregistrationTx,
        };
    }
    async revert(txOrRecipe) {
        // avoid instanceof check
        const transactionsToRevert = BalancingRecipe.isRecipe(txOrRecipe)
            ? BalancingRecipe.getTransactions(txOrRecipe)
            : [txOrRecipe];
        await Promise.all(transactionsToRevert.map((tx) => this.revertTransaction(tx)));
    }
    async revertTransaction(tx) {
        await Promise.all([
            this.shielded.revertTransaction(tx),
            this.unshielded.revertTransaction(tx),
            this.dust.revertTransaction(tx),
        ]).then(() => this.pendingTransactionsService.clear(tx));
    }
    async start(shieldedSecretKeys, dustSecretKey) {
        await Promise.all([
            this.shielded.start(shieldedSecretKeys),
            this.unshielded.start(),
            this.dust.start(dustSecretKey),
            this.pendingTransactionsService.start(),
        ]);
    }
    async stop() {
        await Promise.all([
            this.shielded.stop(),
            this.unshielded.stop(),
            this.dust.stop(),
            this.submissionService.close(),
            this.pendingTransactionsService.stop(),
            Promise.resolve(this.#pendingSubscription?.unsubscribe()),
        ]);
    }
    async queryTxHistoryByHash(hash) {
        const raw = await this.#txHistoryStorage.get(hash);
        return raw && isWalletEntry(raw) ? raw : undefined;
    }
    async getAllFromTxHistory() {
        const allEntries = await this.#txHistoryStorage.getAll();
        return [...allEntries];
    }
}

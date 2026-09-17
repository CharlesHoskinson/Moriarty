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
import { Effect, Either, pipe, BigInt as BigIntOps, Iterable as IterableOps, Option } from 'effect';
import { DustActions, DustRegistration, Intent, SignatureEnabled, Transaction, UnshieldedOffer, addressFromKey, nativeToken, } from '@midnight-ntwrk/ledger-v8';
import { OtherWalletError, TransactingError, InsufficientFundsError } from './WalletError.js';
import { LedgerOps } from '@midnight-ntwrk/wallet-sdk-utilities';
import { CoreWallet } from './CoreWallet.js';
import { BindingMarker, ProofMarker, SignatureMarker } from './Utils.js';
import { getBalanceRecipe, Imbalances as CapImbalances, InsufficientFundsError as BalancingInsufficientFundsError, } from '@midnight-ntwrk/wallet-sdk-capabilities';
export const makeDefaultTransactingCapability = (config, getContext) => {
    return new TransactingCapabilityImplementation(config.networkId, config.costParameters, () => getContext().coinSelection, () => getContext().coinsAndBalancesCapability, () => getContext().keysCapability);
};
export const makeSimulatorTransactingCapability = (config, getContext) => {
    return new TransactingCapabilityImplementation(config.networkId, config.costParameters, () => getContext().coinSelection, () => getContext().coinsAndBalancesCapability, () => getContext().keysCapability);
};
/**
 * Distributes the fee across multiple inputs, draining smaller inputs first when the fee exceeds any single input's
 * value. Finds the next available intent segment id in a transaction.
 *
 * Fallible intent segments occupy the range `[1, 65535]`; segment `0` is reserved for the guaranteed section and is
 * never returned.
 *
 * @param transaction - Transaction whose intent map is inspected.
 * @returns `Some(segmentId)` with the lowest unused id, or `None` if all 65535 fallible segments are taken.
 */
export const findAvailableSegmentId = (transaction) => {
    const used = new Set(transaction.intents?.keys() ?? []);
    return pipe(IterableOps.range(1, 65535), IterableOps.findFirst((segmentId) => !used.has(segmentId)));
};
/**
 * Distributes the fee across multiple inputs, draining smaller inputs first when the fee exceeds any single input's
 * value.
 */
const distributeFeeAcrossInputs = (inputs, fee) => inputs.reduce(({ result, remaining }, input) => {
    const deduction = remaining >= input.value ? input.value : remaining;
    return { result: [...result, { ...input, value: deduction }], remaining: remaining - deduction };
}, { result: [], remaining: fee }).result;
export class TransactingCapabilityImplementation {
    networkId;
    costParams;
    getCoinSelection;
    getCoins;
    getKeys;
    constructor(networkId, costParams, getCoinSelection, getCoins, getKeys) {
        this.getCoins = getCoins;
        this.networkId = networkId;
        this.costParams = costParams;
        this.getCoinSelection = getCoinSelection;
        this.getKeys = getKeys;
    }
    createDustGenerationTransaction(currentTime, ttl, nightUtxos, nightVerifyingKey, dustReceiverAddress) {
        const makeOffer = (utxos) => {
            if (utxos.length === 0) {
                return Option.none();
            }
            const totalValue = pipe(utxos, IterableOps.map((coin) => coin.utxo.value), BigIntOps.sumAll);
            const inputs = utxos.map(({ utxo }) => ({
                ...utxo,
                owner: nightVerifyingKey,
            }));
            const output = {
                owner: addressFromKey(nightVerifyingKey),
                type: nativeToken().raw,
                value: totalValue,
            };
            return Option.some(UnshieldedOffer.new(inputs, [output], []));
        };
        return Either.gen(this, function* () {
            const receiver = dustReceiverAddress ? dustReceiverAddress.data : undefined;
            return yield* LedgerOps.ledgerTry(() => {
                const network = this.networkId;
                const splitResult = this.getCoins().splitNightUtxos(nightUtxos);
                // if receiver is `undefined`, it means the coin(s) are being deregistered so the allowFeePayment should not be used
                const feePayment = receiver
                    ? pipe(splitResult.guaranteed, IterableOps.filter((coin) => !coin.utxo.registeredForDustGeneration), IterableOps.map((coin) => coin.dust.generatedNow), BigIntOps.sumAll)
                    : 0n;
                const maybeGuaranteedOffer = makeOffer(splitResult.guaranteed);
                const maybeFallibleOffer = makeOffer(splitResult.fallible);
                const dustRegistration = new DustRegistration(SignatureMarker.signature, nightVerifyingKey, receiver, feePayment);
                const dustActions = new DustActions(SignatureMarker.signature, ProofMarker.preProof, currentTime, [], [dustRegistration]);
                const intent = pipe(Intent.new(ttl), (intent) => Option.match(maybeGuaranteedOffer, {
                    onNone: () => intent,
                    onSome: (guaranteedOffer) => {
                        intent.guaranteedUnshieldedOffer = guaranteedOffer;
                        return intent;
                    },
                }), (intent) => Option.match(maybeFallibleOffer, {
                    onNone: () => intent,
                    onSome: (fallibleOffer) => {
                        intent.fallibleUnshieldedOffer = fallibleOffer;
                        return intent;
                    },
                }), (intent) => {
                    intent.dustActions = dustActions;
                    return intent;
                });
                return Transaction.fromParts(network, undefined, undefined, intent);
            });
        });
    }
    splitNightUtxosForDustRegistration(utxosWithDustValue, isRegistration) {
        const splitResult = this.getCoins().splitNightUtxos(utxosWithDustValue);
        // Deregistration must not claim dust as fee payment (matches the historical
        // `dustReceiverAddress === undefined` branch of createDustGenerationTransaction).
        const feePayment = isRegistration
            ? pipe(splitResult.guaranteed, IterableOps.filter((coin) => !coin.utxo.registeredForDustGeneration), IterableOps.map((coin) => coin.dust.generatedNow), BigIntOps.sumAll)
            : 0n;
        return {
            guaranteedUtxos: splitResult.guaranteed,
            fallibleUtxos: splitResult.fallible,
            feePayment,
        };
    }
    attachDustRegistration(transaction, currentTime, nightVerifyingKey, dustReceiverAddress, feePayment) {
        return Either.gen(this, function* () {
            const intent = transaction.intents?.get(1);
            if (!intent) {
                return yield* Either.left(new TransactingError({
                    message: 'No intent found at segment 1; expected an intent built by rotateUtxos',
                }));
            }
            if (intent.dustActions !== undefined && intent.dustActions.registrations.length > 0) {
                return yield* Either.left(new TransactingError({ message: 'Intent at segment 1 already has a dust registration attached' }));
            }
            return yield* LedgerOps.ledgerTry(() => {
                const receiver = dustReceiverAddress ? dustReceiverAddress.data : undefined;
                const dustRegistration = new DustRegistration(SignatureMarker.signature, nightVerifyingKey, receiver, feePayment);
                const dustActions = new DustActions(SignatureMarker.signature, ProofMarker.preProof, currentTime, [], [dustRegistration]);
                // Intents fetched from `transaction.intents.get(...)` behave like value copies — assigning
                // to a field doesn't propagate back. Match the addDustGenerationSignature pattern: copy the
                // intent via serialize/deserialize, mutate the copy, then set it back into a fresh
                // transaction's intents map.
                const newIntent = Intent.deserialize(SignatureMarker.signature, ProofMarker.preProof, BindingMarker.preBinding, intent.serialize());
                newIntent.dustActions = dustActions;
                const newTransaction = Transaction.deserialize(SignatureMarker.signature, ProofMarker.preProof, BindingMarker.preBinding, transaction.serialize());
                newTransaction.intents = newTransaction.intents.set(1, newIntent);
                return newTransaction;
            });
        });
    }
    addDustRegistrationSignature(transaction, signatureData) {
        return Either.gen(this, function* () {
            const intent = transaction.intents?.get(1);
            if (!intent) {
                return yield* Either.left(new TransactingError({ message: 'No intent found in the transaction intents with segment = 1' }));
            }
            const { dustActions } = intent;
            if (!dustActions) {
                return yield* Either.left(new TransactingError({ message: 'No dustActions found in intent' }));
            }
            const [registration, ...restRegistrations] = dustActions.registrations;
            if (!registration) {
                return yield* Either.left(new TransactingError({ message: 'No registrations found in dustActions' }));
            }
            return yield* LedgerOps.ledgerTry(() => {
                const signature = new SignatureEnabled(signatureData);
                const registrationWithSignature = new DustRegistration(signature.instance, registration.nightKey, registration.dustAddress, registration.allowFeePayment, signature);
                const newDustActions = new DustActions(signature.instance, ProofMarker.preProof, dustActions.ctime, dustActions.spends, [registrationWithSignature, ...restRegistrations]);
                const newIntent = Intent.deserialize(signature.instance, ProofMarker.preProof, BindingMarker.preBinding, intent.serialize());
                newIntent.dustActions = newDustActions;
                const newTransaction = Transaction.deserialize(signature.instance, ProofMarker.preProof, BindingMarker.preBinding, transaction.serialize());
                newTransaction.intents = newTransaction.intents.set(1, newIntent);
                return newTransaction;
            });
        });
    }
    addDustGenerationSignature(transaction, signatureData) {
        return Either.gen(this, function* () {
            const intent = transaction.intents?.get(1);
            if (!intent) {
                return yield* Either.left(new TransactingError({ message: 'No intent found in the transaction intents with segment = 1' }));
            }
            const { dustActions, guaranteedUnshieldedOffer, fallibleUnshieldedOffer } = intent;
            if (!dustActions) {
                return yield* Either.left(new TransactingError({ message: 'No dustActions found in intent' }));
            }
            if (!guaranteedUnshieldedOffer) {
                return yield* Either.left(new TransactingError({ message: 'No guaranteedUnshieldedOffer found in intent' }));
            }
            const [registration, ...restRegistrations] = dustActions.registrations;
            if (!registration) {
                return yield* Either.left(new TransactingError({ message: 'No registrations found in dustActions' }));
            }
            return yield* LedgerOps.ledgerTry(() => {
                const signature = new SignatureEnabled(signatureData);
                const registrationWithSignature = new DustRegistration(signature.instance, registration.nightKey, registration.dustAddress, registration.allowFeePayment, signature);
                const newDustActions = new DustActions(signature.instance, ProofMarker.preProof, dustActions.ctime, dustActions.spends, [registrationWithSignature, ...restRegistrations]);
                // make a copy of intent to avoid mutation
                const newIntent = Intent.deserialize(signature.instance, ProofMarker.preProof, BindingMarker.preBinding, intent.serialize());
                newIntent.dustActions = newDustActions;
                const inputsLen = guaranteedUnshieldedOffer.inputs.length;
                const signatures = [];
                for (let i = 0; i < inputsLen; ++i) {
                    signatures.push(guaranteedUnshieldedOffer.signatures.at(i) ?? signatureData);
                }
                newIntent.guaranteedUnshieldedOffer = guaranteedUnshieldedOffer.addSignatures(signatures);
                if (fallibleUnshieldedOffer) {
                    const inputsLen = fallibleUnshieldedOffer.inputs.length;
                    const signatures = [];
                    for (let i = 0; i < inputsLen; ++i) {
                        signatures.push(fallibleUnshieldedOffer.signatures.at(i) ?? signatureData);
                    }
                    newIntent.fallibleUnshieldedOffer = fallibleUnshieldedOffer.addSignatures(signatures);
                }
                // make a copy of transaction to avoid mutation
                const newTransaction = Transaction.deserialize(signature.instance, ProofMarker.preProof, BindingMarker.preBinding, transaction.serialize());
                newTransaction.intents = newTransaction.intents.set(1, newIntent);
                return newTransaction;
            });
        });
    }
    calculateFee(transaction, ledgerParams) {
        return (transaction.feesWithMargin(ledgerParams, this.costParams.feeBlocksMargin) +
            (this.costParams.additionalFeeOverhead ?? 0n));
    }
    dryRunFee(recipeInputs, transactions, secretKey, state, ttl, currentTime, ledgerParams) {
        const network = this.networkId;
        // Create a balancing tx from recipe inputs without persisting state changes
        const [spends] = CoreWallet.spendCoins(state, secretKey, recipeInputs, currentTime);
        const intent = Intent.new(ttl);
        intent.dustActions = new DustActions(SignatureMarker.signature, ProofMarker.preProof, currentTime, [...spends], []);
        // Merge existing transactions first so we can pick a segment that doesn't collide
        const [first, ...rest] = transactions.map((tx) => tx.eraseProofs());
        const mergedExisting = first ? rest.reduce((acc, tx) => acc.merge(tx), first) : undefined;
        const segmentId = mergedExisting ? Option.getOrElse(findAvailableSegmentId(mergedExisting), () => 1) : 1;
        const balancingTx = Transaction.fromParts(network).addIntent({ tag: 'specific', value: segmentId }, intent);
        const erasedBalancing = balancingTx.eraseProofs();
        const mergedTx = mergedExisting ? mergedExisting.merge(erasedBalancing) : erasedBalancing;
        return this.calculateFee(mergedTx, ledgerParams);
    }
    static feeImbalance(transaction, totalFee) {
        const [_, imbalance] = transaction
            .imbalances(0, totalFee)
            .entries()
            .find(([tt, _]) => tt.tag === 'dust') ?? [];
        return imbalance ?? 0n;
    }
    computeBalancingRecipe(secretKey, state, transactions, ttl, currentTime, ledgerParams) {
        const initialFees = transactions.reduce((total, transaction) => total +
            TransactingCapabilityImplementation.feeImbalance(transaction, this.calculateFee(transaction, ledgerParams)), 0n);
        const dust = this.getCoins().getAvailableCoinsWithGeneratedDust(state, currentTime);
        return pipe(Effect.iterate({ currentFee: initialFees, recipeInputs: [], converged: false }, {
            while: (s) => !s.converged,
            body: ({ currentFee }) => Effect.try({
                try: () => {
                    const recipe = getBalanceRecipe({
                        coins: dust.map((coin) => ({
                            type: 'dust',
                            value: coin.value,
                            token: coin.token,
                        })),
                        initialImbalances: CapImbalances.fromEntry('dust', currentFee),
                        feeTokenType: 'dust',
                        coinSelection: this.getCoinSelection(),
                        transactionCostModel: {
                            inputFeeOverhead: 0n,
                            outputFeeOverhead: 0n,
                        },
                        createOutput: (coin) => coin,
                        isCoinEqual: (a, b) => a.token.nonce === b.token.nonce,
                    });
                    const recipeInputs = recipe.inputs.map(({ token, value }) => ({ token, value }));
                    const newFee = this.dryRunFee(recipeInputs, transactions, secretKey, state, ttl, currentTime, ledgerParams);
                    const recipeAmountCoverage = recipeInputs.reduce((sum, input) => sum + input.value, 0n);
                    return { currentFee: newFee, recipeInputs, converged: newFee <= recipeAmountCoverage };
                },
                catch: (err) => {
                    if (err instanceof BalancingInsufficientFundsError) {
                        return new InsufficientFundsError({
                            message: err.message,
                            tokenType: err.tokenType,
                        });
                    }
                    else {
                        return new OtherWalletError({
                            message: err instanceof Error ? err.message : 'Dust balancing failed',
                            cause: err,
                        });
                    }
                },
            }),
        }), Effect.either, Effect.runSync, Either.map(({ currentFee, recipeInputs }) => ({
            fee: currentFee,
            recipeInputs: distributeFeeAcrossInputs(recipeInputs, currentFee),
        })));
    }
    estimateFee(secretKey, state, transactions, ttl, currentTime, ledgerParams) {
        return pipe(this.computeBalancingRecipe(secretKey, state, transactions, ttl, currentTime, ledgerParams), Either.map(({ fee }) => fee));
    }
    balanceTransactions(secretKey, state, transactions, ttl, currentTime, ledgerParams) {
        const networkId = this.networkId;
        return pipe(this.computeBalancingRecipe(secretKey, state, transactions, ttl, currentTime, ledgerParams), Either.flatMap(({ recipeInputs }) => {
            return LedgerOps.ledgerTry(() => {
                const intent = Intent.new(ttl);
                const [spends, updatedState] = CoreWallet.spendCoins(state, secretKey, recipeInputs, currentTime);
                intent.dustActions = new DustActions(SignatureMarker.signature, ProofMarker.preProof, currentTime, [...spends], []);
                // Merge existing transactions first so we can pick a segment that doesn't collide
                const [first, ...rest] = transactions.map((tx) => tx.eraseProofs());
                const mergedExisting = first ? rest.reduce((acc, tx) => acc.merge(tx), first) : undefined;
                const segmentId = mergedExisting ? Option.getOrElse(findAvailableSegmentId(mergedExisting), () => 1) : 1;
                const feeTransaction = Transaction.fromParts(networkId).addIntent({ tag: 'specific', value: segmentId }, intent);
                return [feeTransaction, updatedState];
            });
        }));
    }
    revertTransaction(state, transaction) {
        return Either.try({
            try: () => CoreWallet.revertTransaction(state, transaction),
            catch: (err) => {
                return new OtherWalletError({
                    message: `Error while reverting transaction ${transaction.identifiers().at(0)}`,
                    cause: err,
                });
            },
        });
    }
}

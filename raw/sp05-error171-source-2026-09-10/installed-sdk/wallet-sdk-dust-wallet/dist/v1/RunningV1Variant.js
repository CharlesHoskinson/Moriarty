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
import { Effect, SubscriptionRef, Stream, pipe, Scope, Sink, Console, Duration, Schedule, Array as Arr } from 'effect';
import { nativeToken, } from '@midnight-ntwrk/ledger-v8';
import { ProtocolVersion } from '@midnight-ntwrk/wallet-sdk-abstractions';
import { OtherWalletError } from './WalletError.js';
import { ArrayOps, EitherOps } from '@midnight-ntwrk/wallet-sdk-utilities';
import { StateChange, VersionChangeType, } from '@midnight-ntwrk/wallet-sdk-runtime/abstractions';
const progress = (state) => {
    const appliedIndex = state.progress?.appliedIndex ?? 0n;
    const highestRelevantWalletIndex = state.progress?.highestRelevantWalletIndex ?? 0n;
    const highestIndex = state.progress?.highestIndex ?? 0n;
    const highestRelevantIndex = state.progress?.highestRelevantIndex ?? 0n;
    const sourceGap = highestIndex - highestRelevantIndex;
    const applyGap = highestRelevantWalletIndex - appliedIndex;
    return [StateChange.ProgressUpdate({ sourceGap, applyGap })];
};
const protocolVersionChange = (previous, current) => {
    return previous.protocolVersion != current.protocolVersion
        ? [
            StateChange.VersionChange({
                change: VersionChangeType.Version({
                    version: ProtocolVersion.ProtocolVersion(current.protocolVersion),
                }),
            }),
        ]
        : [];
};
export const V1Tag = Symbol('V1');
export class RunningV1Variant {
    __polyTag__ = V1Tag;
    #scope;
    #context;
    #v1Context;
    state;
    constructor(scope, context, v1Context) {
        this.#scope = scope;
        this.#context = context;
        this.#v1Context = v1Context;
        this.state = Stream.fromEffect(context.stateRef.get).pipe(Stream.flatMap((initialState) => context.stateRef.changes.pipe(Stream.mapAccum(initialState, (previous, current) => {
            return [current, [previous, current]];
        }))), Stream.mapConcat(([previous, current]) => {
            // TODO: emit progress only upon actual change
            return [
                StateChange.State({ state: current }),
                ...progress(current),
                ...protocolVersionChange(previous, current),
            ];
        }));
    }
    startSyncInBackground(startAux) {
        return this.startSync(startAux).pipe(Stream.runScoped(Sink.drain), Effect.forkScoped, Effect.provideService(Scope.Scope, this.#scope));
    }
    startSync(startAux) {
        return pipe(SubscriptionRef.get(this.#context.stateRef), Stream.fromEffect, Stream.flatMap((state) => this.#v1Context.syncService.updates(state, startAux)), Stream.mapEffect((update) => SubscriptionRef.modifyEffect(this.#context.stateRef, (state) => Effect.try({
            try: () => {
                const [newState, changesResult] = this.#v1Context.syncCapability.applyUpdate(state, update);
                return [changesResult, newState];
            },
            catch: (err) => new OtherWalletError({
                message: 'Error while applying sync update',
                cause: err,
            }),
        })).pipe(Effect.flatMap(({ changes, protocolVersion }) => 
        // Skip the tx-history fork entirely when there are no changes.
        // Forking unconditionally allocates a fiber per apply call (one
        // for every batch the sync emits, even the all-progress ones),
        // which adds up fast during catch-up.
        changes.length === 0
            ? Effect.void
            : pipe(Effect.forEach(changes, (change) => pipe(this.#v1Context.transactionHistoryService.getTransactionDetails(change.source), Effect.flatMap((metadata) => this.#v1Context.transactionHistoryService.put(change, metadata, protocolVersion)), Effect.catchAllCause((cause) => Console.error('Error processing tx history metadata', cause))), { discard: true, concurrency: 'unbounded' }), Effect.forkScoped)), Effect.provideService(Scope.Scope, this.#scope))), Stream.tapError((error) => Console.error(error)), Stream.retry(pipe(Schedule.exponential(Duration.seconds(1), 2), Schedule.map((delay) => {
            const maxDelay = Duration.minutes(2);
            const jitter = Duration.millis(Math.floor(Math.random() * 1000));
            const delayWithJitter = Duration.toMillis(delay) + Duration.toMillis(jitter);
            return Duration.millis(Math.min(delayWithJitter, Duration.toMillis(maxDelay)));
        }))));
    }
    createDustGenerationTransaction(currentTime, ttl, nightUtxos, nightVerifyingKey, dustReceiverAddress) {
        if (nightUtxos.some((utxo) => utxo.type !== nativeToken().raw)) {
            return Effect.fail(new OtherWalletError({ message: 'Token of a non-Night type received' }));
        }
        return Effect.gen(this, function* () {
            const currentState = yield* SubscriptionRef.get(this.#context.stateRef);
            const blockData = yield* this.#v1Context.syncService.blockData();
            const resolvedTime = currentTime ?? blockData.timestamp;
            const utxosWithDustValue = this.#v1Context.coinsAndBalancesCapability.estimateDustGeneration(currentState, nightUtxos, resolvedTime);
            return yield* this.#v1Context.transactingCapability
                .createDustGenerationTransaction(resolvedTime, ttl, utxosWithDustValue, nightVerifyingKey, dustReceiverAddress)
                .pipe(EitherOps.toEffect);
        });
    }
    splitNightUtxosForDustRegistration(currentTime, nightUtxos, isRegistration) {
        if (nightUtxos.some((utxo) => utxo.type !== nativeToken().raw)) {
            return Effect.fail(new OtherWalletError({ message: 'Token of a non-Night type received' }));
        }
        return Effect.gen(this, function* () {
            const currentState = yield* SubscriptionRef.get(this.#context.stateRef);
            const utxosWithDustValue = this.#v1Context.coinsAndBalancesCapability.estimateDustGeneration(currentState, nightUtxos, currentTime);
            return this.#v1Context.transactingCapability.splitNightUtxosForDustRegistration(utxosWithDustValue, isRegistration);
        });
    }
    attachDustRegistration(transaction, currentTime, nightVerifyingKey, dustReceiverAddress, feePayment) {
        return this.#v1Context.transactingCapability
            .attachDustRegistration(transaction, currentTime, nightVerifyingKey, dustReceiverAddress, feePayment)
            .pipe(EitherOps.toEffect);
    }
    addDustGenerationSignature(transaction, signature) {
        return this.#v1Context.transactingCapability
            .addDustGenerationSignature(transaction, signature)
            .pipe(EitherOps.toEffect);
    }
    addDustRegistrationSignature(transaction, signature) {
        return this.#v1Context.transactingCapability
            .addDustRegistrationSignature(transaction, signature)
            .pipe(EitherOps.toEffect);
    }
    calculateFee(transactions) {
        return pipe(this.#v1Context.syncService.blockData(), Effect.map((blockData) => pipe(transactions, Arr.map((transaction) => this.#v1Context.transactingCapability.calculateFee(transaction, blockData.ledgerParameters)), ArrayOps.sumBigInt)));
    }
    estimateFee(secretKey, transactions, ttl, currentTime) {
        return pipe(Effect.all([SubscriptionRef.get(this.#context.stateRef), this.#v1Context.syncService.blockData()]), Effect.flatMap(([state, blockData]) => pipe(this.#v1Context.transactingCapability.estimateFee(secretKey, state, transactions, ttl, currentTime ?? blockData.timestamp, blockData.ledgerParameters), EitherOps.toEffect)));
    }
    balanceTransactions(secretKey, transactions, ttl, currentTime) {
        return SubscriptionRef.modifyEffect(this.#context.stateRef, (state) => {
            return pipe(this.#v1Context.syncService.blockData(), Effect.flatMap((blockData) => this.#v1Context.transactingCapability.balanceTransactions(secretKey, state, transactions, ttl, currentTime ?? blockData.timestamp, blockData.ledgerParameters)));
        });
    }
    revertTransaction(transaction) {
        return SubscriptionRef.updateEffect(this.#context.stateRef, (state) => {
            return pipe(this.#v1Context.transactingCapability.revertTransaction(state, transaction), EitherOps.toEffect);
        });
    }
}

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
import { DustSecretKey, } from '@midnight-ntwrk/ledger-v8';
import { ProtocolVersion } from '@midnight-ntwrk/wallet-sdk-abstractions';
import { WalletBuilder } from '@midnight-ntwrk/wallet-sdk-runtime';
import { Effect, Either } from 'effect';
import * as rx from 'rxjs';
import { CoreWallet } from './v1/CoreWallet.js';
import { V1Tag } from './v1/RunningV1Variant.js';
import { V1Builder } from './v1/V1Builder.js';
export class DustWalletState {
    static mapState = (variant) => (state) => {
        const { serialization, coinsAndBalances, keys } = variant;
        const { transactionHistory } = variant;
        return new DustWalletState(state, { serialization, coinsAndBalances, keys }, { transactionHistory });
    };
    protocolVersion;
    state;
    capabilities;
    services;
    get totalCoins() {
        return this.capabilities.coinsAndBalances.getTotalCoins(this.state);
    }
    get availableCoins() {
        return this.capabilities.coinsAndBalances.getAvailableCoins(this.state);
    }
    get pendingCoins() {
        return this.capabilities.coinsAndBalances.getPendingCoins(this.state);
    }
    get publicKey() {
        return this.capabilities.keys.getPublicKey(this.state);
    }
    get address() {
        return this.capabilities.keys.getAddress(this.state);
    }
    get progress() {
        return this.state.progress;
    }
    constructor(state, capabilities, services) {
        this.protocolVersion = state.version;
        this.state = state.state;
        this.capabilities = capabilities;
        this.services = services;
    }
    balance(time) {
        return this.capabilities.coinsAndBalances.getWalletBalance(this.state, time);
    }
    estimateDustGeneration(nightUtxos, currentTime) {
        return this.capabilities.coinsAndBalances.estimateDustGeneration(this.state, nightUtxos, currentTime);
    }
    serialize() {
        return this.capabilities.serialization.serialize(this.state);
    }
}
export function DustWallet(configuration) {
    return CustomDustWallet(configuration, new V1Builder().withDefaults());
}
export function CustomDustWallet(configuration, builder) {
    const buildArgs = [configuration];
    const BaseWallet = WalletBuilder.init()
        .withVariant(ProtocolVersion.MinSupportedVersion, builder)
        .build(...buildArgs);
    return class CustomDustWalletImplementation extends BaseWallet {
        static startWithSeed(seed, dustParameters) {
            const dustSecretKey = DustSecretKey.fromSeed(seed);
            return CustomDustWalletImplementation.startFirst(CustomDustWalletImplementation, CoreWallet.initEmpty(dustParameters, dustSecretKey, CustomDustWalletImplementation.configuration.networkId));
        }
        static startWithSecretKey(secretKey, dustParameters) {
            return CustomDustWalletImplementation.startFirst(CustomDustWalletImplementation, CoreWallet.initEmpty(dustParameters, secretKey, CustomDustWalletImplementation.configuration.networkId));
        }
        static restore(serializedState) {
            const deserialized = CustomDustWalletImplementation.allVariantsRecord()[V1Tag].variant.deserializeState(serializedState)
                .pipe(Either.getOrThrow);
            return CustomDustWalletImplementation.startFirst(CustomDustWalletImplementation, deserialized);
        }
        state;
        constructor(runtime, scope) {
            super(runtime, scope);
            this.state = this.rawState.pipe(rx.map(DustWalletState.mapState(CustomDustWalletImplementation.allVariantsRecord()[V1Tag].variant)), rx.shareReplay({ refCount: true, bufferSize: 1 }));
        }
        start(secretKey) {
            return this.runtime.dispatch({ [V1Tag]: (v1) => v1.startSyncInBackground(secretKey) }).pipe(Effect.runPromise);
        }
        async createDustGenerationTransaction(currentTime, ttl, nightUtxos, nightVerifyingKey, dustReceiverAddress) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.createDustGenerationTransaction(currentTime, ttl, nightUtxos, nightVerifyingKey, dustReceiverAddress),
            })
                .pipe(Effect.runPromise);
        }
        async splitNightUtxosForDustRegistration(currentTime, nightUtxos, isRegistration) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.splitNightUtxosForDustRegistration(currentTime, nightUtxos, isRegistration),
            })
                .pipe(Effect.runPromise);
        }
        async attachDustRegistration(transaction, currentTime, nightVerifyingKey, dustReceiverAddress, feePayment) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.attachDustRegistration(transaction, currentTime, nightVerifyingKey, dustReceiverAddress, feePayment),
            })
                .pipe(Effect.runPromise);
        }
        addDustGenerationSignature(transaction, signature) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.addDustGenerationSignature(transaction, signature),
            })
                .pipe(Effect.runPromise);
        }
        addDustRegistrationSignature(transaction, signature) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.addDustRegistrationSignature(transaction, signature),
            })
                .pipe(Effect.runPromise);
        }
        calculateFee(transactions) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.calculateFee(transactions),
            })
                .pipe(Effect.runPromise);
        }
        estimateFee(secretKey, transactions, ttl, currentTime) {
            const effectiveTtl = ttl ?? new Date(Date.now() + 60 * 60 * 1000);
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.estimateFee(secretKey, transactions, effectiveTtl, currentTime),
            })
                .pipe(Effect.runPromise);
        }
        balanceTransactions(secretKey, transactions, ttl, currentTime) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.balanceTransactions(secretKey, transactions, ttl, currentTime),
            })
                .pipe(Effect.runPromise);
        }
        revertTransaction(transaction) {
            return this.runtime
                .dispatch({
                [V1Tag]: (v1) => v1.revertTransaction(transaction),
            })
                .pipe(Effect.runPromise);
        }
        waitForSyncedState(allowedGap = 0n) {
            return rx.firstValueFrom(this.state.pipe(rx.filter((state) => state.state.progress.isCompleteWithin(allowedGap))));
        }
        async waitForGeneratedDust(nightUtxos, requiredAmount, clock, opts) {
            if (nightUtxos.length === 0) {
                throw Error('At least one Night UTXO is required.');
            }
            if (requiredAmount <= 0n) {
                return;
            }
            const timeoutMs = opts?.timeoutMs ?? 300_000;
            // Combine the dust state stream with a 1 s tick — the dust state only emits when sync
            // updates apply, but the generation projection depends on a current-time reading, which
            // advances continuously. Without a periodic tick the filter would never re-run between
            // state emissions on a quiet wallet, and the wait would hang.
            await rx.firstValueFrom(rx.combineLatest([this.state, rx.timer(0, 1000)]).pipe(rx.filter(([dustState]) => {
                // The registration's allow_fee_payment is capped at the single highest-generation
                // unregistered Night UTxO (splitNightUtxos puts only 1 UTxO in the guaranteed slot),
                // so the wait must track that same quantity — summing across UTxOs would resolve
                // optimistically and the registration would still fail the fee check.
                const maxGeneratedNow = dustState
                    .estimateDustGeneration(nightUtxos, clock.now())
                    .filter((u) => !u.utxo.registeredForDustGeneration)
                    .reduce((max, u) => (u.dust.generatedNow > max ? u.dust.generatedNow : max), 0n);
                return maxGeneratedNow >= requiredAmount;
            }), rx.timeout({ first: timeoutMs })));
        }
        serializeState() {
            return rx.firstValueFrom(this.state).then((state) => state.serialize());
        }
        getAddress() {
            return rx.firstValueFrom(this.state).then((state) => state.address);
        }
    };
}

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
import { DateOps } from '@midnight-ntwrk/wallet-sdk-utilities';
import { pipe, Array as Arr, Order } from 'effect';
export const chooseCoin = (coins) => coins
    .filter((coin) => coin.value > 0n)
    .toSorted((a, b) => Number(a.value - b.value))
    .at(0);
const FAKE_NONCE = '0'.repeat(64);
export const makeDefaultCoinsAndBalancesCapability = (_config, getContext) => {
    const getWalletBalance = (state, time) => {
        return state.state.walletBalance(time);
    };
    const getGenerationInfo = (state, coin) => {
        const info = state.state.generationInfo(coin);
        return info && info.dtime
            ? {
                ...info,
                dtime: new Date(+info.dtime), // TODO: remove when the ledger start to return a date instead of the number
            }
            : info;
    };
    const resolveTime = (state, time) => time ?? state.state.syncTime;
    const toFullInfo = (state, coins, time) => coins.flatMap((coin) => {
        const genInfo = getGenerationInfo(state, coin);
        return genInfo ? [{ token: coin, ...getFullDustInfo(state.state.params, genInfo, coin, time) }] : [];
    });
    const availableDustTokens = (state) => {
        const pendingSpends = new Set([...state.pendingDust.values()].map((coin) => coin.nonce));
        return pipe(state.state.utxos, Arr.filter((coin) => !pendingSpends.has(coin.nonce)));
    };
    const getAvailableCoins = (state, time) => toFullInfo(state, availableDustTokens(state), resolveTime(state, time));
    const getPendingCoins = (state, time) => toFullInfo(state, state.pendingDust, resolveTime(state, time));
    const getTotalCoins = (state, time) => {
        const effectiveTime = resolveTime(state, time);
        return [...getAvailableCoins(state, effectiveTime), ...getPendingCoins(state, effectiveTime)];
    };
    const getAvailableCoinsWithGeneratedDust = (state, currentTime) => getAvailableCoins(state, currentTime).map((info) => ({ token: info.token, value: info.generatedNow }));
    const getFullDustInfo = (parameters, genInfo, coin, currentTime) => {
        const generatedValue = ledger.updatedValue(coin.ctime, coin.initialValue, genInfo, currentTime, parameters);
        return {
            dtime: genInfo.dtime,
            maxCap: genInfo.value * parameters.nightDustRatio,
            maxCapReachedAt: DateOps.addSeconds(coin.ctime, parameters.timeToCapSeconds),
            generatedNow: generatedValue,
            rate: genInfo.value * parameters.generationDecayRate,
        };
    };
    const estimateDustGeneration = (state, nightUtxos, currentTime) => {
        const dustPublicKey = getContext().keysCapability.getPublicKey(state);
        return pipe(nightUtxos, Arr.map((utxo) => {
            const genInfo = fakeGenerationInfo(utxo, dustPublicKey);
            const fakeDustCoin = fakeDustToken(dustPublicKey, utxo);
            const details = getFullDustInfo(state.state.params, genInfo, fakeDustCoin, currentTime);
            return { utxo, dust: details };
        }));
    };
    /** Create a fake generation info for a given Utxo. It allows to estimate the Dust generation from it */
    const fakeGenerationInfo = (utxo, dustPublicKey) => {
        return {
            value: utxo.value,
            owner: dustPublicKey,
            nonce: FAKE_NONCE,
            dtime: undefined,
        };
    };
    /** Create a fake dust coin for a given Utxo. It allows to estimate full details of the Dust generation from it */
    const fakeDustToken = (dustPublicKey, utxo) => ({
        initialValue: 0n,
        owner: dustPublicKey,
        nonce: 0n,
        seq: 0,
        ctime: utxo.ctime,
        backingNight: '',
        mtIndex: 0n,
    });
    const splitNightUtxos = (utxos) => {
        const [guaranteed, fallible] = pipe(utxos, Arr.sort(pipe(Order.bigint, Order.reverse, Order.mapInput((coin) => coin.dust.generatedNow))), Arr.splitAt(1));
        return { guaranteed, fallible };
    };
    return {
        getWalletBalance,
        getAvailableCoins,
        getPendingCoins,
        getTotalCoins,
        getAvailableCoinsWithGeneratedDust,
        getGenerationInfo,
        estimateDustGeneration,
        splitNightUtxos,
    };
};

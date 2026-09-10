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
import { Effect, Either, Layer, ParseResult, pipe, Schema, Stream, Duration, Chunk, Schedule, } from 'effect';
import { Event as LedgerEvent, LedgerParameters, } from '@midnight-ntwrk/ledger-v8';
import { BlockHash, DustLedgerEvents } from '@midnight-ntwrk/wallet-sdk-indexer-client';
import { WsSubscriptionClient, HttpQueryClient, ConnectionHelper, } from '@midnight-ntwrk/wallet-sdk-indexer-client/effect';
import { EitherOps, LedgerOps } from '@midnight-ntwrk/wallet-sdk-utilities';
import { WsURL } from '@midnight-ntwrk/wallet-sdk-utilities/networking';
import { OtherWalletError, SyncWalletError } from './WalletError.js';
import { getBlockEventsFrom, getLastBlock, } from '@midnight-ntwrk/wallet-sdk-capabilities/simulation';
import { CoreWallet } from './CoreWallet.js';
import { Uint8ArraySchema } from './Serialization.js';
export const SecretKeysResource = {
    create: (secretKey) => {
        return (cb) => {
            const result = cb(secretKey);
            secretKey.clear();
            return result;
        };
    },
};
const LedgerEventSchema = Schema.declare((input) => input instanceof LedgerEvent).annotations({
    identifier: 'ledger.Event',
});
const LedgerEventFromUInt8Array = Schema.asSchema(Schema.transformOrFail(Uint8ArraySchema, LedgerEventSchema, {
    encode: (e) => Effect.try({
        try: () => e.serialize(),
        catch: (err) => new ParseResult.Unexpected(err, 'Could not serialize Ledger Event'),
    }),
    decode: (bytes) => Effect.try({
        try: () => LedgerEvent.deserialize(bytes),
        catch: (err) => new ParseResult.Unexpected(err, 'Could not deserialize Ledger Event'),
    }),
}));
const HexedEvent = pipe(Schema.Uint8ArrayFromHex, Schema.compose(LedgerEventFromUInt8Array));
export const SyncEventsUpdateSchema = Schema.Struct({
    id: Schema.Number,
    raw: HexedEvent,
    maxId: Schema.Number,
});
export const WalletSyncUpdate = {
    create: (updates, secretKey, timestamp) => {
        return {
            updates,
            secretKey,
            timestamp,
        };
    },
};
export const makeDefaultSyncService = (config) => {
    const indexerSyncService = makeIndexerSyncService(config);
    return {
        updates: (state, secretKey) => {
            const batchSize = config.batchUpdates?.size ?? 10;
            const batchTimeout = Duration.millis(config.batchUpdates?.timeout ?? 1);
            const batchSpacing = config.batchUpdates?.spacing ?? 4;
            return pipe(indexerSyncService.subscribeWallet(state), Stream.groupedWithin(batchSize, batchTimeout), Stream.map(Chunk.toArray), Stream.map((data) => WalletSyncUpdate.create(data, secretKey, new Date())), batchSpacing > 0
                ? Stream.schedule(Schedule.spaced(Duration.millis(batchSpacing)))
                : (eventsStream) => eventsStream, Stream.provideSomeLayer(indexerSyncService.connectionLayer()));
        },
        blockData: () => {
            return Effect.gen(function* () {
                const query = yield* BlockHash;
                const result = yield* query({ offset: null });
                return result.block;
            }).pipe(Effect.provide(indexerSyncService.queryClient()), Effect.scoped, Effect.catchAll((err) => Effect.fail(new OtherWalletError({ message: `Encountered unexpected error: ${err.message}`, cause: err }))), Effect.flatMap((blockData) => {
                if (!blockData) {
                    throw new OtherWalletError({ message: 'Unable to fetch block data' });
                }
                // TODO: convert to schema
                return LedgerOps.ledgerTry(() => ({
                    hash: blockData.hash,
                    height: blockData.height,
                    ledgerParameters: LedgerParameters.deserialize(Buffer.from(blockData.ledgerParameters, 'hex')),
                    timestamp: new Date(blockData.timestamp),
                }));
            }));
        },
    };
};
export const makeIndexerSyncService = (config) => {
    return {
        queryClient() {
            return pipe(HttpQueryClient.layer({
                url: config.indexerClientConnection.indexerHttpUrl,
            }), Layer.mapError((error) => new OtherWalletError(error)));
        },
        connectionLayer() {
            const { indexerClientConnection } = config;
            return ConnectionHelper.createWebSocketUrl(indexerClientConnection.indexerHttpUrl, indexerClientConnection.indexerWsUrl).pipe(Either.flatMap((url) => WsURL.make(url)), Either.match({
                onLeft: (error) => Layer.fail(error),
                onRight: (url) => WsSubscriptionClient.layer({ url, keepAlive: indexerClientConnection.keepAlive }),
            }), Layer.mapError((e) => new SyncWalletError({ message: 'Failed to to obtain correct indexer URLs', cause: e })));
        },
        subscribeWallet(state) {
            const { appliedIndex } = state.progress;
            const bufferSize = config.indexerClientConnection.bufferSize ?? 10000;
            const resumeThreshold = config.indexerClientConnection.resumeThreshold ?? 100;
            // The boundary is load-bearing, not waste: this subscription emits only events (no tip/progress
            // sentinel), and `isConnected`/the tip (`maxId`) are set only when an event is received. So the
            // cursor must stay `<= appliedIndex` — never `appliedIndex + 1`. Requesting one event later would
            // deliver nothing to a wallet already at the tip, so `applyUpdate` would never run and sync would
            // hang.
            //
            // A fresh wallet has `appliedIndex === 0n` (the "nothing applied yet" sentinel), so `resumeFrom`
            // is `-1n` and the `variables` mapping below opens the subscription with no `id` — the indexer
            // streams from the very start. A restored wallet has `appliedIndex >= 1`, so `resumeFrom` is
            // `appliedIndex - 1` and the inclusive cursor re-delivers the boundary event.
            const resumeFrom = appliedIndex - 1n;
            return pipe(
            // Backpressure caps the in-flight queue between the WS push and the
            // apply loop. Without it the JS heap grows linearly with catch-up
            // depth, since `Stream.asyncPush({ bufferSize: 'unbounded' })`
            // buffers every event the indexer pushes regardless of apply rate.
            DustLedgerEvents.runWithBackpressure({
                bufferSize,
                resumeThreshold,
                from: resumeFrom,
                // `resumeFrom < 0n` means a fresh wallet: send no `id` so the indexer streams from the very
                // start, rather than relying on `id: 0` sorting below the first real event id.
                variables: (cursor) => ({ id: cursor < 0n ? null : Number(cursor) }),
                key: (r) => BigInt(r.dustLedgerEvents.id),
            }), Stream.mapEffect((subscription) => pipe(Schema.decodeUnknownEither(SyncEventsUpdateSchema)(subscription.dustLedgerEvents), Either.mapLeft((err) => new SyncWalletError(err)), EitherOps.toEffect)), Stream.mapError((error) => new SyncWalletError(error)));
        },
    };
};
export const makeDefaultSyncCapability = () => {
    return {
        applyUpdate(state, wrappedUpdate) {
            const { updates, secretKey } = wrappedUpdate;
            // Nothing to update yet
            if (updates.length === 0) {
                return [state, { changes: [], protocolVersion: Number(state.protocolVersion) }];
            }
            const appliedIndex = state.progress.appliedIndex;
            const freshUpdates = updates.filter((u) => BigInt(u.id) > appliedIndex);
            const highestRelevantWalletIndex = BigInt(updates.at(-1).maxId);
            const [newState, changes] = freshUpdates.length === 0
                ? [state, []]
                : CoreWallet.applyEventsWithChanges(state, secretKey, freshUpdates.map((u) => u.raw), wrappedUpdate.timestamp);
            const updatedState = CoreWallet.updateProgress(newState, {
                appliedIndex: freshUpdates.length === 0 ? appliedIndex : BigInt(freshUpdates.at(-1).id),
                highestRelevantWalletIndex,
                isConnected: true,
            });
            return [updatedState, { changes, protocolVersion: Number(updatedState.protocolVersion) }];
        },
    };
};
export const makeSimulatorSyncService = (config) => {
    return {
        updates: (_state, secretKey) => {
            // Get the initial state immediately to ensure we process the genesis block.
            // Then subscribe to state$ for subsequent changes, but deduplicate by block number
            // to avoid processing the same block twice.
            let lastSeenBlockNumber;
            return pipe(Stream.fromEffect(config.simulator.getLatestState()), Stream.concat(config.simulator.state$), Stream.filter((state) => {
                const lastBlock = getLastBlock(state);
                if (lastBlock === undefined) {
                    return false; // Skip blank state
                }
                const blockNumber = lastBlock.number;
                // Skip if we've already seen this block (deduplication)
                if (lastSeenBlockNumber !== undefined && blockNumber <= lastSeenBlockNumber) {
                    return false;
                }
                lastSeenBlockNumber = blockNumber;
                return true;
            }), Stream.map((state) => ({ update: state, secretKey })));
        },
        blockData: () => {
            return Effect.gen(function* () {
                const state = yield* config.simulator.getLatestState();
                const lastBlock = getLastBlock(state);
                // Use currentTime instead of lastBlock.timestamp for time-sensitive operations
                // (e.g., Dust generation calculation). The currentTime reflects any fast-forwarding
                // that has been done, while lastBlock.timestamp only reflects when the block was produced.
                return {
                    hash: lastBlock.hash,
                    height: Number(lastBlock.number),
                    ledgerParameters: state.ledger.parameters,
                    timestamp: state.currentTime,
                };
            });
        },
    };
};
export const makeSimulatorSyncCapability = () => {
    return {
        applyUpdate: (state, update) => {
            const lastBlock = getLastBlock(update.update);
            // If no block exists yet (blank simulator), skip update
            if (lastBlock === undefined) {
                return [state, { changes: [], protocolVersion: Number(state.protocolVersion) }];
            }
            // Get all events from blocks starting at appliedIndex (the next block to process).
            // appliedIndex semantics: the first block number we haven't processed yet.
            // Initial: appliedIndex = 0 (haven't processed any blocks)
            // After processing block N: appliedIndex = N + 1 (next block to process)
            const events = [...getBlockEventsFrom(update.update, state.progress.appliedIndex)];
            const [newState, changes] = CoreWallet.applyEventsWithChanges(state, update.secretKey, events, lastBlock.timestamp);
            const updatedState = CoreWallet.updateProgress(newState, {
                appliedIndex: lastBlock.number + 1n,
            });
            return [updatedState, { changes, protocolVersion: Number(updatedState.protocolVersion) }];
        },
    };
};

/**
 * Pure parsers for t1-runtime-schemas-71.
 * Pins: Yaci 29e000052f5cec7e1fdf3605ef9605c855a70963,
 * cardano-cli 01a89dad991e5a19990150b4e1de348a1481a37a,
 * cardano-node stamp 97036a66bcf8c89f687ae57a048eecc0389977ef.
 * No I/O. No process start. measured stays false.
 */
export const PINS = {
  yaciCommit: "29e000052f5cec7e1fdf3605ef9605c855a70963",
  yaciTag: "v0.12.0-beta5",
  cliCommit: "01a89dad991e5a19990150b4e1de348a1481a37a",
  cliTag: "cardano-cli-11.0.0.0",
  cliVersion: "11.0.0.0",
  nodeCommit: "97036a66bcf8c89f687ae57a048eecc0389977ef",
  nodeVersion: "11.0.1",
};

const REFUSED_RUN_IDS = new Set(["default", "t01", "moriarty-eutxo-20261012"]);
const REFUSED_PROFILES = new Set([
  "zero_fee",
  "zero_min_utxo_value",
  "zero_fee_and_min_utxo_value",
]);

function fail(code, detail) {
  return {
    ok: false,
    code,
    detail,
    measured: false,
    parityExecuted: false,
    selfApproval: false,
    nativeBudgetMeasured: false,
  };
}

export function parseEraHistoryEnvelope(value) {
  if (value == null || typeof value !== "object" || Array.isArray(value)) {
    return fail("ERA_HISTORY_WIRE_IS_TEXT_ENVELOPE", "expected a text-envelope object");
  }
  if (
    Object.prototype.hasOwnProperty.call(value, "boundSlot") ||
    Object.prototype.hasOwnProperty.call(value, "boundTime")
  ) {
    return fail(
      "ERA_HISTORY_WIRE_IS_TEXT_ENVELOPE",
      "query era-history writes type, description, and cborHex",
    );
  }
  if (value.type !== "EraHistory" || typeof value.description !== "string") {
    return fail("ERA_HISTORY_WIRE_IS_TEXT_ENVELOPE", "type EraHistory and a string description are required");
  }
  if (
    typeof value.cborHex !== "string" ||
    value.cborHex.length === 0 ||
    value.cborHex.length % 2 !== 0 ||
    !/^[0-9a-fA-F]+$/.test(value.cborHex)
  ) {
    return fail("ERA_HISTORY_CBOR_HEX_REQUIRED", "cborHex must be even-length hex");
  }
  return {
    ok: true,
    wire: "cardano-text-envelope",
    type: "EraHistory",
    description: value.description,
    cborHex: value.cborHex,
    cborPrefixByte: value.cborHex.slice(0, 2).toLowerCase(),
    innerCborFieldNames: null,
    gap: "ERA_HISTORY_CBOR_INNER_FIELDS_IN_CARDANO_API_NOT_OPENED",
    measured: false,
    parityExecuted: false,
    evidenceClass: "cli-text-envelope-fixture-not-a-node-receipt",
    selfApproval: false,
  };
}

export function slotLengthMsFromShelley(slotLength) {
  if (typeof slotLength !== "number" || !Number.isFinite(slotLength)) {
    return fail(
      "SHELLEY_SLOT_LENGTH_IS_SECONDS_NUMBER",
      "shelley slotLength is a finite JSON number of seconds",
    );
  }
  return {
    ok: true,
    unitInFile: "seconds",
    slotLengthSeconds: slotLength,
    slotLengthMs: slotLength * 1000,
    evidenceClass: "yaci-template-seconds-times-1000",
    parityExecuted: false,
    ledgerParserConfirmed: false,
    measured: false,
    selfApproval: false,
  };
}

export function planWindows(input) {
  const { systemStart, tipSlot, slotLengthMs, epochLengthSlots } = input ?? {};
  if (slotLengthMs !== 1000) return fail("SLOT_LENGTH_MS_REQUIRED_1000", "witness path requires 1000 ms");
  if (epochLengthSlots !== 3600) return fail("EPOCH_LENGTH_REQUIRED_3600", "witness path requires 3600 slots");
  if (!Number.isInteger(tipSlot) || tipSlot < 0) return fail("TIP_SLOT_REQUIRED", "tipSlot must be a non-negative integer");
  const start = Date.parse(systemStart);
  if (!Number.isFinite(start)) return fail("SYSTEM_START_REQUIRED", "systemStart must be a parseable UTC stamp");
  const marginSlots = 30;
  const windowSlots = 60;
  const horizonMs = 1800000;
  const lookaheadSlots = marginSlots + 2 * windowSlots;
  const lookaheadMs = lookaheadSlots * slotLengthMs;
  if (!(lookaheadMs < horizonMs)) return fail("HORIZON_EXCEEDED", "lookahead must stay inside 1800000 ms");
  const posix = (slot) => start + slot * slotLengthMs;
  const releaseSlot = tipSlot + marginSlots;
  const partitionSlot = releaseSlot + windowSlots;
  const refundExclusiveSlot = partitionSlot + windowSlots;
  return {
    ok: true,
    evidenceClass: "genesis-arithmetic-not-a-node-probe",
    parityExecuted: false,
    measured: false,
    selfApproval: false,
    marginSlots,
    windowSlots,
    horizonMs,
    horizonSeconds: 1800,
    lookaheadSlots,
    lookaheadMs,
    tipSlot,
    tipPlus30Slot: tipSlot + marginSlots,
    tipPlus60Slot: tipSlot + windowSlots,
    releaseSlot,
    partitionSlot,
    refundExclusiveSlot,
    tipPosixMs: posix(tipSlot),
    releaseLowerPosixMs: posix(releaseSlot),
    partitionPosixMs: posix(partitionSlot),
    refundExclusivePosixMs: posix(refundExclusiveSlot),
    epochLengthSlots,
    slotLengthMs,
    systemStart,
    eraHistorySuppliesTheseSlots: false,
  };
}

export function yaciLayout(home, runId) {
  if (typeof home !== "string" || !home.startsWith("/")) {
    return fail("YACI_HOME_MUST_BE_ABSOLUTE", "ClusterConfig exits when yaci.cli.home is not absolute");
  }
  if (home.endsWith("/.yaci-cli") || home.endsWith("/.yaci-cli/")) {
    return fail("DEFAULT_YACI_HOME_REFUSED", "owned home must be a private absolute directory");
  }
  if (home.includes("tooling/runtime/devnet") || home.includes("/local-clusters/default")) {
    return fail("DEVNET_HOME_REFUSED", "owned home must stay off the shared devnet path");
  }
  if (typeof runId !== "string" || !/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/.test(runId)) {
    return fail("RUN_ID_REQUIRED", "run id must be one path segment");
  }
  if (REFUSED_RUN_IDS.has(runId) || runId.includes("t01")) {
    return fail("RUN_ID_REFUSED", runId);
  }
  const cluster = `${home}/local-clusters/${runId}`;
  return {
    ok: true,
    jvmProperty: `-Dyaci.cli.home=${home}`,
    home,
    runId,
    clusterHome: `${home}/local-clusters`,
    clusterFolder: cluster,
    genesisKeysFolder: `${home}/genesis-keys/${runId}`,
    poolKeysFolder: `${home}/pool-keys/${runId}`,
    nodeDir: `${cluster}/node`,
    nodeCwd: `${cluster}/node`,
    shelleyGenesis: `${cluster}/node/genesis/shelley-genesis.json`,
    byronGenesis: `${cluster}/node/genesis/byron-genesis.json`,
    alonzoGenesis: `${cluster}/node/genesis/alonzo-genesis.json`,
    conwayGenesis: `${cluster}/node/genesis/conway-genesis.json`,
    dijkstraGenesis: `${cluster}/node/genesis/dijkstra-genesis.json`,
    configuration: `${cluster}/node/configuration.json`,
    socketRelative: "node.sock",
    socketPath: `${cluster}/node/node.sock`,
    clusterInfo: `${cluster}/cluster-info.json`,
    utxoKeysDir: `${cluster}/utxo-keys`,
    networkMagicSource: "generated-shelley-genesis-networkMagic",
    javaDefaultProtocolMagic: 42,
    packagedNodePropertyOverridesMagic: false,
    systemStartCreateTimeLiteral: "2022-09-15T04:09:11.577484Z",
    systemStartAfterFirstStart: "read-node-genesis-shelley-systemStart",
    startFalseSkipsRewrite: true,
    portFieldInConfigurationJson: false,
    portWriter: "node.sh --port substitution",
    submitApiPortWriter: "submit-api.sh SUBMIT_API_PORT",
    measured: false,
    executed: false,
    selfApproval: false,
  };
}

export function freshFaucetKeys(home, runId) {
  const layout = yaciLayout(home, runId);
  if (!layout.ok) return layout;
  const keys = [1, 2, 3].map((n) => ({
    index: n,
    sourceVkey: `${layout.genesisKeysFolder}/utxo-keys/shelley.00${n - 1}.vkey`,
    sourceSkey: `${layout.genesisKeysFolder}/utxo-keys/shelley.00${n - 1}.skey`,
    clusterVkey: `${layout.utxoKeysDir}/utxo${n}.vkey`,
    clusterSkey: `${layout.utxoKeysDir}/utxo${n}.skey`,
  }));
  return {
    ok: true,
    nGenesisUtxoKeys: 3,
    generateNewKeys: true,
    defaultKeysPathUsed: false,
    keys,
    faucetVkey: keys[0].clusterVkey,
    addressArgv: [
      "cardano-cli",
      "legacy",
      "genesis",
      "initial-addr",
      "--verification-key-file",
      keys[0].clusterVkey,
    ],
    fundedInNodeInitialFunds: false,
    code: "FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS",
    measured: false,
    executed: false,
    selfApproval: false,
  };
}

export function portMapping(nodePort, submitApiPort) {
  if (!Number.isInteger(nodePort) || !Number.isInteger(submitApiPort)) {
    return fail("PORTS_MUST_BE_RECHECKED_INTEGERS", "caller supplies already rechecked ports");
  }
  return {
    ok: true,
    nodePort,
    submitApiPort,
    nodeScriptFlag: "--port",
    configurationJsonContainsPort: false,
    helpTextSaysCreateOptionOnly: true,
    writerStillSubstitutes: true,
    code: "YACI_PORT_HELP_SAYS_CREATE_OPTION_ONLY",
    reservedByThisPacket: false,
    measured: false,
    executed: false,
    selfApproval: false,
  };
}

export function createNodeArgvShape(home, runId, nodePort, submitApiPort) {
  const layout = yaciLayout(home, runId);
  if (!layout.ok) return layout;
  const ports = portMapping(nodePort, submitApiPort);
  if (!ports.ok) return ports;
  return {
    ok: true,
    executed: false,
    generateNewKeys: true,
    start: false,
    slotLengthFlagSeconds: 1,
    epochLengthSlots: 3600,
    era: "conway",
    refusedProfiles: [...REFUSED_PROFILES],
    jvmProperty: layout.jvmProperty,
    argv: [
      "create-node",
      "--name",
      runId,
      "--overwrite",
      "true",
      "--slot-length",
      "1",
      "--block-time",
      "1",
      "--epoch-length",
      "3600",
      "--era",
      "conway",
      "--generate-new-keys",
      "true",
      "--start",
      "false",
      "--port",
      String(nodePort),
      "--submit-api-port",
      String(submitApiPort),
    ],
    selfApproval: false,
  };
}

export function parseProtocolParams(value) {
  if (value == null || typeof value !== "object" || Array.isArray(value)) {
    return fail("PROTOCOL_PARAMS_OBJECT_REQUIRED", "expected a JSON object");
  }
  if (Object.prototype.hasOwnProperty.call(value, "bppCoinsPerUTxOByte")) {
    return fail("BPP_COINS_PER_UTXO_BYTE_ABSENT_FROM_CLI_11", "CLI 11 protocol fixture uses utxoCostPerByte");
  }
  if (!Object.prototype.hasOwnProperty.call(value, "utxoCostPerByte")) {
    if (Object.prototype.hasOwnProperty.call(value, "coinsPerUTxOByte")) {
      return fail("PROTOCOL_FIELD_IS_UTXO_COST_PER_BYTE", "the protocol input name is utxoCostPerByte");
    }
    return fail("UTXO_COST_PER_BYTE_REQUIRED", "utxoCostPerByte is required");
  }
  const units = value.maxTxExecutionUnits;
  const rate = value.utxoCostPerByte;
  const mem = units && units.memory;
  const steps = units && units.steps;
  const maxTxSize = value.maxTxSize;
  if (![rate, mem, steps, maxTxSize].every((n) => Number.isInteger(n))) {
    return fail(
      "PROTOCOL_INTEGER_FIELDS_REQUIRED",
      "utxoCostPerByte, maxTxExecutionUnits.memory, maxTxExecutionUnits.steps, and maxTxSize must be integers",
    );
  }
  return {
    ok: true,
    utxoCostPerByte: rate,
    maxTxExecutionUnits: { memory: mem, steps },
    maxTxSize,
    nativeBudgetMeasured: false,
    measured: false,
    source: "protocol-params-json",
    evidenceClass: "schema-parse-not-a-node-query",
    selfApproval: false,
  };
}

export function parsePlutusScriptCost(value) {
  if (
    value &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    value.memory === 1 &&
    value.steps === 1
  ) {
    return fail("SUPPLIED_UNITS_NOT_A_MEASUREMENT", "1/1 is a caller label");
  }
  if (!Array.isArray(value)) {
    return fail("PLUTUS_COST_ARRAY_REQUIRED", "the writer encodes a list of ScriptCostOutput");
  }
  const items = [];
  for (const item of value) {
    if (!item || typeof item.scriptHash !== "string" || item.scriptHash.length === 0) {
      return fail("SCRIPT_HASH_REQUIRED", "scriptHash string is required");
    }
    const eu = item.executionUnits;
    if (!eu || !Number.isInteger(eu.memory) || !Number.isInteger(eu.steps)) {
      return fail("EXECUTION_UNITS_REQUIRED", "executionUnits.memory and executionUnits.steps must be integers");
    }
    if (!Number.isInteger(item.lovelaceCost)) {
      return fail("LOVELACE_COST_REQUIRED", "lovelaceCost must be an integer");
    }
    items.push({
      scriptHash: item.scriptHash,
      executionUnits: { memory: eu.memory, steps: eu.steps },
      lovelaceCost: item.lovelaceCost,
    });
  }
  return {
    ok: true,
    items,
    measured: false,
    executionSource: null,
    nativeBudgetMeasured: false,
    evidenceClass: "schema-parse-not-a-node-query",
    selfApproval: false,
  };
}

export function parseMinFee(value) {
  if (typeof value === "string") {
    const matched = value.match(/^(\d+) Lovelace\n?$/);
    if (!matched) return fail("MIN_FEE_TEXT_SHAPE", "text form is digits followed by Lovelace");
    return {
      ok: true,
      fee: Number(matched[1]),
      format: "text",
      txBytes: null,
      code: "TX_BYTES_NOT_A_COMMAND_FIELD",
      measured: false,
      selfApproval: false,
    };
  }
  if (!value || typeof value !== "object" || Array.isArray(value) || !Number.isInteger(value.fee)) {
    return fail("MIN_FEE_JSON_SHAPE", "JSON form is an object with integer fee");
  }
  if (Object.prototype.hasOwnProperty.call(value, "txBytes")) {
    return fail("TX_BYTES_NOT_A_COMMAND_FIELD", "calculate-min-fee emits fee");
  }
  return {
    ok: true,
    fee: value.fee,
    format: "json",
    txBytes: null,
    measured: false,
    evidenceClass: "schema-parse-not-a-measurement",
    selfApproval: false,
  };
}

export function refuseCallerMinAdaFormula(rate, txBytes) {
  return fail(
    "MIN_ADA_FORMULA_IS_NOT_THE_CLI_COMMAND",
    `caller label rate*(160+txBytes) with rate=${rate} txBytes=${txBytes}; CLI command is calculate-min-required-utxo`,
  );
}

export function parseTip(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return fail("TIP_OBJECT_REQUIRED", "query tip JSON is an object");
  }
  const hasSlot = Object.prototype.hasOwnProperty.call(value, "slot");
  const hasHash = Object.prototype.hasOwnProperty.call(value, "hash");
  const hasBlock = Object.prototype.hasOwnProperty.call(value, "block");
  if (!hasSlot && !hasHash && !hasBlock) {
    return {
      ok: true,
      kind: "ChainTipAtGenesis",
      slot: null,
      hash: null,
      block: null,
      measured: false,
      parityExecuted: false,
      selfApproval: false,
    };
  }
  if (!(hasSlot && hasHash && hasBlock)) {
    return fail("TIP_SLOT_HASH_BLOCK_ALL_OR_NONE", "ChainTip requires slot, hash, and block together");
  }
  if (!Number.isInteger(value.slot) || typeof value.hash !== "string" || !Number.isInteger(value.block)) {
    return fail("TIP_FIELD_TYPES", "slot and block are integers and hash is a string");
  }
  return {
    ok: true,
    kind: "ChainTip",
    slot: value.slot,
    hash: value.hash,
    block: value.block,
    era: value.era ?? null,
    epoch: value.epoch ?? null,
    slotInEpoch: value.slotInEpoch ?? null,
    slotsToEpochEnd: value.slotsToEpochEnd ?? null,
    syncProgress: value.syncProgress ?? null,
    measured: false,
    parityExecuted: false,
    evidenceClass: "schema-parse-not-a-node-query",
    selfApproval: false,
  };
}

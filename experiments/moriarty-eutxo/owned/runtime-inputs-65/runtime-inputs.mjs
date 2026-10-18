/**
 * Command shapes for the existing native-runtime caller.
 * executeNow is always false. This module does not spawn, bind, sign, or submit.
 */
import path from "node:path";

export const FORBIDDEN_PID = 3095908;

export const PATHS = {
  cli: "/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli",
  node: "/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-node",
  yaci: "/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@bloxbean/yaci-devkit-linux-x64/yaci-cli",
  yaciConfig: "/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@bloxbean/yaci-devkit-linux-x64/config",
  exampleHome: "/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-inputs-65/examples/owned-home-NOT-CREATED",
};

const REFUSED_RUN_IDS = new Set(["default", "t01", "moriarty-eutxo-20261012"]);
const RUN_ID = /^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/;
const TX_IN = /^[0-9a-f]{64}#\d+$/;
const STAMP = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/;
const SEED_REF = "current compiled script, supplied at the audited run";

export function pins() {
  return {
    cliVersion: "11.0.0.0",
    cliGit: "97036a66bcf8c89f687ae57a048eecc0389977ef",
    cliSha256: "4f9d8b7de9886b0a63b933a7ad299568415f92d7544ab9148b16e44824b839dd",
    nodeVersion: "11.0.1",
    nodeGit: "97036a66bcf8c89f687ae57a048eecc0389977ef",
    nodeSha256: "8ba95b65c2905e0049057a76e1b99e4ad5456e8c834346a8ff08dc4ba7e67eba",
    yaciTag: "0.12.0-beta5",
    yaciBannerCommit: "29e0000",
    yaciSha256: "b9fcc164a777e906e101e9c5dd95536b4307237c9544bd31068cbf08cbd5061c",
    yaciVersionCommand: false,
    slotLengthMs: 1000,
    yaciSlotLengthSeconds: 1,
    yaciBlockTimeSeconds: 1,
    yaciEpochLengthSlots: 3600,
    yaciEpochLengthHelpDefault: 600,
    marginSlots: 30,
    windowSlots: 60,
    horizonMs: 1800000,
    lookaheadSlots: 150,
    cashQuantity: 100,
    unitPayout: 2000000,
    suppliedMinAdaFormula: "4310*(160+100)",
    suppliedMinAdaLovelace: 1120600,
    nativeBudgetMeasured: false,
    forbiddenPid: FORBIDDEN_PID,
    selfApproval: false,
  };
}

function closed(row) {
  const out = { ...row };
  out.executeNow = false;
  out.ledgerExecution = false;
  out.nativePins = false;
  out.selfApproval = false;
  return out;
}

function fail(code, extra = {}) {
  return closed({
    ok: false,
    code,
    argv: null,
    generatesKeys: false,
    readyToSubmit: false,
    ...extra,
    ok: false,
    argv: null,
  });
}

function pass(code, extra = {}) {
  return closed({ ok: true, code, readyToSubmit: false, generatesKeys: false, ...extra });
}

function inside(child, home) {
  if (typeof child !== "string" || typeof home !== "string") return false;
  if (!path.isAbsolute(child) || !path.isAbsolute(home)) return false;
  const rel = path.relative(home, child);
  return rel !== "" && !rel.startsWith("..") && !path.isAbsolute(rel);
}

function sharedHome(home) {
  const n = home.replace(/\/+$/, "");
  if (n.endsWith("/.yaci-cli")) return true;
  if (n.includes(`${path.sep}local-clusters${path.sep}default`)) return true;
  if (n.includes("moriarty-eutxo-20261012")) return true;
  if (n.split(path.sep).includes("t01")) return true;
  if (n.endsWith(`${path.sep}tooling${path.sep}runtime${path.sep}devnet`)) return true;
  return false;
}

function defaultKeys(file) {
  return typeof file === "string" && file.split(/[/\\]/).includes("default-keys");
}

function magicArgs(magic, magicSource) {
  if (magicSource !== "shelley-networkMagic") return fail("MAGIC_SOURCE_REFUSED");
  if (!Number.isInteger(magic) || magic < 0) return fail("MAGIC_SOURCE_REFUSED");
  return ["--testnet-magic", String(magic)];
}

function requireCli(cli) {
  if (typeof cli !== "string" || path.basename(cli) !== "cardano-cli" || !path.isAbsolute(cli)) {
    return fail("CLI_REQUIRED");
  }
  return null;
}

function requireSocket(socketPath, ownedHome) {
  if (!path.isAbsolute(ownedHome)) return fail("OWNED_HOME_REQUIRED");
  if (!inside(socketPath, ownedHome)) return fail("SOCKET_NOT_OWNED");
  return null;
}

function keyModeCode(mode) {
  if (mode == null) return "KEY_MODE_UNVERIFIED";
  if (mode !== 0o600 && mode !== "0600") return "KEY_MODE_REFUSED";
  return null;
}

export function yaciCreateNodeArgv(input) {
  return buildYaci(input, false);
}

export function futureStartArgv(input) {
  if (input.executeNow === true) return fail("START_NOT_IN_THIS_PACKET");
  const built = buildYaci({ ...input, start: false, executeNow: false }, true);
  if (!built.ok) return built;
  return pass("FUTURE_START_AFTER_AUDITS", {
    ...built,
    generatesKeys: true,
    executeAfterTwoAudits: true,
  });
}

function buildYaci(input, start) {
  if (input.executeNow === true || (input.start === true && !start)) {
    return fail("START_NOT_IN_THIS_PACKET");
  }
  const bin = input.yaciBin;
  if (typeof bin === "string" && bin.includes("start.mjs")) return fail("YACI_WRAPPER_REFUSED");
  if (typeof bin !== "string" || path.basename(bin) !== "yaci-cli" || !path.isAbsolute(bin)) {
    return fail("YACI_BINARY_REQUIRED");
  }
  const home = input.ownedHome;
  if (typeof home !== "string" || !path.isAbsolute(home)) return fail("OWNED_HOME_REQUIRED");
  if (sharedHome(home)) return fail("YACI_WRAPPER_REFUSED");
  const config = input.packagedConfigDir;
  if (typeof config !== "string" || !path.isAbsolute(config)) return fail("PACKAGED_CONFIG_REQUIRED");
  if (typeof input.runId !== "string" || !RUN_ID.test(input.runId) || REFUSED_RUN_IDS.has(input.runId)) {
    return fail("RUN_ID_REQUIRED");
  }
  const nodePort = input.nodePort;
  const submitPort = input.submitApiPort;
  if (!Number.isInteger(nodePort) || !Number.isInteger(submitPort) || nodePort < 1 || submitPort < 1 || nodePort > 65535 || submitPort > 65535) {
    return fail("PORT_RECHECK_REQUIRED");
  }
  if (nodePort === submitPort) return fail("PORT_COLLISION");
  if (String(input.slotLengthSeconds) !== "1") return fail("SLOT_LENGTH_REFUSED");
  if (String(input.blockTimeSeconds) !== "1") return fail("BLOCK_TIME_REFUSED");
  if (String(input.epochLengthSlots) !== "3600") return fail("EPOCH_LENGTH_REFUSED", { helpDefault: 600 });
  if (input.era !== "conway") return fail("ERA_REFUSED");
  if (input.generateNewKeys !== true || input.overwrite !== true) return fail("FRESH_KEYS_REQUIRED");
  if (input.genesisProfile != null) return fail("GENESIS_PROFILE_REFUSED");
  if (input.enableMultiNode === true) return fail("MULTI_NODE_REFUSED");
  const spring = [
    "optional:file:" + path.join(config, "application.properties"),
    "optional:file:" + path.join(config, "download.properties"),
    "optional:file:" + path.join(config, "node.properties"),
  ].join(",");
  const argv = [
    bin,
    `-Dyaci.cli.home=${home}`,
    `-Dspring.config.import=${spring}`,
    "create-node",
    "--name", input.runId,
    "--overwrite", "true",
    "--slot-length", "1",
    "--block-time", "1",
    "--epoch-length", "3600",
    "--era", "conway",
    "--generate-new-keys", "true",
    "--start", start ? "true" : "false",
    "--port", String(nodePort),
    "--submit-api-port", String(submitPort),
  ];
  if (argv.includes("create-cluster") || argv.includes("start.mjs")) return fail("YACI_WRAPPER_REFUSED");
  return pass(start ? "FUTURE_START_AFTER_AUDITS" : "CREATE_NODE_ARGV", {
    argv,
    generatesKeys: true,
    executeAfterTwoAudits: start,
    portsReserved: false,
    recheckBeforeLaunch: true,
    outputs: yaciOutputSchema(),
  });
}

function yaciOutputSchema() {
  return {
    notedAs: "help-and-binary-path-strings-not-an-executed-bootstrap",
    helpListsFilenames: false,
    helpEpochDefault: 600,
    passedEpochLength: 3600,
    slotLengthFlagUnit: "seconds",
    slotLengthFlag: 1,
    requiredMeasuredSlotLengthMs: 1000,
    writtenSlotUnit: "YACI_WRITTEN_SLOT_UNIT_UNCONFIRMED",
    networkMagic: null,
    networkMagicSourceAfterCreate: "shelley-networkMagic",
    binaryPathStrings: [
      "cluster-info.json",
      "node.sock",
      "configuration.json",
      "shelley-genesis.json",
      "alonzo-genesis.json",
      "conway-genesis.json",
      "byron-genesis.json",
    ],
    absentFromBeta5Binary: ["proof.txt", "pids/", "yaci-cli.pid", "node.socket", "genesis create-cardano"],
    refuseWhenGenesisIs: ["missing", "empty-object", "placeholder.txt"],
    faucet: {
      code: "GENERATED_FAUCET_KEY_DIR_UNLISTED",
      helpText: "Generate new genesis keys, pool keys instead of default keys",
      flagDefault: false,
      binaryListsOnly: "localcluster/templates/devnet/default-keys/utxo-keys/utxo1.skey",
      callerUsesAfterCreate: [
        "record the key path the generator writes",
        "cardano-cli legacy genesis initial-addr --verification-key-file --testnet-magic",
        "cardano-cli query utxo --address of that address",
      ],
    },
  };
}

export function portClaim() {
  return pass("PORT_RECHECK_REQUIRED", {
    reserved: false,
    recheckBeforeLaunch: true,
    helpDefaults: { nodePort: 3001, submitApiPort: 8090 },
  });
}

export function rejectLauncher(argv) {
  const text = Array.isArray(argv) ? argv.join(" ") : String(argv);
  if (text.includes("pkill")) return fail("GLOBAL_SIGNAL_REFUSED");
  if (text.includes("start.mjs") || text.includes(".yaci-cli/yaci-cli.pid")) return fail("YACI_WRAPPER_REFUSED");
  return pass("LAUNCHER_NOT_WRAPPER");
}

export function assessReadiness(obs) {
  const shape = readinessShape(obs);
  if (shape.code === "FORBIDDEN_PID") return shape;
  if (obs.allowRealNode !== true) {
    return closed({
      ...shape,
      ok: false,
      ready: false,
      code: "REAL_NODE_NOT_ARMED",
      realQuery: "REAL_QUERY_NOT_ARMED",
      shapeCode: shape.code,
      evidenceClass: "readiness-predicate-not-a-node",
    });
  }
  return shape;
}

function readinessShape(obs) {
  const base = { ready: false, evidenceClass: "readiness-predicate-not-a-node", realNodeStarted: false };
  if (obs.pid === FORBIDDEN_PID || obs.pgid === FORBIDDEN_PID) return fail("FORBIDDEN_PID", base);
  if (obs.proofTxtOnly === true) return fail("PROOF_TXT_NOT_READINESS", base);
  if (typeof obs.ownedHome !== "string" || !path.isAbsolute(obs.ownedHome)) return fail("OWNED_HOME_REQUIRED", base);
  if (!Number.isInteger(obs.pid) || !Number.isInteger(obs.pgid)) return fail("OWNED_PID_REQUIRED", base);
  if (obs.pgid === obs.ownPgid) return fail("PROCESS_GROUP_NOT_OWNED", base);
  if (!inside(obs.socketPath, obs.ownedHome)) return fail("SOCKET_NOT_OWNED", base);
  const tip = typeof obs.tip === "string" ? parseTipJson(obs.tip) : parseTipJson(JSON.stringify(obs.tip ?? {}));
  if (tip.code !== "TIP_PARSED") return fail(tip.code, base);
  if (!obs.runId || obs.transcriptRunId !== obs.runId) return fail("RUN_ID_REQUIRED", base);
  if (obs.protocolQueried !== true) return fail("PROTOCOL_QUERY_REQUIRED", base);
  return pass("READY_TIP_TAGGED", { ...base, ready: true, slot: tip.slot, realNodeStarted: false });
}

export function parseTipJson(text) {
  const raw = typeof text === "string" ? text : JSON.stringify(text);
  if (/slotLengthMs=|lucidPosix=|cliPosix=|scriptContextLower=|tipSlot=/.test(raw)) {
    return fail("PRIVATE_TEXT_REFUSED");
  }
  let obj;
  try {
    obj = JSON.parse(raw);
  } catch {
    return fail("PARITY_NOT_MEASURED");
  }
  if (obj == null || typeof obj.slot !== "number" || !Number.isFinite(obj.slot)) return fail("PARITY_NOT_MEASURED");
  return pass("TIP_PARSED", { slot: obj.slot });
}

export function slotLengthMsFromGenesis(shelley) {
  const unit = shelley?.slotLengthUnit ?? shelley?.slotLength?.unit ?? null;
  const value = shelley?.slotLength != null && typeof shelley.slotLength === "object"
    ? shelley.slotLength.value
    : shelley?.slotLength;
  if (unit == null) return fail("SLOT_LENGTH_REFUSED", { gap: "YACI_WRITTEN_SLOT_UNIT_UNCONFIRMED" });
  if (unit === "seconds" && Number(value) === 1) return pass("SLOT_LENGTH_MS", { slotLengthMs: 1000 });
  if (unit === "milliseconds" && Number(value) === 1000) return pass("SLOT_LENGTH_MS", { slotLengthMs: 1000 });
  return fail("SLOT_LENGTH_REFUSED");
}

export function inspectGenesis(value) {
  if (value == null || value === "{}" || (typeof value === "string" && value.trim() === "{}")) {
    return fail("EMPTY_GENESIS");
  }
  let obj = value;
  if (typeof value === "string") {
    try {
      obj = JSON.parse(value);
    } catch {
      return fail("EMPTY_GENESIS");
    }
  }
  if (obj == null || typeof obj !== "object" || Object.keys(obj).length === 0) return fail("EMPTY_GENESIS");
  for (const key of ["slotLength", "epochLength", "systemStart", "networkMagic"]) {
    if (obj[key] == null) return fail("GENESIS_FIELDS_MISSING", { missing: key });
  }
  if (Number(obj.epochLength) !== 3600) return fail("EPOCH_LENGTH_REFUSED");
  const slot = slotLengthMsFromGenesis(obj);
  if (!slot.ok) return slot;
  return pass("GENESIS_FIELDS_PRESENT", {
    slotLengthMs: slot.slotLengthMs,
    networkMagic: obj.networkMagic,
    bootstrapped: false,
    evidenceClass: "inspected-object-not-an-executed-node",
  });
}

export function assessGeneratedLayout(listing) {
  const required = [
    "genesis/shelley-genesis.json",
    "genesis/alonzo-genesis.json",
    "genesis/conway-genesis.json",
    "genesis/byron-genesis.json",
  ];
  const files = listing?.files ?? {};
  for (const name of required) {
    const state = files[name];
    if (state == null || state === "missing" || state === "{}" || state === "empty-object" || state === "placeholder") {
      return fail("EMPTY_GENESIS", { file: name, bootstrapped: false });
    }
  }
  return pass("GENESIS_NOT_EXECUTED", { bootstrapped: false, evidenceClass: "layout-check-not-an-executed-node" });
}

export function assessNodeConfig(config, bodies = {}) {
  for (const key of ["ShelleyGenesisFile", "AlonzoGenesisFile", "ConwayGenesisFile", "ByronGenesisFile"]) {
    if (typeof config?.[key] !== "string" || config[key].length === 0) return fail("EMPTY_GENESIS", { missing: key });
  }
  for (const [name, body] of Object.entries(bodies)) {
    if (body === "{}" || (body && typeof body === "object" && Object.keys(body).length === 0)) {
      return fail("EMPTY_GENESIS", { file: name });
    }
  }
  return pass("GENESIS_PATHS_NAMED", { bootstrapped: false });
}

export function posixFromGenesis({ systemStart, slot, slotLengthMs }) {
  if (slotLengthMs !== 1000) return fail("SLOT_LENGTH_REFUSED");
  if (typeof systemStart !== "string" || !STAMP.test(systemStart)) return fail("SYSTEM_START_REQUIRED");
  const start = Date.parse(systemStart);
  if (!Number.isFinite(start)) return fail("SYSTEM_START_REQUIRED");
  return pass("GENESIS_ARITHMETIC", {
    posixMs: start + slot * 1000,
    parityExecuted: false,
    evidenceClass: "genesis-arithmetic-not-a-node-probe",
  });
}

export function horizonCheck({ lookaheadSlots, slotLengthMs, horizonMs }) {
  if (lookaheadSlots * slotLengthMs >= horizonMs) return fail("HORIZON_EXCEEDED");
  return pass("HORIZON_OK");
}

export function planWindows({ systemStart, tipSlot, slotLengthMs = 1000 }) {
  if (slotLengthMs !== 1000) return fail("SLOT_LENGTH_REFUSED");
  const margin = 30;
  const window = 60;
  const horizonMs = 1800 * 1000;
  const lookaheadSlots = margin + 2 * window;
  const horizon = horizonCheck({ lookaheadSlots, slotLengthMs, horizonMs });
  if (!horizon.ok) return horizon;
  const start = posixFromGenesis({ systemStart, slot: 0, slotLengthMs });
  if (!start.ok) return start;
  const at = (slot) => start.posixMs + slot * slotLengthMs;
  const releaseSlot = tipSlot + margin;
  const partitionSlot = releaseSlot + window;
  const refundExclusiveSlot = partitionSlot + window;
  const releaseLower = at(releaseSlot);
  return pass("GENESIS_ARITHMETIC", {
    evidenceClass: "genesis-arithmetic-not-a-node-probe",
    parityExecuted: false,
    gap: "POSIX_PARITY_NOT_RUN",
    lucidPosix: releaseLower,
    cliPosix: releaseLower,
    scriptContextLower: releaseLower,
    tipPosix: at(tipSlot),
    releaseSlot,
    releaseLower,
    partitionSlot,
    partitionPosix: at(partitionSlot),
    refundExclusiveSlot,
    refundExclusivePosix: at(refundExclusiveSlot),
    lookaheadSlots,
    horizonMs,
  });
}

export function interpretEraHistory(value) {
  const found = [];
  walkBounds(value, found);
  if (found.length === 0) {
    return fail("ERA_HISTORY_WIRE_KEYS_UNCONFIRMED", { parityExecuted: false, gap: "POSIX_PARITY_NOT_RUN" });
  }
  return pass("ERA_BOUNDS_PRESENT", {
    bounds: found,
    parityExecuted: false,
    evidenceClass: "bounds-extracted-not-a-v3-probe",
    gap: "POSIX_PARITY_NOT_RUN",
  });
}

function walkBounds(value, found) {
  if (Array.isArray(value)) {
    for (const item of value) walkBounds(item, found);
    return;
  }
  if (value == null || typeof value !== "object") return;
  if ("boundSlot" in value && "boundTime" in value) {
    found.push({ boundSlot: value.boundSlot, boundTime: value.boundTime });
  }
  for (const item of Object.values(value)) walkBounds(item, found);
}

export function querySlotNumberArgv(input) {
  const gate = queryGate(input);
  if (gate) return gate;
  if (!STAMP.test(input.timestamp)) return fail("SYSTEM_START_REQUIRED");
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  return pass(input.allowRealNode === true ? "QUERY_ARGV" : "REAL_QUERY_NOT_ARMED", {
    argv: [input.cli, "query", "slot-number", "--socket-path", input.socketPath, ...magic, "--volatile-tip", input.timestamp],
    epochSlotsPassed: false,
    executed: false,
  });
}

function queryGate(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  return requireSocket(input.socketPath, input.ownedHome);
}

function queryBase(input, args, purpose) {
  const gate = queryGate(input);
  if (gate) return gate;
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  const argv = [input.cli, ...args, "--socket-path", input.socketPath, ...magic];
  if (argv.includes("--epoch-slots")) return fail("EPOCH_LENGTH_REFUSED");
  return pass(input.allowRealNode === true ? "QUERY_ARGV" : "REAL_QUERY_NOT_ARMED", {
    argv,
    purpose,
    executed: false,
  });
}

export function queryTipArgv(input) {
  const row = queryBase(input, ["query", "tip", "--output-json"], "tip");
  return row;
}

export function queryEraHistoryArgv(input) {
  return queryBase(input, ["query", "era-history"], "era-history");
}

export function queryProtocolArgv(input) {
  return queryBase(input, ["query", "protocol-parameters", "--output-json"], "protocol");
}

export function protocolMinAda(params) {
  const distinct = { cashQuantity: 100, unitPayout: 2000000, nativeBudgetMeasured: false };
  if (!params || params.source !== "owned-protocol-query") {
    return fail("FEE_SHAPE_READY_MIN_ADA_UNMEASURED", { measured: false, ...distinct });
  }
  const field = params.utxoCostPerByte != null ? "utxoCostPerByte" : params.bppCoinsPerUTxOByte != null ? "bppCoinsPerUTxOByte" : null;
  if (field == null) return fail("FEE_SHAPE_READY_MIN_ADA_UNMEASURED", { measured: false, ...distinct });
  return pass("PROTOCOL_FIELD_PRESENT", { measured: true, field, value: params[field], ...distinct });
}

export function parseUtxoQuery(doc, options = {}) {
  if (options.syntheticFixture !== true) return fail("UTXO_VALUE_UNPARSED", { gap: "UTXO_JSON_SAMPLE_NOT_CAPTURED" });
  const rows = [];
  if (doc == null || typeof doc !== "object" || Array.isArray(doc)) return fail("UTXO_VALUE_UNPARSED");
  for (const [ref, row] of Object.entries(doc)) {
    if (!TX_IN.test(ref)) return fail("NO_TX_IN");
    const lovelace = row?.value?.lovelace;
    if (typeof row?.address !== "string" || typeof lovelace !== "number") return fail("UTXO_VALUE_UNPARSED");
    rows.push({ ref, address: row.address, lovelace });
  }
  return pass("UTXO_SYNTHETIC_FIXTURE", { rows, nodeCapture: false });
}

export function selectFaucetInput(utxos, need) {
  const required = need.actorLovelace + need.feeLovelace + need.minChangeLovelace;
  if (!Array.isArray(utxos) || utxos.length === 0) return fail("NO_TX_IN", { need: required });
  const parsed = [];
  for (const row of utxos) {
    if (typeof row?.ref !== "string" || !TX_IN.test(row.ref) || typeof row.lovelace !== "number") {
      return fail("NO_TX_IN");
    }
    const [txid, index] = row.ref.split("#");
    parsed.push({ ...row, txid, index: Number(index) });
  }
  const eligible = parsed.filter((row) => row.lovelace >= required);
  if (eligible.length === 0) return fail("NO_TX_IN", { need: required });
  eligible.sort((a, b) => a.lovelace - b.lovelace || a.index - b.index || (a.txid < b.txid ? -1 : a.txid > b.txid ? 1 : 0));
  const chosen = eligible[0];
  const changeLovelace = chosen.lovelace - need.actorLovelace - need.feeLovelace;
  return pass("FAUCET_INPUT_SELECTED", { ref: chosen.ref, changeLovelace, nodeCapture: false });
}

function addressOk(address, addressClass) {
  if (typeof address !== "string" || address.length === 0) return fail("ADDRESS_NOT_DERIVED");
  if (address === "addr_test1unitenterprise") return fail("INVALID_FAUCET_ADDRESS");
  if (address.includes("\"") || address.includes(" ") || address.includes("+")) return fail("QUOTED_TX_OUT");
  if (addressClass !== "placeholder-not-derived" && addressClass !== "derived-from-owned-vkey") {
    return fail("ADDRESS_NOT_DERIVED");
  }
  return null;
}

export function faucetBuildArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (typeof input.txIn !== "string" || !TX_IN.test(input.txIn)) return fail("NO_TX_IN");
  if (input.asset != null) return fail("FAUCET_LOVELACE_ONLY", { gap: "cash-mint" });
  if (input.inlineDatumValue != null) return fail("QUOTED_TX_OUT");
  const actor = addressOk(input.actorAddress, input.actorAddressClass);
  if (actor) return actor;
  const faucet = addressOk(input.faucetAddress, input.faucetAddressClass);
  if (faucet) return faucet;
  if (defaultKeys(input.keyPath)) return fail("DEFAULT_KEYS_REFUSED");
  if (!Number.isInteger(input.feeLovelace) || input.feeLovelace < 0) return fail("FEE_SHAPE_READY_MIN_ADA_UNMEASURED");
  if (input.feeLovelace === 200000 && input.feeSource !== "calculate-min-fee") return fail("HARDCODED_FEE");
  if (input.feeSource === "hardcoded") return fail("HARDCODED_FEE");
  if (!Number.isInteger(input.inputLovelace) || !Number.isInteger(input.actorLovelace)) return fail("NO_TX_IN");
  const changeLovelace = input.inputLovelace - input.actorLovelace - input.feeLovelace;
  if (changeLovelace < 0) return fail("CHANGE_SHORT");
  if (typeof input.outFile !== "string" || !inside(input.outFile, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  const actorOut = `${input.actorAddress}+${input.actorLovelace}`;
  const changeOut = `${input.faucetAddress}+${changeLovelace}`;
  if (actorOut.includes("\"") || changeOut.includes("\"")) return fail("QUOTED_TX_OUT");
  const argv = [
    input.cli, "conway", "transaction", "build-raw",
    "--tx-in", input.txIn,
    "--tx-out", actorOut,
    "--tx-out", changeOut,
    "--fee", String(input.feeLovelace),
    "--out-file", input.outFile,
  ];
  if (input.inlineDatumFile != null) {
    if (!inside(input.inlineDatumFile, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
    argv.push("--tx-out-inline-datum-file", input.inlineDatumFile);
  }
  const derived = input.actorAddressClass === "derived-from-owned-vkey" && input.faucetAddressClass === "derived-from-owned-vkey";
  const measured = input.protocolMinAdaMeasured === true && input.feeSource === "calculate-min-fee";
  const provisional = input.feeSource === "provisional";
  return pass("FAUCET_BUILD_ARGV", {
    argv,
    changeLovelace,
    submitBlocked: provisional || !measured || !derived,
    readyToSubmit: measured && derived && !provisional,
    feeCode: measured ? "FEE_FROM_CALCULATE_MIN_FEE" : "FEE_SHAPE_READY_MIN_ADA_UNMEASURED",
    addressDerived: derived,
    plusFormProvenAgainstThisCli: false,
  });
}

export function calculateMinFeeArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (input.txInCount != null || input.testnetMagic != null || input.byronWitnessCount != null) {
    return fail("DEPRECATED_FLAG_REFUSED");
  }
  if (!inside(input.txBodyFile, input.ownedHome) || !inside(input.protocolParamsFile, input.ownedHome)) {
    return fail("OWNED_HOME_REQUIRED");
  }
  if (!Number.isInteger(input.witnessCount) || input.witnessCount < 1) return fail("FEE_SHAPE_READY_MIN_ADA_UNMEASURED");
  return pass("CALCULATE_MIN_FEE_ARGV", {
    argv: [
      input.cli, "conway", "transaction", "calculate-min-fee",
      "--tx-body-file", input.txBodyFile,
      "--protocol-params-file", input.protocolParamsFile,
      "--witness-count", String(input.witnessCount),
      "--output-json",
    ],
  });
}

export function signArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (defaultKeys(input.signingKeyFile)) return fail("DEFAULT_KEYS_REFUSED");
  const mode = keyModeCode(input.keyMode);
  if (mode) return fail(mode);
  if (!inside(input.signingKeyFile, input.ownedHome) || !inside(input.txBodyFile, input.ownedHome) || !inside(input.outFile, input.ownedHome)) {
    return fail("OWNED_HOME_REQUIRED");
  }
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  return pass("SIGN_ARGV", {
    argv: [
      input.cli, "conway", "transaction", "sign",
      "--tx-body-file", input.txBodyFile,
      "--signing-key-file", input.signingKeyFile,
      ...magic,
      "--out-file", input.outFile,
    ],
    generatesKeys: false,
  });
}

export function submitArgv(input) {
  if (input.helperPath != null) return fail("HELPER_PATH_REFUSED");
  const gate = queryGate(input);
  if (gate) return gate;
  if (!inside(input.txFile, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  const argv = [input.cli, "conway", "transaction", "submit", "--socket-path", input.socketPath, ...magic, "--tx-file", input.txFile];
  if (argv.some((item) => String(item).includes("helper"))) return fail("HELPER_PATH_REFUSED");
  return pass(input.allowRealNode === true ? "SUBMIT_ARGV" : "REAL_QUERY_NOT_ARMED", {
    argv,
    submitted: false,
    readyToSubmit: false,
  });
}

export function queryUtxoArgv(input) {
  const selectors = [input.address ? "address" : null, input.txIn ? "tx-in" : null, input.wholeUtxo === true ? "whole" : null].filter(Boolean);
  if (selectors.length !== 1) return fail("UTXO_SELECTOR_REFUSED");
  if (input.purpose === "faucet" && selectors[0] !== "address") return fail("UTXO_SELECTOR_REFUSED");
  if (input.purpose === "confirm" && selectors[0] === "whole") return fail("UTXO_SELECTOR_REFUSED");
  if (input.txIn != null && !TX_IN.test(input.txIn)) return fail("NO_TX_IN");
  const extra = ["query", "utxo", "--output-json"];
  if (selectors[0] === "address") extra.push("--address", input.address);
  if (selectors[0] === "tx-in") extra.push("--tx-in", input.txIn);
  if (selectors[0] === "whole") extra.push("--whole-utxo");
  const row = queryBase(input, extra, input.purpose ?? "utxo");
  if (!row.ok) return row;
  return { ...row, headDatumReadback: false, gap: "readback" };
}

export function fundingConfirmArgv(input) {
  return queryUtxoArgv({ ...input, purpose: "confirm", wholeUtxo: false });
}

export function addressKeyGenArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (defaultKeys(input.signingKeyFile) || defaultKeys(input.verificationKeyFile)) return fail("DEFAULT_KEYS_REFUSED");
  if (!inside(input.signingKeyFile, input.ownedHome) || !inside(input.verificationKeyFile, input.ownedHome)) {
    return fail("OWNED_HOME_REQUIRED");
  }
  return pass("REAL_KEYGEN_NOT_ARMED", {
    argv: [
      input.cli, "address", "key-gen",
      "--verification-key-file", input.verificationKeyFile,
      "--signing-key-file", input.signingKeyFile,
    ],
    generatesKeys: true,
    keyOutput: "text-envelope-default",
    keyType: "normal-key-default",
  });
}

export function addressBuildArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (input.stake != null) return fail("ADDRESS_BUILD_FAILED");
  if (defaultKeys(input.paymentVkeyFile)) return fail("DEFAULT_KEYS_REFUSED");
  if (!inside(input.paymentVkeyFile, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  return pass("ADDRESS_BUILD_ARGV", {
    argv: [input.cli, "address", "build", "--payment-verification-key-file", input.paymentVkeyFile, ...magic],
    enterprise: true,
  });
}

export function initialAddrArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (defaultKeys(input.verificationKeyFile)) return fail("DEFAULT_KEYS_REFUSED");
  if (!inside(input.verificationKeyFile, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  const magic = magicArgs(input.magic, input.magicSource);
  if (!Array.isArray(magic)) return magic;
  return pass("INITIAL_ADDR_ARGV", {
    argv: [input.cli, "legacy", "genesis", "initial-addr", "--verification-key-file", input.verificationKeyFile, ...magic],
    executed: false,
  });
}

export function enterpriseResult(input) {
  if (input.stakeFlagsUsed === true) return fail("ADDRESS_BUILD_FAILED");
  const mode = keyModeCode(input.keyMode);
  if (mode) return fail(mode);
  if (input.address === "addr_test1unitenterprise") return fail("INVALID_FAUCET_ADDRESS");
  if (input.addressBuilt !== true) return fail("ADDRESS_BUILD_FAILED");
  return pass("ENTERPRISE_ADDRESS", { ok: true });
}

export function cardanoNodeRunArgv(input) {
  const node = input.nodeBin;
  if (typeof node !== "string" || path.basename(node) !== "cardano-node" || !path.isAbsolute(node)) return fail("CLI_REQUIRED");
  if (!path.isAbsolute(input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  for (const file of [input.topology, input.databasePath, input.socketPath, input.config]) {
    if (!inside(file, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  }
  const bodies = input.genesisBodies ?? {};
  for (const body of Object.values(bodies)) {
    if (body === "{}" || (body && typeof body === "object" && !Object.keys(body).length)) return fail("EMPTY_GENESIS");
  }
  if (input.proofTxt != null) return fail("PROOF_TXT_NOT_READINESS");
  const argv = [
    node, "run",
    "--topology", input.topology,
    "--database-path", input.databasePath,
    "--socket-path", input.socketPath,
    "--config", input.config,
    "--host-addr", input.hostAddr,
    "--port", String(input.port),
  ];
  if (argv.some((item) => String(item).includes("proof.txt"))) return fail("PROOF_TXT_NOT_READINESS");
  return pass(input.allowRealNode === true ? "NODE_RUN_ARGV" : "REAL_NODE_NOT_ARMED", {
    argv,
    poolCredentials: false,
    realNodeStarted: false,
  });
}

export function legacyCreateCardanoArgv(input) {
  const cli = requireCli(input.cli);
  if (cli) return cli;
  if (input.executeNow === true) return fail("START_NOT_IN_THIS_PACKET");
  if (input.slotLength !== 1000) return fail("SLOT_LENGTH_REFUSED");
  if (!Number.isInteger(input.genUtxoKeys) || input.genUtxoKeys < 1) return fail("FRESH_KEYS_REQUIRED");
  if (input.conwayEra !== true) return fail("ERA_REFUSED");
  if (input.magicSource === "packaged-comment") return fail("MAGIC_SOURCE_REFUSED");
  if (input.magicSource !== "caller-chosen-for-genesis" || !Number.isInteger(input.magic)) return fail("MAGIC_SOURCE_REFUSED");
  const files = [input.genesisDir, input.byronTemplate, input.shelleyTemplate, input.alonzoTemplate, input.conwayTemplate];
  for (const file of files) {
    if (!inside(file, input.ownedHome)) return fail("OWNED_HOME_REQUIRED");
  }
  return pass("LEGACY_CREATE_CARDANO_ARGV", {
    argv: [
      input.cli, "legacy", "genesis", "create-cardano",
      "--conway-era",
      "--genesis-dir", input.genesisDir,
      "--gen-utxo-keys", String(input.genUtxoKeys),
      "--slot-length", "1000",
      "--testnet-magic", String(input.magic),
      "--byron-template", input.byronTemplate,
      "--shelley-template", input.shelleyTemplate,
      "--alonzo-template", input.alonzoTemplate,
      "--conway-template", input.conwayTemplate,
    ],
    generatesKeys: true,
    gap: "CREATE_CARDANO_FILE_UNIT_UNCONFIRMED",
    helpSlotUnit: "milliseconds",
    helpGenUtxoKeysDefault: 0,
  });
}

export function topLevelCreateCardanoRefused() {
  return fail("TOPLEVEL_CREATE_CARDANO_INVALID", {
    evidence: "logs/help/genesis-create-cardano.help.txt",
  });
}

export function byronGenesisRefused() {
  return fail("BYRON_SECRET_SEED_REFUSED");
}

export function cleanupArgv(input) {
  if (input.command === "pkill" || String(input.command ?? "").includes("pkill")) return fail("GLOBAL_SIGNAL_REFUSED");
  if (input.pgid === FORBIDDEN_PID || input.pid === FORBIDDEN_PID) return fail("FORBIDDEN_PID");
  if (!Number.isInteger(input.pgid) || input.pgid <= 1) return fail("OWNED_PGID_REQUIRED");
  return pass("OWNED_PGID_TERM", { argv: ["kill", "-TERM", "--", `-${input.pgid}`] });
}

export function fundingCases(input) {
  if (typeof input.seedRef === "string" && /^[0-9a-f]{64}$/i.test(input.seedRef)) return fail("SEED_MATERIAL_REFUSED");
  if (input.seedRef === "default-mnemonic" || String(input.seedRef).includes("default-mnemonic")) {
    return fail("SEED_MATERIAL_REFUSED");
  }
  if (input.seedRef !== SEED_REF) return fail("SEED_REF_REQUIRED");
  const cases = [1, 2, 3, 4, 5].map((index) => ({
    id: `cash-head-${index}`,
    cashSeedRef: SEED_REF,
    headSeedRef: SEED_REF,
    actor: { kind: "future-enterprise", mode: "0600", generated: false },
    faucetSpends: 1,
    cashQuantity: 100,
    unitPayout: 2000000,
    protocolMinAda: "unmeasured",
  }));
  return pass("FUNDING_CASES", {
    cases,
    sharedHeadRace: {
      id: "shared-head-race",
      executed: false,
      observedError: null,
      observedErrorRequiredToSetExecuted: "BadInputsUTxO",
      stubExit0SetsExecuted: false,
    },
  });
}

function exampleInput() {
  const home = PATHS.exampleHome;
  return {
    home,
    yaci: {
      yaciBin: PATHS.yaci,
      ownedHome: home,
      packagedConfigDir: PATHS.yaciConfig,
      runId: "t1-65-example",
      nodePort: 14011,
      submitApiPort: 14012,
      slotLengthSeconds: "1",
      blockTimeSeconds: "1",
      epochLengthSlots: "3600",
      era: "conway",
      generateNewKeys: true,
      overwrite: true,
      executeNow: false,
    },
    query: {
      cli: PATHS.cli,
      ownedHome: home,
      socketPath: path.join(home, "node.sock"),
      magic: 764,
      magicSource: "shelley-networkMagic",
      allowRealNode: false,
    },
  };
}

export function exampleCommands() {
  const ex = exampleInput();
  const home = ex.home;
  const txIn = `${"0".repeat(64)}#0`;
  const fee = 172341;
  const actorLovelace = 2000000;
  const inputLovelace = 10000000;
  return {
    label: "shape-not-an-executed-node",
    createdOwnedHome: false,
    portsReserved: false,
    selfApproval: false,
    ledgerExecution: false,
    nativePins: false,
    createNode: yaciCreateNodeArgv(ex.yaci),
    futureStart: futureStartArgv(ex.yaci),
    portClaim: portClaim(),
    queryTip: queryTipArgv(ex.query),
    queryEraHistory: queryEraHistoryArgv(ex.query),
    queryProtocol: queryProtocolArgv(ex.query),
    querySlotNumber: querySlotNumberArgv({ ...ex.query, timestamp: "2026-01-01T00:00:40Z" }),
    calculateMinFee: calculateMinFeeArgv({
      cli: PATHS.cli,
      ownedHome: home,
      txBodyFile: path.join(home, "draft.body"),
      protocolParamsFile: path.join(home, "protocol.json"),
      witnessCount: 1,
    }),
    faucetBuild: faucetBuildArgv({
      cli: PATHS.cli,
      ownedHome: home,
      txIn,
      actorAddress: "addr_test1placeholderactor",
      actorAddressClass: "placeholder-not-derived",
      faucetAddress: "addr_test1placeholderfaucet",
      faucetAddressClass: "placeholder-not-derived",
      actorLovelace,
      inputLovelace,
      feeLovelace: fee,
      feeSource: "calculate-min-fee",
      protocolMinAdaMeasured: false,
      outFile: path.join(home, "funding.body"),
    }),
    sign: signArgv({
      cli: PATHS.cli,
      ownedHome: home,
      txBodyFile: path.join(home, "funding.body"),
      signingKeyFile: path.join(home, "keys", "faucet.skey"),
      outFile: path.join(home, "funding.signed"),
      keyMode: "0600",
      magic: 764,
      magicSource: "shelley-networkMagic",
    }),
    submit: submitArgv({
      ...ex.query,
      txFile: path.join(home, "funding.signed"),
    }),
    confirm: fundingConfirmArgv({
      ...ex.query,
      address: "addr_test1placeholderactor",
    }),
    addressKeyGen: addressKeyGenArgv({
      cli: PATHS.cli,
      ownedHome: home,
      verificationKeyFile: path.join(home, "keys", "actor.vkey"),
      signingKeyFile: path.join(home, "keys", "actor.skey"),
    }),
    addressBuild: addressBuildArgv({
      cli: PATHS.cli,
      ownedHome: home,
      paymentVkeyFile: path.join(home, "keys", "actor.vkey"),
      magic: 764,
      magicSource: "shelley-networkMagic",
    }),
    initialAddr: initialAddrArgv({
      cli: PATHS.cli,
      ownedHome: home,
      verificationKeyFile: path.join(home, "keys", "faucet.vkey"),
      magic: 764,
      magicSource: "shelley-networkMagic",
    }),
    fundingCases: fundingCases({ seedRef: SEED_REF }),
    readinessUnarmed: assessReadiness({
      allowRealNode: false,
      pid: 4242,
      pgid: 4242,
      ownPgid: 1000,
      ownedHome: home,
      socketPath: path.join(home, "node.sock"),
      tip: { slot: 10 },
      runId: "t1-65-example",
      transcriptRunId: "t1-65-example",
      protocolQueried: true,
    }),
    windows: planWindows({ systemStart: "2026-01-01T00:00:00Z", tipSlot: 10 }),
    topLevelCreateCardano: topLevelCreateCardanoRefused(),
    byronGenesis: byronGenesisRefused(),
  };
}

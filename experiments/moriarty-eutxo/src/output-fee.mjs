/**
 * Worktree repair of the offline output-fee helper.
 * Frozen t1-output-fee-adapter-77/own77/output-fee.mjs is not this file.
 * Review 84 approves only that frozen source. It does not approve these bytes.
 * Cost integers are command stdout. This file does not store them and does not
 * apply a byte formula. Caller provenance strings are not a live query.
 */
import { spawn } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import { createReadStream } from "node:fs";
import { mkdir, readFile, stat, writeFile } from "node:fs/promises";
import path from "node:path";

/** Identity of cardano-cli 11.0.0.0. This is the binary pin, not a fee or a coin. */
const PINNED_CLI_SHA256 = "4f9d8b7de9886b0a63b933a7ad299568415f92d7544ab9148b16e44824b839dd";
const PINNED_CLI_BYTES = 95153384;
const MAX_LOVELACE = 45000000000000000n;
const MAX_QUANTITY = (1n << 64n) - 1n;
const MAX_WITNESS_COUNT = 64n;
const EVIDENCE_CLASS = "offline-pinned-protocol-file";

export class OutputFeeError extends Error {
  constructor(code, message, details = {}) {
    super(message);
    this.name = "OutputFeeError";
    this.code = code;
    this.details = details;
    this.calculated = false;
    this.nativeBudgetMeasured = false;
    this.ledgerExecuted = false;
  }
}

export function canonicalInteger(value, { name, min, max, rangeCode }) {
  let text;
  if (typeof value === "bigint") text = value.toString(10);
  else if (typeof value === "number") {
    if (!Number.isSafeInteger(value)) {
      throw new OutputFeeError("NON_CANONICAL_INTEGER", `${name} is not a safe integer`, { spawned: false });
    }
    text = String(value);
  } else if (typeof value === "string") text = value;
  else {
    throw new OutputFeeError("NON_CANONICAL_INTEGER", `${name} must be an integer`, { spawned: false });
  }
  if (!/^-?(0|[1-9][0-9]*)$/.test(text)) {
    throw new OutputFeeError("NON_CANONICAL_INTEGER", `${name} is not a canonical decimal integer`, { spawned: false });
  }
  const parsed = BigInt(text);
  if (parsed < min || parsed > max) {
    throw new OutputFeeError(rangeCode, `${name} is outside ${min.toString(10)}..${max.toString(10)}`, {
      spawned: false,
    });
  }
  return parsed;
}

export function integerBand(value) {
  const n = BigInt(value);
  if (n < 0n) return "negative";
  if (n < 24n) return "cbor-u8-direct";
  if (n < 256n) return "cbor-u8";
  if (n < 65536n) return "cbor-u16";
  if (n < 4294967296n) return "cbor-u32";
  if (n < 18446744073709551616n) return "cbor-u64";
  return "above-u64";
}

export function parseCoinShow(stdout) {
  if (typeof stdout !== "string" || !stdout.endsWith("\n") || stdout.slice(0, -1).includes("\n")) {
    throw new OutputFeeError("FAKE_COIN", "calculate-min-required-utxo stdout must be one Coin line", {
      spawned: false,
    });
  }
  const matched = /^Coin\s+(\d+)$/.exec(stdout.slice(0, -1));
  if (!matched || !/^(0|[1-9][0-9]*)$/.test(matched[1])) {
    throw new OutputFeeError("FAKE_COIN", "stdout is not Coin followed by canonical digits", { spawned: false });
  }
  return matched[1];
}

export function parseFeeStdout(stdout) {
  if (typeof stdout !== "string" || stdout.length === 0) {
    throw new OutputFeeError("FAKE_FEE", "calculate-min-fee stdout is empty", { spawned: false });
  }
  let parsed;
  try {
    parsed = JSON.parse(stdout);
  } catch {
    throw new OutputFeeError("FAKE_FEE", "default calculate-min-fee stdout must be JSON", { spawned: false });
  }
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new OutputFeeError("FAKE_FEE", "fee stdout must be a JSON object", { spawned: false });
  }
  const keys = Object.keys(parsed);
  if (Object.prototype.hasOwnProperty.call(parsed, "txBytes")) {
    throw new OutputFeeError("TX_BYTES_NOT_A_COMMAND_FIELD", "calculate-min-fee emits fee", { spawned: false });
  }
  if (keys.length !== 1 || keys[0] !== "fee") {
    throw new OutputFeeError("FEE_ALIAS_FIELD", "fee JSON has one integer field named fee", { spawned: false });
  }
  const token = stdout.match(/"fee"\s*:\s*([^\s,}\n]+)/);
  if (!token || stdout.split('"fee"').length !== 2 || !/^(0|[1-9][0-9]*)$/.test(token[1])) {
    throw new OutputFeeError("FAKE_FEE", "fee token is not one canonical integer", { spawned: false });
  }
  if (typeof parsed.fee !== "number" || !Number.isInteger(parsed.fee) || String(parsed.fee) !== token[1]) {
    throw new OutputFeeError("FAKE_FEE", "fee JSON integer does not match the raw token", { spawned: false });
  }
  return token[1];
}

export function classifyWitnessCountProvenance(value) {
  if (value === undefined) {
    return {
      witnessCountVerified: false,
      witnessCountProvenanceClass: "undefined-not-verified",
      liveQueryProof: false,
    };
  }
  let callerText = "";
  try {
    callerText = typeof value === "string" ? value : JSON.stringify(value);
  } catch {
    callerText = Object.prototype.toString.call(value);
  }
  if (value === "counted-from-signing-key-paths") {
    return {
      witnessCountVerified: false,
      witnessCountProvenanceClass: "counted-from-signing-key-paths",
      liveQueryProof: false,
      callerText: value,
    };
  }
  const assumption = typeof value === "string" && /^assum/i.test(value);
  return {
    witnessCountVerified: false,
    witnessCountProvenanceClass: assumption ? "assumption-not-verified" : "caller-string-not-verified",
    liveQueryProof: false,
    callerText,
  };
}

export function assertStableDigest(label, before, after) {
  if (before !== after) {
    throw new OutputFeeError("UNSTABLE_FILE", `${label} changed during the command`, {
      spawned: true,
      before,
      after,
    });
  }
}

function markSpawned(error) {
  if (error instanceof OutputFeeError) error.details = { ...error.details, spawned: true };
  return error;
}

function sha256Text(text) {
  return createHash("sha256").update(text).digest("hex");
}

function hashStream(file) {
  return new Promise((resolve, reject) => {
    const hash = createHash("sha256");
    const stream = createReadStream(file);
    stream.on("error", reject);
    stream.on("data", (chunk) => hash.update(chunk));
    stream.on("end", () => resolve(hash.digest("hex")));
  });
}

async function digestFile(file) {
  const buf = await readFile(file);
  return { buf, bytes: buf.length, sha256: createHash("sha256").update(buf).digest("hex") };
}

function childEnv() {
  const env = { ...process.env };
  const removedEnvKeys = [];
  for (const key of Object.keys(env)) {
    if (key === "CARDANO_NODE_NETWORK_ID" || key.startsWith("YACI") || key.includes("SOCKET")) {
      delete env[key];
      removedEnvKeys.push(key);
    }
  }
  return { env, removedEnvKeys };
}

function expectedProtocolSha(provenance) {
  if (provenance == null) return null;
  if (typeof provenance === "string") {
    return /^[0-9a-fA-F]{64}$/.test(provenance) ? provenance.toLowerCase() : null;
  }
  if (typeof provenance !== "object" || Array.isArray(provenance)) {
    throw new OutputFeeError("MALFORMED_PROVENANCE", "protocolProvenance must be a string or an object", {
      spawned: false,
    });
  }
  const value = provenance.expectedSha256 ?? provenance.sha256 ?? null;
  if (value == null) return null;
  if (typeof value !== "string" || !/^[0-9a-fA-F]{64}$/.test(value)) {
    throw new OutputFeeError("PROTOCOL_HASH_DRIFT", "protocol provenance hash is not 64 hex characters", {
      spawned: false,
    });
  }
  return value.toLowerCase();
}

function assertPlainPath(value, name) {
  if (typeof value !== "string" || value.length === 0 || /['"`]/.test(value)) {
    throw new OutputFeeError("QUOTED_ARG", `${name} is missing or contains a quote`, { spawned: false });
  }
}

async function pinInputs({ cliPath, protocolPath, protocolProvenance, cliIdentity = null }) {
  assertPlainPath(cliPath, "cliPath");
  assertPlainPath(protocolPath, "protocolPath");
  const expected = expectedProtocolSha(protocolProvenance);
  let protocol;
  try {
    protocol = await digestFile(protocolPath);
  } catch (error) {
    throw new OutputFeeError("MISSING_PATH", "protocol file is not readable", {
      spawned: false,
      cause: error.message,
    });
  }
  if (expected && expected !== protocol.sha256) {
    throw new OutputFeeError("PROTOCOL_HASH_DRIFT", "protocol file sha256 does not match the caller provenance hash", {
      spawned: false,
      expectedSha256: expected,
      actualSha256: protocol.sha256,
    });
  }
  let cliStat;
  try {
    cliStat = await stat(cliPath);
  } catch (error) {
    throw new OutputFeeError("BAD_CLI_SHA", "cli path is not readable", { spawned: false, cause: error.message });
  }
  if (cliIdentity === "unit-stub") {
    if (!cliStat.isFile() || cliStat.size < 1) {
      throw new OutputFeeError("BAD_CLI_SHA", "unit stub cli is not a file", { spawned: false });
    }
    const cliSha256 = await hashStream(cliPath);
    const { env, removedEnvKeys } = childEnv();
    return {
      protocolSha256: protocol.sha256,
      protocolBytes: protocol.bytes,
      cliSha256,
      cliBytes: cliStat.size,
      env,
      removedEnvKeys,
      callerProtocolProvenance: protocolProvenance === undefined ? null : protocolProvenance,
      cliIdentity: "unit-stub",
      unitOnly: true,
    };
  }
  if (!cliStat.isFile() || cliStat.size !== PINNED_CLI_BYTES) {
    throw new OutputFeeError("BAD_CLI_SHA", "cli file identity does not match the pinned cardano-cli 11 binary", {
      spawned: false,
      bytes: cliStat.size,
    });
  }
  const cliSha256 = await hashStream(cliPath);
  if (cliSha256 !== PINNED_CLI_SHA256) {
    throw new OutputFeeError("BAD_CLI_SHA", "cli sha256 does not match the pinned cardano-cli 11 binary", {
      spawned: false,
      actualSha256: cliSha256,
    });
  }
  const { env, removedEnvKeys } = childEnv();
  return {
    protocolSha256: protocol.sha256,
    protocolBytes: protocol.bytes,
    cliSha256,
    cliBytes: cliStat.size,
    env,
    removedEnvKeys,
    callerProtocolProvenance: protocolProvenance === undefined ? null : protocolProvenance,
  };
}

function assertArgv(argv) {
  if (!argv.every((item) => typeof item === "string" && item.length > 0 && !/['"`]/.test(item))) {
    throw new OutputFeeError("QUOTED_ARG", "an argument contains a quote or is empty", { spawned: false });
  }
}

function execCli(argv, { cwd, env }) {
  assertArgv(argv);
  return new Promise((resolve, reject) => {
    const child = spawn(argv[0], argv.slice(1), {
      cwd,
      env,
      shell: false,
      stdio: ["ignore", "pipe", "pipe"],
    });
    const out = [];
    const err = [];
    let outLen = 0;
    let errLen = 0;
    let settled = false;
    const timer = setTimeout(() => {
      child.kill();
      finish(() => reject(new OutputFeeError("CLI_TIMEOUT", "cardano-cli exceeded 60s", { spawned: true })));
    }, 60_000);
    function finish(fn) {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      fn();
    }
    child.stdout.on("data", (chunk) => {
      outLen += chunk.length;
      if (outLen > 1_000_000) {
        child.kill();
        finish(() => reject(new OutputFeeError("CLI_OUTPUT_LIMIT", "stdout exceeded 1000000 bytes", { spawned: true })));
        return;
      }
      out.push(chunk);
    });
    child.stderr.on("data", (chunk) => {
      errLen += chunk.length;
      if (errLen > 1_000_000) {
        child.kill();
        finish(() => reject(new OutputFeeError("CLI_OUTPUT_LIMIT", "stderr exceeded 1000000 bytes", { spawned: true })));
        return;
      }
      err.push(chunk);
    });
    child.on("error", (error) => {
      finish(() => reject(new OutputFeeError("CLI_SPAWN", error.message, { spawned: true })));
    });
    child.on("close", (code) => {
      finish(() =>
        resolve({
          exitCode: code,
          stdout: Buffer.concat(out).toString("utf8"),
          stderr: Buffer.concat(err).toString("utf8"),
        }),
      );
    });
  });
}

async function finishCommand(run, label) {
  if (run.exitCode !== 0) {
    throw new OutputFeeError("CLI_NONZERO", `${label} exited ${run.exitCode}`, {
      spawned: true,
      exitCode: run.exitCode,
      rawStdout: run.stdout,
      rawStderr: run.stderr,
    });
  }
  if (run.stderr.length !== 0) {
    throw new OutputFeeError("UNEXPECTED_STDERR", `${label} wrote stderr`, {
      spawned: true,
      rawStdout: run.stdout,
      rawStderr: run.stderr,
    });
  }
  return run;
}

function acceptDatumInt(value) {
  if (typeof value === "string") {
    throw new OutputFeeError("MALFORMED_DATUM", "string datum int is rejected before spawn", { spawned: false });
  }
  if (typeof value === "number" && Number.isSafeInteger(value)) return;
  throw new OutputFeeError("MALFORMED_DATUM", "int must be a safe integer", { spawned: false });
}

function validateDetailed(node, depth) {
  if (depth > 32) throw new OutputFeeError("MALFORMED_DATUM", "datum nesting exceeds 32", { spawned: false });
  if (!node || typeof node !== "object" || Array.isArray(node)) {
    throw new OutputFeeError("MALFORMED_DATUM", "detailed schema node must be an object", { spawned: false });
  }
  const keys = Object.keys(node);
  if (keys.length === 1 && keys[0] === "int") {
    acceptDatumInt(node.int);
    return;
  }
  if (keys.length === 1 && keys[0] === "bytes") {
    if (typeof node.bytes !== "string" || !/^(?:[0-9a-fA-F]{2})*$/.test(node.bytes)) {
      throw new OutputFeeError("MALFORMED_DATUM", "bytes must be even-length hex", { spawned: false });
    }
    return;
  }
  if (keys.length === 1 && keys[0] === "list") {
    if (!Array.isArray(node.list)) throw new OutputFeeError("MALFORMED_DATUM", "list must be an array", { spawned: false });
    for (const item of node.list) validateDetailed(item, depth + 1);
    return;
  }
  if (keys.length === 1 && keys[0] === "map") {
    if (!Array.isArray(node.map)) throw new OutputFeeError("MALFORMED_DATUM", "map must be an array", { spawned: false });
    for (const entry of node.map) {
      if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
        throw new OutputFeeError("MALFORMED_DATUM", "map entry must be an object", { spawned: false });
      }
      const entryKeys = Object.keys(entry);
      if (entryKeys.length !== 2 || !entryKeys.includes("k") || !entryKeys.includes("v")) {
        throw new OutputFeeError("MALFORMED_DATUM", "map entry must have k and v", { spawned: false });
      }
      validateDetailed(entry.k, depth + 1);
      validateDetailed(entry.v, depth + 1);
    }
    return;
  }
  if (keys.length === 2 && keys.includes("constructor") && keys.includes("fields")) {
    if (typeof node.constructor !== "number" || !Number.isInteger(node.constructor) || node.constructor < 0) {
      throw new OutputFeeError("MALFORMED_DATUM", "constructor must be a non-negative integer", { spawned: false });
    }
    if (!Array.isArray(node.fields)) {
      throw new OutputFeeError("MALFORMED_DATUM", "constructor fields must be an array", { spawned: false });
    }
    for (const field of node.fields) validateDetailed(field, depth + 1);
    return;
  }
  throw new OutputFeeError("MALFORMED_DATUM", "node is not detailed schema", { spawned: false });
}

function normalizeAssets(assets) {
  if (!Array.isArray(assets)) {
    throw new OutputFeeError("MALFORMED_ASSET", "assets must be an array", { spawned: false });
  }
  return assets.map((asset) => {
    if (!asset || typeof asset !== "object" || Array.isArray(asset)) {
      throw new OutputFeeError("MALFORMED_ASSET", "asset must be an object", { spawned: false });
    }
    const { policyId, assetNameHex, quantity } = asset;
    if (typeof policyId !== "string" || typeof assetNameHex !== "string") {
      throw new OutputFeeError("MALFORMED_ASSET", "policyId and assetNameHex must be strings", { spawned: false });
    }
    if (/['"`]/.test(policyId) || /['"`]/.test(assetNameHex)) {
      throw new OutputFeeError("QUOTED_ARG", "asset id contains a quote", { spawned: false });
    }
    if (!/^[0-9a-fA-F]{56}$/.test(policyId)) {
      throw new OutputFeeError("MALFORMED_ASSET", "policyId must be 56 hex characters", { spawned: false });
    }
    if (!/^[0-9a-fA-F]*$/.test(assetNameHex) || assetNameHex.length % 2 !== 0 || assetNameHex.length > 64) {
      throw new OutputFeeError("MALFORMED_ASSET_NAME", "asset name must be hex bytes, not an ASCII label", {
        spawned: false,
      });
    }
    const qty = canonicalInteger(quantity, {
      name: "quantity",
      min: 1n,
      max: MAX_QUANTITY,
      rangeCode: "MALFORMED_QUANTITY",
    });
    return {
      policyId: policyId.toLowerCase(),
      assetNameHex: assetNameHex.toLowerCase(),
      quantity: qty.toString(10),
    };
  });
}

function encodeTxOut(address, lovelaceText, assets) {
  if (typeof address !== "string") {
    throw new OutputFeeError("UNSUPPORTED_OUTPUT", "address must be a string", { spawned: false });
  }
  if (/['"`]/.test(address)) throw new OutputFeeError("QUOTED_ARG", "address contains a quote", { spawned: false });
  if (!/^addr(?:_test)?1[0-9a-z]+$/.test(address)) {
    throw new OutputFeeError("UNSUPPORTED_OUTPUT", "address must be an addr or addr_test bech32 value", {
      spawned: false,
    });
  }
  const parts = [`${address}+${lovelaceText}`];
  for (const asset of assets) parts.push(`${asset.quantity} ${asset.policyId}.${asset.assetNameHex}`);
  const txOut = parts.join("+");
  if (/['"`]/.test(txOut)) throw new OutputFeeError("QUOTED_ARG", "tx-out contains a quote", { spawned: false });
  return txOut;
}

function rejectWitnessScriptKeys(output) {
  for (const key of ["script", "witnessScript", "sourceWitnessScript", "plutusScript", "scripts"]) {
    if (output[key] != null) {
      throw new OutputFeeError("UNSUPPORTED_OUTPUT", "a source witness script is not an output reference script", {
        spawned: false,
        key,
      });
    }
  }
}

function baseFlags() {
  return {
    calculated: true,
    nativeBudgetMeasured: false,
    ledgerExecuted: false,
    liveProtocolQueried: false,
    formulaSubstituted: false,
    selfApproval: false,
    evidenceClass: EVIDENCE_CLASS,
    repairedCopy: true,
    frozenHelperApprovalCoversThisFile: false,
  };
}

export async function calculateOutputMinimum({
  cliPath,
  protocolPath,
  output,
  workDir,
  protocolProvenance,
  cliIdentity = null,
}) {
  if (!output || typeof output !== "object" || Array.isArray(output)) {
    throw new OutputFeeError("UNSUPPORTED_OUTPUT", "output must be an object", { spawned: false });
  }
  assertPlainPath(workDir, "workDir");
  rejectWitnessScriptKeys(output);
  const lovelace = canonicalInteger(output.lovelace, {
    name: "lovelace",
    min: 0n,
    max: MAX_LOVELACE,
    rangeCode: "UNSUPPORTED_OUTPUT",
  });
  const assets = normalizeAssets(output.assets);
  let datumPath = null;
  let datumSha = null;
  let fieldCount = null;
  if (output.datumData != null) {
    if (typeof output.datumData === "number") {
      throw new OutputFeeError("MALFORMED_DATUM", "pass the full detailed schema object, not a field count", {
        spawned: false,
      });
    }
    validateDetailed(output.datumData, 0);
    fieldCount = Array.isArray(output.datumData.fields) ? output.datumData.fields.length : null;
    await mkdir(workDir, { recursive: true });
    datumPath = path.join(workDir, `inline-datum-${randomBytes(8).toString("hex")}.json`);
    const text = `${JSON.stringify(output.datumData)}\n`;
    await writeFile(datumPath, text, { mode: 0o644 });
    datumSha = sha256Text(text);
  }
  let referenceScriptSha = null;
  const referenceScriptPath = output.referenceScriptPath;
  if (referenceScriptPath != null) {
    if (typeof referenceScriptPath !== "string" || referenceScriptPath.length === 0) {
      throw new OutputFeeError("UNSUPPORTED_OUTPUT", "referenceScriptPath must be a file path when present", {
        spawned: false,
      });
    }
    assertPlainPath(referenceScriptPath, "referenceScriptPath");
    try {
      const script = await digestFile(referenceScriptPath);
      if (!(await stat(referenceScriptPath)).isFile()) {
        throw new OutputFeeError("REFERENCE_SCRIPT_MISSING", "reference script path is not a file", { spawned: false });
      }
      referenceScriptSha = script.sha256;
    } catch (error) {
      if (error instanceof OutputFeeError) throw error;
      throw new OutputFeeError("REFERENCE_SCRIPT_MISSING", "reference script path is not readable", {
        spawned: false,
        cause: error.message,
      });
    }
  }
  const pinned = await pinInputs({ cliPath, protocolPath, protocolProvenance, cliIdentity });
  await mkdir(workDir, { recursive: true });
  const declaredText = lovelace.toString(10);
  const txOut = encodeTxOut(output.address, declaredText, assets);
  const argv = [
    cliPath,
    "conway",
    "transaction",
    "calculate-min-required-utxo",
    "--tx-out",
    txOut,
    "--protocol-params-file",
    protocolPath,
  ];
  if (datumPath) argv.push("--tx-out-inline-datum-file", datumPath);
  if (referenceScriptPath) argv.push("--tx-out-reference-script-file", referenceScriptPath);
  const first = await finishCommand(await execCli(argv, { cwd: workDir, env: pinned.env }), "calculate-min-required-utxo");
  let coinText;
  try {
    coinText = parseCoinShow(first.stdout);
  } catch (error) {
    throw markSpawned(error);
  }
  const protocolAfter = (await digestFile(protocolPath)).sha256;
  assertStableDigest("protocol file", pinned.protocolSha256, protocolAfter);
  if (datumPath) assertStableDigest("inline datum", datumSha, sha256Text(await readFile(datumPath, "utf8")));
  if (referenceScriptPath) {
    assertStableDigest("reference script", referenceScriptSha, (await digestFile(referenceScriptPath)).sha256);
  }
  const adjustedText = coinText;
  const adjustedTxOut = encodeTxOut(output.address, adjustedText, assets);
  const repeatArgv = argv.map((item) => (item === txOut ? adjustedTxOut : item));
  const second = await finishCommand(
    await execCli(repeatArgv, { cwd: workDir, env: pinned.env }),
    "calculate-min-required-utxo stability",
  );
  let repeatText;
  try {
    repeatText = parseCoinShow(second.stdout);
  } catch (error) {
    throw markSpawned(error);
  }
  if (repeatText !== coinText) {
    throw new OutputFeeError("UNSTABLE_MIN_COIN", "the bounded Coin repeat returned a different integer", {
      spawned: true,
      first: coinText,
      second: repeatText,
      boundedCalls: 2,
    });
  }
  const protocolFinal = (await digestFile(protocolPath)).sha256;
  assertStableDigest("protocol file", pinned.protocolSha256, protocolFinal);
  if (datumPath) assertStableDigest("inline datum after second call", datumSha, sha256Text(await readFile(datumPath, "utf8")));
  if (referenceScriptPath) {
    assertStableDigest("reference script after second call", referenceScriptSha, (await digestFile(referenceScriptPath)).sha256);
  }
  const cliAfterCoin = await digestFile(cliPath);
  assertStableDigest("cli binary", pinned.cliSha256, cliAfterCoin.sha256);
  const minimum = BigInt(coinText);
  let comparison = "equal-to-requirement";
  if (lovelace < minimum) comparison = "below-requirement";
  else if (lovelace > minimum) comparison = "above-requirement";
  return {
    ...baseFlags(),
    command: "conway transaction calculate-min-required-utxo",
    exitCode: 0,
    rawStdout: first.stdout,
    rawStderr: first.stderr,
    argv,
    txOut,
    digests: {
      cliSha256: pinned.cliSha256,
      cliBytes: pinned.cliBytes,
      protocolSha256: pinned.protocolSha256,
      protocolSha256Before: pinned.protocolSha256,
      protocolSha256After: protocolFinal,
      stdoutSha256: sha256Text(first.stdout),
      datumSha256: datumSha,
      referenceScriptSha256: referenceScriptSha,
    },
    parsed: { lovelace: coinText },
    declaredLovelace: declaredText,
    minimumLovelace: coinText,
    declaredBelowMinimum: comparison === "below-requirement",
    shortfallLovelace: lovelace < minimum ? (minimum - lovelace).toString(10) : "0",
    comparison,
    declaredIntegerBand: integerBand(lovelace),
    minimumIntegerBand: integerBand(minimum),
    sameIntegerBand: integerBand(lovelace) === integerBand(minimum),
    stability: {
      boundedCalls: 2,
      maxCalls: 2,
      looped: false,
      adjustedLovelace: adjustedText,
      rawStdout: second.stdout,
      parsedLovelace: repeatText,
      stable: true,
      argv: repeatArgv,
    },
    assets,
    datum: datumPath
      ? { present: true, schema: "detailed-json", fieldCount, sha256: datumSha, path: datumPath, passedAsFieldCount: false }
      : { present: false, schema: null, fieldCount: null, sha256: null, path: null, passedAsFieldCount: false },
    referenceScript: {
      present: referenceScriptPath != null,
      counted: referenceScriptPath != null,
      path: referenceScriptPath ?? null,
      sha256: referenceScriptSha,
      sourceWitnessScriptCounted: false,
    },
    callerProtocolProvenance: pinned.callerProtocolProvenance,
    protocolProvenanceIsLiveQueryProof: false,
    removedEnvKeys: pinned.removedEnvKeys,
  };
}

function readEnvelope(buf) {
  let body;
  try {
    body = JSON.parse(buf.toString("utf8"));
  } catch {
    throw new OutputFeeError("MALFORMED_BODY", "body file is not JSON", { spawned: false });
  }
  if (!body || typeof body !== "object" || Array.isArray(body)) {
    throw new OutputFeeError("MALFORMED_BODY", "body file must be a JSON object", { spawned: false });
  }
  const hex = body.cborHex;
  if (typeof hex !== "string" || !/^(?:[0-9a-fA-F]{2})+$/.test(hex)) {
    throw new OutputFeeError("MALFORMED_BODY", "raw body bytes come from cborHex, not from a fee txBytes field", {
      spawned: false,
    });
  }
  return {
    envelopeType: typeof body.type === "string" ? body.type : null,
    description: typeof body.description === "string" ? body.description : null,
    cborHexSha256: sha256Text(hex),
    rawBodyCborBytes: String(hex.length / 2),
  };
}

function executionUnitsClass(provenance, bodySha) {
  if (provenance == null) return "units-not-supplied";
  if (provenance?.measured !== true || provenance?.command !== "calculate-plutus-script-cost") {
    return "stored-units-not-a-measurement";
  }
  if (provenance.bodySha256 !== bodySha) return "units-not-bound-to-this-body";
  return "units-bound-awaiting-ledger-proof";
}

export async function calculateBodyMinimumFee({
  cliPath,
  protocolPath,
  bodyPath,
  witnessCount,
  witnessCountProvenance,
  workDir,
  protocolProvenance,
  referenceScriptSize = null,
  requiresReferenceScriptFee = false,
  executionUnitsProvenance = null,
  signingKeyPaths = null,
  cliIdentity = null,
}) {
  assertPlainPath(workDir, "workDir");
  assertPlainPath(bodyPath, "bodyPath");
  const witnesses = canonicalInteger(witnessCount, {
    name: "witnessCount",
    min: 0n,
    max: MAX_WITNESS_COUNT,
    rangeCode: "UNSUPPORTED_WITNESS_COUNT",
  });
  const provenance = classifyWitnessCountProvenance(witnessCountProvenance);
  if (Array.isArray(signingKeyPaths) && BigInt(signingKeyPaths.length) !== witnesses) {
    throw new OutputFeeError("WITNESS_COUNT_KEY_MISMATCH", "witness count is not the signing-key count", {
      spawned: false,
      signingKeyCount: signingKeyPaths.length,
    });
  }
  if (requiresReferenceScriptFee === true && referenceScriptSize == null) {
    throw new OutputFeeError("REFERENCE_SCRIPT_SIZE_REQUIRED", "a reference-script fee needs the script size in bytes", {
      spawned: false,
    });
  }
  let referenceBytes = null;
  if (referenceScriptSize != null) {
    referenceBytes = canonicalInteger(referenceScriptSize, {
      name: "referenceScriptSize",
      min: 0n,
      max: 1000000000n,
      rangeCode: "REFERENCE_SCRIPT_SIZE_REQUIRED",
    });
  }
  let bodyBefore;
  try {
    bodyBefore = await digestFile(bodyPath);
  } catch (error) {
    throw new OutputFeeError("MISSING_PATH", "body file is not readable", { spawned: false, cause: error.message });
  }
  const envelope = readEnvelope(bodyBefore.buf);
  const pinned = await pinInputs({ cliPath, protocolPath, protocolProvenance, cliIdentity });
  await mkdir(workDir, { recursive: true });
  const argv = [
    cliPath,
    "conway",
    "transaction",
    "calculate-min-fee",
    "--tx-body-file",
    bodyPath,
    "--protocol-params-file",
    protocolPath,
    "--witness-count",
    witnesses.toString(10),
  ];
  if (referenceBytes != null) argv.push("--reference-script-size", referenceBytes.toString(10));
  const run = await finishCommand(await execCli(argv, { cwd: workDir, env: pinned.env }), "calculate-min-fee");
  let feeText;
  try {
    feeText = parseFeeStdout(run.stdout);
  } catch (error) {
    throw markSpawned(error);
  }
  const bodyAfter = await digestFile(bodyPath);
  const protocolAfter = await digestFile(protocolPath);
  assertStableDigest("body file", bodyBefore.sha256, bodyAfter.sha256);
  assertStableDigest("protocol file", pinned.protocolSha256, protocolAfter.sha256);
  const cliAfterFee = await digestFile(cliPath);
  assertStableDigest("cli binary", pinned.cliSha256, cliAfterFee.sha256);
  return {
    ...baseFlags(),
    command: "conway transaction calculate-min-fee",
    exitCode: 0,
    rawStdout: run.stdout,
    rawStderr: run.stderr,
    argv,
    digests: {
      cliSha256: pinned.cliSha256,
      cliBytes: pinned.cliBytes,
      protocolSha256: pinned.protocolSha256,
      protocolSha256Before: pinned.protocolSha256,
      protocolSha256After: protocolAfter.sha256,
      bodySha256: bodyBefore.sha256,
      bodySha256Before: bodyBefore.sha256,
      bodySha256After: bodyAfter.sha256,
      stdoutSha256: sha256Text(run.stdout),
      cborHexSha256: envelope.cborHexSha256,
    },
    parsed: { fee: feeText },
    witnessCount: witnesses.toString(10),
    witnessCountProvenance: provenance,
    rawBodyCborBytes: envelope.rawBodyCborBytes,
    bodyFileBytes: String(bodyBefore.bytes),
    envelopeType: envelope.envelopeType,
    description: envelope.description,
    txBytes: null,
    executionUnitsMeasured: false,
    executionUnitsProvenanceClass: executionUnitsClass(executionUnitsProvenance, bodyBefore.sha256),
    feeEstablishesFeeAfterRealExecutionUnits: false,
    referenceScriptFeePriced: referenceBytes != null,
    referenceScriptSize: referenceBytes == null ? null : referenceBytes.toString(10),
    signingKeyCount: Array.isArray(signingKeyPaths) ? signingKeyPaths.length : null,
    callerProtocolProvenance: pinned.callerProtocolProvenance,
    protocolProvenanceIsLiveQueryProof: false,
    removedEnvKeys: pinned.removedEnvKeys,
    omittedFlags: ["--output-json", "--byron-witness-count", "--tx-in-count", "--tx-out-count", "--mainnet", "--testnet-magic"].concat(referenceBytes == null ? ["--reference-script-size"] : []),
  };
}

#!/usr/bin/env node
/** Local financial case driver API. Import is inert. No operational admission is invented. */
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {isDirectRun} from '../custody/generate.mjs';
import {createFinancialProviders} from './providers.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const SYNTHETIC_RECIPIENTS = new Set([
  'b001010101010101010101010101010101010101010101010101010101010101',
  'b002020202020202020202020202020202020202020202020202020202020202',
  'b003030303030303030303030303030303030303030303030303030303030303',
  'b004040404040404040404040404040404040404040404040404040404040404',
]);
const KNOWN = new Set([
  'case', 'networkAdmission', 'sourceManifest', 'provenAssetManifest', 'walletContext',
  'roleCapabilities', 'recipientAddresses', 'callHints', 'blockTime', 'eventSink',
  'limits', 'operationalAdmission', 'privateStateLocation', 'resourceCounters',
  'onSubmission', 'deadlineMs', 'adapters', 'publicSink',
]);

function hexAddr(value) {
  return String(value || '').replace(/^0x/i, '').toLowerCase();
}
function fail(msg) { throw new Error(msg); }

export function validateCaseOptions(options) {
  if (!options || typeof options !== 'object') fail('options required');
  for (const k of Object.keys(options)) {
    if (!KNOWN.has(k)) fail('unknown option ' + k);
  }
  if (options.case !== 'loan' && options.case !== 'swap') fail('case must be loan or swap');
  const net = options.networkAdmission;
  if (!net || typeof net !== 'object') fail('networkAdmission required');
  if (net.logicalTag !== 'local') fail('unknown or non-local network');
  if (net.bound !== true || !net.observedProtocol) fail('constructor text does not bind network observation');
  const proven = options.provenAssetManifest;
  if (!proven) fail('provenAssetManifest required');
  if (proven.proven === true && !proven.inspectedProofAssets) fail('false proven-asset manifest');
  const limits = options.limits;
  if (!limits || typeof limits.attempts !== 'number' || limits.attempts < 1) fail('malformed attempts bound');
  if (typeof limits.deadlineMs !== 'number' || !(limits.deadlineMs > 0)) fail('malformed deadline');
  if (limits.spend == null || BigInt(limits.spend) < 0n) fail('malformed spend bound');
  const roles = options.roleCapabilities || {};
  if (options.case === 'loan') {
    if (!roles.borrower || !roles.lender) fail('missing loan role bindings');
  } else if (!roles.trader || !roles.provider) fail('missing swap role bindings');
  const rec = options.recipientAddresses || {};
  const needed = options.case === 'loan' ? ['borrower', 'lender'] : ['trader', 'provider'];
  for (const n of needed) {
    const addr = hexAddr(rec[n]);
    if (!addr || addr.length < 32) fail('missing recipient ' + n);
    if (SYNTHETIC_RECIPIENTS.has(addr)) fail('reused synthetic recipient');
  }
  const horizon = Number(options.sourceManifest?.bindings?.time?.horizon || 2000000000);
  const time = Number(options.blockTime);
  if (!Number.isFinite(time) || time >= horizon) fail('invalid time window');
  if (!options.operationalAdmission) fail('missing operational admission');
  return true;
}

export async function runLocalFinancialCase(options) {
  validateCaseOptions(options);
  const admission = options.operationalAdmission;
  if (admission.allowLocalExecution !== true || admission.reviewed !== true) {
    fail('missing operational admission');
  }
  if (!options.walletContext) fail('live WalletContext handle required');
  const providers = await createFinancialProviders({
    walletContext: options.walletContext,
    networkConfig: admission.networkConfig || options.networkAdmission.networkConfig,
    provenAssetManifest: options.provenAssetManifest,
    privateStateLocation: options.privateStateLocation || admission.privateStateLocation,
    resourceCounters: options.resourceCounters || {submission: 1n, grossSpend: BigInt(options.limits.spend), reservedSubmission: 0n, reservedGross: 0n},
    onSubmission: options.onSubmission || ((id) => options.eventSink?.({kind: 'submitted', id})),
    eventSink: options.eventSink,
    deadlineMs: options.deadlineMs || (Date.now() + options.limits.deadlineMs),
    adapters: options.adapters,
  });
  const order = options.case === 'loan' ? ['deploy', 'initialize', 'accrue', 'settle'] : ['deploy', 'initialize', 'swap', 'close'];
  return {
    status: 'admitted-not-executed',
    case: options.case,
    order,
    providersBound: true,
    note: 'Execution still requires inspected proven assets and live local endpoints from the reviewed admission.',
  };
}

export function runExplainCli() {
  process.stdout.write(
    'run-local requires a reviewed operational admission. Missing operational admission fields: allowLocalExecution, reviewed, networkConfig, recovered identities, spend ceilings, absolute deadline.\n'
    + 'This CLI does not load secrets, invent identities, or start a network.\n',
  );
  return 0;
}

if (isDirectRun(import.meta.url)) {
  const argv = process.argv.slice(2);
  if (argv.includes('--explain') || argv.includes('--help') || argv.length === 0) {
    process.exitCode = runExplainCli();
  } else {
    throw new Error('unknown CLI option');
  }
}

import {isDeepStrictEqual} from 'node:util';
import {randomBytes} from 'node:crypto';

const order = 6554484396890773809930967563523245729705921265872317281365359162392183254199n;

// A fresh rejection-sampled scalar is essential even when the same grant is signed twice.
function nonce() {
  for (;;) {
    const bytes = randomBytes(32);
    bytes[0] &= 15;
    const n = BigInt('0x'+bytes.toString('hex'));
    if (n > 0n && n < order) return n;
  }
}

export function signOrder(runtime, circuits, grant, secret) {
  if (typeof secret !== 'bigint' || secret <= 0n || secret >= order) throw RangeError('Invalid owner signing scalar');
  const owner = runtime.ecMulGenerator(secret);
  if (owner.x !== grant.owner.x || owner.y !== grant.owner.y) throw Error('Owner key mismatch');
  const k = nonce();
  const announcement = runtime.ecMulGenerator(k);
  const c = circuits.orderChallenge(grant.terms,grant.contract,owner,grant.action,grant.revision,announcement);
  return {announcement,response:(k + c * secret) % order};
}

// Wallet enrollment and the application grant use different keys and signature schemes.
// A fixed-position tuple gives each role, amount, key coordinate and verifier exactly one encoding.
// The owner enrolls the complete installation: every entry point and verifier key, the
// maintenance committee and threshold, and any pinned contract fields such as feed keys.
const sorted = object => Object.entries(object ?? {}).sort(([a],[b]) => a < b ? -1 : a > b ? 1 : 0);
function enrollmentMessage(deployment,accepted) {
  const {terms,owner} = accepted;
  const hex = bytes => Buffer.from(bytes).toString('hex');
  if (!deployment.maintenance) throw Error('Missing pinned maintenance authority');
  return Buffer.from(JSON.stringify([
    'moriarty:wallet-order-enrollment:2',deployment.network,deployment.address,
    sorted(deployment.verifiers),deployment.maintenance.committee,deployment.maintenance.threshold,
    sorted(deployment.pins),
    hex(terms.network),hex(terms.orderId),hex(terms.program),hex(terms.account.bytes),
    hex(terms.recipient.bytes),hex(terms.executor.bytes),hex(terms.assetA),hex(terms.assetB),
    String(terms.epoch),String(terms.expiry),String(terms.input),String(terms.minimum),String(terms.fee),
    String(owner.x),String(owner.y),
  ]),'utf8');
}
// Only the enrolled owner wallet may maintain the order contract, alone and with threshold one.
function requireOwnerMaintenance(deployment,enrollment) {
  const {committee,threshold} = deployment.maintenance ?? {};
  if (threshold !== 1 || committee?.length !== 1 || !isDeepStrictEqual(committee[0], enrollment.key)) {
    throw Error('Order maintenance must belong to the enrolled owner wallet');
  }
}
export function verifyEnrollment(ledger,runtime,deployment,accepted,enrollment) {
  if (!enrollment) throw Error('Missing owner wallet enrollment');
  const address = runtime.decodeUserAddress(accepted.terms.account.bytes);
  if (ledger.addressFromKey(enrollment.key) !== address) throw Error('Wrong owner wallet for enrollment');
  requireOwnerMaintenance(deployment,enrollment);
  if (!ledger.verifySignature(enrollment.key,enrollmentMessage(deployment,accepted),enrollment.signature)) {
    throw Error('Invalid owner wallet enrollment signature');
  }
}
export function issueEnrollment(ledger,runtime,deployment,accepted,walletKey) {
  const key = ledger.signatureVerifyingKey(walletKey);
  const signature = ledger.signData(walletKey,enrollmentMessage(deployment,accepted));
  const enrollment = {key,signature};
  verifyEnrollment(ledger,runtime,deployment,accepted,enrollment);
  return enrollment;
}

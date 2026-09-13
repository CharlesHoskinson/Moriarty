/** Pure prover lifetime arithmetic and closed control-record bytes. No clock,
 * process, wallet, network, Docker or store is consulted by this module.
 */

export const PROVER_LATEST_START_SECONDS=120;
export const PROVER_KILL_DEADLINE_SECONDS=1620;
export const PROVER_MAX_RUNTIME_SECONDS=1500;
export const PROVER_CONTROL_SCHEMA='moriarty.prover-lifetime/1';
export const PROVER_CONTROL_MAX_BYTES=512;

const NS_PER_SECOND=1000000000n;
const check=(ok,code)=>{if(!ok)throw Error(code);};
const plain=o=>o&&Object.getPrototypeOf(o)===Object.prototype;
function exact(o,keys,code='PROVER_LIFETIME_CONTROL_FIELDS'){
 check(plain(o),code);const descriptors=Object.getOwnPropertyDescriptors(o);
 check(Reflect.ownKeys(descriptors).length===Object.keys(descriptors).length&&Object.values(descriptors).every(d=>Object.hasOwn(d,'value')&&d.enumerable)&&Object.keys(descriptors).sort().join('|')===keys.split(',').sort().join('|'),code);
}
const nonnegativeBigInt=value=>typeof value==='bigint'&&value>=0n;
const canonicalDecimal=value=>/^(0|[1-9][0-9]*)$/.test(value);
const uuid=value=>typeof value==='string'&&/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/.test(value);
const digest=value=>typeof value==='string'&&/^[0-9a-f]{64}$/.test(value);

export function computeProverLifetimeBounds(outerStartMonotonicNs){
 check(nonnegativeBigInt(outerStartMonotonicNs),'PROVER_LIFETIME_OUTER_START');
 return {
  latestStartMonotonicNs:outerStartMonotonicNs+BigInt(PROVER_LATEST_START_SECONDS)*NS_PER_SECOND,
  killDeadlineMonotonicNs:outerStartMonotonicNs+BigInt(PROVER_KILL_DEADLINE_SECONDS)*NS_PER_SECOND
 };
}

export function validateWrapperEntry(entryMonotonicNs,latestStartMonotonicNs){
 check(nonnegativeBigInt(entryMonotonicNs),'PROVER_LIFETIME_ENTRY');
 check(nonnegativeBigInt(latestStartMonotonicNs),'PROVER_LIFETIME_LATEST_START');
 check(entryMonotonicNs<latestStartMonotonicNs,'PROVER_LIFETIME_LATE_ENTRY');return true;
}

export function computeWrapperKillDeadline(entryMonotonicNs,controlKillDeadlineNs){
 check(nonnegativeBigInt(entryMonotonicNs),'PROVER_LIFETIME_ENTRY');
 check(nonnegativeBigInt(controlKillDeadlineNs),'PROVER_LIFETIME_CONTROL_DEADLINE');
 const own=entryMonotonicNs+BigInt(PROVER_MAX_RUNTIME_SECONDS)*NS_PER_SECOND;
 return own<controlKillDeadlineNs?own:controlKillDeadlineNs;
}

function inode(value){
 check(typeof value==='bigint'?value>=0n:Number.isSafeInteger(value)&&value>=0,'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE');
 return BigInt(value);
}

export function encodeProverControlRecord(value){
 exact(value,'bootId,timeNamespaceInode,latestStartMonotonicNs,killDeadlineMonotonicNs,invocationDigest');
 check(uuid(value.bootId),'PROVER_LIFETIME_CONTROL_BOOT_ID');const timeNamespaceInode=inode(value.timeNamespaceInode);
 check(nonnegativeBigInt(value.latestStartMonotonicNs),'PROVER_LIFETIME_CONTROL_LATEST_START');
 check(nonnegativeBigInt(value.killDeadlineMonotonicNs),'PROVER_LIFETIME_CONTROL_KILL_DEADLINE');
 check(digest(value.invocationDigest),'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST');
 const encoded=Buffer.from(`${PROVER_CONTROL_SCHEMA}\n${value.bootId}\n${timeNamespaceInode}\n${value.latestStartMonotonicNs}\n${value.killDeadlineMonotonicNs}\n${value.invocationDigest}\n`,'ascii');
 check(encoded.length<=PROVER_CONTROL_MAX_BYTES,'PROVER_LIFETIME_CONTROL_SIZE');return encoded;
}

export function decodeProverControlRecord(buffer){
 check(Buffer.isBuffer(buffer),'PROVER_LIFETIME_CONTROL_BUFFER');
 check(buffer.length<=PROVER_CONTROL_MAX_BYTES,'PROVER_LIFETIME_CONTROL_SIZE');
 check(!buffer.includes(0x0d),'PROVER_LIFETIME_CONTROL_CR');
 check([...buffer].every(byte=>byte<=0x7f),'PROVER_LIFETIME_CONTROL_ASCII');
 const lines=buffer.toString('ascii').split('\n');check(lines.length===7&&lines[6]==='','PROVER_LIFETIME_CONTROL_LINES');
 const [schema,bootId,timeNamespace,latestStart,killDeadline,invocationDigest]=lines;
 check(schema===PROVER_CONTROL_SCHEMA,'PROVER_LIFETIME_CONTROL_SCHEMA');
 check(uuid(bootId),'PROVER_LIFETIME_CONTROL_BOOT_ID');
 check(canonicalDecimal(timeNamespace),'PROVER_LIFETIME_CONTROL_TIME_NAMESPACE');
 check(canonicalDecimal(latestStart),'PROVER_LIFETIME_CONTROL_LATEST_START');
 check(canonicalDecimal(killDeadline),'PROVER_LIFETIME_CONTROL_KILL_DEADLINE');
 check(digest(invocationDigest),'PROVER_LIFETIME_CONTROL_INVOCATION_DIGEST');
 return {bootId,timeNamespaceInode:BigInt(timeNamespace),latestStartMonotonicNs:BigInt(latestStart),killDeadlineMonotonicNs:BigInt(killDeadline),invocationDigest};
}

const identity=value=>typeof value==='bigint'||typeof value==='number'?String(value):value;
export function validateClockIdentity(value){
 exact(value,'parentBootId,parentTimeNamespaceInode,childBootId,childTimeNamespaceInode','PROVER_LIFETIME_CLOCK_IDENTITY');
 check(value.parentBootId===value.childBootId&&identity(value.parentTimeNamespaceInode)===identity(value.childTimeNamespaceInode),'PROVER_LIFETIME_CLOCK_IDENTITY');return true;
}

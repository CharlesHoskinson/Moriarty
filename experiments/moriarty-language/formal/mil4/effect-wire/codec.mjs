// W-D2E finite candidate. Equality is not authentication or ledger admission.
import { createHash } from 'node:crypto';
import { decodeAuthorization } from '../wire/codec.mjs';

export const HEADER = 'moriarty-s0-effects/1\0';
export const CAPS = Object.freeze({ recordBytes: 4096, id: 64, effects: 6,
  balances: 3, allowances: 1, obligations: 1, replay: 16,
  u64: (1n<<64n)-1n, u128: (1n<<128n)-1n, nominal: (1n<<127n)-1n });
const PREMISES = ['canonical-intent-signature', 'snapshot-to-head', 'head-extension',
  'atomic-ledger-compare-and-consume'];
const status = ['Outstanding', 'Settled'];
const effects = {
  Debit: [1, [['account','id'],['asset','id'],['amount','nominal']]],
  Credit: [2, [['account','id'],['asset','id'],['amount','nominal']]],
  SetObligation: [3, [['id','id'],['principal','nominal'],['accrued','nominal'],
    ['outstanding','nominal'],['status','status']]],
  UseAllowance: [4, [['owner','id'],['amount','nominal']]],
  UseReplay: [5, [['key','replay']]],
  AdvanceHead: [6, [['predecessor','hash'],['successor','hash']]],
};
const balance = [['account','id'],['before','u128'],['after','u128']];
const allowance = [['owner','id'],['remainingBefore','u128'],['spentBefore','u128'],
  ['remainingAfter','u128'],['spentAfter','u128']];
const obligation = [['id','id'],['debtor','id'],['creditor','id'],['asset','id'],
  ['principalBefore','nominal'],['accruedBefore','nominal'],['outstandingBefore','nominal'],
  ['statusBefore','status'],['principalAfter','nominal'],['accruedAfter','nominal'],
  ['outstandingAfter','nominal'],['statusAfter','status']];
const footprint = [['balances','balances'],['allowances','allowances'],['obligations','obligations']];
const consumption = [['workRemainingBefore','u128'],['workSpentBefore','u128'],
  ['workRemainingAfter','u128'],['workSpentAfter','u128'],
  ['replayBefore','replays'],['replayAfter','replays']];
const root = [['core','literal','moriarty-core/5',5],['domain','id'],['asset','id'],
  ['scale','scale'],['operationKind','operationKind'],['round','u64'],['preHead','hash'],
  ['successor','hash'],['effects','effects'],['footprint','footprint'],
  ['consumption','consumption'],['requiredPremises','premises']];

function fail(code, field) { const e = new Error(`${code}: ${field}`); e.code=code; throw e; }
function own(value, key, name) {
  const d=Object.getOwnPropertyDescriptor(value,key);
  if (!d || !('value' in d) || !d.enumerable) fail('SHAPE',name);
  return d.value;
}
function snapshot(value, names, name, captured = {}) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE',name);
  const keys=Reflect.ownKeys(value);
  if (keys.length!==names.length || keys.some(k=>typeof k!=='string'||!names.includes(k))) fail('SHAPE',name);
  const result=Object.create(null);
  for (const key of names) result[key]=Object.hasOwn(captured,key) ? captured[key] : own(value,key,`${name}.${key}`);
  return result;
}
function arraySnapshot(value, max, name) {
  if (!Array.isArray(value)) fail('SHAPE',name);
  const d=Object.getOwnPropertyDescriptor(value,'length');
  if (!d || !('value' in d) || !Number.isInteger(d.value) || d.value<0) fail('SHAPE',name);
  const length=d.value;
  if (length>max) fail('LENGTH',name);
  const names=Array.from({length},(_,i)=>String(i));
  const keys=Reflect.ownKeys(value);
  if (keys.length!==length+1 || keys.some(k=>k!=='length'&&!names.includes(k))) fail('SHAPE',name);
  return names.map(key=>own(value,key,`${name}.${key}`));
}
function uint(value, width) {
  const out=Buffer.alloc(width);
  for(let i=width-1;i>=0;i--){out[i]=Number(value&255n);value>>=8n;}
  return out;
}
function concat(parts,name) {
  const size=parts.reduce((n,b)=>n+b.length,0);
  if(size>CAPS.recordBytes) fail('LENGTH',name);
  return Buffer.concat(parts,size);
}
function record(value,schema,name,captured) {
  const snap=snapshot(value,schema.map(f=>f[0]),name,captured);
  return concat(schema.map(([key,type,literal,byte])=>primitive(snap[key],type,`${name}.${key}`,literal,byte)),name);
}
function line(value,name) {
  if(!value || typeof value!=='object' || Array.isArray(value)) fail('SHAPE',name);
  const kind=own(value,'kind',`${name}.kind`);
  if(typeof kind!=='string' || !Object.hasOwn(effects,kind)) fail('VARIANT',`${name}.kind`);
  const [tag,schema]=effects[kind];
  return record(value,[['kind','literal',kind,tag],...schema],name,{kind});
}
function rows(value,max,name,encode) {
  const values=arraySnapshot(value,max,name);
  return concat([uint(BigInt(values.length),2),...values.map((x,i)=>encode(x,`${name}.${i}`))],name);
}
function primitive(value,type,name,literal,byte) {
  switch(type) {
    case 'literal': if(value!==literal) fail('LITERAL',name); return Buffer.from([byte]);
    case 'id': {
      if(typeof value!=='string') fail('ID',name);
      if(value.length>CAPS.id) fail('LENGTH',name);
      if(!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID',name);
      const bytes=Buffer.from(value,'ascii');
      return Buffer.concat([uint(BigInt(bytes.length),2),bytes]);
    }
    case 'hash':
      if(typeof value!=='string'||value.length!==64||!/^[0-9a-f]{64}$/.test(value)) fail('HEX',name);
      return Buffer.from(value,'hex');
    case 'u64': case 'u128': case 'nominal': {
      if(typeof value!=='string') fail('INTEGER',name);
      if(value.length>(type==='u64'?20:39)) fail('RANGE',name);
      if(!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER',name);
      const n=BigInt(value);
      if(n>CAPS[type]) fail('RANGE',name);
      return uint(n,type==='u64'?8:16);
    }
    case 'scale':
      if(!Number.isInteger(value)||Object.is(value,-0)||value<0||value>38) fail('RANGE',name);
      return Buffer.from([value]);
    case 'status': case 'operationKind': {
      const variants=type==='status'?status:['transfer','repayment'];
      const i=variants.indexOf(value);
      if(i<0) fail('VARIANT',name);
      return Buffer.from([i+1]);
    }
    case 'replay': {
      if(typeof value!=='string'||value.length>202) fail('REPLAY',name);
      let parts;
      try { parts=JSON.parse(value); } catch { fail('REPLAY',name); }
      if(!Array.isArray(parts)||parts.length!==3||JSON.stringify(parts)!==value) fail('REPLAY',name);
      try { return Buffer.concat([primitive(parts[0],'id',name),primitive(parts[1],'id',name),primitive(parts[2],'hash',name)]); }
      catch { fail('REPLAY',name); }
    }
    case 'effects': return rows(value,CAPS.effects,name,line);
    case 'balances': return rows(value,CAPS.balances,name,(v,n)=>record(v,balance,n));
    case 'allowances': return rows(value,CAPS.allowances,name,(v,n)=>record(v,allowance,n));
    case 'obligations': return rows(value,CAPS.obligations,name,(v,n)=>record(v,obligation,n));
    case 'replays': return rows(value,CAPS.replay,name,(v,n)=>primitive(v,'replay',n));
    case 'footprint': return record(value,footprint,name);
    case 'consumption': return record(value,consumption,name);
    case 'premises': {
      const values=arraySnapshot(value,4,name);
      if(values.length!==4||values.some((v,i)=>v!==PREMISES[i])) fail('LITERAL',name);
      return Buffer.from([15]);
    }
    default: fail('SHAPE',name);
  }
}
export function encodeEffects(value) {
  const snap=snapshot(value,['schemaVersion',...root.map(f=>f[0])],'effects');
  if(snap.schemaVersion!=='moriarty-s0-effects/1') fail('LITERAL','schemaVersion');
  return concat([Buffer.from(HEADER,'ascii'),...root.map(([k,t,l,b])=>primitive(snap[k],t,k,l,b))],'effects');
}
export function effectCommitment(prepared) {
  return createHash('sha256').update(encodeEffects(prepared)).digest('hex');
}
export function compareEffectCommitment(canonicalAuthorizationBytes,prepared) {
  const authorization=decodeAuthorization(canonicalAuthorizationBytes);
  const commitment=effectCommitment(prepared);
  return authorization.effectCommitment===commitment
    ? {status:'CommitmentEqualUnqualified',commitment}
    : {status:'Rejected',code:'EFFECT_COMMITMENT_MISMATCH'};
}

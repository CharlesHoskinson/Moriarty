// Pre-proof public guard. Pure: no fs, SDK, wallet, network or native. The bound-D callback calls it after the PI helper
// and before link.bindCallback, the before-proof PI record and route.prove: it checks the helper vector, then derives the
// public capture bytes from the pinned private projection against independently supplied expected data through the
// unchanged capture-public derivePublic. Nothing expected is taken from the projection; missing expected data refuses.
// A pass is typed equality only, not proof that the expected data is authentic; that stays with the caller.
import {derivePublic,CODES as CPUB} from '../integration/capture-public.mjs';
export const PI_COUNT=15;
export const GUARD_CODES=Object.freeze(['ARGS','EXPECTED','PI_COUNT','PI_FORM','PI_RANGE','BINDING','UNCLASSIFIED'].map(c=>'ERR_PPG_'+c));
const P=BigInt('0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001'),HEX=/^[0-9a-f]{64}$/;
const req=(ok,c)=>{if(!ok)throw Error('ERR_PPG_'+c);};
const hex=v=>typeof v==='string'&&HEX.test(v);
// handkey fr_hex form: 32-byte little-endian lowercase hex.
const field=v=>BigInt('0x'+Buffer.from(v,'hex').reverse().toString('hex'));
// The expected slot must be caller-frozen data all the way down, so it cannot change after it was supplied.
const frozenDeep=v=>v===null||typeof v!=='object'||Object.isFrozen(v)&&Object.values(v).every(frozenDeep);
function check({pis,bindingLe,projection,expected}){
  req(hex(bindingLe)&&projection instanceof Uint8Array,'ARGS');
  req(expected!==null&&typeof expected==='object'&&frozenDeep(expected),'EXPECTED');
  req(Array.isArray(pis),'PI_COUNT');
  const v=[...pis]; // snapshot: holes become undefined and later caller writes cannot reach the checks
  req(v.length===PI_COUNT,'PI_COUNT');
  req(v.every(hex),'PI_FORM');
  req(v.every(x=>field(x)<P),'PI_RANGE');
  req(v[0]===bindingLe,'BINDING');
  const out=derivePublic(Buffer.from(projection),expected);
  return Object.freeze({schema:out.schema,canonical:Buffer.from(out.canonical),sha256:out.sha256});
}
// Every refusal is a fresh Error(fixed code) without cause; capture-public codes pass through, foreign throws collapse.
export function guardPreproofPublic(input){
  try{return check(input??{});}catch(e){let m=null;try{m=e instanceof Error?e.message:null;}catch{}throw Error(GUARD_CODES.includes(m)||CPUB.includes(m)?m:'ERR_PPG_UNCLASSIFIED');}
}

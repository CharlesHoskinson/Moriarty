/** Admission registry for this semantic profile. The digest pins exact file bytes,
 * including whitespace and every domain entry; a matching schema label is insufficient.
 */
import {readFileSync} from 'node:fs';
import {hashRawDomain,utf8} from './codec.ts';
import {diagnostic,throwDiagnostic} from './diagnostics.ts';
import type {Bounds} from './types.ts';
export const REGISTERED_BOUNDS_HASH='ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c';
export const REGISTERED_BOUNDS_UTF8_BYTES=8861;
const typedArrayByteLength=Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype),'byteLength')!.get!;
const BOUNDS_DOMAIN='MORIARTY-BOUNDS-bounded-atomic/1';
let registeredText:string|undefined;
function invalid():never{return throwDiagnostic(diagnostic('PROGRAM_ENCODING'));}
/** Returns fresh copies so a caller cannot alter the admitted registry in memory. */
export function registeredBounds():{bytes:Uint8Array;bounds:Bounds} {
  if(registeredText===undefined){
    let bytes:Uint8Array;
    try{bytes=readFileSync(new URL('../spec/bounds.json',import.meta.url));}catch{return invalid();}
    if(bytes.byteLength!==REGISTERED_BOUNDS_UTF8_BYTES||hashRawDomain(BOUNDS_DOMAIN,bytes)!==REGISTERED_BOUNDS_HASH)return invalid();
    // The pinned bytes are a reviewed well-formed JSON document. Never parse user bounds.
    registeredText=new TextDecoder('utf-8',{fatal:true}).decode(bytes);
  }
  return {bytes:utf8(registeredText),bounds:JSON.parse(registeredText) as Bounds};
}
/** Accept only the registered byte sequence. No configurable limit/domain override. */
export function admitRegisteredBounds(input:unknown):void {
  let bytes:Uint8Array;
  if(typeof input==='string'){
    if(input.length>REGISTERED_BOUNDS_UTF8_BYTES)return invalid();
    bytes=utf8(input);
    if(bytes.byteLength!==REGISTERED_BOUNDS_UTF8_BYTES)return invalid();
  }
  else {
    // Reject proxies and non-byte views before invoking native crypto APIs. Copying
    // also prevents a mutable/shared caller buffer becoming the execution registry.
    if(!ArrayBuffer.isView(input)||!(input instanceof Uint8Array))return invalid();
    if(typedArrayByteLength.call(input)!==REGISTERED_BOUNDS_UTF8_BYTES)return invalid();
    try{bytes=new Uint8Array(input);}catch(error){if(error instanceof TypeError)return invalid();throw error;}
  }
  if(hashRawDomain(BOUNDS_DOMAIN,bytes)!==REGISTERED_BOUNDS_HASH)return invalid();
}

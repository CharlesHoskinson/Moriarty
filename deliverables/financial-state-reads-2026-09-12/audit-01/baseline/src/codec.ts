import { createHash } from 'node:crypto';
import type { EncodingLimits, Span } from './types.ts';
export class FrontendError extends Error {
  code:string;
  primarySpan:Span;
  constructor(code:string, message:string, span:Span = {startByte:'0',endByte:'0'}) { super(`${code}: ${message}`); this.name='FrontendError'; this.code=code; this.primarySpan=span; }
}
export function fail(code:string,message:string,span?:Span):never { throw new FrontendError(code,message,span); }
export const utf8 = (s:string):Uint8Array => new TextEncoder().encode(s);
export function scalarText(s:string):boolean {
  for(let i=0;i<s.length;i++) { const c=s.charCodeAt(i); if(c>=0xd800&&c<=0xdbff) { const d=s.charCodeAt(++i); if(!(d>=0xdc00&&d<=0xdfff))return false; } else if(c>=0xdc00&&c<=0xdfff)return false; } return true;
}
export function sourceText(input:string|Uint8Array):string {
  if(typeof input==='string') { if(!scalarText(input))fail('INVALID_UTF8','lone surrogate in source'); return input; }
  try { return new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(input); } catch { return fail('INVALID_UTF8','source is not well-formed UTF-8'); }
}
export function sha256(bytes:string|Uint8Array):string { return createHash('sha256').update(bytes).digest('hex'); }
export function hashDomain(domain:string,value:unknown):string { return createHash('sha256').update(utf8(domain)).update(new Uint8Array([0])).update(utf8(canonicalEncode(value))).digest('hex'); }
export function hashRawDomain(domain:string,bytes:Uint8Array):string { return createHash('sha256').update(utf8(domain)).update(new Uint8Array([0])).update(bytes).digest('hex'); }
/** Canonical value codec; callers separately enforce the closed record schema. */
export function canonicalEncode(value:unknown):string {
  const active = new Set<object>();
  function emit(v:unknown,depth:number):string {
    if(depth>256)fail('ENCODING_DEPTH','codec nesting exceeds defensive traversal ceiling');
    if(typeof v==='string') { if(!scalarText(v))fail('INVALID_UNICODE','lone surrogate'); return JSON.stringify(v); }
    if(typeof v==='boolean')return v?'true':'false';
    if(v===null||typeof v!=='object')fail('NON_CANONICAL_VALUE','only records, arrays, strings and booleans are allowed');
    if(active.has(v))fail('CYCLIC_VALUE','cyclic object'); active.add(v);
    let out:string;
    if(Array.isArray(v)) {
      const descriptors=Object.getOwnPropertyDescriptors(v);
      const keys=Reflect.ownKeys(v);
      if(keys.length!==v.length+1||keys.some(key=>typeof key!=='string'||key!=='length'&&(!/^(0|[1-9][0-9]*)$/.test(key)||Number(key)>=v.length)))fail('NON_CANONICAL_VALUE','sparse or decorated array');
      const elements:string[]=[];
      for(let i=0;i<v.length;i++){const d=descriptors[String(i)];if(!d||!Object.hasOwn(d,'value')||!d.enumerable)fail('NON_CANONICAL_VALUE','sparse or accessor array');elements.push(emit(d.value,depth+1));}
      out='['+elements.join(',')+']';
    }
    else {
      if(Object.getPrototypeOf(v)!==Object.prototype&&Object.getPrototypeOf(v)!==null)fail('NON_CANONICAL_VALUE','record prototype');
      const keys=Object.keys(v).sort();
      const descriptors=Object.getOwnPropertyDescriptors(v);
      if(keys.some(k=>!Object.hasOwn(descriptors[k],'value')))fail('NON_CANONICAL_VALUE','accessor record');
      if(Reflect.ownKeys(v).length!==keys.length)fail('NON_CANONICAL_VALUE','non-enumerable or symbol key');
      if(keys.some(k=>!(/^[\x20-\x7e]+$/.test(k))))fail('NON_ASCII_KEY','record keys must be ASCII');
      out='{'+keys.map(k=>JSON.stringify(k)+':'+emit(descriptors[k].value,depth+1)).join(',')+'}';
    }
    active.delete(v); return out;
  }
  return emit(value,0);
}
export function canonicalDecode(bytes:string|Uint8Array,validate?:(value:unknown)=>void):unknown {
  const text=sourceText(bytes); let value:unknown;
  try { value=JSON.parse(text); } catch { return fail('INVALID_JSON','invalid JSON'); }
  if(canonicalEncode(value)!==text)fail('NON_CANONICAL_BYTES','decode/re-encode mismatch (including duplicate keys)');
  validate?.(value); return value;
}
export type EncodingMetrics = {utf8Bytes:number;decodedDepthRootZero:number;decodedNodes:number;keysPerRecord:number;arrayLength:number;textUtf8Bytes:number;textJavascriptCodeUnits:number};
export function measureEncoding(value:unknown):EncodingMetrics {
  const encoded=canonicalEncode(value);
  const metrics:EncodingMetrics={utf8Bytes:utf8(encoded).length,decodedDepthRootZero:0,decodedNodes:0,keysPerRecord:0,arrayLength:0,textUtf8Bytes:0,textJavascriptCodeUnits:0};
  function visit(v:unknown,depth:number):void {
    metrics.decodedNodes++; metrics.decodedDepthRootZero=Math.max(metrics.decodedDepthRootZero,depth);
    if(typeof v==='string') { metrics.textUtf8Bytes=Math.max(metrics.textUtf8Bytes,utf8(v).length); metrics.textJavascriptCodeUnits=Math.max(metrics.textJavascriptCodeUnits,v.length); }
    else if(Array.isArray(v)) { metrics.arrayLength=Math.max(metrics.arrayLength,v.length); v.forEach(x=>visit(x,depth+1)); }
    else if(typeof v==='object'&&v!==null) { const keys=Object.keys(v); metrics.keysPerRecord=Math.max(metrics.keysPerRecord,keys.length); for(const key of keys) { metrics.textUtf8Bytes=Math.max(metrics.textUtf8Bytes,utf8(key).length); metrics.textJavascriptCodeUnits=Math.max(metrics.textJavascriptCodeUnits,key.length); visit((v as Record<string,unknown>)[key],depth+1); } }
  }
  visit(value,0); return metrics;
}
export function checkEncoding(value:unknown,limits:EncodingLimits,label:string):EncodingMetrics {
  const metrics=measureEncoding(value); for(const key of Object.keys(metrics) as (keyof EncodingMetrics)[]) { if(!Number.isSafeInteger(limits[key])||limits[key]<1&&key!=='decodedDepthRootZero')fail('INVALID_BOUNDS',`${label}.${key}`); if(metrics[key]>limits[key])fail('ENCODING_BOUND',`${label}.${key}: ${metrics[key]} > ${limits[key]}`); } return metrics;
}

/** Generic canonical JSON decoding alone is not a profile/schema acceptance check.
 * This entry point requires an explicit closed-schema validator, including bounds.
 */
export function decodeCanonicalRecord<T>(bytes:string|Uint8Array,validate:(value:unknown)=>asserts value is T):T {
  if(typeof validate!=='function')fail('NON_CANONICAL_VALUE','a closed-schema validator is required');
  const value=canonicalDecode(bytes);validate(value);return value;
}

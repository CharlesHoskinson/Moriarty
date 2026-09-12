import { parseSource as rawParseSource } from './parser.ts';
import { checkAndLower } from './checker.ts';
import { registeredBounds, admitRegisteredBounds } from './registered-bounds.ts';
import { validateSource, assertSourceShape } from './validate.ts';
import { normalizeError, throwDiagnostic, type Diagnostic } from './diagnostics.ts';
import type { SourceAST, TypedProgram, BoundProgram } from './types.ts';
export { canonicalEncode, canonicalDecode, decodeCanonicalRecord, measureEncoding, checkEncoding, FrontendError, hashDomain, hashRawDomain, sha256 } from './codec.ts';
export type * from './types.ts';
export type { Diagnostic, DiagnosticCode } from './diagnostics.ts';
/** Throwing complete-syntax and closed SourceAST-shape convenience API. */
export function parseSource(input:string|Uint8Array):SourceAST{try{const source=rawParseSource(input);assertSourceShape(source);return source;}catch(e){throwDiagnostic(normalizeError(e));}}
/** No caller-provided AST or typed records are trusted by this source-byte entry point. */
export function compile(input:string|Uint8Array,boundsInput:string|Uint8Array) {
  try {
    const source=rawParseSource(input),{bytes,bounds}=registeredBounds();
    const error=validateSource(source,bounds);if(error)throwDiagnostic(error);assertSourceShape(source);
    const lowered=checkAndLower(source,bytes);
    admitRegisteredBounds(boundsInput);
    return {source,...lowered};
  }catch(e){throwDiagnostic(normalizeError(e));}
}
/** Nonthrowing profile boundary: returns the record or one exact Diagnostic directly. */
export function parse(input:string|Uint8Array,boundsInput:string|Uint8Array):SourceAST|Diagnostic{
  try{const source=rawParseSource(input),{bounds}=registeredBounds();const error=validateSource(source,bounds,4);if(error)return error;assertSourceShape(source);admitRegisteredBounds(boundsInput);return source;}catch(e){return normalizeError(e);}
}
export function check(input:string|Uint8Array,boundsInput:string|Uint8Array):TypedProgram|Diagnostic{try{const source=rawParseSource(input),{bytes,bounds}=registeredBounds();const error=validateSource(source,bounds);if(error)return error;assertSourceShape(source);const typed=checkAndLower(source,bytes,false).typed;admitRegisteredBounds(boundsInput);return typed;}catch(e){return normalizeError(e);}}
export function elaborate(input:string|Uint8Array,boundsInput:string|Uint8Array):BoundProgram|Diagnostic{try{return compile(input,boundsInput).bound;}catch(e){return normalizeError(e);}}

import { parseSource as rawParseSource } from './parser.ts';
import { checkAndLower } from './checker.ts';
import { utf8, sourceText, FrontendError } from './codec.ts';
import { validateSource, assertSourceShape } from './validate.ts';
import { normalizeError, throwDiagnostic, type Diagnostic } from './diagnostics.ts';
import type { Bounds, SourceAST, TypedProgram, BoundProgram } from './types.ts';
export { canonicalEncode, canonicalDecode, decodeCanonicalRecord, measureEncoding, checkEncoding, FrontendError, hashDomain, hashRawDomain, sha256 } from './codec.ts';
export type * from './types.ts';
export type { Diagnostic, DiagnosticCode } from './diagnostics.ts';
function readBounds(input:string|Uint8Array):{bytes:Uint8Array;bounds:Bounds}{const bytes=typeof input==='string'?utf8(input):input;try{const bounds=JSON.parse(sourceText(bytes)) as Bounds;if(bounds.schemaVersion!=='moriarty-bounds/1'||bounds.semanticProfile!=='moriarty-bounded-atomic/1')throw new Error('registry');return {bytes,bounds};}catch{throwDiagnostic({code:'PROGRAM_ENCODING',message:'PROGRAM_ENCODING',stage:'8',primarySpan:{startByte:'0',endByte:'0'},relatedSpans:[]});}}
/** Throwing complete-syntax and closed SourceAST-shape convenience API. */
export function parseSource(input:string|Uint8Array):SourceAST{try{const source=rawParseSource(input);assertSourceShape(source);return source;}catch(e){throwDiagnostic(normalizeError(e));}}
/** No caller-provided AST or typed records are trusted by this source-byte entry point. */
export function compile(input:string|Uint8Array,boundsInput:string|Uint8Array) {
  try {
    const source=rawParseSource(input),{bytes,bounds}=readBounds(boundsInput);
    const error=validateSource(source,bounds);if(error)throwDiagnostic(error);assertSourceShape(source);
    return {source,...checkAndLower(source,bytes)};
  }catch(e){throwDiagnostic(normalizeError(e));}
}
/** Nonthrowing profile boundary: returns the record or one exact Diagnostic directly. */
export function parse(input:string|Uint8Array,boundsInput:string|Uint8Array):SourceAST|Diagnostic{
  try{const source=rawParseSource(input),{bounds}=readBounds(boundsInput);const error=validateSource(source,bounds,4);if(error)return error;assertSourceShape(source);return source;}catch(e){return normalizeError(e);}
}
export function check(input:string|Uint8Array,boundsInput:string|Uint8Array):TypedProgram|Diagnostic{try{const source=rawParseSource(input),{bytes,bounds}=readBounds(boundsInput);const error=validateSource(source,bounds);if(error)return error;assertSourceShape(source);return checkAndLower(source,bytes,false).typed;}catch(e){return normalizeError(e);}}
export function elaborate(input:string|Uint8Array,boundsInput:string|Uint8Array):BoundProgram|Diagnostic{try{return compile(input,boundsInput).bound;}catch(e){return normalizeError(e);}}

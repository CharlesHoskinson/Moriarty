import { parseSource } from './parser.ts';
import { checkAndLower } from './checker.ts';
import { utf8 } from './codec.ts';
export { parseSource } from './parser.ts';
export { canonicalEncode, canonicalDecode, measureEncoding, checkEncoding, FrontendError, hashDomain, hashRawDomain, sha256 } from './codec.ts';
export type * from './types.ts';
/** No caller-provided AST or typed records are trusted by this entry point. */
export function compile(input:string|Uint8Array,bounds:string|Uint8Array) {
  const source=parseSource(input);
  return {source,...checkAndLower(source,typeof bounds==='string'?utf8(bounds):bounds)};
}

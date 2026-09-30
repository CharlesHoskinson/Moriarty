/// <reference types="node" />
import type {SourceDefinition, Image, ContentComparison} from '../hash-image-codec/codec.mjs';
export const SOURCE_IMAGE_SUITE: Readonly<{sourceVersion: 6; wireProfile: 1;
  profile: 'moriarty-financial-agreement-source/6'; purpose: 1}>;
export class SourceImageAdapterError extends Error {
  code: string;
  scope: 'local-source-image-projection';
  constructor(code: string, message: string);
}
/** Forms Source/6 itself; never accepts an unverified caller AST. */
export function projectSource6Definition(source: string): Readonly<SourceDefinition>;
export function encodeSource6Definition(source: string): Image;
export function compareSource6DefinitionContent(source: string, claimedDigest: string): ContentComparison;

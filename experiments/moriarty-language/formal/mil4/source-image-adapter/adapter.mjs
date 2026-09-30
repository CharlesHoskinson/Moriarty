/** Selected-definition content projection only; formation is not authentication. */
import { parseSource6 } from '../../../src/successor/financial-agreement-source-v6-frontend.ts';
import { SOURCE_PROFILE, id, encodeImage, compareContent } from '../hash-image-codec/codec.mjs';

/** Local serialization-suite metadata; wireProfile is absent from Source6Ast. */
export const SOURCE_IMAGE_SUITE = Object.freeze({sourceVersion: 6, wireProfile: 1,
  profile: SOURCE_PROFILE, purpose: 1});

export class SourceImageAdapterError extends Error {
  constructor(code, message) {
    super(message);
    this.name = 'SourceImageAdapterError';
    this.code = code;
    this.scope = 'local-source-image-projection';
  }
}

/** Parse the entire Source/6 document, then privately project its owned AST.
 * No externally supplied AST, coercion, version override or claim fill is used.
 * Parser formation errors are propagated unchanged, including byte offset.
 */
export function projectSource6Definition(source) {
  if (typeof source !== 'string')
    throw new SourceImageAdapterError('SOURCE_TEXT_REQUIRED', 'Primitive Source/6 text required');
  const ast = parseSource6(source);
  return Object.freeze({
    sourceVersion: SOURCE_IMAGE_SUITE.sourceVersion,
    profile: ast.profile,
    wireProfile: SOURCE_IMAGE_SUITE.wireProfile,
    agreementInstanceId: id('agreement', ast.programId),
    domain: id('domain', ast.domain),
    asset: id('asset', ast.settlement.asset),
    scale: ast.settlement.scale,
    selectedActionId: id('action', ast.selected.actionId),
    operationKind: ast.intent.signedAction.kind,
  });
}

/** Purpose 1 only. Does not encode or inspect embedded source/policy claims. */
export function encodeSource6Definition(source) {
  return encodeImage(SOURCE_IMAGE_SUITE.purpose, projectSource6Definition(source));
}

/** Local content comparison; no registry/provider/signature authority supplied. */
export function compareSource6DefinitionContent(source, claimedDigest) {
  return compareContent(SOURCE_IMAGE_SUITE.purpose, projectSource6Definition(source), claimedDigest);
}

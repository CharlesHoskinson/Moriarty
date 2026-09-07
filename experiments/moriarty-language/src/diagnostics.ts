import type { Span } from './types.ts';
import { FrontendError } from './codec.ts';
export const frontendStages={SOURCE_ENCODING:1,LEXICAL_TOKEN:2,PARSE_ERROR:3,DECLARATION_SCHEMA:4,DUPLICATE_NAME:4,AST_BOUNDS:4,NAME_RESOLUTION:5,TYPE_MISMATCH:6,UINT_RANGE:6,UNIT_VECTOR:6,POLICY_TARGET:6,POLICY_ROUNDING:6,SETTLEMENT_DECLARATION:6,SETTLEMENT_BINDING_AMBIGUOUS:6,STATUS_RULE:6,ACTOR_PARAMETER:6,OBSERVATION_SCHEMA:6,RESERVE_RULE:6,PROGRAM_BOUNDS:7,SOURCE_MAP:8,PROGRAM_ENCODING:8} as const;
export type DiagnosticCode=keyof typeof frontendStages;
export type Diagnostic={code:DiagnosticCode;message:string;primarySpan:Span;relatedSpans:Span[];stage:string};
const aliases:Record<string,DiagnosticCode>={INVALID_UTF8:'SOURCE_ENCODING',SOURCE_BOUND:'SOURCE_ENCODING',LEXICAL_ERROR:'LEXICAL_TOKEN',INVALID_IDENTIFIER:'LEXICAL_TOKEN',INVALID_STRING:'LEXICAL_TOKEN',TEXT_BOUND:'LEXICAL_TOKEN',UNSUPPORTED_PROFILE:'DECLARATION_SCHEMA',ACTION_REQUIRED:'DECLARATION_SCHEMA',EFFECT_SCHEMA:'DECLARATION_SCHEMA',DUPLICATE_DECLARATION:'DUPLICATE_NAME',DUPLICATE_LOCAL:'DUPLICATE_NAME',DUPLICATE_WRITE:'DUPLICATE_NAME',UNRESOLVED_NAME:'NAME_RESOLUTION',UINT128_RANGE:'UINT_RANGE',LIFETIME_ZERO:'UINT_RANGE',NON_STORED_TYPE:'TYPE_MISMATCH',NON_STORED_VALUE:'TYPE_MISMATCH',POLICY_DUPLICATE_TARGET:'POLICY_TARGET',POLICY_UNIT:'POLICY_TARGET',POLICY_MISSING_TARGET:'POLICY_TARGET',SETTLEMENT_TYPE:'DECLARATION_SCHEMA',SETTLEMENT_BINDING:'SETTLEMENT_DECLARATION',STATUS_DUPLICATE:'STATUS_RULE',STATUS_REQUIRED:'STATUS_RULE',OBSERVATION_NOW_REQUIRED:'OBSERVATION_SCHEMA',RESERVE_GUARD:'RESERVE_RULE',SHAPE_BOUND:'PROGRAM_BOUNDS',ENCODING_BOUND:'PROGRAM_ENCODING',INVALID_BOUNDS:'PROGRAM_ENCODING'};
export function diagnostic(code:DiagnosticCode,primarySpan:Span={startByte:'0',endByte:'0'}):Diagnostic{return {code,message:code,primarySpan,relatedSpans:[],stage:String(frontendStages[code])};}
export function normalizeError(error:unknown,fallback:DiagnosticCode='PROGRAM_ENCODING'):Diagnostic {
  if(!(error instanceof FrontendError))throw error;
  const code=Object.hasOwn(frontendStages,error.code)?error.code as DiagnosticCode:aliases[error.code]??fallback;
  return diagnostic(code,error.primarySpan);
}
export function firstDiagnostic(errors:Diagnostic[]):Diagnostic|undefined{return errors.sort((a,b)=>Number(a.stage)-Number(b.stage)||Number(BigInt(a.primarySpan.startByte)-BigInt(b.primarySpan.startByte))||Number(BigInt(a.primarySpan.endByte)-BigInt(b.primarySpan.endByte))||(a.code<b.code?-1:a.code>b.code?1:0))[0];}
export function throwDiagnostic(d:Diagnostic):never{const error=new FrontendError(d.code,d.message,d.primarySpan);Object.assign(error,{diagnostic:d});throw error;}

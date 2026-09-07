import type { ActionDecl, AmountLiteral, BinaryTag, EffectKind, FieldPolicyDecl, PolicyTarget, Rounding, SourceAST, SourceDeclaration, SourceExpression, SourceLiteral, SourceType, Span, Statement, TextLiteral } from './types.ts';
import { fail, scalarText, sha256, sourceText, utf8 } from './codec.ts';
export const PROFILE='moriarty-bounded-atomic/1' as const;
export const UINT_MAX=(1n<<128n)-1n;
const keywords=new Set(('agreement profile lifetime horizon unit const state observation settlement asset quantum policy targets write derivation rounding remainder comparison proof none floor status episode closed_when remaining_notional no_remaining_notional effect action guard let set emit UInt128 Text Amount Transfer Fee DueCreated DueSettled uint text amount true false remaining floor_div arg obs not and or reserve for').split(' '));
const reserved=new Set(['constructor','prototype','__proto__']);
type Token={kind:'word'|'uint'|'string'|'punct'|'eof';text:string;start:number;end:number};
const span=(start:number,end:number):Span=>({endByte:String(end),startByte:String(start)});
export function checkUInt(token:string,where?:Span):string { if(!/^(0|[1-9][0-9]*)$/.test(token)||token.length>39||BigInt(token)>UINT_MAX)fail('UINT128_RANGE',`invalid UInt128 ${token}`,where); return token; }
function lex(source:string):Token[] {
  const tokens:Token[]=[]; let index=0,offset=0;
  function push(kind:Token['kind'],text:string):void {const end=offset+utf8(text).length; tokens.push({kind,text,start:offset,end}); index+=text.length;offset=end;}
  while(index<source.length) {
    const c=source[index];
    if(/[ \t\r\n]/.test(c)){index++;offset++;continue;}
    const suffix=source.slice(index);
    if(/[A-Za-z_]/.test(c)) { const match=/^[A-Za-z_][A-Za-z0-9_]*/.exec(suffix)![0]; if(match.length>64||reserved.has(match)||!(/^[A-Za-z]/.test(match)))fail('INVALID_IDENTIFIER',match,span(offset,offset+match.length)); push('word',match);continue; }
    if(/[0-9]/.test(c)) {const match=/^[0-9]+/.exec(suffix)![0];checkUInt(match,span(offset,offset+match.length));push('uint',match);continue;}
    if(c==='"') {
      let end=index+1,escaped=false;
      for(;end<source.length;end++){const s=source[end]; if(!escaped&&s==='"')break; if(!escaped&&s==='\\')escaped=true;else escaped=false;}
      if(end>=source.length)fail('INVALID_STRING','unterminated JSON string',span(offset,utf8(source).length));
      const text=source.slice(index,end+1);let decoded:unknown;
      try{decoded=JSON.parse(text);}catch{fail('INVALID_STRING','invalid JSON string',span(offset,offset+utf8(text).length));}
      if(typeof decoded!=='string'||!scalarText(decoded))fail('INVALID_STRING','JSON string must decode to Unicode scalars',span(offset,offset+utf8(text).length));
      if(utf8(decoded).length>256)fail('TEXT_BOUND','language text exceeds 256 UTF8 bytes',span(offset,offset+utf8(text).length));
      push('string',text);continue;
    }
    const operator=/^(==|<=|>=|[{}();,:.<>+*=-])/.exec(suffix);
    if(operator){push('punct',operator[0]);continue;}
    fail('LEXICAL_ERROR',`unexpected character ${JSON.stringify(c)}`,span(offset,offset+utf8(c).length));
  }
  tokens.push({kind:'eof',text:'',start:offset,end:offset});return tokens;
}
class Parser {
  tokens:Token[];index=0;last:Token;source:string;nesting=0;
  constructor(source:string){this.source=source;this.tokens=lex(source);this.last=this.tokens[0];}
  peek():Token{return this.tokens[this.index];}
  pop():Token{this.last=this.tokens[this.index++];return this.last;}
  at(text:string):boolean{return this.peek().text===text;}
  take(text:string):Token {if(!this.at(text))fail('PARSE_ERROR',`expected ${text}, got ${this.peek().text||'end of source'}`,span(this.peek().start,this.peek().end));return this.pop();}
  maybe(text:string):boolean{if(!this.at(text))return false;this.pop();return true;}
  extent(start:Token):Span{return span(start.start,this.last.end);}
  identifier():string {const token=this.peek();if(token.kind!=='word'||keywords.has(token.text)||reserved.has(token.text))fail('INVALID_IDENTIFIER',`expected identifier, got ${token.text}`,span(token.start,token.end));this.pop();return token.text;}
  field():string {if(this.at('asset')||this.at('amount'))return this.pop().text;return this.identifier();}
  uint():Token {const token=this.peek();if(token.kind!=='uint')fail('PARSE_ERROR','expected unsigned integer token',span(token.start,token.end));return this.pop();}
  text():TextLiteral {const token=this.peek();if(token.kind!=='string')fail('PARSE_ERROR','expected JSON string',span(token.start,token.end));this.pop();return {decoded:JSON.parse(token.text) as string,span:this.extent(token),tag:'TextLiteral',token:token.text};}
  literal():SourceLiteral {
    const start=this.peek();
    if(this.maybe('true')||this.maybe('false'))return {span:this.extent(start),tag:'BoolLiteral',value:start.text==='true'};
    if(this.maybe('uint')){this.take('(');const token=this.uint().text;this.take(')');return {span:this.extent(start),tag:'UIntLiteral',token};}
    if(this.maybe('amount')){this.take('(');const token=this.uint().text;this.take(',');const unit=this.identifier();this.take(')');return {span:this.extent(start),tag:'AmountLiteral',token,unit};}
    if(this.maybe('text')){this.take('(');const value=this.text();this.take(')');return {...value,span:this.extent(start)};}
    return fail('PARSE_ERROR','expected typed literal',span(start.start,start.end));
  }
  type(effect=false):SourceType {
    const start=this.peek();
    if(this.maybe('UInt128')||this.maybe('Text'))return {tag:start.text as 'UInt128'|'Text',span:this.extent(start)};
    this.take('Amount');
    if(effect&&!this.at('<'))return {tag:'BareAmount',span:this.extent(start)};
    this.take('<');const unit=this.identifier();this.take('>');return {tag:'Amount',unit,span:this.extent(start)};
  }
  effectKind():EffectKind{const t=this.peek();if(!['Transfer','Fee','DueCreated','DueSettled'].includes(t.text))fail('PARSE_ERROR','unknown effect kind',span(t.start,t.end));this.pop();return t.text as EffectKind;}
  expression():SourceExpression{return this.binary(0);}
  binary(level:number):SourceExpression {
    const groups:Record<string,BinaryTag>[]=[{or:'Or'},{and:'And'},{'==':'Eq','<':'Lt','<=':'Lte','>':'Gt','>=':'Gte'},{'+':'Add','-':'Sub'},{'*':'Mul'}];
    if(level===groups.length)return this.unary();
    let left=this.binary(level+1);let used=false;
    while(Object.hasOwn(groups[level],this.peek().text)) {if(level===2&&used)fail('PARSE_ERROR','comparisons cannot chain',span(this.peek().start,this.peek().end));used=true;const tag=groups[level][this.pop().text];const right=this.binary(level+1);left={left,right,span:{startByte:left.span.startByte,endByte:right.span.endByte},tag};}
    return left;
  }
  unary():SourceExpression {const start=this.peek();if(this.maybe('not')){const operand=this.primary();return {operand,span:this.extent(start),tag:'Not'};}return this.primary();}
  primary():SourceExpression {
    const start=this.peek();
    if(['uint','amount','text','true','false'].includes(start.text)){const literal=this.literal();return {literal,span:literal.span,tag:'Literal'};}
    if(this.maybe('remaining'))return {span:this.extent(start),tag:'Remaining'};
    if(this.at('(')||this.at('floor_div')) {
      this.nesting++;if(this.nesting>128)fail('PARSE_DEPTH','parenthesized syntax exceeds defensive parser nesting ceiling',span(start.start,start.end));
      let result:SourceExpression;
      if(this.maybe('(')){result=this.expression();this.take(')');result={...result,span:this.extent(start)};}
      else {this.take('floor_div');this.take('(');const numerator=this.expression();this.take(',');const denominator=this.expression();this.take(')');result={denominator,numerator,span:this.extent(start),tag:'FloorDiv'};}
      this.nesting--;return result;
    }
    const refs={state:'StateRef',arg:'ArgRef',obs:'ObservationRef',const:'ConstRef'} as const;
    if(Object.hasOwn(refs,start.text)){this.pop();this.take('.');const name=this.identifier();return {name,span:this.extent(start),tag:refs[start.text as keyof typeof refs]};}
    const name=this.identifier();return {name,span:this.extent(start),tag:'LocalRef'};
  }
  target():PolicyTarget {
    const start=this.peek();
    if(this.maybe('write')){this.take('(');const action=this.identifier();this.take(',');const field=this.identifier();this.take(')');return {action,field,span:this.extent(start),tag:'Write'};}
    this.take('effect');this.take('(');const action=this.identifier();this.take(',');const ordinal=this.uint().text;this.take(',');const field=this.field();this.take(')');return {action,field,ordinal,span:this.extent(start),tag:'Effect'};
  }
  statement():Statement {
    const start=this.peek();
    if(this.maybe('guard')){const condition=this.expression();this.take(',');const message=this.text();this.take(';');return {condition,message,span:this.extent(start),tag:'Guard'};}
    if(this.maybe('let')){const name=this.identifier();this.take('=');const expression=this.expression();this.take(';');return {expression,name,span:this.extent(start),tag:'Let'};}
    if(this.maybe('set')){const field=this.identifier();this.take('=');const expression=this.expression();this.take(';');return {expression,field,span:this.extent(start),tag:'Set'};}
    this.take('emit');const kind=this.effectKind();this.take('{');const fields=[];
    do{const fieldStart=this.peek();const label=this.field();this.take(':');const expression=this.expression();fields.push({expression,label,span:this.extent(fieldStart)});}while(this.maybe(','));
    this.take('}');this.take(';');return {fields,kind,span:this.extent(start),tag:'Emit'};
  }
  declaration():SourceDeclaration {
    const start=this.peek();
    if(this.maybe('unit')){const name=this.identifier();this.take(';');return {name,span:this.extent(start),tag:'UnitDecl'};}
    if(this.maybe('const')||this.maybe('state')){const name=this.identifier();this.take(':');const type=this.type();this.take('=');const value=this.literal();this.take(';');return {name,span:this.extent(start),tag:start.text==='const'?'ConstDecl':'StateDecl',type,value};}
    if(this.maybe('observation')){const name=this.identifier();this.take(':');const type=this.type();this.take(';');return {name,span:this.extent(start),tag:'ObservationDecl',type};}
    if(this.maybe('settlement')){const name=this.identifier();this.take('asset');const asset=this.literal();this.take('quantum');const quantum=this.literal();this.take(';');if(asset.tag!=='TextLiteral'||quantum.tag!=='AmountLiteral')fail('SETTLEMENT_TYPE','settlement requires Text asset and Amount quantum',this.extent(start));return {asset,name,quantum,span:this.extent(start),tag:'SettlementDecl'};}
    if(this.maybe('reserve')){const action=this.identifier();this.take('for');const closure=this.identifier();this.take(';');return {action,closure,span:this.extent(start),tag:'ReserveDecl'};}
    if(this.maybe('status')) {
      if(this.maybe('episode')){this.take('closed_when');const field=this.identifier();this.take('==');const literal=this.literal();this.take(';');return {field,literal,span:this.extent(start),tag:'EpisodeStatusDecl'};}
      this.take('agreement');if(this.maybe('no_remaining_notional')){this.take(';');return {span:this.extent(start),tag:'NoNotionalStatusDecl'};}
      this.take('remaining_notional');const field=this.identifier();this.take(';');return {field,span:this.extent(start),tag:'NotionalStatusDecl'};
    }
    if(this.maybe('policy')) {
      const name=this.identifier();this.take('targets');const targets=[this.target()];while(this.maybe(','))targets.push(this.target());this.take('{');this.take('unit');const unit=this.identifier();this.take(';');
      this.take('derivation');const derivation=this.text();this.take(';');this.take('rounding');const rs=this.peek();let rounding:Rounding;
      if(this.maybe('none'))rounding={span:this.extent(rs),tag:'None'};else{this.take('floor');this.take('(');const action=this.identifier();this.take(',');const local=this.identifier();this.take(')');rounding={action,local,span:this.extent(rs),tag:'Floor'};}
      this.take(';');this.take('remainder');const remainder=this.text();this.take(';');this.take('comparison');const comparison=this.text();this.take(';');this.take('proof');const proof=this.text();this.take(';');this.take('}');
      return {comparison,derivation,name,proof,remainder,rounding,span:this.extent(start),tag:'FieldPolicyDecl',targets,unit};
    }
    if(this.maybe('effect')) {const kind=this.effectKind();this.take('{');const fields=[];do{const fs=this.peek();const label=this.field();this.take(':');const type=this.type(true);this.take(';');fields.push({label,span:this.extent(fs),type});}while(!this.at('}'));this.take('}');return {fields,kind,span:this.extent(start),tag:'EffectDecl'};}
    this.take('action');const name=this.identifier();this.take('(');const parameters=[];
    if(!this.at(')'))do{const ps=this.peek();const name=this.identifier();this.take(':');const type=this.type();parameters.push({name,span:this.extent(ps),type});}while(this.maybe(','));
    this.take(')');this.take('{');const statements:Statement[]=[];while(!this.at('}'))statements.push(this.statement());this.take('}');return {name,parameters,span:this.extent(start),statements,tag:'ActionDecl'};
  }
  parse():SourceAST {
    const start=this.take('agreement');const name=this.identifier();this.take('profile');const profile=this.text();if(profile.decoded!==PROFILE)fail('UNSUPPORTED_PROFILE',profile.decoded,profile.span);this.take('{');
    this.take('lifetime');const l=this.uint();this.take(';');this.take('horizon');const h=this.uint();this.take(';');const declarations:SourceDeclaration[]=[];
    while(!this.at('}'))declarations.push(this.declaration());this.take('}');const rootSpan=this.extent(start);if(this.peek().kind!=='eof')fail('PARSE_ERROR','trailing input',span(this.peek().start,this.peek().end));
    return {declarations,horizon:{span:span(h.start,h.end),token:h.text},lifetime:{span:span(l.start,l.end),token:l.text},name,profile:PROFILE,schemaVersion:'moriarty-ast/1',sourceHash:sha256(utf8(this.source)),sourceUtf8Bytes:String(utf8(this.source).length),span:rootSpan};
  }
}
export function parseSource(input:string|Uint8Array):SourceAST {const source=sourceText(input);if(utf8(source).length>65536)fail('SOURCE_BOUND','source exceeds 65536 UTF8 bytes');return new Parser(source).parse();}

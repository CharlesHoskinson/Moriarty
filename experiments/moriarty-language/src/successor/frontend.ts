/** Bounded lexer/parser for moriarty-successor-syntax/0. */

export const SYNTAX_PROFILE = 'moriarty-successor-syntax/0';

export const SYNTAX_BOUNDS = Object.freeze({
  sourceUtf8Bytes: 65536,
  identifierAsciiCharacters: 64,
  decodedStringUtf8Bytes: 1024,
  integerTokenDigits: 78,
  tokenCount: 8192,
  astNodes: 8192,
  nestingDepth: 64,
  totalDeclarations: 256,
  statementsPerAction: 256,
  callArguments: 64,
  parameters: 64,
  recordFields: 64,
});

export class SuccessorSyntaxError extends Error {
  readonly code: string;
  readonly start: number;
  readonly end: number;
  constructor(code: string, message: string, start: number = 0, end: number = start) {
    super(message);
    this.name = 'SuccessorSyntaxError';
    this.code = code;
    this.start = start;
    this.end = end;
  }
}

export interface Span {
  start: number;
  end: number;
}

export interface Program {
  tag: 'Program';
  profile: ProfileDecl;
  agreement: AgreementDecl;
  span: Span;
}

export interface ProfileDecl {
  tag: 'ProfileDecl';
  value: string;
  raw: string;
  span: Span;
}

export interface AgreementDecl {
  tag: 'AgreementDecl';
  name: string;
  declarations: Declaration[];
  span: Span;
}

export interface TypeNode {
  tag: 'Type';
  name: string;
  arguments: TypeNode[];
  span: Span;
}

export interface Parameter {
  tag: 'Parameter';
  name: string;
  type: TypeNode;
  span: Span;
}

export interface EffectField {
  tag: 'EffectField';
  name: string;
  expression: Expression;
  span: Span;
}

export interface UnitDecl {
  tag: 'UnitDecl';
  name: string;
  span: Span;
}

export interface PartyDecl {
  tag: 'PartyDecl';
  name: string;
  span: Span;
}

export interface AssetDecl {
  tag: 'AssetDecl';
  name: string;
  type: TypeNode;
  span: Span;
}

export interface ConstDecl {
  tag: 'ConstDecl';
  name: string;
  type: TypeNode;
  value: Expression;
  span: Span;
}

export interface StateDecl {
  tag: 'StateDecl';
  name: string;
  type: TypeNode;
  value: Expression;
  span: Span;
}

export interface ActionDecl {
  tag: 'ActionDecl';
  name: string;
  parameters: Parameter[];
  statements: Statement[];
  postconditions: Ensures[];
  span: Span;
}

export type Declaration = UnitDecl | PartyDecl | AssetDecl | ConstDecl | StateDecl | ActionDecl;

export interface Requires {
  tag: 'Requires';
  expression: Expression;
  span: Span;
}

export interface Let {
  tag: 'Let';
  name: string;
  expression: Expression;
  span: Span;
}

export interface Next {
  tag: 'Next';
  name: string;
  expression: Expression;
  span: Span;
}

export interface Emit {
  tag: 'Emit';
  type: TypeNode;
  fields: EffectField[];
  span: Span;
}

export type Statement = Requires | Let | Next | Emit;

export interface Ensures {
  tag: 'Ensures';
  expression: Expression;
  span: Span;
}

export interface Identifier {
  tag: 'Identifier';
  name: string;
  span: Span;
}

export interface IntegerLiteral {
  tag: 'IntegerLiteral';
  value: string;
  span: Span;
}

export interface StringLiteral {
  tag: 'StringLiteral';
  raw: string;
  decoded: string;
  span: Span;
}

export interface BooleanLiteral {
  tag: 'BooleanLiteral';
  value: boolean;
  span: Span;
}

export interface Call {
  tag: 'Call';
  name: string;
  arguments: Expression[];
  span: Span;
}

export interface Projection {
  tag: 'Projection';
  object: Expression;
  field: string;
  span: Span;
}

export interface Unary {
  tag: 'Unary';
  operator: 'not';
  operand: Expression;
  span: Span;
}

export interface Binary {
  tag: 'Binary';
  operator: '*' | '+' | '-' | 'and' | 'or';
  left: Expression;
  right: Expression;
  span: Span;
}

export interface Comparison {
  tag: 'Comparison';
  operator: '==' | '!=' | '<' | '<=' | '>' | '>=';
  left: Expression;
  right: Expression;
  span: Span;
}

export type Expression =
  | Identifier
  | IntegerLiteral
  | StringLiteral
  | BooleanLiteral
  | Call
  | Projection
  | Unary
  | Binary
  | Comparison;

const KEYWORDS = new Set([
  'profile', 'agreement', 'unit', 'party', 'asset', 'const', 'state', 'action',
  'requires', 'let', 'next', 'emit', 'ensures',
  'true', 'false', 'not', 'and', 'or',
]);

const DECL_KW = new Set(['unit', 'party', 'asset', 'const', 'state', 'action']);
const CMP_OPS = new Set(['==', '!=', '<', '<=', '>', '>=']);
const ONE_CHAR = new Set(['{', '}', '(', ')', '<', '>', ',', ':', ';', '.', '=', '+', '-', '*']);
const IDENT_LIKE = /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u;

interface Token {
  kind: string;
  start: number;
  end: number;
  text: string;
  raw?: string;
  decoded?: string;
}

function fail(code: string, message: string, start = 0, end = start): never {
  throw new SuccessorSyntaxError(code, message, start, end);
}

function isAsciiLetter(c: number): boolean {
  return (c >= 65 && c <= 90) || (c >= 97 && c <= 122);
}

function isAsciiIdentCont(c: number): boolean {
  return isAsciiLetter(c) || (c >= 48 && c <= 57) || c === 95;
}

function isDigit(c: number): boolean {
  return c >= 48 && c <= 57;
}

function utf8SizeCu(c: number): number {
  return c < 0x80 ? 1 : c < 0x800 ? 2 : 3;
}

function utf8SizeCp(cp: number): number {
  if (cp < 0x80) return 1;
  if (cp < 0x800) return 2;
  if (cp < 0x10000) return 3;
  return 4;
}

function hexDigit(c: number): number {
  if (c >= 48 && c <= 57) return c - 48;
  if (c >= 65 && c <= 70) return c - 55;
  if (c >= 97 && c <= 102) return c - 87;
  return -1;
}

function codePointAt(source: string, i: number): number {
  const c = source.charCodeAt(i);
  if (c >= 0xd800 && c <= 0xdbff) {
    return 0x10000 + ((c - 0xd800) << 10) + (source.charCodeAt(i + 1) - 0xdc00);
  }
  return c;
}

function isNonAsciiIdentLike(cp: number): boolean {
  return cp >= 128 && IDENT_LIKE.test(String.fromCodePoint(cp));
}

function validateSource(source: string): void {
  const n = source.length;
  if (n > SYNTAX_BOUNDS.sourceUtf8Bytes) {
    fail('SOURCE_BOUND', 'source exceeds UTF-8 byte bound');
  }
  let bytes = 0;
  for (let i = 0; i < n; i++) {
    const c = source.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff) {
      const d = i + 1 < n ? source.charCodeAt(i + 1) : 0;
      if (d < 0xdc00 || d > 0xdfff) {
        fail('INVALID_UTF8', 'lone UTF-16 surrogate', bytes, bytes);
      }
      bytes += 4;
      i++;
    } else if (c >= 0xdc00 && c <= 0xdfff) {
      fail('INVALID_UTF8', 'lone UTF-16 surrogate', bytes, bytes);
    } else {
      bytes += utf8SizeCu(c);
    }
    if (bytes > SYNTAX_BOUNDS.sourceUtf8Bytes) {
      fail('SOURCE_BOUND', 'source exceeds UTF-8 byte bound');
    }
  }
}

class Lexer {
  i = 0;
  b = 0;
  readonly n: number;
  readonly tokens: Token[] = [];
  readonly source: string;

  constructor(source: string) {
    this.source = source;
    this.n = source.length;
  }

  tokenize(): Token[] {
    while (true) {
      this.skipIgnored();
      if (this.i >= this.n) break;
      this.scanToken();
    }
    this.tokens.push({ kind: 'eof', start: this.b, end: this.b, text: '' });
    return this.tokens;
  }

  private bumpAscii(): void {
    this.i++;
    this.b++;
  }

  private bumpCp(): void {
    const c = this.source.charCodeAt(this.i);
    if (c >= 0xd800 && c <= 0xdbff) {
      this.i += 2;
      this.b += 4;
    } else {
      this.i += 1;
      this.b += utf8SizeCu(c);
    }
  }

  private emit(kind: string, startB: number, startI: number, extra?: { raw?: string; decoded?: string }): void {
    const tok: Token = {
      kind,
      start: startB,
      end: this.b,
      text: this.source.slice(startI, this.i),
      ...extra,
    };
    if (this.tokens.length >= SYNTAX_BOUNDS.tokenCount - 1) {
      fail('TOKEN_BOUND', 'token bound exceeded', tok.start, tok.end);
    }
    this.tokens.push(tok);
  }

  private skipIgnored(): void {
    const src = this.source;
    while (this.i < this.n) {
      const c = src.charCodeAt(this.i);
      if (c === 32 || c === 9 || c === 13 || c === 10) {
        this.bumpAscii();
        continue;
      }
      if (c === 47 && this.i + 1 < this.n) {
        const d = src.charCodeAt(this.i + 1);
        if (d === 47) {
          this.bumpAscii();
          this.bumpAscii();
          while (this.i < this.n && src.charCodeAt(this.i) !== 10) this.bumpCp();
          continue;
        }
        if (d === 42) {
          const startB = this.b;
          this.bumpAscii();
          this.bumpAscii();
          let closed = false;
          while (this.i < this.n) {
            if (src.charCodeAt(this.i) === 42 && this.i + 1 < this.n && src.charCodeAt(this.i + 1) === 47) {
              this.bumpAscii();
              this.bumpAscii();
              closed = true;
              break;
            }
            this.bumpCp();
          }
          if (!closed) fail('UNTERMINATED_COMMENT', 'unterminated block comment', startB, this.b);
          continue;
        }
      }
      break;
    }
  }

  private scanToken(): void {
    const src = this.source;
    const c = src.charCodeAt(this.i);
    const startB = this.b;
    const startI = this.i;
    const c2 = this.i + 1 < this.n ? src.charCodeAt(this.i + 1) : -1;

    if (c === 61 && c2 === 61) {
      this.bumpAscii();
      this.bumpAscii();
      this.emit('==', startB, startI);
      return;
    }
    if (c === 33 && c2 === 61) {
      this.bumpAscii();
      this.bumpAscii();
      this.emit('!=', startB, startI);
      return;
    }
    if (c === 60 && c2 === 61) {
      this.bumpAscii();
      this.bumpAscii();
      this.emit('<=', startB, startI);
      return;
    }
    if (c === 62 && c2 === 61) {
      this.bumpAscii();
      this.bumpAscii();
      this.emit('>=', startB, startI);
      return;
    }

    const ch = src[this.i]!;
    if (ONE_CHAR.has(ch)) {
      this.bumpAscii();
      this.emit(ch, startB, startI);
      return;
    }

    if (c === 34) {
      this.scanString(startB, startI);
      return;
    }

    if (isDigit(c)) {
      while (this.i < this.n && isDigit(src.charCodeAt(this.i))) this.bumpAscii();
      const digits = src.slice(startI, this.i);
      if (digits.length > 1 && digits[0] === '0') {
        fail('INTEGER_TOKEN', 'invalid integer token', startB, this.b);
      }
      if (digits.length > SYNTAX_BOUNDS.integerTokenDigits) {
        fail('INTEGER_BOUND', 'integer digit bound exceeded', startB, this.b);
      }
      this.emit('integer', startB, startI);
      return;
    }

    if (isAsciiLetter(c)) {
      this.bumpAscii();
      while (this.i < this.n && isAsciiIdentCont(src.charCodeAt(this.i))) this.bumpAscii();
      if (this.i < this.n && isNonAsciiIdentLike(codePointAt(src, this.i))) {
        fail('NON_ASCII_IDENTIFIER', 'non-ASCII identifier', this.b, this.b);
      }
      const name = src.slice(startI, this.i);
      if (name.length > SYNTAX_BOUNDS.identifierAsciiCharacters) {
        fail('IDENTIFIER_BOUND', 'identifier length bound exceeded', startB, this.b);
      }
      this.emit(KEYWORDS.has(name) ? name : 'ident', startB, startI);
      return;
    }

    if (c > 127 && isNonAsciiIdentLike(codePointAt(src, this.i))) {
      fail('NON_ASCII_IDENTIFIER', 'non-ASCII identifier', startB, startB);
    }
    fail('UNEXPECTED_CHAR', 'unexpected character', startB, startB);
  }

  private scanString(startB: number, startI: number): void {
    const src = this.source;
    this.bumpAscii();
    let decoded = '';
    let decodedBytes = 0;

    const appendCp = (cp: number): void => {
      decoded += String.fromCodePoint(cp);
      decodedBytes += utf8SizeCp(cp);
      if (decodedBytes > SYNTAX_BOUNDS.decodedStringUtf8Bytes) {
        fail('STRING_BOUND', 'decoded string UTF-8 bound exceeded', startB, this.b);
      }
    };

    while (this.i < this.n) {
      const c = src.charCodeAt(this.i);
      if (c === 34) {
        this.bumpAscii();
        const raw = src.slice(startI, this.i);
        this.emit('string', startB, startI, { raw, decoded });
        return;
      }
      if (c <= 0x1f) {
        fail('INVALID_STRING', 'unescaped control in string', this.b, this.b);
      }
      if (c === 92) {
        this.bumpAscii();
        if (this.i >= this.n) fail('UNTERMINATED_STRING', 'unterminated string', startB, this.b);
        const e = src.charCodeAt(this.i);
        if (e === 34) {
          appendCp(34);
          this.bumpAscii();
        } else if (e === 92) {
          appendCp(92);
          this.bumpAscii();
        } else if (e === 47) {
          appendCp(47);
          this.bumpAscii();
        } else if (e === 98) {
          appendCp(8);
          this.bumpAscii();
        } else if (e === 102) {
          appendCp(12);
          this.bumpAscii();
        } else if (e === 110) {
          appendCp(10);
          this.bumpAscii();
        } else if (e === 114) {
          appendCp(13);
          this.bumpAscii();
        } else if (e === 116) {
          appendCp(9);
          this.bumpAscii();
        } else if (e === 117) {
          this.bumpAscii();
          const hex = this.readHex4(startB);
          if (hex >= 0xd800 && hex <= 0xdbff) {
            if (
              this.i + 1 < this.n &&
              src.charCodeAt(this.i) === 92 &&
              src.charCodeAt(this.i + 1) === 117
            ) {
              this.bumpAscii();
              this.bumpAscii();
              const low = this.readHex4(startB);
              if (low < 0xdc00 || low > 0xdfff) {
                fail('INVALID_SURROGATE', 'invalid surrogate pair', startB, this.b);
              }
              appendCp(0x10000 + ((hex - 0xd800) << 10) + (low - 0xdc00));
            } else {
              fail('INVALID_SURROGATE', 'lone surrogate escape', startB, this.b);
            }
          } else if (hex >= 0xdc00 && hex <= 0xdfff) {
            fail('INVALID_SURROGATE', 'lone surrogate escape', startB, this.b);
          } else {
            appendCp(hex);
          }
        } else {
          fail('INVALID_STRING', 'unknown string escape', this.b, this.b);
        }
        continue;
      }
      if (c >= 0xd800 && c <= 0xdbff) {
        appendCp(codePointAt(src, this.i));
        this.bumpCp();
      } else {
        appendCp(c);
        this.bumpCp();
      }
    }
    fail('UNTERMINATED_STRING', 'unterminated string', startB, this.b);
  }

  private readHex4(stringStart: number): number {
    let hex = 0;
    for (let k = 0; k < 4; k++) {
      if (this.i >= this.n) fail('UNTERMINATED_STRING', 'unterminated string', stringStart, this.b);
      const h = hexDigit(this.source.charCodeAt(this.i));
      if (h < 0) fail('INVALID_STRING', 'invalid unicode escape', this.b, this.b);
      hex = (hex << 4) | h;
      this.bumpAscii();
    }
    return hex;
  }
}

class Parser {
  pos = 0;
  nodeCount = 0;
  synDepth = 0;
  readonly depths = new WeakMap<object, number>();
  extraTokenCount = 0;
  readonly tokens: Token[];

  constructor(tokens: Token[]) {
    this.tokens = tokens;
  }

  parseProgram(): Program {
    const profile = this.parseProfile();
    const agreement = this.parseAgreement();
    if (!this.at('eof')) {
      const t = this.peek();
      this.fail('TRAILING_INPUT', 'trailing input after program', t.start, t.end);
    }
    return this.counted({
      tag: 'Program',
      profile,
      agreement,
      span: { start: profile.span.start, end: agreement.span.end },
    });
  }

  private peek(): Token {
    return this.tokens[this.pos]!;
  }

  private at(kind: string): boolean {
    return this.peek().kind === kind;
  }

  private advance(): Token {
    const t = this.peek();
    if (t.kind !== 'eof') this.pos++;
    return t;
  }

  private fail(code: string, message: string, start = this.peek().start, end = this.peek().end): never {
    fail(code, message, start, end);
  }

  private unexpected(tok = this.peek()): never {
    this.fail(
      'UNEXPECTED_TOKEN',
      tok.kind === 'eof' ? 'unexpected end of input' : `unexpected token ${tok.kind}`,
      tok.start,
      tok.end,
    );
  }

  private expect(kind: string): Token {
    if (this.at(kind)) return this.advance();
    this.unexpected();
  }

  private expectIdent(): Token {
    if (this.at('ident')) return this.advance();
    this.unexpected();
  }

  private counted<T extends { span: Span }>(node: T): T {
    this.nodeCount++;
    if (this.nodeCount > SYNTAX_BOUNDS.astNodes) {
      this.fail('AST_NODE_BOUND', 'AST node bound exceeded', node.span.start, node.span.end);
    }
    return node;
  }

  private expr<T extends object>(node: T, depth: number): T {
    if (depth > SYNTAX_BOUNDS.nestingDepth) {
      const span = (node as { span: Span }).span;
      this.fail('NESTING_BOUND', 'nesting depth bound exceeded', span.start, span.end);
    }
    this.counted(node as { span: Span });
    this.depths.set(node, depth);
    return node;
  }

  private depthOf(node: object): number {
    return this.depths.get(node) ?? 1;
  }

  private enterNest(at: Token): void {
    if (this.synDepth >= SYNTAX_BOUNDS.nestingDepth) {
      this.fail('NESTING_BOUND', 'nesting depth bound exceeded', at.start, at.end);
    }
    this.synDepth++;
  }

  private leaveNest(): void {
    this.synDepth--;
  }

  private parseProfile(): ProfileDecl {
    const kw = this.expect('profile');
    const str = this.expect('string');
    if (str.decoded !== SYNTAX_PROFILE) {
      this.fail('PROFILE_MISMATCH', `profile must be ${SYNTAX_PROFILE}`, str.start, str.end);
    }
    const semi = this.expect(';');
    return this.counted({
      tag: 'ProfileDecl',
      value: str.decoded!,
      raw: str.raw!,
      span: { start: kw.start, end: semi.end },
    });
  }

  private parseAgreement(): AgreementDecl {
    const kw = this.expect('agreement');
    const name = this.expectIdent();
    this.expect('{');
    const declarations: Declaration[] = [];
    while (!this.at('}') && !this.at('eof')) {
      if (declarations.length >= SYNTAX_BOUNDS.totalDeclarations) {
        this.fail('DECLARATION_BOUND', 'declaration bound exceeded');
      }
      declarations.push(this.parseDeclaration());
    }
    const close = this.expect('}');
    return this.counted({
      tag: 'AgreementDecl',
      name: name.text,
      declarations,
      span: { start: kw.start, end: close.end },
    });
  }

  private parseDeclaration(): Declaration {
    if (this.at('unit')) return this.parseNamedSimple('UnitDecl');
    if (this.at('party')) return this.parseNamedSimple('PartyDecl');
    if (this.at('asset')) return this.parseAsset();
    if (this.at('const')) return this.parseConstOrState('ConstDecl');
    if (this.at('state')) return this.parseConstOrState('StateDecl');
    if (this.at('action')) return this.parseAction();
    const t = this.peek();
    if (t.kind === 'ident' || KEYWORDS.has(t.kind)) {
      this.fail('UNKNOWN_DECLARATION', `unknown declaration ${t.text || t.kind}`, t.start, t.end);
    }
    this.unexpected();
  }

  private parseNamedSimple(tag: 'UnitDecl' | 'PartyDecl'): UnitDecl | PartyDecl {
    const kw = this.advance();
    const name = this.expectIdent();
    const semi = this.expect(';');
    return this.counted({
      tag,
      name: name.text,
      span: { start: kw.start, end: semi.end },
    });
  }

  private parseAsset(): AssetDecl {
    const kw = this.advance();
    const name = this.expectIdent();
    this.expect(':');
    const type = this.parseType();
    const semi = this.expect(';');
    return this.counted({
      tag: 'AssetDecl',
      name: name.text,
      type,
      span: { start: kw.start, end: semi.end },
    });
  }

  private parseConstOrState(tag: 'ConstDecl' | 'StateDecl'): ConstDecl | StateDecl {
    const kw = this.advance();
    const name = this.expectIdent();
    this.expect(':');
    const type = this.parseType();
    this.expect('=');
    const value = this.parseExpression();
    const semi = this.expect(';');
    return this.counted({
      tag,
      name: name.text,
      type,
      value,
      span: { start: kw.start, end: semi.end },
    });
  }

  private parseAction(): ActionDecl {
    const kw = this.advance();
    const name = this.expectIdent();
    this.expect('(');
    const parameters: Parameter[] = [];
    if (!this.at(')')) {
      parameters.push(this.parseParameter());
      while (this.at(',')) {
        if (parameters.length >= SYNTAX_BOUNDS.parameters) {
          this.fail('ARITY_BOUND', 'parameter bound exceeded');
        }
        this.advance();
        parameters.push(this.parseParameter());
      }
    }
    this.expect(')');
    this.expect('{');
    const statements: Statement[] = [];
    const postconditions: Ensures[] = [];
    let seenEnsures = false;
    while (!this.at('}') && !this.at('eof')) {
      if (statements.length + postconditions.length >= SYNTAX_BOUNDS.statementsPerAction) {
        this.fail('STATEMENT_BOUND', 'statement bound exceeded');
      }
      if (this.at('ensures')) {
        seenEnsures = true;
        postconditions.push(this.parseEnsures());
        continue;
      }
      if (seenEnsures) {
        this.fail('STATEMENT_AFTER_ENSURES', 'statement after ensures');
      }
      statements.push(this.parseStatement());
    }
    const close = this.expect('}');
    return this.counted({
      tag: 'ActionDecl',
      name: name.text,
      parameters,
      statements,
      postconditions,
      span: { start: kw.start, end: close.end },
    });
  }

  private parseParameter(): Parameter {
    const name = this.expectIdent();
    this.expect(':');
    const type = this.parseType();
    return this.counted({
      tag: 'Parameter',
      name: name.text,
      type,
      span: { start: name.start, end: type.span.end },
    });
  }

  private parseStatement(): Statement {
    if (this.at('requires')) {
      const kw = this.advance();
      const expression = this.parseExpression();
      const semi = this.expect(';');
      return this.counted({
        tag: 'Requires',
        expression,
        span: { start: kw.start, end: semi.end },
      });
    }
    if (this.at('let')) {
      const kw = this.advance();
      const name = this.expectIdent();
      this.expect('=');
      const expression = this.parseExpression();
      const semi = this.expect(';');
      return this.counted({
        tag: 'Let',
        name: name.text,
        expression,
        span: { start: kw.start, end: semi.end },
      });
    }
    if (this.at('next')) {
      const kw = this.advance();
      this.expect('.');
      const name = this.expectIdent();
      this.expect('=');
      const expression = this.parseExpression();
      const semi = this.expect(';');
      return this.counted({
        tag: 'Next',
        name: name.text,
        expression,
        span: { start: kw.start, end: semi.end },
      });
    }
    if (this.at('emit')) {
      const kw = this.advance();
      const type = this.parseType();
      this.expect('{');
      const fields: EffectField[] = [];
      if (!this.at('}')) {
        fields.push(this.parseEffectField());
        while (this.at(',')) {
          if (fields.length >= SYNTAX_BOUNDS.recordFields) {
            this.fail('ARITY_BOUND', 'record field bound exceeded');
          }
          this.advance();
          fields.push(this.parseEffectField());
        }
      }
      this.expect('}');
      const semi = this.expect(';');
      return this.counted({
        tag: 'Emit',
        type,
        fields,
        span: { start: kw.start, end: semi.end },
      });
    }
    this.unexpected();
  }

  private parseEffectField(): EffectField {
    const name = this.expectIdent();
    this.expect(':');
    const expression = this.parseExpression();
    return this.counted({
      tag: 'EffectField',
      name: name.text,
      expression,
      span: { start: name.start, end: expression.span.end },
    });
  }

  private parseEnsures(): Ensures {
    const kw = this.advance();
    const expression = this.parseExpression();
    const semi = this.expect(';');
    return this.counted({
      tag: 'Ensures',
      expression,
      span: { start: kw.start, end: semi.end },
    });
  }

  private parseType(): TypeNode {
    const name = this.expectIdent();
    let args: TypeNode[] = [];
    let depth = 1;
    let end = name.end;
    if (this.at('<')) {
      this.enterNest(this.peek());
      this.advance();
      if (this.at('>') || this.at('>=')) {
        this.fail('EMPTY_TYPE_ARGS', 'empty type argument list', this.peek().start, this.peek().start + 1);
      }
      args.push(this.parseType());
      while (this.at(',')) {
        this.advance();
        args.push(this.parseType());
      }
      end = this.expectGt();
      this.leaveNest();
      let maxD = 0;
      for (const a of args) maxD = Math.max(maxD, this.depthOf(a));
      depth = 1 + maxD;
    }
    return this.expr(
      {
        tag: 'Type' as const,
        name: name.text,
        arguments: args,
        span: { start: name.start, end },
      },
      depth,
    );
  }

  private expectGt(): number {
    const t = this.peek();
    if (t.kind === '>') {
      this.advance();
      return t.end;
    }
    if (t.kind === '>=') {
      if (this.tokens.length + this.extraTokenCount + 1 > 8192) {
        this.fail('TOKEN_BOUND', 'token bound exceeded', t.start, t.end);
      }
      this.extraTokenCount++;
      const gtEnd = t.start + 1;
      this.tokens[this.pos] = { kind: '=', start: gtEnd, end: t.end, text: '=' };
      return gtEnd;
    }
    this.unexpected();
  }

  private parseExpression(): Expression {
    return this.parseDisjunction();
  }

  private parseDisjunction(): Expression {
    return this.parseLeftBin(() => this.parseConjunction(), ['or']);
  }

  private parseConjunction(): Expression {
    return this.parseLeftBin(() => this.parseNegation(), ['and']);
  }

  private parseLeftBin(next: () => Expression, kinds: string[]): Expression {
    let left = next();
    while (kinds.includes(this.peek().kind)) {
      const op = this.advance();
      const right = next();
      left = this.makeBinary(op, left, right);
    }
    return left;
  }

  private parseNegation(): Expression {
    const nots: Token[] = [];
    while (this.at('not')) nots.push(this.advance());
    let expr = this.parseComparison();
    for (let i = nots.length - 1; i >= 0; i--) {
      const tok = nots[i]!;
      expr = this.expr(
        {
          tag: 'Unary' as const,
          operator: 'not' as const,
          operand: expr,
          span: { start: tok.start, end: expr.span.end },
        },
        1 + this.depthOf(expr),
      );
    }
    return expr;
  }

  private parseComparison(): Expression {
    const left = this.parseSum();
    const kind = this.peek().kind;
    if (!CMP_OPS.has(kind)) return left;
    const op = this.advance();
    const right = this.parseSum();
    const node = this.expr(
      {
        tag: 'Comparison' as const,
        operator: op.kind as Comparison['operator'],
        left,
        right,
        span: { start: left.span.start, end: right.span.end },
      },
      1 + Math.max(this.depthOf(left), this.depthOf(right)),
    );
    if (CMP_OPS.has(this.peek().kind)) {
      const extra = this.peek();
      this.fail('CHAINED_COMPARISON', 'chained comparison', extra.start, extra.end);
    }
    return node;
  }

  private parseSum(): Expression {
    return this.parseLeftBin(() => this.parseProduct(), ['+', '-']);
  }

  private parseProduct(): Expression {
    return this.parseLeftBin(() => this.parsePostfix(), ['*']);
  }

  private parsePostfix(): Expression {
    let expr = this.parsePrimary();
    while (this.at('.')) {
      this.advance();
      const field = this.expectIdent();
      expr = this.expr(
        {
          tag: 'Projection' as const,
          object: expr,
          field: field.text,
          span: { start: expr.span.start, end: field.end },
        },
        1 + this.depthOf(expr),
      );
    }
    return expr;
  }

  private parsePrimary(): Expression {
    if (this.at('integer')) {
      const t = this.advance();
      return this.expr(
        { tag: 'IntegerLiteral' as const, value: t.text, span: { start: t.start, end: t.end } },
        1,
      );
    }
    if (this.at('string')) {
      const t = this.advance();
      return this.expr(
        {
          tag: 'StringLiteral' as const,
          raw: t.raw!,
          decoded: t.decoded!,
          span: { start: t.start, end: t.end },
        },
        1,
      );
    }
    if (this.at('true') || this.at('false')) {
      const t = this.advance();
      return this.expr(
        {
          tag: 'BooleanLiteral' as const,
          value: t.kind === 'true',
          span: { start: t.start, end: t.end },
        },
        1,
      );
    }
    if (this.at('ident')) {
      const name = this.advance();
      if (this.at('(')) {
        this.enterNest(this.peek());
        this.advance();
        const args: Expression[] = [];
        if (!this.at(')')) {
          args.push(this.parseExpression());
          while (this.at(',')) {
            if (args.length >= SYNTAX_BOUNDS.callArguments) {
              this.fail('ARITY_BOUND', 'call argument bound exceeded');
            }
            this.advance();
            args.push(this.parseExpression());
          }
        }
        const close = this.expect(')');
        this.leaveNest();
        let depth = 1;
        for (const a of args) depth = Math.max(depth, 1 + this.depthOf(a));
        return this.expr(
          {
            tag: 'Call' as const,
            name: name.text,
            arguments: args,
            span: { start: name.start, end: close.end },
          },
          depth,
        );
      }
      return this.expr(
        { tag: 'Identifier' as const, name: name.text, span: { start: name.start, end: name.end } },
        1,
      );
    }
    if (this.at('(')) {
      this.enterNest(this.peek());
      this.advance();
      const expr = this.parseExpression();
      this.expect(')');
      this.leaveNest();
      return expr;
    }
    this.unexpected();
  }

  private makeBinary(op: Token, left: Expression, right: Expression): Binary {
    return this.expr(
      {
        tag: 'Binary' as const,
        operator: op.kind as Binary['operator'],
        left,
        right,
        span: { start: left.span.start, end: right.span.end },
      },
      1 + Math.max(this.depthOf(left), this.depthOf(right)),
    );
  }
}

export function parseSuccessorSource(source: string): Program {
  if (typeof source !== 'string') {
    fail('SOURCE_TYPE', 'source must be a string', 0, 0);
  }
  validateSource(source);
  const tokens = new Lexer(source).tokenize();
  return new Parser(tokens).parseProgram();
}

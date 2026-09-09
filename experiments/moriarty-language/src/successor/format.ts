import {
  parseSuccessorSource,
  SuccessorSyntaxError,
  SYNTAX_BOUNDS,
  type Declaration,
  type EffectField,
  type Ensures,
  type Expression,
  type Parameter,
  type Program,
  type Statement,
  type TypeNode,
} from './frontend.ts';

const PREC_NONE = 0;
const PREC_OR = 1;
const PREC_AND = 2;
const PREC_NOT = 3;
const PREC_CMP = 4;
const PREC_SUM = 5;
const PREC_MUL = 6;
const PREC_POSTFIX = 7;
const PREC_PRIMARY = 8;

type Side = 'left' | 'right' | 'operand' | 'none';

function utf8ByteLength(text: string): number {
  return new TextEncoder().encode(text).length;
}

function indent(level: number): string {
  return '  '.repeat(level);
}

function exprPrec(expr: Expression): number {
  switch (expr.tag) {
    case 'Identifier':
    case 'IntegerLiteral':
    case 'StringLiteral':
    case 'BooleanLiteral':
    case 'Call':
      return PREC_PRIMARY;
    case 'Projection':
      return PREC_POSTFIX;
    case 'Unary':
      return PREC_NOT;
    case 'Comparison':
      return PREC_CMP;
    case 'Binary':
      switch (expr.operator) {
        case 'or':
          return PREC_OR;
        case 'and':
          return PREC_AND;
        case '+':
        case '-':
          return PREC_SUM;
        case '*':
          return PREC_MUL;
      }
  }
}

function needsParens(expr: Expression, parentPrec: number, side: Side): boolean {
  const prec = exprPrec(expr);
  if (prec < parentPrec) return true;
  if (prec > parentPrec) return false;
  if (expr.tag === 'Comparison') return true;
  if (expr.tag === 'Binary') return side === 'right';
  return false;
}

function formatExpression(expr: Expression, parentPrec: number, side: Side): string {
  const inner = formatExpressionInner(expr);
  return needsParens(expr, parentPrec, side) ? `(${inner})` : inner;
}

function formatExpressionInner(expr: Expression): string {
  switch (expr.tag) {
    case 'Identifier':
      return expr.name;
    case 'IntegerLiteral':
      return expr.value;
    case 'StringLiteral':
      return expr.raw;
    case 'BooleanLiteral':
      return expr.value ? 'true' : 'false';
    case 'Call': {
      const args = expr.arguments.map((argument) => formatExpression(argument, PREC_NONE, 'none'));
      return `${expr.name}(${args.join(', ')})`;
    }
    case 'Projection':
      return `${formatExpression(expr.object, PREC_POSTFIX, 'operand')}.${expr.field}`;
    case 'Unary':
      return `not ${formatExpression(expr.operand, PREC_NOT, 'operand')}`;
    case 'Binary': {
      const prec = exprPrec(expr);
      const left = formatExpression(expr.left, prec, 'left');
      const right = formatExpression(expr.right, prec, 'right');
      return `${left} ${expr.operator} ${right}`;
    }
    case 'Comparison': {
      const left = formatExpression(expr.left, PREC_CMP, 'left');
      const right = formatExpression(expr.right, PREC_CMP, 'right');
      return `${left} ${expr.operator} ${right}`;
    }
  }
}

function formatType(type: TypeNode): string {
  if (type.arguments.length === 0) return type.name;
  return `${type.name}<${type.arguments.map(formatType).join(', ')}>`;
}

function formatParameter(parameter: Parameter): string {
  return `${parameter.name}: ${formatType(parameter.type)}`;
}

function formatEffectField(field: EffectField): string {
  return `${field.name}: ${formatExpression(field.expression, PREC_NONE, 'none')}`;
}

function formatStatement(statement: Statement): string {
  switch (statement.tag) {
    case 'Requires':
      return `requires ${formatExpression(statement.expression, PREC_NONE, 'none')};`;
    case 'Let':
      return `let ${statement.name} = ${formatExpression(statement.expression, PREC_NONE, 'none')};`;
    case 'Next':
      return `next.${statement.name} = ${formatExpression(statement.expression, PREC_NONE, 'none')};`;
    case 'Emit': {
      const type = formatType(statement.type);
      if (statement.fields.length === 0) return `emit ${type} {};`;
      return `emit ${type} { ${statement.fields.map(formatEffectField).join(', ')} };`;
    }
  }
}

function formatEnsures(ensures: Ensures): string {
  return `ensures ${formatExpression(ensures.expression, PREC_NONE, 'none')};`;
}

function formatDeclaration(declaration: Declaration, level: number): string {
  const pad = indent(level);
  switch (declaration.tag) {
    case 'UnitDecl':
      return `${pad}unit ${declaration.name};`;
    case 'PartyDecl':
      return `${pad}party ${declaration.name};`;
    case 'AssetDecl':
      return `${pad}asset ${declaration.name}: ${formatType(declaration.type)};`;
    case 'ConstDecl':
      return `${pad}const ${declaration.name}: ${formatType(declaration.type)} = ${formatExpression(declaration.value, PREC_NONE, 'none')};`;
    case 'StateDecl':
      return `${pad}state ${declaration.name}: ${formatType(declaration.type)} = ${formatExpression(declaration.value, PREC_NONE, 'none')};`;
    case 'ActionDecl': {
      const params = declaration.parameters.map(formatParameter).join(', ');
      const lines: string[] = [];
      for (const statement of declaration.statements) {
        lines.push(`${indent(level + 1)}${formatStatement(statement)}`);
      }
      for (const ensures of declaration.postconditions) {
        lines.push(`${indent(level + 1)}${formatEnsures(ensures)}`);
      }
      const body = lines.length === 0 ? '' : `${lines.join('\n')}\n`;
      return `${pad}action ${declaration.name}(${params}) {\n${body}${pad}}`;
    }
  }
}

function formatProgram(program: Program): string {
  const decls = program.agreement.declarations.map((declaration) => formatDeclaration(declaration, 1));
  const inner = decls.length === 0 ? '' : `${decls.join('\n')}\n`;
  return `profile ${program.profile.raw};\n\nagreement ${program.agreement.name} {\n${inner}}\n`;
}

export function formatSuccessorSource(source: string): string {
  const program = parseSuccessorSource(source);
  const formatted = formatProgram(program);
  if (utf8ByteLength(formatted) > SYNTAX_BOUNDS.sourceUtf8Bytes) {
    throw new SuccessorSyntaxError(
      'FORMAT_BOUND',
      `formatted source exceeds ${SYNTAX_BOUNDS.sourceUtf8Bytes} UTF-8 bytes`,
      0,
      0,
    );
  }
  return formatted;
}
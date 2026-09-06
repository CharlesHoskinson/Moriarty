export type Values = Record<string, string>;
export type CoreState = { instance: string; revision: string; remaining: string; values: Values };
export type Action = { name: string; args: Values };
export type Effect = { kind: 'Transfer' | 'Fee' | 'DueCreated' | 'DueSettled'; fields: Values };
export type Evaluation =
  | { outcome: 'evaluated'; before: CoreState; after: CoreState; effects: Effect[]; writes: string[]; steps: string }
  | { outcome: 'rejected'; code: string; message: string };

export type FieldType = 'UInt128' | 'String';
export type Expr =
  | { op: 'literal'; value: string | boolean }
  | { op: 'state'; field: string }
  | { op: 'arg'; name: string }
  | { op: 'local'; name: string }
  | { op: 'remaining' }
  | { op: 'eq' | 'lt' | 'lte' | 'gt' | 'gte' | 'add' | 'sub' | 'mul' | 'div'; left: Expr; right: Expr }
  | { op: 'and' | 'or'; left: Expr; right: Expr }
  | { op: 'not'; value: Expr };
export type Instruction =
  | { op: 'guard'; condition: Expr; message: string }
  | { op: 'let'; name: string; value: Expr }
  | { op: 'set'; field: string; value: Expr }
  | { op: 'emit'; kind: Effect['kind']; fields: Record<string, Expr> };
export type Program = {
  version: 'moriarty-core/1';
  stateSchema: Record<string, FieldType>;
  effectSchemas: Partial<Record<Effect['kind'], Record<string, FieldType>>>;
  entrypoints: Record<string, { argSchema: Record<string, FieldType>; instructions: Instruction[] }>;
};

const UINT_MAX = (1n << 128n) - 1n;
const LIMITS = { sourceBytes: 65_536, instructions: 64, depth: 16, nodes: 256, fields: 64, effects: 16 };
const canonicalUInt = /^(0|[1-9][0-9]*)$/;
const identifier = /^[A-Za-z][A-Za-z0-9_.:-]{0,63}$/;
const reservedKeys = new Set(['constructor', 'prototype', '__proto__']);
const hasOwn = (record: object, key: PropertyKey): boolean => Object.prototype.hasOwnProperty.call(record, key);

class Reject extends Error {
  constructor(readonly code: string, message: string) { super(message); }
}

function ownRecord(value: unknown, label: string): Record<string, unknown> {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) throw new Reject('INVALID_SCHEMA', `${label} must be an object`);
  return value as Record<string, unknown>;
}

function exactKeys(value: Record<string, unknown>, allowed: readonly string[], label: string): void {
  const actual = Object.keys(value);
  if (actual.some((key) => !allowed.includes(key))) throw new Reject('UNKNOWN_SYNTAX', `${label} has an unknown field`);
  if (allowed.some((key) => !hasOwn(value, key))) throw new Reject('INVALID_SCHEMA', `${label} is missing a field`);
}

function uint(value: unknown, label: string): bigint {
  if (typeof value !== 'string' || value.length > 39 || !canonicalUInt.test(value)) throw new Reject('INVALID_UINT128', `${label} must be a canonical UInt128 string`);
  const parsed = BigInt(value);
  if (parsed > UINT_MAX) throw new Reject('UINT128_OVERFLOW', `${label} exceeds UInt128`);
  return parsed;
}

function checked(value: bigint): string {
  if (value < 0n) throw new Reject('UINT128_UNDERFLOW', 'UInt128 subtraction underflow');
  if (value > UINT_MAX) throw new Reject('UINT128_OVERFLOW', 'UInt128 intermediate overflow');
  return value.toString();
}

function stringValue(value: unknown, label: string): string {
  if (typeof value !== 'string' || value.length > 256) throw new Reject('INVALID_STRING', `${label} must be a bounded string`);
  return value;
}

function schema(value: unknown, label: string): Record<string, FieldType> {
  const record = ownRecord(value, label);
  if (Object.keys(record).length > LIMITS.fields) throw new Reject('LIMIT_EXCEEDED', `${label} has too many fields`);
  const result: Record<string, FieldType> = {};
  for (const [name, type] of Object.entries(record)) {
    if (!identifier.test(name) || reservedKeys.has(name) || (type !== 'UInt128' && type !== 'String')) throw new Reject('INVALID_SCHEMA', `${label}.${name} is invalid`);
    result[name] = type;
  }
  return result;
}

function validateValue(value: unknown, type: FieldType, label: string): string {
  return type === 'UInt128' ? uint(value, label).toString() : stringValue(value, label);
}

function validateValues(value: unknown, expected: Record<string, FieldType>, label: string): Values {
  const record = ownRecord(value, label);
  const expectedKeys = Object.keys(expected);
  if (Object.keys(record).length !== expectedKeys.length || Object.keys(record).some((key) => !hasOwn(expected, key))) {
    throw new Reject('INVALID_SCHEMA', `${label} fields do not match the schema`);
  }
  return Object.fromEntries(expectedKeys.map((key) => [key, validateValue(record[key], expected[key], `${label}.${key}`)]));
}

export function safeJsonValue(value: unknown, label = 'input'): unknown {
  const active = new Set<object>(); let nodes = 0; let bytes = 0;
  const countText = (text: string): void => {
    bytes += new TextEncoder().encode(text).length;
    if (bytes > LIMITS.sourceBytes) throw new Reject('LIMIT_EXCEEDED', `${label} exceeds 64 KiB`);
  };
  const walk = (item: unknown, depth: number): unknown => {
    if (++nodes > 8192 || depth > 40) throw new Reject('LIMIT_EXCEEDED', `${label} structure is too large`);
    if (item === null || typeof item === 'boolean') return item;
    if (typeof item === 'string') { countText(item); return item; }
    if (typeof item !== 'object') throw new Reject('NON_JSON_VALUE', `${label} must contain JSON values and no JSON numbers`);
    if (active.has(item)) throw new Reject('CYCLIC_PROGRAM', `${label} must be acyclic`);
    const proto = Object.getPrototypeOf(item);
    if (Array.isArray(item)) {
      if (proto !== Array.prototype) throw new Reject('ACTIVE_OBJECT', `${label} arrays must use the standard prototype`);
      const descriptors = Object.getOwnPropertyDescriptors(item);
      const descriptorRecord = descriptors as unknown as Record<string, PropertyDescriptor>;
      const keys = Reflect.ownKeys(descriptors);
      if (keys.some((key) => typeof key === 'symbol')) throw new Reject('ACTIVE_OBJECT', `${label} cannot contain symbol fields`);
      if (keys.some((key) => key !== 'length' && (!/^(0|[1-9][0-9]*)$/.test(key as string) || !descriptorRecord[key as string].enumerable))) throw new Reject('ACTIVE_OBJECT', `${label} arrays cannot contain custom fields`);
      if ((descriptorRecord.length.value as number) !== item.length || keys.length !== item.length + 1) throw new Reject('ACTIVE_OBJECT', `${label} arrays must be dense`);
      active.add(item); const output = Array.from({ length: item.length }, (_, index) => walk(descriptorRecord[String(index)].value, depth + 1)); active.delete(item); return output;
    }
    if (proto !== Object.prototype && proto !== null) throw new Reject('ACTIVE_OBJECT', `${label} records must be plain objects`);
    const descriptors = Object.getOwnPropertyDescriptors(item);
    const keys = Reflect.ownKeys(descriptors);
    if (keys.length > LIMITS.fields || keys.some((key) => typeof key === 'symbol')) throw new Reject('LIMIT_EXCEEDED', `${label} has too many or symbolic fields`);
    const output: Record<string, unknown> = Object.create(null);
    active.add(item);
    for (const rawKey of keys) {
      const key = rawKey as string; const descriptor = descriptors[key];
      if (!descriptor.enumerable || !hasOwn(descriptor, 'value') || reservedKeys.has(key)) throw new Reject('ACTIVE_OBJECT', `${label} cannot contain accessors, hidden, or reserved fields`);
      countText(key); output[key] = walk(descriptor.value, depth + 1);
    }
    active.delete(item); return output;
  };
  const safe = walk(value, 0);
  const encoded = JSON.stringify(safe);
  if (new TextEncoder().encode(encoded).length > LIMITS.sourceBytes) throw new Reject('LIMIT_EXCEEDED', `${label} exceeds 64 KiB`);
  return safe;
}

function validateExpr(value: unknown, depth: number, counter: { nodes: number }): Expr {
  if (depth > LIMITS.depth || ++counter.nodes > LIMITS.nodes) throw new Reject('LIMIT_EXCEEDED', 'expression bounds exceeded');
  const expr = ownRecord(value, 'expression');
  if (typeof expr.op !== 'string') throw new Reject('UNKNOWN_SYNTAX', 'expression operation is missing');
  switch (expr.op) {
    case 'literal':
      exactKeys(expr, ['op', 'value'], 'literal');
      if (typeof expr.value !== 'string' && typeof expr.value !== 'boolean') throw new Reject('NON_JSON_VALUE', 'literal must be a string or boolean');
      return expr as Expr;
    case 'state': exactKeys(expr, ['op', 'field'], 'state read'); stringValue(expr.field, 'field'); return expr as Expr;
    case 'arg': exactKeys(expr, ['op', 'name'], 'argument read'); stringValue(expr.name, 'argument'); return expr as Expr;
    case 'local': exactKeys(expr, ['op', 'name'], 'local read'); stringValue(expr.name, 'local'); return expr as Expr;
    case 'remaining': exactKeys(expr, ['op'], 'remaining read'); return expr as Expr;
    case 'eq': case 'lt': case 'lte': case 'gt': case 'gte': case 'add': case 'sub': case 'mul': case 'div': case 'and': case 'or':
      exactKeys(expr, ['op', 'left', 'right'], expr.op);
      validateExpr(expr.left, depth + 1, counter); validateExpr(expr.right, depth + 1, counter); return expr as Expr;
    case 'not': exactKeys(expr, ['op', 'value'], 'not'); validateExpr(expr.value, depth + 1, counter); return expr as Expr;
    default: throw new Reject('UNKNOWN_SYNTAX', `unknown expression operation ${expr.op}`);
  }
}

type ValueType = FieldType | 'Bool';
function inferType(expr: Expr, stateFields: Record<string, FieldType>, argFields: Record<string, FieldType>, locals: Map<string, ValueType>): ValueType {
  switch (expr.op) {
    case 'state': if (!hasOwn(stateFields, expr.field)) throw new Reject('UNKNOWN_FIELD', `unknown state field ${expr.field}`); return stateFields[expr.field];
    case 'arg': if (!hasOwn(argFields, expr.name)) throw new Reject('UNKNOWN_FIELD', `unknown argument ${expr.name}`); return argFields[expr.name];
    case 'local': { const type = locals.get(expr.name); if (!type) throw new Reject('UNKNOWN_FIELD', `unknown or forward local ${expr.name}`); return type; }
    case 'remaining': return 'UInt128';
    case 'literal':
      if (typeof expr.value === 'boolean') return 'Bool';
      if (canonicalUInt.test(expr.value)) { uint(expr.value, 'numeric literal'); return 'UInt128'; }
      return 'String';
    case 'not': if (inferType(expr.value, stateFields, argFields, locals) !== 'Bool') throw new Reject('TYPE_ERROR', 'not requires Bool'); return 'Bool';
    default: {
      const left = inferType(expr.left, stateFields, argFields, locals);
      const right = inferType(expr.right, stateFields, argFields, locals);
      if (expr.op === 'eq') {
        if (left !== right) throw new Reject('TYPE_ERROR', 'equality operands must have matching types');
        return 'Bool';
      }
      if (expr.op === 'and' || expr.op === 'or') {
        if (left !== 'Bool' || right !== 'Bool') throw new Reject('TYPE_ERROR', `${expr.op} requires Bool operands`);
        return 'Bool';
      }
      if (left !== 'UInt128' || right !== 'UInt128') throw new Reject('TYPE_ERROR', `${expr.op} requires UInt128 operands`);
      return ['lt', 'lte', 'gt', 'gte'].includes(expr.op) ? 'Bool' : 'UInt128';
    }
  }
}

function validateProgram(input: unknown): Program {
  const program = ownRecord(safeJsonValue(input, 'program'), 'program');
  exactKeys(program, ['version', 'stateSchema', 'effectSchemas', 'entrypoints'], 'program');
  if (program.version !== 'moriarty-core/1') throw new Reject('INVALID_SCHEMA', 'unsupported Core version');
  const stateSchema = schema(program.stateSchema, 'stateSchema');
  const effectsRaw = ownRecord(program.effectSchemas, 'effectSchemas');
  const effectSchemas: Program['effectSchemas'] = {};
  for (const [kind, fields] of Object.entries(effectsRaw)) {
    if (!identifier.test(kind) || reservedKeys.has(kind) || !['Transfer', 'Fee', 'DueCreated', 'DueSettled'].includes(kind)) throw new Reject('INVALID_SCHEMA', `unknown effect kind ${kind}`);
    effectSchemas[kind as Effect['kind']] = schema(fields, `effectSchemas.${kind}`);
  }
  const entriesRaw = ownRecord(program.entrypoints, 'entrypoints');
  if (Object.keys(entriesRaw).length > LIMITS.fields) throw new Reject('LIMIT_EXCEEDED', 'too many entrypoints');
  const entrypoints: Program['entrypoints'] = {};
  for (const [name, raw] of Object.entries(entriesRaw)) {
    if (!identifier.test(name) || reservedKeys.has(name)) throw new Reject('INVALID_SCHEMA', `invalid entrypoint name ${name}`);
    const entry = ownRecord(raw, `entrypoint ${name}`); exactKeys(entry, ['argSchema', 'instructions'], `entrypoint ${name}`);
    const argSchema = schema(entry.argSchema, `entrypoint ${name} args`);
    if (!Array.isArray(entry.instructions) || entry.instructions.length > LIMITS.instructions) throw new Reject('LIMIT_EXCEEDED', `entrypoint ${name} instruction bound exceeded`);
    let effects = 0; const locals = new Map<string, ValueType>(); const instructions: Instruction[] = []; const expressionCounter = { nodes: 0 };
    for (const rawInstruction of entry.instructions) {
      const instruction = ownRecord(rawInstruction, 'instruction');
      switch (instruction.op) {
        case 'guard': {
          exactKeys(instruction, ['op', 'condition', 'message'], 'guard');
          const expr = validateExpr(instruction.condition, 1, expressionCounter);
          if (inferType(expr, stateSchema, argSchema, locals) !== 'Bool') throw new Reject('TYPE_ERROR', 'guard condition must be Bool');
          stringValue(instruction.message, 'guard message'); break;
        }
        case 'let': {
          exactKeys(instruction, ['op', 'name', 'value'], 'let'); const name = stringValue(instruction.name, 'local name');
          if (!identifier.test(name) || reservedKeys.has(name)) throw new Reject('INVALID_SCHEMA', `invalid local name ${name}`);
          if (locals.has(name)) throw new Reject('INVALID_SCHEMA', `duplicate local ${name}`);
          const expr = validateExpr(instruction.value, 1, expressionCounter); locals.set(name, inferType(expr, stateSchema, argSchema, locals)); break;
        }
        case 'set': {
          exactKeys(instruction, ['op', 'field', 'value'], 'set'); const field = stringValue(instruction.field, 'state field');
          if (!hasOwn(stateSchema, field)) throw new Reject('UNKNOWN_FIELD', `unknown state field ${field}`);
          const expr = validateExpr(instruction.value, 1, expressionCounter);
          if (inferType(expr, stateSchema, argSchema, locals) !== stateSchema[field]) throw new Reject('TYPE_ERROR', `set ${field} type does not match its schema`);
          break;
        }
        case 'emit': {
          exactKeys(instruction, ['op', 'kind', 'fields'], 'emit');
          if (typeof instruction.kind !== 'string' || !hasOwn(effectSchemas, instruction.kind)) throw new Reject('INVALID_EFFECT', 'emitted effect has no schema');
          const fields = ownRecord(instruction.fields, 'effect fields');
          if (Object.keys(fields).length > LIMITS.fields) throw new Reject('LIMIT_EXCEEDED', 'too many effect fields');
          const expectedFields = effectSchemas[instruction.kind as Effect['kind']]!;
          if (Object.keys(fields).length !== Object.keys(expectedFields).length || Object.keys(fields).some((field) => !hasOwn(expectedFields, field))) {
            throw new Reject('INVALID_EFFECT', `${instruction.kind} fields do not match its schema`);
          }
          for (const [field, expression] of Object.entries(fields)) {
            const expr = validateExpr(expression, 1, expressionCounter);
            if (inferType(expr, stateSchema, argSchema, locals) !== expectedFields[field]) throw new Reject('TYPE_ERROR', `${instruction.kind}.${field} type does not match its schema`);
          }
          effects += 1; break;
        }
        default: throw new Reject('UNKNOWN_SYNTAX', `unknown instruction operation ${String(instruction.op)}`);
      }
      instructions.push(instruction as Instruction);
    }
    if (locals.size > LIMITS.fields || effects > LIMITS.effects) throw new Reject('LIMIT_EXCEEDED', `entrypoint ${name} collection bound exceeded`);
    entrypoints[name] = { argSchema, instructions };
  }
  return { version: 'moriarty-core/1', stateSchema, effectSchemas, entrypoints };
}

type RuntimeValue = string | boolean;
function expression(expr: Expr, state: Values, args: Values, locals: Record<string, RuntimeValue>, remaining: string): RuntimeValue {
  const getString = (value: RuntimeValue): string => { if (typeof value !== 'string') throw new Reject('TYPE_ERROR', 'expected string value'); return value; };
  const getBool = (value: RuntimeValue): boolean => { if (typeof value !== 'boolean') throw new Reject('TYPE_ERROR', 'expected boolean value'); return value; };
  switch (expr.op) {
    case 'literal': return expr.value;
    case 'state': if (!hasOwn(state, expr.field)) throw new Reject('UNKNOWN_FIELD', `unknown state field ${expr.field}`); return state[expr.field];
    case 'arg': if (!hasOwn(args, expr.name)) throw new Reject('UNKNOWN_FIELD', `unknown argument ${expr.name}`); return args[expr.name];
    case 'local': if (!hasOwn(locals, expr.name)) throw new Reject('UNKNOWN_FIELD', `unknown local ${expr.name}`); return locals[expr.name];
    case 'remaining': return remaining;
    case 'not': return !getBool(expression(expr.value, state, args, locals, remaining));
    case 'and': return getBool(expression(expr.left, state, args, locals, remaining)) && getBool(expression(expr.right, state, args, locals, remaining));
    case 'or': return getBool(expression(expr.left, state, args, locals, remaining)) || getBool(expression(expr.right, state, args, locals, remaining));
    default: {
      const leftValue = expression(expr.left, state, args, locals, remaining);
      const rightValue = expression(expr.right, state, args, locals, remaining);
      if (expr.op === 'eq') return leftValue === rightValue;
      const left = getString(leftValue); const right = getString(rightValue);
      if (['lt', 'lte', 'gt', 'gte'].includes(expr.op)) {
        const a = uint(left, 'comparison operand'), b = uint(right, 'comparison operand');
        return expr.op === 'lt' ? a < b : expr.op === 'lte' ? a <= b : expr.op === 'gt' ? a > b : a >= b;
      }
      const a = uint(left, 'arithmetic operand'), b = uint(right, 'arithmetic operand');
      if (expr.op === 'add') return checked(a + b);
      if (expr.op === 'sub') return checked(a - b);
      if (expr.op === 'mul') return checked(a * b);
      if (b === 0n) throw new Reject('DIVISION_BY_ZERO', 'division by zero');
      return checked(a / b);
    }
  }
}

export function evaluate(programInput: Program, stateInput: CoreState, actionInput: Action): Evaluation {
  try {
    const program = validateProgram(programInput);
    const stateRecord = ownRecord(safeJsonValue(stateInput, 'state'), 'state'); exactKeys(stateRecord, ['instance', 'revision', 'remaining', 'values'], 'state');
    const before: CoreState = {
      instance: stringValue(stateRecord.instance, 'state.instance'), revision: uint(stateRecord.revision, 'state.revision').toString(),
      remaining: uint(stateRecord.remaining, 'state.remaining').toString(), values: validateValues(stateRecord.values, program.stateSchema, 'state.values'),
    };
    const actionRecord = ownRecord(safeJsonValue(actionInput, 'action'), 'action'); exactKeys(actionRecord, ['name', 'args'], 'action');
    const actionName = stringValue(actionRecord.name, 'action.name');
    const entry = program.entrypoints[actionName];
    if (!entry) throw new Reject('UNKNOWN_ACTION', `unknown action ${actionName}`);
    const args = validateValues(actionRecord.args, entry.argSchema, 'action.args');
    const remaining = uint(before.remaining, 'state.remaining');
    if (remaining === 0n) throw new Reject('BUDGET_EXHAUSTED', 'no remaining step allowance');
    const nextValues = { ...before.values }; const locals: Record<string, RuntimeValue> = {}; const effects: Effect[] = []; const writes: string[] = [];
    for (const instruction of entry.instructions) {
      if (instruction.op === 'guard') {
        if (expression(instruction.condition, nextValues, args, locals, before.remaining) !== true) throw new Reject('GUARD_FAILED', instruction.message);
      } else if (instruction.op === 'let') {
        locals[instruction.name] = expression(instruction.value, nextValues, args, locals, before.remaining);
      } else if (instruction.op === 'set') {
        const fieldType = program.stateSchema[instruction.field];
        if (!fieldType) throw new Reject('UNKNOWN_FIELD', `unknown state field ${instruction.field}`);
        nextValues[instruction.field] = validateValue(expression(instruction.value, nextValues, args, locals, before.remaining), fieldType, `state.${instruction.field}`);
        if (!writes.includes(instruction.field)) writes.push(instruction.field);
      } else {
        const effectSchema = program.effectSchemas[instruction.kind];
        if (!effectSchema) throw new Reject('INVALID_EFFECT', `unknown effect kind ${instruction.kind}`);
        const computed = Object.fromEntries(Object.entries(instruction.fields).map(([key, expr]) => [key, expression(expr, nextValues, args, locals, before.remaining)]));
        effects.push({ kind: instruction.kind, fields: validateValues(computed, effectSchema, `effect.${instruction.kind}`) });
      }
    }
    const after: CoreState = {
      instance: before.instance,
      revision: checked(uint(before.revision, 'revision') + 1n), remaining: checked(remaining - 1n),
      values: validateValues(nextValues, program.stateSchema, 'result.values'),
    };
    return { outcome: 'evaluated', before: structuredClone(before), after, effects, writes, steps: String(entry.instructions.length) };
  } catch (error) {
    return error instanceof Reject
      ? { outcome: 'rejected', code: error.code, message: error.message }
      : { outcome: 'rejected', code: 'INVALID_INPUT', message: error instanceof Error ? error.message : 'invalid input' };
  }
}

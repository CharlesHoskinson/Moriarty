import { safeJsonValue, type Action, type CoreState, type Effect, type Expr, type FieldType, type Instruction, type Program, type Values } from './core.js';

export type AgreementSource = { language: 'moriarty-r2/1'; package: 'Actus.LAM.FirstPeriod' | 'Exchange.ConstantProduct'; terms: Values };
export type ElaboratedBundle = { outcome: 'elaborated'; program: Program; initialState: CoreState; source: AgreementSource; description: string };
export type Bundle = ElaboratedBundle | { outcome: 'rejected'; code: string; message: string };

const UINT_MAX = (1n << 128n) - 1n;
const canonicalUInt = /^(0|[1-9][0-9]*)$/;
const hasOwn = (record: object, key: PropertyKey): boolean => Object.prototype.hasOwnProperty.call(record, key);
const effects: Program['effectSchemas'] = {
  Transfer: { asset: 'String', from: 'String', to: 'String', amount: 'UInt128' },
  Fee: { asset: 'String', from: 'String', to: 'String', amount: 'UInt128' },
  DueCreated: { dueId: 'String', debtor: 'String', creditor: 'String', denomination: 'String', amount: 'UInt128' },
  DueSettled: { dueId: 'String', debtor: 'String', creditor: 'String', denomination: 'String', amount: 'UInt128', asset: 'String' },
};
const u = (value: string): Expr => ({ op: 'literal', value });
const s = (field: string): Expr => ({ op: 'state', field });
const a = (name: string): Expr => ({ op: 'arg', name });
const l = (name: string): Expr => ({ op: 'local', name });
const bin = (op: 'eq'|'lt'|'lte'|'gt'|'gte'|'add'|'sub'|'mul'|'div'|'and'|'or', left: Expr, right: Expr): Expr => ({ op, left, right });
const guard = (condition: Expr, message: string): Instruction => ({ op: 'guard', condition, message });
const let_ = (name: string, value: Expr): Instruction => ({ op: 'let', name, value });
const set = (field: string, value: Expr): Instruction => ({ op: 'set', field, value });
const emit = (kind: Effect['kind'], fields: Record<string, Expr>): Instruction => ({ op: 'emit', kind, fields });

const loanTerms = ['instance','denomination','asset','borrower','lender','notional','principalPayment','rateNumerator','rateDenominator','dayCountNumerator','dayCountDenominator','borrowerCash','lenderCash'] as const;
const poolTerms = ['instance','assetA','assetB','trader','provider','reserveA','reserveB','traderA','traderB','providerA','providerB','feeNumerator','feeDenominator'] as const;
const numericLoan = new Set(['notional','principalPayment','rateNumerator','rateDenominator','dayCountNumerator','dayCountDenominator','borrowerCash','lenderCash']);
const numericPool = new Set(['reserveA','reserveB','traderA','traderB','providerA','providerB','feeNumerator','feeDenominator']);

const examples: Record<'loan'|'swap', AgreementSource> = {
  loan: { language: 'moriarty-r2/1', package: 'Actus.LAM.FirstPeriod', terms: {
    instance: 'loan:demo', denomination: 'USD', asset: 'demo:USD6', borrower: 'borrower', lender: 'lender',
    notional: '5000000000', principalPayment: '500000000', rateNumerator: '8', rateDenominator: '100',
    dayCountNumerator: '31', dayCountDenominator: '365', borrowerCash: '20000000000', lenderCash: '0',
  } },
  swap: { language: 'moriarty-r2/1', package: 'Exchange.ConstantProduct', terms: {
    instance: 'pool:demo', assetA: 'demo:A', assetB: 'demo:B', trader: 'trader', provider: 'provider',
    reserveA: '1000000', reserveB: '2000000', traderA: '100000', traderB: '0', providerA: '0', providerB: '0',
    feeNumerator: '997', feeDenominator: '1000',
  } },
};

export function exampleSource(kind: 'loan'|'swap'): string { return JSON.stringify(examples[kind], null, 2); }

function reject(code: string, message: string): Bundle { return { outcome: 'rejected', code, message }; }
function parseSource(input: unknown): AgreementSource | Bundle {
  let safe: unknown;
  try { safe = safeJsonValue(input, 'agreement source'); } catch (error) { return reject('INVALID_SOURCE', error instanceof Error ? error.message : 'source must be inert finite JSON'); }
  if (!safe || typeof safe !== 'object' || Array.isArray(safe)) return reject('INVALID_SOURCE', 'source must be a bounded object');
  const source = safe as Record<string, unknown>;
  if (Object.keys(source).length !== 3 || !hasOwn(source, 'language') || !hasOwn(source, 'package') || !hasOwn(source, 'terms')) return reject('INVALID_SOURCE', 'source keys must be language, package, and terms');
  if (source.language !== 'moriarty-r2/1' || !['Actus.LAM.FirstPeriod','Exchange.ConstantProduct'].includes(String(source.package))) return reject('UNKNOWN_PACKAGE', 'unsupported language or package');
  if (!source.terms || typeof source.terms !== 'object' || Array.isArray(source.terms)) return reject('INVALID_TERMS', 'terms must be an object');
  const terms = source.terms as Record<string, unknown>;
  const names = source.package === 'Actus.LAM.FirstPeriod' ? loanTerms : poolTerms;
  const numeric = source.package === 'Actus.LAM.FirstPeriod' ? numericLoan : numericPool;
  if (Object.keys(terms).length !== names.length || Object.keys(terms).some((key) => !names.includes(key as never))) return reject('INVALID_TERMS', `terms must contain exactly: ${names.join(', ')}`);
  for (const name of names) {
    const value = terms[name];
    if (typeof value !== 'string' || value.length > 256) return reject('INVALID_TERMS', `${name} must be a bounded string`);
    if (numeric.has(name) && (value.length > 39 || !canonicalUInt.test(value) || BigInt(value) > UINT_MAX)) return reject('INVALID_TERMS', `${name} must be a canonical UInt128 string`);
  }
  for (const denominator of source.package === 'Actus.LAM.FirstPeriod' ? ['rateDenominator','dayCountDenominator'] : ['feeDenominator']) {
    if (terms[denominator] === '0') return reject('INVALID_TERMS', `${denominator} must be nonzero`);
  }
  const idPattern = /^[A-Za-z][A-Za-z0-9_.:-]{0,127}$/;
  const idNames = source.package === 'Actus.LAM.FirstPeriod'
    ? ['instance','asset','borrower','lender']
    : ['instance','assetA','assetB','trader','provider'];
  for (const name of idNames) if (!idPattern.test(terms[name] as string)) return reject('INVALID_TERMS', `${name} must be a canonical ASCII identifier`);
  if (source.package === 'Actus.LAM.FirstPeriod') {
    if (terms.denomination !== 'USD') return reject('INVALID_TERMS', 'the first-period micro-USD profile requires denomination USD');
    if (terms.borrower === terms.lender) return reject('INVALID_TERMS', 'borrower and lender must differ');
    if (terms.notional === '0' || terms.principalPayment === '0') return reject('INVALID_TERMS', 'notional and principal payment must be positive');
    if (BigInt(terms.principalPayment as string) > BigInt(terms.notional as string)) return reject('INVALID_TERMS', 'principal payment exceeds notional');
  } else {
    if (terms.assetA === terms.assetB) return reject('INVALID_TERMS', 'pool assets must differ');
    if (new Set([terms.instance, terms.trader, terms.provider]).size !== 3) return reject('INVALID_TERMS', 'pool instance, trader, and provider must differ');
    if (terms.reserveA === '0' || terms.reserveB === '0') return reject('INVALID_TERMS', 'pool reserves must be positive');
    const feeNumerator = BigInt(terms.feeNumerator as string), feeDenominator = BigInt(terms.feeDenominator as string);
    if (feeNumerator === 0n || feeNumerator > feeDenominator) return reject('INVALID_TERMS', 'fee multiplier must be in (0, denominator]');
  }
  return { language:'moriarty-r2/1', package:source.package, terms:Object.fromEntries(names.map((name) => [name, terms[name] as string])) } as AgreementSource;
}

function loan(source: AgreementSource): ElaboratedBundle {
  const t = source.terms;
  const interestNumerator = BigInt(t.notional) * BigInt(t.rateNumerator) * BigInt(t.dayCountNumerator);
  const interestDenominator = BigInt(t.rateDenominator) * BigInt(t.dayCountDenominator);
  const interestFloor = interestNumerator / interestDenominator;
  const remainder = interestNumerator % interestDenominator;
  const isDefaultReference = t.notional === '5000000000' && t.principalPayment === '500000000' && t.rateNumerator === '8' && t.rateDenominator === '100' && t.dayCountNumerator === '31' && t.dayCountDenominator === '365';
  const stateSchema: Record<string, FieldType> = {
    notional:'UInt128', principalDue:'UInt128', interestDue:'UInt128', principalPaid:'UInt128', interestPaid:'UInt128',
    borrowerCash:'UInt128', lenderCash:'UInt128', cursor:'UInt128', closed:'UInt128',
  };
  const accrue: Instruction[] = [
    guard(bin('eq', a('actor'), u(t.borrower)), 'borrower actor required'), guard(bin('eq', a('event'), u('PR_IP')), 'PR_IP event required'),
    guard(bin('eq', s('cursor'), u('0')), 'first-period accrual is out of order'), guard(bin('eq', s('closed'), u('0')), 'loan is closed'),
    guard(bin('lte', u(t.principalPayment), s('notional')), 'principal payment exceeds notional'),
    let_('interestNumerator', bin('mul', bin('mul', bin('mul', s('notional'), u(t.rateNumerator)), u(t.dayCountNumerator)), u('1'))),
    let_('interestDenominator', bin('mul', u(t.rateDenominator), u(t.dayCountDenominator))),
    let_('interest', bin('div', l('interestNumerator'), l('interestDenominator'))),
    set('principalDue', u(t.principalPayment)), set('interestDue', l('interest')), set('notional', bin('sub', s('notional'), u(t.principalPayment))),
    emit('DueCreated', { dueId:u(`${t.instance}:principal`), debtor:u(t.borrower), creditor:u(t.lender), denomination:u(t.denomination), amount:s('principalDue') }),
    emit('DueCreated', { dueId:u(`${t.instance}:interest`), debtor:u(t.borrower), creditor:u(t.lender), denomination:u(t.denomination), amount:s('interestDue') }),
    set('cursor', u('1')),
  ];
  const settle: Instruction[] = [
    guard(bin('eq', a('actor'), u(t.borrower)), 'borrower actor required'), guard(bin('eq', a('asset'), u(t.asset)), 'settlement asset mismatch'),
    guard(bin('eq', s('cursor'), u('1')), 'settlement is out of order'), guard(bin('eq', s('closed'), u('0')), 'loan is closed'),
    let_('total', bin('add', s('principalDue'), s('interestDue'))), guard(bin('eq', a('amount'), l('total')), 'full due amount required'),
    guard(bin('lte', l('total'), s('borrowerCash')), 'insufficient borrower balance'),
    set('borrowerCash', bin('sub', s('borrowerCash'), l('total'))), set('lenderCash', bin('add', s('lenderCash'), l('total'))),
    set('principalPaid', bin('add', s('principalPaid'), s('principalDue'))), set('interestPaid', bin('add', s('interestPaid'), s('interestDue'))),
    emit('Transfer', { asset:a('asset'), from:u(t.borrower), to:u(t.lender), amount:l('total') }),
    emit('DueSettled', { dueId:u(`${t.instance}:principal`), debtor:u(t.borrower), creditor:u(t.lender), denomination:u(t.denomination), amount:s('principalDue'), asset:a('asset') }),
    emit('DueSettled', { dueId:u(`${t.instance}:interest`), debtor:u(t.borrower), creditor:u(t.lender), denomination:u(t.denomination), amount:s('interestDue'), asset:a('asset') }),
    set('principalDue', u('0')), set('interestDue', u('0')), set('cursor', u('2')), set('closed', u('1')),
  ];
  return {
    outcome:'elaborated', source, program:{ version:'moriarty-core/1', stateSchema, effectSchemas:structuredClone(effects), entrypoints:{
      accrue:{ argSchema:{actor:'String',event:'String'}, instructions:accrue },
      settle:{ argSchema:{actor:'String',asset:'String',amount:'UInt128'}, instructions:settle },
    } },
    initialState:{ instance:t.instance, revision:'0', remaining:'2', values:{ notional:t.notional, principalDue:'0', interestDue:'0', principalPaid:'0', interestPaid:'0', borrowerCash:t.borrowerCash, lenderCash:t.lenderCash, cursor:'0', closed:'0' } },
    description:`Fixed first-period LAM episode. Configured accrual floors ${interestNumerator}/${interestDenominator} micro-USD to ${interestFloor} micro-USD and discards remainder ${remainder}/${interestDenominator}.${isDefaultReference ? ' For the default sample, this is 33.972602 USD versus the 33.972602739726… USD reference decimal.' : ''} This is not full ACTUS conformance.`,
  };
}

function pool(source: AgreementSource): ElaboratedBundle {
  const t = source.terms;
  const stateSchema: Record<string, FieldType> = { reserveA:'UInt128',reserveB:'UInt128',traderA:'UInt128',traderB:'UInt128',providerA:'UInt128',providerB:'UInt128',closed:'UInt128' };
  const swap: Instruction[] = [
    guard(bin('eq', a('actor'), u(t.trader)), 'trader actor required'), guard(bin('eq', a('recipient'), u(t.trader)), 'first-slice recipient must be trader'),
    guard(bin('eq', a('assetIn'), u(t.assetA)), 'assetIn mismatch'), guard(bin('eq', a('assetOut'), u(t.assetB)), 'assetOut mismatch'),
    guard(bin('eq', s('closed'), u('0')), 'pool is closed'), guard(bin('gt', a('amountIn'), u('0')), 'input must be positive'),
    guard(bin('lte', a('amountIn'), s('traderA')), 'insufficient trader balance'),
    let_('effective', bin('mul', a('amountIn'), u(t.feeNumerator))),
    let_('numerator', bin('mul', s('reserveB'), l('effective'))),
    let_('denominator', bin('add', bin('mul', s('reserveA'), u(t.feeDenominator)), l('effective'))),
    let_('amountOut', bin('div', l('numerator'), l('denominator'))), guard(bin('gt', l('amountOut'), u('0')), 'quantized output must be positive'), guard(bin('lte', a('minOut'), l('amountOut')), 'minimum output not met'),
    guard(bin('lt', l('amountOut'), s('reserveB')), 'output would empty reserve'),
    set('traderA', bin('sub', s('traderA'), a('amountIn'))), set('reserveA', bin('add', s('reserveA'), a('amountIn'))),
    set('reserveB', bin('sub', s('reserveB'), l('amountOut'))), set('traderB', bin('add', s('traderB'), l('amountOut'))),
    emit('Transfer', {asset:a('assetIn'),from:u(t.trader),to:u(t.instance),amount:a('amountIn')}),
    emit('Transfer', {asset:a('assetOut'),from:u(t.instance),to:a('recipient'),amount:l('amountOut')}),
  ];
  // A package-level remaining guard is represented in Core, keeping the evaluator generic.
  swap.splice(4, 0, guard(bin('gt', {op:'remaining'}, u('1')), 'final allowance is reserved for closure'));
  const close: Instruction[] = [
    guard(bin('eq', a('actor'), u(t.provider)), 'provider actor required'), guard(bin('eq', s('closed'), u('0')), 'pool is closed'),
    set('providerA', bin('add', s('providerA'), s('reserveA'))), set('providerB', bin('add', s('providerB'), s('reserveB'))),
    emit('Transfer',{asset:u(t.assetA),from:u(t.instance),to:u(t.provider),amount:s('reserveA')}),
    emit('Transfer',{asset:u(t.assetB),from:u(t.instance),to:u(t.provider),amount:s('reserveB')}),
    set('reserveA',u('0')),set('reserveB',u('0')),set('closed',u('1')),
  ];
  return { outcome:'elaborated', source, program:{version:'moriarty-core/1',stateSchema,effectSchemas:structuredClone(effects),entrypoints:{
    swap:{argSchema:{actor:'String',amountIn:'UInt128',minOut:'UInt128',recipient:'String',assetIn:'String',assetOut:'String'},instructions:swap},
    close:{argSchema:{actor:'String'},instructions:close},
  }}, initialState:{instance:t.instance,revision:'0',remaining:'8',values:{reserveA:t.reserveA,reserveB:t.reserveB,traderA:t.traderA,traderB:t.traderB,providerA:t.providerA,providerB:t.providerB,closed:'0'}},
  description:`Bounded constant-product demo epoch with explicit synthetic balances and ${t.feeNumerator}/${t.feeDenominator} pricing multiplier. The pricing fee is retained inside the full input credited to reserve A; it is not a second balance movement. It makes no custody claim.`, };
}

export function elaborate(input: unknown): Bundle {
  const parsed = parseSource(input); if (hasOwn(parsed, 'outcome')) return parsed as Extract<Bundle, {outcome:'rejected'}>;
  const source = parsed as AgreementSource;
  return source.package === 'Actus.LAM.FirstPeriod' ? loan(source) : pool(source);
}

export function defaultAction(bundle: ElaboratedBundle, state: CoreState): Action {
  const t = bundle.source.terms;
  if (bundle.source.package === 'Actus.LAM.FirstPeriod') {
    if (state.values.cursor === '0') return {name:'accrue',args:{actor:t.borrower,event:'PR_IP'}};
    const amount = (BigInt(state.values.principalDue) + BigInt(state.values.interestDue)).toString();
    return {name:'settle',args:{actor:t.borrower,asset:t.asset,amount}};
  }
  return {name:'swap',args:{actor:t.trader,amountIn:'10000',minOut:'19743',recipient:t.trader,assetIn:t.assetA,assetOut:t.assetB}};
}

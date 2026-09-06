/** Developer illustrations only. This module is neither a DSL interpreter nor a verifier. */
export type Example = 'loan' | 'calendar' | 'option' | 'swap' | 'mandate' | 'join' | 'partial';
export type Fault = 'none' | 'missing-observation' | 'stale-observation' | 'wrong-authority'
  | 'missing-proof' | 'altered-proof' | 'wrong-domain' | 'stale-predecessor'
  | 'duplicate-consumption' | 'cancelled' | 'rounding-mutation';
export type Config = { example: Example; principal: string; rate: string; days: string;
  reserveA: string; reserveB: string; amountIn: string; minOut: string; actor: string; fault: Fault };
export type Preview = { kind: 'illustration'; ok: boolean; headline: string;
  metrics: [string, string][]; diagnostics: { code: string; message: string }[];
  effects: string[]; source: string };
export type SimulatedEvidence = { kind: 'SimulatedEvidence'; statement: string;
  proof: 'DEMO_ONLY_NOT_A_CRYPTOGRAPHIC_PROOF'; realVerification: 'unavailable' };
export type Session = { stage: number; statement: string; evidence: SimulatedEvidence | null; receipt: string | null };

const actors: Record<Example, string> = { loan: 'borrower', calendar: 'analyst', option: 'holder',
  swap: 'trader', mandate: 'allocator', join: 'composer', partial: 'trader' };
const examples = Object.keys(actors);
const faults: Fault[] = ['none', 'missing-observation', 'stale-observation', 'wrong-authority',
  'missing-proof', 'altered-proof', 'wrong-domain', 'stale-predecessor', 'duplicate-consumption', 'cancelled', 'rounding-mutation'];
const LIMIT = 1_000_000_000_000n;

export function defaultConfig(example: Example): Config {
  return { example, principal: '5000', rate: '0.08', days: '31', reserveA: '1000000',
    reserveB: '2000000', amountIn: example === 'partial' ? '5000' : '10000',
    minOut: example === 'partial' ? '9800' : '19700', actor: actors[example], fault: 'none' };
}

function uint(value: string, positive = true, limit = LIMIT): bigint {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]{0,12})$/.test(value)) throw new Error('Use a bounded decimal integer without signs or exponents.');
  const n = BigInt(value);
  if (n > limit || (positive && n === 0n)) throw new Error(`Value must be ${positive ? 'positive' : 'nonnegative'} and at most ${limit}.`);
  return n;
}

function decimal(value: string): [bigint, bigint] {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]{0,11})(\.[0-9]{1,12})?$/.test(value)) throw new Error('Use up to 12 integer and 12 decimal digits; no exponent notation.');
  const [whole, fraction = ''] = value.split('.');
  return [BigInt(whole + fraction), 10n ** BigInt(fraction.length)];
}

function rational(n: bigint, d: bigint): string {
  let a = n, b = d;
  while (b !== 0n) [a, b] = [b, a % b];
  return `${n / a}/${d / a}`;
}

/** Fixed resource limits are mock input limits, not a certified Moriarty bound profile. */
export function evaluate(config: Config): Preview {
  const p: Preview = { kind: 'illustration', ok: true, headline: '', metrics: [], diagnostics: [], effects: [], source: '' };
  const reject = (code: string, message: string): Preview => ({ ...p, ok: false,
    headline: 'Illustration rejected', effects: [], diagnostics: [...p.diagnostics, { code, message }] });
  try {
    if (!examples.includes(config.example) || !faults.includes(config.fault)) return reject('InvalidScenario', 'Choose a supported example and failure scenario.');
    if (typeof config.actor !== 'string' || config.actor.length > 64) return reject('InvalidActor', 'Actor must be a short named demo role.');
    if (config.fault === 'missing-observation') return reject('MissingObservation', 'Injected scenario: the required observation is absent. No effects are admitted.');
    if (config.fault === 'stale-observation') return reject('StaleObservation', 'Injected scenario: the observation is outside the declared freshness window.');
    if (config.fault === 'wrong-authority' || config.actor !== actors[config.example]) return reject('Unauthorized', `This illustration expects the ${actors[config.example]} role. This is a demo policy check, not a signature verification.`);
    if (['swap', 'join', 'partial'].includes(config.example)) {
      const x = uint(config.reserveA), y = uint(config.reserveB), dx = uint(config.amountIn), min = uint(config.minOut, false);
      if (x + dx > LIMIT) return reject('BoundsExceeded', 'The new A reserve exceeds the mock limit of 1,000,000,000,000.');
      if (config.example === 'partial' && dx > 10000n) return reject('AllowanceExceeded', 'This partial-fill illustration has a 10,000 A total allowance.');
      const n = dx * 997n * y, d = x * 1000n + dx * 997n, amount = n / d;
      if (amount === 0n) return reject('ZeroOutput', 'This input produces no B after floor rounding.');
      p.metrics = [['Output B', `${amount}`], ['New reserve A', `${x + dx}`], ['New reserve B', `${y - amount}`],
        ['Fee multiplier', '997 / 1000'], ['Minimum output B', `${min}`]];
      p.source = `agreement pool = Exchange.ConstantProduct(A, B, fee: 997/1000)\nintent swap = SwapExactIn(${dx} A, receiveAtLeast: ${min} B)\n// Illustrative source only — no DSL parser or compiler connected.`;
      if (amount < min) return reject('SlippageExceeded', `Calculated ${amount} B is below the authorized minimum ${min} B.`);
      if (config.fault === 'rounding-mutation') return reject('RoundingMismatch', `Injected ceiling rule would produce ${(n + d - 1n) / d} B; the package requires floor output ${amount} B.`);
      p.effects = [`Proposed demo transfer: ${dx} A from trader to pool.`, `Proposed demo transfer: ${amount} B from pool to trader.`, 'No assets have moved.'];
      p.headline = 'Exact integer swap illustration';
      if (config.example === 'join') {
        p.headline = 'Two-parent composition illustration';
        p.metrics.push(['Writable predecessor 1', 'demo-pool@0'], ['Writable predecessor 2', 'demo-loan@0']);
        p.effects.push('Separate child rule: record the imported loan principal due of 500 USD, not a token payment.');
        p.diagnostics.push({ code: 'CompositionUnproved', message: 'This scripted join binds two demo identities. No composition certificate or PCD construction is implemented.' });
      }
      if (config.example === 'partial') {
        p.metrics.push(['Remaining allowance A', `${10000n - dx}`]);
        p.diagnostics.push({ code: 'FreshSignatureRequired', message: 'This is one fill at revision 0. A successor fill needs new state and a fresh signature; automatic continuation is not implemented.' });
      }
    } else if (config.example === 'loan') {
      const [principal, ps] = decimal(config.principal), [rate, rs] = decimal(config.rate), days = uint(config.days, true, 3660n);
      if (principal === 0n || rate > rs) return reject('BoundsExceeded', 'Principal must be positive and this first-period preview permits a rate from 0 to 1.');
      p.metrics = [['Interest, exact USD', rational(principal * rate * days, ps * rs * 365n)],
        ['Accrual days', `${days}`], ['Convention', 'Actual / 365 — first-period illustration'], ['Settlement asset', 'Not configured']];
      p.headline = 'First-period interest illustration';
      p.effects = ['The imported schedule records a 500 USD principal due; it is not paid.', 'The edited preview does not regenerate the ACTUS schedule or settle any obligation.'];
      p.diagnostics.push({ code: 'NumericProfileUnresolved', message: 'Exact rational arithmetic is separate from the pinned finite-decimal trace. No all-field ACTUS compatibility is claimed.' });
      p.source = `agreement repayment = Actus.LAM(principal: ${config.principal} USD, rate: ${config.rate})\npreview firstPeriod(days: ${days}, dayCount: Actual365)\n// Illustrative source only. The imported timeline remains unchanged.`;
    } else if (config.example === 'calendar') {
      p.headline = 'Calculation date is separate from payment date';
      p.metrics = [['CSF interest, imported USD', '26.6666666666667'], ['SCF interest, imported USD', '27.5'], ['Payment date', '2013-04-01'], ['Comparison', 'pam08 versus pam09 — imported reference']];
      p.effects = ['Both imported payments shift to a business day; calculation-before/after-shift changes interest.', 'No calendar engine or payment has been executed.'];
      p.source = 'compare imported ACTUS pam08(CSF) with pam09(SCF)\n// Source contrast, not generated evaluation.';
    } else if (config.example === 'option') {
      p.headline = 'Exercise and settlement are different events';
      p.metrics = [['Underlying, imported', 'AAPL 91.2'], ['Strike, imported', '80'], ['Exercise amount, imported', '11.2'], ['XD payoff, imported', '0'], ['STD payoff, imported', '11.2']];
      p.effects = ['Exercise fixes the amount; the later imported STD event describes settlement cash flow.', 'Actual payment evidence is unavailable.'];
      p.source = 'agreement option = Actus.OPTNS(imported: "option02")\n// Imported exercise/settlement trace; no option evaluator connected.';
    } else {
      const amount = uint(config.amountIn);
      if (amount > 20000n) return reject('MandateCapExceeded', 'This scripted mandate permits at most 20,000 units to the destination.');
      p.headline = 'Authority and allocation are separate checks';
      p.metrics = [['Actor', config.actor], ['Destination cap', '20000'], ['Allocation', `${amount}`], ['Loss bearer', 'Supplier assets in the scripted lending scenario']];
      p.effects = [`Proposed demo allocation of ${amount} units within the destination cap.`, 'Health, accrual and liquidation are unimplemented; no lending certificate is produced.'];
      p.diagnostics.push({ code: 'HealthUnverified', message: 'Authority/cap illustration only. Collateral health and loss-allocation correctness require the lending package.' });
      p.source = `intent allocate = Mandate.move(${amount}, actor: allocator, cap: 20000)\n// Scripted authority/cap illustration; not a MetaMorpho implementation.`;
    }
    return p;
  } catch (error) {
    return reject('BoundsExceeded', error instanceof Error ? error.message : 'Invalid bounded input.');
  }
}

export function newSession(): Session { return { stage: 0, statement: '', evidence: null, receipt: null }; }

function statement(config: Config): string {
  // All known fields are bound explicitly; this is a demo snapshot, not a signing codec.
  return JSON.stringify(Object.fromEntries(Object.keys(defaultConfig(config.example)).sort().map(key => [key, config[key as keyof Config]])));
}

function predecessors(config: Config): string[] {
  if (config.example === 'join') return ['demo-pool@0', 'demo-loan@0'];
  return [config.example === 'swap' || config.example === 'partial' ? 'demo-pool@0' : `demo-${config.example}@0`];
}

export function advance(session: Session, config: Config, consumed: string[]): { session: Session; consumed: string[]; message: string } {
  const unchanged = (message: string) => ({ session, consumed, message });
  const snapshot = statement(config);
  if (session.stage > 0 && session.statement !== snapshot) return { session: newSession(), consumed, message: 'Inputs changed. Prepared demo authorization and evidence were cleared.' };
  const preview = evaluate(config);
  if (!preview.ok) return unchanged(preview.diagnostics.map(d => `${d.code}: ${d.message}`).join(' '));
  if (session.stage === 0) return { session: { ...newSession(), stage: 1, statement: snapshot }, consumed, message: 'Demo plan prepared. Required real property certificates remain unavailable.' };
  if (session.stage === 1) return { session: { ...session, stage: 2 }, consumed, message: 'Simulated authorization recorded. No wallet or real signature was used.' };
  if (session.stage === 2) return { session: { ...session, stage: 3, evidence: config.fault === 'missing-proof' ? null : {
    kind: 'SimulatedEvidence', statement: config.fault === 'altered-proof' ? 'altered-demo-statement' : snapshot,
    proof: 'DEMO_ONLY_NOT_A_CRYPTOGRAPHIC_PROOF', realVerification: 'unavailable'
  } }, consumed, message: 'Demo proof response created. It is not cryptographic evidence.' };
  if (session.stage === 3) {
    if (config.fault === 'wrong-domain') return unchanged('WrongDomain: injected deployment mismatch; demo proof check rejected.');
    if (!session.evidence || session.evidence.kind !== 'SimulatedEvidence' || session.evidence.statement !== snapshot) return unchanged('InvalidProof: missing or altered simulated evidence.');
    return { session: { ...session, stage: 4 }, consumed, message: 'Demo statement check passed. Real verification remains unavailable; live-state checks are separate.' };
  }
  if (session.stage === 4) {
    if (config.fault === 'stale-predecessor') return unchanged('StalePredecessor: demo proof check passed, but the selected state is no longer current.');
    if (config.fault === 'cancelled') return unchanged('Cancelled: the scripted cancellation precedes this fill.');
    const inputs = predecessors(config);
    if (config.fault === 'duplicate-consumption' || inputs.some(id => consumed.includes(id))) return unchanged('AlreadyConsumed: demo ledger rejected a conflicting use of the same predecessor.');
    if (consumed.length + inputs.length > 64) return unchanged('DemoStorageFull: reset the local demo ledger before creating further scenarios.');
    return { session: { ...session, stage: 5, receipt: `simulated-submission:${inputs.join('+')}` },
      consumed: [...consumed, ...inputs], message: 'Simulated submission recorded locally. No transaction was sent to a chain.' };
  }
  return unchanged('This simulated submission is already recorded. Repeating it does not create a new payment.');
}

export function verifyRealProof(_evidence: unknown): { outcome: 'unavailable'; reason: string } {
  return { outcome: 'unavailable', reason: 'No cryptographic verifier, contract checker or real ledger adapter is connected.' };
}

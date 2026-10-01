/**
 * Facts for the eight operation-family reference entries. Every value was read from the shipped file at the
 * pinned commit (SHA-256 and byte size of the committed bytes) or from the output of `mori inspect` on it.
 * The page renders all eight entries from this one shape, so their fields appear in the same order.
 */
import amm from '../../../../packages/moriarty-beta/examples/amm.mori?raw';
import lending from '../../../../packages/moriarty-beta/examples/lending.mori?raw';
import stablecoin from '../../../../packages/moriarty-beta/examples/stablecoin.mori?raw';
import derivatives from '../../../../packages/moriarty-beta/examples/derivatives.mori?raw';
import oracle from '../../../../packages/moriarty-beta/examples/oracle.mori?raw';
import governance from '../../../../packages/moriarty-beta/examples/governance.mori?raw';
import bridge from '../../../../packages/moriarty-beta/examples/bridge.mori?raw';
import staking from '../../../../packages/moriarty-beta/examples/staking.mori?raw';
import type { FamilyId } from '../data';

/** One action of a family file: the action, the intent it uses, the operation call and the intent's other fields. */
export interface FamilyOperation {
  action: string;
  intent: string;
  operation: string;
  /** Intent fields besides `operation`, as written in the file. */
  other: string[];
}

/** One declaration of a family file. `fields` lists the fields besides `domain` and `id`. */
export interface FamilyDeclaration {
  name: string;
  kind: string;
  id: string | null;
  domain: string | null;
  fields: string;
}

/** One number written in a family file. `type` is `Qty<Asset>`, `scalar` or `string`. */
export interface FamilyValue {
  literal: string;
  where: string;
  type: string;
  scale: string;
  value: string;
}

export interface FamilyEntry {
  id: FamilyId;
  file: string;
  sha256: string;
  bytes: number;
  agreement: string;
  source: string;
  operations: FamilyOperation[];
  declarations: FamilyDeclaration[];
  values: FamilyValue[];
}

/** The five declarations every family file starts with, in source order. */
const BASE: FamilyDeclaration[] = [
  { name: 'Preview', kind: 'domain', id: 'Midnight', domain: null, fields: 'chain: "midnight", network: "preview"' },
  { name: 'Alice', kind: 'account', id: 'Alice', domain: 'Preview', fields: '' },
  { name: 'Bob', kind: 'account', id: 'Bob', domain: 'Preview', fields: '' },
  { name: 'USD', kind: 'asset', id: 'USDCanonical', domain: 'Preview', fields: 'scale: 2, representation: "canonical", symbol: "USD"' },
  { name: 'GOLD', kind: 'asset', id: 'GoldCanonical', domain: 'Preview', fields: 'scale: 3, representation: "canonical", symbol: "GOLD"' },
];

const usd = (literal: string, where: string, atoms: string): FamilyValue => ({ literal, where, type: 'Qty<USD>', scale: '2', value: atoms });
const gold = (literal: string, where: string, atoms: string): FamilyValue => ({ literal, where, type: 'Qty<GOLD>', scale: '3', value: atoms });
const scalar = (literal: string, where: string): FamilyValue => ({ literal, where, type: 'scalar', scale: '', value: literal });

export const FAMILY_ENTRIES: FamilyEntry[] = [
  {
    id: 'amm',
    file: 'examples/amm.mori',
    sha256: '6e1c3790202964bd4202be9fa67cd7f81beb559c25918055e023442ba9c0ef11',
    bytes: 1095,
    agreement: 'AmmDemo',
    source: amm,
    operations: [
      { action: 'swap', intent: 'Swap', operation: 'amm.swap_exact_input', other: ['rounding: "output floor benefits pool"', 'failure: "first failure; no effects"'] },
      { action: 'mint_lp', intent: 'MintLP', operation: 'amm.mint', other: [] },
      { action: 'redeem_lp', intent: 'RedeemLP', operation: 'amm.redeem', other: [] },
    ],
    declarations: [...BASE, { name: 'Spot', kind: 'pool', id: 'Spot', domain: 'Preview', fields: 'assets: [USD, GOLD]' }],
    values: [
      usd('100.00 USD', 'Swap: input', '10000'),
      gold('0.900 GOLD', 'Swap: net_floor', '900'),
      usd('0.30 USD', 'Swap: fee_cap', '30'),
      usd('100.00 USD', 'MintLP: amounts[0]', '10000'),
      gold('1.000 GOLD', 'MintLP: amounts[1]', '1000'),
      scalar('10', 'MintLP: minimum_share_atoms'),
      scalar('10', 'RedeemLP: share_atoms'),
    ],
  },
  {
    id: 'lending',
    file: 'examples/lending.mori',
    sha256: '2a85e347e953120c070982dd844c65c7afbe2899ec198c242e0aceea09995bb3',
    bytes: 1062,
    agreement: 'LendingDemo',
    source: lending,
    operations: [
      { action: 'borrow', intent: 'Borrow', operation: 'lending.originate', other: ['relation: "aggregate locks and funded principal; financial validation open"'] },
      { action: 'roll', intent: 'Roll', operation: 'lending.roll_forward', other: [] },
      { action: 'liquidate', intent: 'Liquidate', operation: 'lending.liquidate', other: ['continuation: "residual debt persists"'] },
    ],
    declarations: [...BASE, { name: 'Loan', kind: 'obligation', id: 'Loan', domain: 'Preview', fields: 'asset: USD' }],
    values: [usd('500.00 USD', 'Borrow: principal', '50000'), gold('1.000 GOLD', 'Borrow: collateral', '1000'), scalar('200', 'Roll: to_round')],
  },
  {
    id: 'stablecoins',
    file: 'examples/stablecoin.mori',
    sha256: 'a9695866e7bdbc3474735c632cef207682d1c50a7be25694489ebe36a6ce7400',
    bytes: 1138,
    agreement: 'StablecoinDemo',
    source: stablecoin,
    operations: [
      { action: 'mint', intent: 'Mint', operation: 'stablecoin.mint', other: ['observation: "authenticated peg required; open"'] },
      { action: 'redeem', intent: 'Redeem', operation: 'stablecoin.redeem', other: [] },
      { action: 'emergency', intent: 'Emergency', operation: 'stablecoin.emergency_settle', other: ['continuation: "existing redemption duties preserved"'] },
    ],
    declarations: [...BASE, { name: 'Stable', kind: 'instrument', id: 'Stable', domain: 'Preview', fields: 'asset: USD, backing: GOLD' }],
    values: [
      usd('100.00 USD', 'Mint: supply', '10000'),
      gold('0.200 GOLD', 'Mint: backing', '200'),
      usd('100.00 USD', 'Redeem: burn', '10000'),
      gold('0.190 GOLD', 'Redeem: minimum_backing', '190'),
      usd('100.00 USD', 'Emergency: claim', '10000'),
    ],
  },
  {
    id: 'options',
    file: 'examples/derivatives.mori',
    sha256: '17b0e9ea666e0867b032c40605a5a89b02beddc043affa9a1d1d6256cf187449',
    bytes: 1244,
    agreement: 'DerivativesDemo',
    source: derivatives,
    operations: [
      { action: 'fix', intent: 'Fix', operation: 'option.fix', other: [] },
      { action: 'exercise', intent: 'Exercise', operation: 'option.exercise', other: ['continuation: "missing fixing retains reserve and exercise duty"'] },
      { action: 'settle', intent: 'Settle', operation: 'option.settle', other: ['rounding: "payoff floor benefits reserve"'] },
    ],
    declarations: [
      ...BASE,
      {
        name: 'Call',
        kind: 'instrument',
        id: 'Call',
        domain: 'Preview',
        fields: 'underlying: GOLD, settlement: USD, strike_units: "USD per GOLD", strike: "100.00", exercise_round: 150, collateral: 500.00 USD',
      },
      { name: 'Fixing', kind: 'observation', id: 'Fixing', domain: 'Preview', fields: 'unit: "USD per GOLD", source: "feed", observed_round: 150' },
    ],
    values: [
      { literal: '"100.00"', where: 'Call: strike', type: 'string', scale: '', value: 'not converted' },
      scalar('150', 'Call: exercise_round'),
      usd('500.00 USD', 'Call: collateral', '50000'),
      scalar('150', 'Fixing: observed_round'),
      usd('100.00 USD', 'Settle: payoff', '10000'),
    ],
  },
  {
    id: 'oracles',
    file: 'examples/oracle.mori',
    sha256: '2da41606e65fe6385df8e148bc4ade41a802d377245323d8bc7838944372ac04',
    bytes: 897,
    agreement: 'OracleDemo',
    source: oracle,
    operations: [
      {
        action: 'read_fixing',
        intent: 'Read',
        operation: 'oracle.select',
        other: ['failure: "ObservationStale or ObservationDisputed"', 'continuation: "Pending retains dependent duty"'],
      },
    ],
    declarations: [
      ...BASE,
      {
        name: 'Fixing',
        kind: 'observation',
        id: 'Fixing',
        domain: 'Preview',
        fields: 'unit: "USD per GOLD", source: "approved-feed", observed_round: 140, maximum_age: 5, finality: "finalized", provenance: "provider binding open"',
      },
    ],
    values: [scalar('140', 'Fixing: observed_round'), scalar('5', 'Fixing: maximum_age')],
  },
  {
    id: 'governance',
    file: 'examples/governance.mori',
    sha256: '1a9eb7c670f957429d5a4c82d89ed0f3dd1e8147210c3e524bbfd34885b2af4f',
    bytes: 971,
    agreement: 'GovernanceDemo',
    source: governance,
    operations: [
      {
        action: 'queue',
        intent: 'Queue',
        operation: 'governance.queue',
        other: ['earliest_round: 180', 'veto_before: 179', 'relation: "existing signed duties retain terms"'],
      },
      { action: 'execute', intent: 'Execute', operation: 'governance.execute', other: [] },
      { action: 'veto', intent: 'Veto', operation: 'governance.veto', other: [] },
    ],
    declarations: [...BASE, { name: 'Terms', kind: 'policy', id: 'Terms', domain: null, fields: 'epoch: 3, source_hash: "terms-v3", duty_preservation: "required"' }],
    values: [scalar('3', 'Terms: epoch'), scalar('4', 'Queue: next_epoch'), scalar('180', 'Queue: earliest_round'), scalar('179', 'Queue: veto_before')],
  },
  {
    id: 'bridges',
    file: 'examples/bridge.mori',
    sha256: 'ed1899d32b244e5f2d7cab829baa4135c13722cc3340fbe90febaa08aaf12c27',
    bytes: 1227,
    agreement: 'BridgeDemo',
    source: bridge,
    operations: [
      { action: 'escrow', intent: 'Escrow', operation: 'bridge.escrow', other: [] },
      { action: 'claim', intent: 'Claim', operation: 'bridge.claim', other: [] },
      { action: 'recover', intent: 'Recover', operation: 'bridge.recover', other: ['failure: "timeout alone is not proof of foreign nonreceipt"'] },
    ],
    declarations: [
      ...BASE,
      { name: 'Foreign', kind: 'domain', id: 'Foreign', domain: null, fields: 'chain: "example", network: "test"' },
      { name: 'ForeignAlice', kind: 'account', id: 'ForeignAlice', domain: 'Foreign', fields: '' },
      { name: 'WrappedUSD', kind: 'asset', id: 'WrappedUSD', domain: 'Foreign', fields: 'scale: 2, representation: "bridge-claim"' },
    ],
    values: [
      usd('100.00 USD', 'Escrow: amount', '10000'),
      { literal: '100.00 WrappedUSD', where: 'Claim: amount', type: 'Qty<WrappedUSD>', scale: '2', value: '10000' },
      usd('100.00 USD', 'Recover: amount', '10000'),
    ],
  },
  {
    id: 'staking',
    file: 'examples/staking.mori',
    sha256: '301cfa8c15f1a848db7b6a23aa3b246fd646a970b8212458d5990e1da8a9b385',
    bytes: 1303,
    agreement: 'StakingDemo',
    source: staking,
    operations: [
      { action: 'deposit', intent: 'Deposit', operation: 'staking.deposit', other: ['rounding: "share mint floor benefits vault"'] },
      { action: 'reward', intent: 'Reward', operation: 'staking.reward', other: [] },
      { action: 'slash', intent: 'Slash', operation: 'staking.slash', other: ['loss: "signed slash priority; pending withdrawal remains explicit"'] },
      { action: 'unbond', intent: 'Unbond', operation: 'staking.unbond', other: [] },
      { action: 'withdraw', intent: 'Withdraw', operation: 'staking.withdraw', other: [] },
    ],
    declarations: [...BASE, { name: 'Shares', kind: 'share_class', id: 'VaultShare', domain: 'Preview', fields: 'backing: USD' }],
    values: [
      usd('100.00 USD', 'Deposit: backing', '10000'),
      usd('5.00 USD', 'Reward: amount', '500'),
      usd('3.00 USD', 'Slash: amount', '300'),
      scalar('100', 'Unbond: share_atoms'),
      scalar('100', 'Withdraw: share_atoms'),
      usd('90.00 USD', 'Withdraw: minimum_backing', '9000'),
    ],
  },
];

/**
 * The closed named-argument registry (`SCHEMAS` in src/frontend.ts, reported as `operationSchemas` by
 * `mori check --json`). Argument order and role names are the tool's own.
 */
export const REGISTRY: [string, [string, string][]][] = [
  ['atoms', [['asset', 'asset'], ['value', 'scalar']]],
  ['min', [['a', 'numeric'], ['b', 'numeric']]],
  ['max', [['a', 'numeric'], ['b', 'numeric']]],
  ['rounds', [['domain', 'domain'], ['from', 'scalar'], ['to', 'scalar']]],
  ['transfer', [['from', 'account'], ['to', 'account'], ['fee_to', 'account'], ['value', 'qty'], ['fee', 'qty']]],
  ['repay', [['obligation', 'obligation'], ['payer', 'account'], ['amount', 'qty']]],
  ['amm.swap_exact_input', [['pool', 'pool'], ['owner', 'account'], ['input', 'qty'], ['output_asset', 'asset'], ['net_floor', 'qty'], ['fee_cap', 'qty']]],
  ['amm.redeem', [['pool', 'pool'], ['owner', 'account'], ['share_atoms', 'scalar']]],
  ['amm.mint', [['pool', 'pool'], ['owner', 'account'], ['amounts', 'qty[]'], ['minimum_share_atoms', 'scalar']]],
  ['lending.originate', [['obligation', 'obligation'], ['debtor', 'account'], ['creditor', 'account'], ['principal', 'qty'], ['collateral', 'qty']]],
  ['lending.liquidate', [['obligation', 'obligation']]],
  ['lending.roll_forward', [['obligation', 'obligation'], ['to_round', 'scalar']]],
  ['stablecoin.mint', [['instrument', 'instrument'], ['owner', 'account'], ['supply', 'qty'], ['backing', 'qty']]],
  ['stablecoin.redeem', [['instrument', 'instrument'], ['owner', 'account'], ['burn', 'qty'], ['minimum_backing', 'qty']]],
  ['stablecoin.emergency_settle', [['instrument', 'instrument'], ['owner', 'account'], ['claim', 'qty']]],
  ['option.fix', [['instrument', 'instrument'], ['observation', 'observation']]],
  ['option.exercise', [['instrument', 'instrument'], ['holder', 'account']]],
  ['option.settle', [['instrument', 'instrument'], ['holder', 'account'], ['payoff', 'qty']]],
  ['oracle.select', [['observation', 'observation']]],
  ['governance.queue', [['policy', 'policy'], ['next_epoch', 'scalar']]],
  ['governance.execute', [['policy', 'policy']]],
  ['governance.veto', [['policy', 'policy']]],
  ['bridge.escrow', [['owner', 'account'], ['amount', 'qty'], ['destination', 'domain'], ['claim_id', 'string']]],
  ['bridge.claim', [['owner', 'account'], ['amount', 'qty'], ['source', 'domain'], ['claim_id', 'string']]],
  ['bridge.recover', [['owner', 'account'], ['amount', 'qty'], ['claim_id', 'string']]],
  ['staking.deposit', [['owner', 'account'], ['shares', 'share_class'], ['backing', 'qty']]],
  ['staking.reward', [['shares', 'share_class'], ['amount', 'qty']]],
  ['staking.slash', [['shares', 'share_class'], ['amount', 'qty']]],
  ['staking.unbond', [['owner', 'account'], ['shares', 'share_class'], ['share_atoms', 'scalar']]],
  ['staking.withdraw', [['owner', 'account'], ['shares', 'share_class'], ['share_atoms', 'scalar'], ['minimum_backing', 'qty']]],
];

export const REGISTRY_MAP = new Map(REGISTRY);

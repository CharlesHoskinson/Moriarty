/**
 * The 24 DeFi action reference targets, DA01-DA24.
 *
 * The families are the human-facing map; these are what the language must
 * actually execute. The distinguishing test is the case that separates a
 * correct implementation from a plausible-looking wrong one.
 */

import type { FamilyId } from './categories';

export interface ActionTarget {
  id: string;
  /** Families this target belongs to. DA12 legitimately belongs to two. */
  families: FamilyId[];
  action: string;
  /** What the action must mean. */
  requirement: string;
  /** The case that separates correct from plausible-but-wrong. */
  distinguishingTest: string;
  /** Roadmap data. Belongs in the delivery section, not in a category tab. */
  disposition: string;
}

export const ACTION_TARGETS: readonly ActionTarget[] = [
  {
    id: 'DA01',
    families: ['F1'],
    action: 'swap exact input / exact output',
    requirement: 'asset-indexed exchange; fee and slippage accounting',
    distinguishingTest: 'rounding; reserve safety; net minimum',
    disposition: 'local fixed exact-input example only',
  },
  {
    id: 'DA02',
    families: ['F1'],
    action: 'provide / remove liquidity',
    requirement: 'share mint and burn; reserve contributions',
    distinguishingTest: 'proportional entitlement; donation and zero-supply boundaries',
    disposition: 'needs extension',
  },
  {
    id: 'DA03',
    families: ['F1'],
    action: 'open / adjust / close liquidity position',
    requirement: 'bounded position identity and range',
    distinguishingTest: 'fee allocation and finite tick or range traversal',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA04',
    families: ['F2'],
    action: 'supply / redeem lending claims',
    requirement: 'claim shares and liquidity-constrained withdrawal',
    distinguishingTest: 'insufficient pool liquidity rejects without erasing the claim',
    disposition: 'needs extension',
  },
  {
    id: 'DA05',
    families: ['F2'],
    action: 'post / release collateral',
    requirement: 'encumbrance and debt-dependent release',
    distinguishingTest: 'release cannot violate the collateral rule',
    disposition: 'needs extension',
  },
  {
    id: 'DA06',
    families: ['F2'],
    action: 'borrow / accrue / repay',
    requirement: 'nominal debt distinct from transfers; rate and time arithmetic',
    distinguishingTest: 'partial repayment preserves principal and interest allocation',
    disposition: 'local bounded loan episode only',
  },
  {
    id: 'DA07',
    families: ['F2'],
    action: 'liquidate / recognize default',
    requirement: 'authorized seizure; loss and residual debt allocation',
    distinguishingTest: 'partial liquidation and close factor; maturity before forfeiture',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA08',
    families: ['F2'],
    action: 'flash borrow / repay atomically',
    requirement: 'atomic multi-leg settlement with fee',
    distinguishingTest: 'every loan repaid within the same atomic transaction',
    disposition: 'needs normative atomicity fixture',
  },
  {
    id: 'DA09',
    families: ['F2'],
    action: 'refinance / novate / capitalize',
    requirement:
      'workflow over old debt, new debt, collateral and signed liability authority',
    distinguishingTest:
      'reject refinance without old-debt discharge; capitalization changes liability',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA10',
    families: ['F2'],
    action: 'issue / burn debt-backed stablecoin',
    requirement: 'issuance authority plus debt and collateral',
    distinguishingTest:
      'burn amount and released collateral obey the specified debt rule',
    disposition: 'needs extension',
  },
  {
    id: 'DA11',
    families: ['F3'],
    action: 'open / margin / fund / close derivative',
    requirement: 'position notional, margin and funding obligations',
    distinguishingTest: 'funding signs; insolvency; precise settlement convention',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA12',
    families: ['F3', 'P'],
    action: 'write / exercise / expire contingent claim',
    requirement: 'choice authority and exercise or payment dates',
    distinguishingTest: 'expiry does not erase an already exercised payment duty',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA13',
    families: ['P'],
    action: 'split / merge / resolve event claims',
    requirement: 'outcome-indexed claims; resolution evidence',
    distinguishingTest: 'no duplicate winning claim; invalid resolution rejects',
    disposition: 'needs primary lifecycle source',
  },
  {
    id: 'DA14',
    families: ['F4'],
    action: 'stake / account rewards / slash',
    requirement: 'share-rate or rebase accounting; loss allocation',
    distinguishingTest:
      'the same nominal token balance can have a changing entitlement',
    disposition: 'needs pinned protocol fixture',
  },
  {
    id: 'DA15',
    families: ['F4'],
    action: 'request unstake / claim exit',
    requirement: 'pending exit identity; custody; delayed completion',
    distinguishingTest: 'an exit request is not immediate token delivery',
    disposition: 'needs primary lifecycle source',
  },
  {
    id: 'DA16',
    families: ['F5'],
    action: 'issue / redeem external claim',
    requirement: 'attested claim; custody and legal assumptions',
    distinguishingTest:
      'missing external settlement evidence cannot discharge the duty',
    disposition: 'needs primary lifecycle source',
  },
  {
    id: 'DA17',
    families: ['F6'],
    action: 'deposit / mint / withdraw / redeem vault shares',
    requirement: 'asset and share conversions with method-specific rounding',
    distinguishingTest:
      'fees, rounding direction, initial donation and zero shares',
    disposition: 'source-defined target; implementation open',
  },
  {
    id: 'DA18',
    families: ['F6'],
    action: 'request / fulfill / claim asynchronous redemption',
    requirement: 'Pending to Claimable to Claimed; residual request amount',
    distinguishingTest:
      'partial claim and changed exchange rate; double claim rejects',
    disposition: 'source-defined target; implementation open',
  },
  {
    id: 'DA19',
    families: ['F6'],
    action: 'allocate / harvest / reinvest / unwind / rebalance',
    requirement: 'bounded workflow of trades, debt, shares and fees',
    distinguishingTest: 'losses and debt persist through unwind; liquidity shortage',
    disposition: 'needs pinned strategy fixture',
  },
  {
    id: 'DA20',
    families: ['X'],
    action: 'observe price / time / external event',
    requirement: 'typed observations with provenance, freshness and domain',
    distinguishingTest: 'stale or unauthorized evidence rejects',
    disposition: 'simulated observations only',
  },
  {
    id: 'DA21',
    families: ['X'],
    action: 'authorize exact plan / refine outcome intent',
    requirement: 'gross debit; net receipt; recipients; calls; new liabilities',
    distinguishingTest:
      'a refund cannot restore gross capacity; fees count against the net goal',
    disposition: 'local restricted profiles only',
  },
  {
    id: 'DA22',
    families: ['X'],
    action: 'change parameters / pause / migrate',
    requirement: 'bounded administrative action under a fixed claim policy',
    distinguishingTest:
      'cannot downgrade claims or reset work; affects existing positions',
    disposition: 'needs pinned policy fixture',
  },
  {
    id: 'DA23',
    families: ['X'],
    action: 'send / receive / refund pending message',
    requirement: 'bounded pending commitments and finality evidence',
    distinguishingTest: 'delayed or duplicate delivery; explicit refund duty',
    disposition: 'needs primary lifecycle source',
  },
  {
    id: 'DA24',
    families: ['X'],
    action: 'sequence / parallel / interleave / synchronize / message',
    requirement: 'operator-specific authority, duties, conflicts and fan-in',
    distinguishingTest:
      'split partitions work and claims; join cannot duplicate resource',
    disposition: 'specified only',
  },
] as const;

/** The five composition operators named by DA24. Always five. */
export const COMPOSITION_OPERATORS = [
  'sequence',
  'parallel',
  'interleave',
  'synchronize',
  'message',
] as const;

/**
 * The composition audit, independently reproduced locally.
 * Cross-category composition fails 5.14 times as often as within-category.
 * An earlier informal "sixty times" claim was checked and is wrong.
 */
export const COMPOSITION_RESULT = {
  eligiblePairs: 1830,
  clean: 1645,
  failures: 185,
  crossCategoryFailures: 182,
  withinCategoryFailures: 3,
  withinCategoryPairs: 143,
  crossCategoryPairs: 1687,
  withinCategoryRate: 0.020979,
  crossCategoryRate: 0.107884,
  ratio: 5.1425,
} as const;

export const targetsForFamily = (id: FamilyId): ActionTarget[] =>
  ACTION_TARGETS.filter((t) => t.families.includes(id));

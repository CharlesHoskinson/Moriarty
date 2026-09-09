/** Delivery roadmap: twelve sprints, three parallel tracks, milestones reached. */

export interface Sprint {
  id: string;
  deliverable: string;
  /** What decisively completes it. Not a status label — a test. */
  evidence: string;
}

export const SPRINTS: readonly Sprint[] = [
  {
    id: 'SP01',
    deliverable: 'Freeze behavior and reuse accepted foundations',
    evidence:
      'complete behavior, source and authority crosswalk; bounded native go or no-go',
  },
  {
    id: 'SP02',
    deliverable: 'Finish the language contract and authoring tools',
    evidence:
      'full lexical, EBNF and static specification plus working check and format',
  },
  {
    id: 'SP03',
    deliverable: 'Make K execute the financial distinctions',
    evidence:
      'runnable K and evaluator agreement; discharged base-domain correspondence claims',
  },
  {
    id: 'SP04',
    deliverable: 'Decide whether the full native verifier works',
    evidence:
      'all native and outer verifier controls, including the final accumulator decision',
  },
  {
    id: 'SP05',
    deliverable: 'Finalize loan and swap effects on the development network',
    evidence: 'both finalized with complete independently checked effects',
  },
  {
    id: 'SP06',
    deliverable: 'Produce and independently verify real recursive history',
    evidence:
      'real two-step recursive proof; retained bytes independently verified; mutations rejected',
  },
  {
    id: 'SP07',
    deliverable: 'Implement ACTUS without losing debt or fields',
    evidence: '277 fixtures, all fields, 18 executable types, 32 dispositions',
  },
  {
    id: 'SP08',
    deliverable: 'Implement behavior-driven DeFi and intent libraries',
    evidence:
      'all 72 DeFi rows, DA24, intent and request lifecycles, modeled regressions',
  },
  {
    id: 'SP09',
    deliverable: 'Require complete proof and authority at ledger acceptance',
    evidence:
      'all four mandatory claims, proved correspondence, verification-enabled acceptance',
  },
  {
    id: 'SP10',
    deliverable: 'Prove private handoff and bounded composition',
    evidence:
      'isolated private handoff, real split and join, all five composition operators',
  },
  {
    id: 'SP11',
    deliverable: 'Qualify the complete financial and formal scope',
    evidence:
      'every required behavior qualified across semantics, proof and network evidence',
  },
  {
    id: 'SP12',
    deliverable: 'Release something developers can use and reproduce',
    evidence:
      'usable end-to-end flow, two clean builders, two non-toy pilots, every gate',
  },
] as const;

export const TRACKS = [
  { name: 'Language', path: ['SP01', 'SP02', 'SP03'] },
  { name: 'Native feasibility and proofs', path: ['SP01', 'SP04', 'SP06'] },
  { name: 'Financial network integration', path: ['SP05'] },
] as const;

/** Milestones already reached. Each is a measured result, not a status label. */
export const MILESTONES = [
  {
    title: 'Bounded financial semantics execute in K',
    detail:
      'Transfer-only and Transfer-then-Repay, with AccrualFirst, PrincipalFirst or ProRata allocation and explicit none, floor or ceil conversion rounding. All 16 frozen cases match independent financial expectations, including exact rejection code and index.',
    figures: [
      { label: 'frozen cases matched', value: '16 / 16' },
      { label: 'transfer-only guards', value: '15' },
      { label: 'repayment guards', value: '28' },
    ],
  },
  {
    title: 'The successor syntax profile parses and formats',
    detail:
      'Separate lexical rules, an ISO/IEC 14977 EBNF grammar, a bounded parser, a canonical formatter and a read-only command line.',
    figures: [{ label: 'canonical format bound', value: '65,536 bytes' }],
  },
  {
    title: 'A contract deployed and settled on a local network',
    detail:
      'Deployment and call finalized with exact state readback and indexed blocks below the finalized head.',
    figures: [],
  },
] as const;

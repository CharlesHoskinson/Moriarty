/**
 * Facts shared by every documentation page. One home per fact: pages import these
 * constants rather than restating a version, a commit or a family's scope.
 */

export const REPOSITORY = 'https://github.com/CharlesHoskinson/Moriarty';
export const PACKAGE_VERSION = '0.1.0-beta.1';
export const PROFILE = 'moriarty-beta/1';

/** The commit every GitHub file link points at, so a link shows the bytes the page was written against. */
export const PINNED = 'df8b155143e90450168af04fbfd20f7620043620';
export const PINNED_SHORT = PINNED.slice(0, 7);

/** A link to a repository file at the pinned commit. `path` is relative to the repository root. */
export function blob(path: string): string {
  return `${REPOSITORY}/blob/${PINNED}/${path}`;
}

export const BETA_PATH = 'packages/moriarty-beta';
export const GETTING_STARTED = blob(`${BETA_PATH}/GETTING-STARTED.md`);
export const SIGNED_INTENT = blob(`${BETA_PATH}/SIGNED-INTENT.md`);
export const PRODUCT_CONTRACT = blob('docs/MORIARTY-PRODUCT-CONTRACT.md');

/** The five documentation pages and the two neighbours the masthead links to. */
export type PageKey = 'documentation' | 'tutorial' | 'howto' | 'reference' | 'explanation';

export interface PageInfo {
  key: PageKey;
  href: string;
  label: string;
  /** Short masthead label for phones. */
  short: string;
}

export const PAGES: PageInfo[] = [
  { key: 'documentation', href: 'documentation.html', label: 'Documentation', short: 'Docs' },
  { key: 'tutorial', href: 'tutorial.html', label: 'Tutorials', short: 'Learn' },
  { key: 'howto', href: 'how-to.html', label: 'How-to guides', short: 'How-to' },
  { key: 'reference', href: 'reference.html', label: 'Reference', short: 'Ref' },
  { key: 'explanation', href: 'explanation.html', label: 'Explanation', short: 'Explain' },
];

export const PAGE_HREF: Record<PageKey, string> = Object.fromEntries(PAGES.map((p) => [p.key, p.href])) as Record<PageKey, string>;

/** Diátaxis modes, used for the small badge on cross-links so a reader sees what kind of page a link opens. */
export type Mode = 'tutorial' | 'howto' | 'reference' | 'explanation';
export const MODE_LETTER: Record<Mode, string> = { tutorial: 'T', howto: 'H', reference: 'R', explanation: 'E' };
export const MODE_NAME: Record<Mode, string> = {
  tutorial: 'Tutorial',
  howto: 'How-to guide',
  reference: 'Reference',
  explanation: 'Explanation',
};

/** The eight operation families. Every one is SpecifiedOnly: checked for structure and names, refused for execution. */
export const FAMILY_IDS = ['amm', 'lending', 'stablecoins', 'options', 'oracles', 'governance', 'bridges', 'staking'] as const;
export type FamilyId = (typeof FAMILY_IDS)[number];

export interface Family {
  id: FamilyId;
  name: string;
  /** One line: what the example states. */
  scope: string;
  /** The shipped file, relative to packages/moriarty-beta. */
  file: string;
  /** The agreement the how-to recipe writes, with its article: "How to write <agreement>". */
  agreement: string;
}

export const FAMILIES: Family[] = [
  { id: 'amm', name: 'AMM', scope: 'Swap a fixed input for at least a named output; mint and redeem liquidity.', file: 'examples/amm.mori', agreement: 'an AMM agreement' },
  { id: 'lending', name: 'Lending', scope: 'Borrow, roll and liquidate.', file: 'examples/lending.mori', agreement: 'a lending agreement' },
  { id: 'stablecoins', name: 'Stablecoins', scope: 'Mint against backing, redeem, and keep the redemption duty.', file: 'examples/stablecoin.mori', agreement: 'a stablecoin agreement' },
  { id: 'options', name: 'Options and derivatives', scope: 'Fixing, exercise and settlement intents for a call.', file: 'examples/derivatives.mori', agreement: 'an option agreement' },
  { id: 'oracles', name: 'Oracles', scope: 'A named observation an agreement selects. It publishes no price.', file: 'examples/oracle.mori', agreement: 'an oracle agreement' },
  { id: 'governance', name: 'Governance', scope: 'Queue, execute and veto a delayed policy change.', file: 'examples/governance.mori', agreement: 'a governance agreement' },
  { id: 'bridges', name: 'Bridges', scope: 'Escrow, claim and recovery across two domains.', file: 'examples/bridge.mori', agreement: 'a bridge agreement' },
  { id: 'staking', name: 'Staking', scope: 'Deposit, reward, slash and exit.', file: 'examples/staking.mori', agreement: 'a staking agreement' },
];

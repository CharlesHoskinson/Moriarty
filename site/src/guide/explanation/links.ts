import { PAGE_HREF, blob } from '../data';

/** Page-local link helpers for the Explanation page. */
export const T = PAGE_HREF.tutorial;
export const H = PAGE_HREF.howto;
export const R = PAGE_HREF.reference;
export const HUB = PAGE_HREF.documentation;

/** The proposal document that defines the horizon profile, at the pinned commit. */
export const HORIZON_DOC = blob('wiki-llm/beta-language-2026-09-30/FULL-LANGUAGE-HORIZON.md');
export const ROADMAP = blob('ROADMAP.md');
export const CONSOLIDATED_DESIGN = blob('docs/MORIARTY-CONSOLIDATED-DESIGN.md');
export const FRONTEND = blob('packages/moriarty-beta/src/frontend.ts');

/** Link to a glossary entry on the reference page (ids are term-<slug>). */
export const term = (slug: string): string => `${R}#term-${slug}`;

/** The Midnight ledger source at the revision the native verifier pins. */
export const MIDNIGHT_LEDGER_INTENT =
  'https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/ledger/src/structure.rs';
export const VERIFIER_CRATE = blob('experiments/midnight-crypto/Cargo.toml');

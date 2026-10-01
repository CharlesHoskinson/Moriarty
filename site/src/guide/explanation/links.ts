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

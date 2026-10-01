import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, type MouseEvent, type ReactNode } from 'react';
import { createPortal } from 'react-dom';
import { GettingStarted } from './GettingStarted';
import { LanguagePatterns } from './LanguagePatterns';
import { Markets } from './Markets';
import { RiskGovernance } from './RiskGovernance';
import { CrossChain } from './CrossChain';

/**
 * Shell for tutorial.html. It owns the masthead, the lesson and on-this-page
 * navigation, local search, copy buttons and the overview and architecture
 * sections. The five lesson modules are siblings written elsewhere; the shell
 * reads their rendered sections (`section.guide-section[id]` with an `h2`) so
 * navigation, search and copy work for any content that follows that shape.
 */

const REPOSITORY = 'https://github.com/CharlesHoskinson/Moriarty';
const PACKAGE_VERSION = '0.1.0-beta.1';
const PROFILE = 'moriarty-beta/1';

/** Verbatim from packages/moriarty-beta/src/starter.ts (`starterSource`), the file `mori init` writes as invoice.mori. */
const TRANSFER_SOURCE = `profile "moriarty-beta/1";
agreement Invoice {
 domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
 account Buyer = { domain: Preview, id: "Owner" };
 account Seller = { domain: Preview, id: "Recipient" };
 account Treasury = { domain: Preview, id: "Fee" };
 asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical", symbol: "USD" };
 const price: Qty<USD> = 10.00 USD;
 const fee = 0.10 USD;
 intent Payment = {
  domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
  valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: price + fee, fee_cap: fee, net_floor: price,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee),
  source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };
 action pay uses Payment;
}`;

const GROUPS = ['Start building', 'Language', 'DeFi examples', 'Design and limits'] as const;

/**
 * Navigation label and group for each lesson id. A section may override either
 * with `data-nav` / `data-group` attributes; otherwise this table applies, then
 * the enclosing module wrapper and the section's h2.
 */
const META: Record<string, { nav: string; group: (typeof GROUPS)[number] }> = {
  overview: { nav: 'Overview', group: 'Start building' },
  'getting-started': { nav: 'Install and first check', group: 'Start building' },
  transfer: { nav: 'Transfer with a fee', group: 'Start building' },
  repayment: { nav: 'Loan repayment', group: 'Start building' },
  testing: { nav: 'Testing', group: 'Start building' },
  syntax: { nav: 'Surface syntax', group: 'Language' },
  patterns: { nav: 'Design patterns', group: 'Language' },
  amm: { nav: 'AMM', group: 'DeFi examples' },
  lending: { nav: 'Lending', group: 'DeFi examples' },
  stablecoins: { nav: 'Stablecoins', group: 'DeFi examples' },
  options: { nav: 'Options / derivatives', group: 'DeFi examples' },
  oracles: { nav: 'Oracles', group: 'DeFi examples' },
  governance: { nav: 'Governance', group: 'DeFi examples' },
  bridges: { nav: 'Bridges', group: 'DeFi examples' },
  staking: { nav: 'Staking', group: 'DeFi examples' },
  trust: { nav: 'Support and trust', group: 'Design and limits' },
  architecture: { nav: 'Architecture', group: 'Design and limits' },
};

/** The beginner route. Previous/next links are added to every lesson on it except the last. */
const PATH = ['overview', 'getting-started', 'transfer', 'repayment', 'testing', 'syntax', 'patterns', 'amm'] as const;

const AREAS = ['amm', 'lending', 'stablecoins', 'options', 'oracles', 'governance', 'bridges', 'staking'] as const;

/** One line of scope per area. Every area is SpecifiedOnly; the LocalS0 repayment lesson is a separate operation. */
const AREA_SCOPE: Record<(typeof AREAS)[number], string> = {
  amm: 'Swap a fixed input for at least a named output; mint and redeem liquidity.',
  lending: 'Borrow, roll and liquidate.',
  stablecoins: 'Mint against backing, redeem, and keep the redemption duty.',
  options: 'Fixing, exercise and settlement intents for a call.',
  oracles: 'A named observation an agreement selects. It publishes no price.',
  governance: 'Queue, execute and veto a delayed policy change.',
  bridges: 'Escrow, claim and recovery across two domains.',
  staking: 'Deposit, reward, slash and exit.',
};

interface Lesson {
  id: string;
  title: string;
  group: string;
}

interface Heading {
  id: string;
  title: string;
}

interface Hit {
  target: string;
  lesson: string;
  heading: string;
  excerpt: string;
}

const COPY_FAILED = 'Copy failed. Select the text and copy it manually.';

function Status({ children }: { children: ReactNode }) {
  return (
    <span className="guide-status" data-status={typeof children === 'string' ? children : undefined}>
      {children}
    </span>
  );
}

/** Clipboard write that also works without navigator.clipboard (insecure origins); resolves false when nothing could copy. */
async function copyText(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard && typeof navigator.clipboard.writeText === 'function') {
      await navigator.clipboard.writeText(text);
      return true;
    }
  } catch {
    // fall through to the selection route
  }
  const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  try {
    const area = document.createElement('textarea');
    area.value = text;
    area.setAttribute('readonly', '');
    area.setAttribute('aria-hidden', 'true');
    area.style.cssText = 'position:fixed;top:0;left:0;opacity:0;pointer-events:none';
    document.body.append(area);
    area.select();
    const ok = typeof document.execCommand === 'function' && document.execCommand('copy');
    area.remove();
    return ok;
  } catch {
    return false;
  } finally {
    previous?.focus();
  }
}

/** Copy control with visible, announced feedback. The copied text is passed in, never scraped from the button. */
function CopyButton({ text, label, done }: { text: string; label: string; done: string }) {
  const [state, setState] = useState<'idle' | 'ok' | 'fail'>('idle');
  const timer = useRef<number | undefined>(undefined);
  useEffect(() => () => window.clearTimeout(timer.current), []);
  const copy = async () => {
    const ok = await copyText(text);
    setState(ok ? 'ok' : 'fail');
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setState('idle'), 4000);
  };
  return (
    <div className="guide-copybar">
      <button type="button" className="guide-copy" onClick={copy}>
        {label}
      </button>
      <span role="status" className={`guide-copy-state guide-copy-${state}`}>
        {state === 'ok' ? done : state === 'fail' ? COPY_FAILED : ''}
      </span>
    </div>
  );
}

function Overview() {
  return (
    <section className="guide-section" id="overview" aria-labelledby="overview-h">
      <h2 id="overview-h">What Moriarty is for</h2>
      <p>
        Moriarty states a financial agreement so that every part of the money is explicit: which asset,
        in which domain, from which account to which account, how many atoms, under what cap and until
        which round. The intent in the source records every term an owner would sign, so nothing is left to
        a default the owner never saw. The starter program below is unsigned and authorizes nothing; the
        separate signed-intent flow is where a signature over exactly these bytes is checked.
      </p>

      <h3 id="overview-first">Your first program: a transfer</h3>
      <p>
        <Status>LocalS0</Status> <Status>PreparedUnqualified</Status> This is the complete program that{' '}
        <code>mori init</code> writes as <code>invoice.mori</code>. It moves 10.00 USD from the account
        with ID <code>Owner</code> to <code>Recipient</code>, plus a 0.10 USD fee to <code>Fee</code>, all in
        asset <code>A</code> at scale 2 on the Midnight preview domain. Scale 2 means 10.00 USD is 1000
        atoms, so the gross debit is 1000 + 10 = 1010 atoms.
      </p>
      <CopyButton text={TRANSFER_SOURCE} label="Copy complete program" done="Copied the complete program." />
      <pre className="guide-code" tabIndex={0} aria-label="invoice.mori, complete program" data-copy-owned="true">
        <code>{TRANSFER_SOURCE}</code>
      </pre>
      <p>
        Preparation produces a local candidate from a stipulated scenario. It is not signed, proved,
        sent or deployed, and nothing in this first lesson does any of those. <Status>Open</Status>{' '}
        Authority of the key, proof and ledger acceptance remain open for this example. The{' '}
        <a href="#getting-started">install and first check lesson</a> runs it, and the{' '}
        <a href="#transfer">transfer lesson</a> shows the command output next to this source.
      </p>

      <figure className="guide-figure guide-hero">
        <img
          src="tutorial-assets/intention-workbench.png"
          width={1536}
          height={1024}
          alt="Decorative illustration: a person at a desk with an open ledger and a signed paper with a red seal. Clear boxes of cubes, spheres, pyramids and coins feed a black central box, whose four outputs meet at a document with a check mark; an arrow from it leads to an arched stone gate with open wooden doors."
          decoding="async"
        />
        <figcaption>
          Conceptual illustration: an agreement is written down and its parts are kept separate. The check
          mark and the open gate are decoration. They depict no transaction, balance, check result or
          acceptance, and nothing on this page is signed, proved or settled.
        </figcaption>
      </figure>

      <p>
        The language is the product. Any developer may write, check and prepare programs in it without
        approval from a maintainer, a registry or a hosted service. The Federated DeFi Kernel is an
        optional federation that coordinates evidence and routing; no tutorial here depends on it, and the{' '}
        <a href="kernel.html">kernel explanation</a> describes it separately.
      </p>

      <h3 id="overview-areas">Find a DeFi example</h3>
      <p>
        Eight areas are covered. Each has a lesson, and the <a href="#trust">support catalog</a> lists what
        the tooling does with every shipped family file. All eight are <Status>SpecifiedOnly</Status>:
        the source is checked for structure and names, and execution is refused as Unsupported.
      </p>
      <ul className="guide-areas">
        {AREAS.map((id) => (
          <li key={id}>
            <a href={`#${id}`}>
              <strong>{META[id]?.nav ?? id}</strong> <Status>SpecifiedOnly</Status>
              <span>{AREA_SCOPE[id]}</span>
            </a>
            {id === 'lending' ? (
              <a className="guide-area-local" href="#repayment">
                Repayment lesson, a separate operation: <Status>LocalS0</Status>
              </a>
            ) : null}
          </li>
        ))}
      </ul>

      <div className="guide-callout">
        <h3 id="overview-names">Names and identities</h3>
        <p>
          <code>Buyer</code> is a source name; its <code>id</code> <code>Owner</code> is the economic identity
          that scenarios and results use. The symbol <code>USD</code> is display only and grants no
          conversion to any other asset.
        </p>
      </div>

      <h3 id="overview-legend">Support legend</h3>
      <p>
        Each example carries the label that says what the tooling does with it today. Labels qualify the
        example next to them; they are not a score of the project.
      </p>
      <dl className="guide-legend">
        <dt><Status>LocalS0</Status></dt>
        <dd>Runs locally against a stipulated scenario. The scenario is a stipulation, not a fact about any ledger.</dd>
        <dt><Status>PreparedUnqualified</Status></dt>
        <dd>A candidate result exists. It is not authenticated, proved, signed or settled.</dd>
        <dt><Status>SignedPreparedUnqualified</Status></dt>
        <dd>The owner's signature over the exact source bytes was verified by the separate native verifier. Key authority, proof and ledger acceptance are still not established.</dd>
        <dt><Status>SpecifiedOnly</Status></dt>
        <dd>The source is checked for structure and names; execution is refused as <Status>Unsupported</Status>.</dd>
        <dt><Status>Open</Status></dt>
        <dd>A capability or relation the example names but nothing yet establishes.</dd>
      </dl>

      <h3 id="overview-split">Authorization, acceptance and coordination</h3>
      <p>
        Three different questions carry three different answers. A label on one never answers another.
      </p>
      <dl className="guide-legend">
        <dt>Owner authorization</dt>
        <dd>A local native signature check exists (<Status>SignedPreparedUnqualified</Status>). Authority of the key and revocation are <Status>Open</Status>.</dd>
        <dt>Objective acceptance</dt>
        <dd>The proof and the ledger's own judgment. Neither is produced by the beta tooling: <Status>Open</Status>.</dd>
        <dt>Optional coordination</dt>
        <dd>The federation can coordinate evidence, routing and recovery. It never accepts a transaction and no program needs it.</dd>
      </dl>
      <h3 id="overview-source">Release and sources</h3>
      <p>
        Repository: <a href={REPOSITORY}>CharlesHoskinson/Moriarty</a>. Package <code>moriarty-beta</code>{' '}
        version {PACKAGE_VERSION}, language profile <code>{PROFILE}</code>. It is built from a checkout; no npm
        registry publication is claimed. Exact wording lives in the package's{' '}
        <a href={`${REPOSITORY}/blob/main/packages/moriarty-beta/GETTING-STARTED.md`}>GETTING-STARTED.md</a>,{' '}
        <a href={`${REPOSITORY}/blob/main/packages/moriarty-beta/SIGNED-INTENT.md`}>SIGNED-INTENT.md</a> and the{' '}
        <a href={`${REPOSITORY}/blob/main/docs/MORIARTY-PRODUCT-CONTRACT.md`}>product contract</a>. The older
        successor-syntax profile is a separate <a href="docs/language.html">language reference</a>; do not paste
        its samples into a {PROFILE} project.
      </p>
    </section>
  );
}

function Architecture() {
  return (
    <section className="guide-section" id="architecture" aria-labelledby="architecture-h">
      <h2 id="architecture-h">How an intention becomes a candidate</h2>
      <p>
        The path from source to result has a fixed order, and each step answers one question. Author and a
        local Propose step are covered by the beta tooling; Authorize is an optional, separate
        external-verifier walkthrough; Prove and Settle are the target.
      </p>
      <figure className="guide-figure">
        <img
          src="tutorial-assets/intention-pipeline.png"
          width={1536}
          height={1024}
          alt="Diagram titled From intent to effects: five numbered cards joined by arrows, 1 Author, 2 Authorize, 3 Propose, 4 Prove, 5 Settle. A bracket labelled Target pipeline spans Prove and Settle. A box labelled Reject or revise has an arrow up to Propose and a dashed line to an X mark. The plus and minus figures in the ledger on the last card are illustrative, not results. The numbered list below gives the same steps in text."
          decoding="async"
          loading="lazy"
        />
        <figcaption>
          Conceptual flow from the list below. Prove and Settle are the target pipeline. A proposal that is
          rejected returns to Propose to be revised; it is not carried forward. Ledger figures in the
          picture are illustrative and are not results. The optional kernel is not part of this diagram.
        </figcaption>
      </figure>
      <ol className="guide-flow">
        <li><strong>Author.</strong> A <code>.mori</code> program declares domains, accounts, assets, intents and actions, and <code>mori check</code> reads names, units and shapes. A check can pass while a particular scenario is later rejected. Local. <Status>LocalS0</Status></li>
        <li>
          <strong>Authorize.</strong> The owner's signature over the exact source bytes is verified by an
          external native signature verifier, not by the local authoring tools; see the{' '}
          <a href={`${REPOSITORY}/blob/main/packages/moriarty-beta/SIGNED-INTENT.md`}>signed intent walkthrough</a>.
          Verifying a signature does not establish that the key has authority over the account.{' '}
          <Status>SignedPreparedUnqualified</Status> when that check is run; key authority is <Status>Open</Status>.
          See <a href="#overview-split">authorization, acceptance and coordination</a>.
        </li>
        <li><strong>Propose.</strong> <code>mori simulate</code> applies a stipulated scenario locally and returns ordered effects and a candidate post-state, or a rejection to revise. <Status>PreparedUnqualified</Status></li>
        <li><strong>Prove.</strong> Target. Native proof production on Midnight is not produced by any beta tooling. <Status>Open</Status></li>
        <li><strong>Settle.</strong> Target. Ledger acceptance is not established by any example here. <Status>Open</Status></li>
      </ol>
      <p>
        Today's tooling covers Author and, locally, Propose. Authorize is a separate, optional signature
        check whose key authority is still open. Prove and Settle are shown so you can see where a result
        stops. A rejected proposal is revised and proposed again.
      </p>
      <div className="guide-callout">
        <h3 id="architecture-kernel">Where the kernel fits</h3>
        <p>
          The optional federation coordinates evidence and routing around this path. It does not provide the
          ledger acceptance that Settle requires, and the owner's authorization and the ledger's own
          judgment stay separate from it. Authoring, checking and preparation never call it, and no program
          needs its approval. Read the <a href="kernel.html">kernel explanation</a> for how it is meant to work.
        </p>
      </div>
    </section>
  );
}

/** Group, label and heading for each module wrapper; the wrapper's first sections are read from the DOM. */
function Module({ group, children }: { group: string; children: ReactNode }) {
  return (
    <div className="guide-module" data-group={group}>
      {children}
    </div>
  );
}

function PrevNext({ id, titles }: { id: string; titles: Map<string, string> }) {
  const i = PATH.indexOf(id as (typeof PATH)[number]);
  const prev = i > 0 ? PATH[i - 1] : undefined;
  const next = i >= 0 && i < PATH.length - 1 ? PATH[i + 1] : undefined;
  return (
    <nav className="guide-prevnext" aria-label="Lesson order">
      {prev ? (
        <a rel="prev" href={`#${prev}`}>
          <span>Previous</span>
          {titles.get(prev) ?? prev}
        </a>
      ) : (
        <span />
      )}
      {next ? (
        <a rel="next" href={`#${next}`}>
          <span>Next</span>
          {titles.get(next) ?? next}
        </a>
      ) : null}
    </nav>
  );
}

function slug(text: string): string {
  return text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'section';
}

function sectionEls(root: HTMLElement | null): HTMLElement[] {
  return Array.from(root?.querySelectorAll<HTMLElement>('section.guide-section[id]') ?? []);
}

/** Give every h3 an id derived from its section and text, before any hash is resolved. */
function assignHeadingIds(root: HTMLElement | null) {
  for (const section of sectionEls(root)) {
    for (const h of Array.from(section.querySelectorAll<HTMLElement>('h3'))) {
      if (h.id) continue;
      const base = `${section.id}-${slug(h.textContent ?? '')}`;
      let id = base;
      for (let n = 2; document.getElementById(id); n += 1) id = `${base}-${n}`;
      h.id = id;
    }
  }
}

const COPY_LABEL = /^\s*copy/i;

function hasCopyControl(scope: Element): boolean {
  return Array.from(scope.querySelectorAll<HTMLElement>('button')).some((b) =>
    COPY_LABEL.test(`${b.getAttribute('aria-label') ?? ''} ${b.textContent ?? ''}`.trim()),
  );
}

/** Scope names that mean the text is not a program or command a reader should paste. */
const NO_COPY_SCOPE = /^(horizon|proposed|derived|illustrative|output)$/i;

/**
 * Whether the shell may add a copy control to this block. Authored controls win: a module that marks its
 * block (`data-copy-ready`, `data-copy-owned`), builds its own toolbar or status, or sets `data-copy="none"`
 * is never given a second one, and proposed or derived text is never offered as something to paste.
 */
function shellMayCopy(pre: HTMLElement): boolean {
  const frame = pre.closest<HTMLElement>('figure') ?? pre;
  for (const el of [pre, frame, pre.parentElement, frame.closest<HTMLElement>('[data-copy],[data-scope],[data-kind]')]) {
    if (!el) continue;
    const d = el.dataset;
    if (d.copy === 'none' || d.copyReady || d.copyOwned) return false;
    if (NO_COPY_SCOPE.test(d.scope ?? '') || NO_COPY_SCOPE.test(d.kind ?? '')) return false;
  }
  if (/^\s*(derived|illustrative|proposed)\b/i.test(pre.getAttribute('aria-label') ?? '')) return false;
  if (frame.querySelector('.guide-code-toolbar, .guide-copybar, .guide-copy, .guide-copy-status, [role="status"]')) return false;
  if (hasCopyControl(frame)) return false;
  const before = pre.previousElementSibling;
  return !(before?.classList.contains('guide-copybar') || before?.classList.contains('guide-code-toolbar'));
}

/** Name the copied text by what the block says it is; when nothing says, claim no more than "the text shown". */
function copyScope(pre: HTMLElement): { label: string; done: string } {
  const scope = pre.closest<HTMLElement>('[data-scope]')?.dataset.scope ?? '';
  const summary = pre.closest('details')?.querySelector('summary')?.textContent ?? '';
  if (scope === 'complete' || /\b(complete|full)\b/i.test(summary)) return { label: 'Copy complete file', done: 'Copied the complete file.' };
  if (scope === 'excerpt' || /\bexcerpt\b/i.test(summary)) return { label: 'Copy excerpt', done: 'Copied the excerpt.' };
  if (scope === 'command') return { label: 'Copy command', done: 'Copied the command.' };
  return { label: 'Copy shown text', done: 'Copied the text shown.' };
}

/** Add one copy control per code block that has none of its own. Idempotent. */
function addCopyBars(root: HTMLElement | null) {
  for (const pre of Array.from(root?.querySelectorAll<HTMLElement>('pre') ?? [])) {
    if (pre.dataset.copyReady) continue;
    if (!shellMayCopy(pre)) {
      pre.dataset.copyReady = 'own';
      continue;
    }
    pre.dataset.copyReady = 'added';
    const { label, done } = copyScope(pre);
    const bar = document.createElement('div');
    bar.className = 'guide-copybar';
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'guide-copy';
    btn.textContent = label;
    const state = document.createElement('span');
    state.setAttribute('role', 'status');
    state.className = 'guide-copy-state';
    btn.addEventListener('click', () => {
      void copyText(pre.textContent ?? '').then((ok) => {
        state.textContent = ok ? done : COPY_FAILED;
        state.className = `guide-copy-state ${ok ? 'guide-copy-ok' : 'guide-copy-fail'}`;
      });
    });
    bar.append(btn, state);
    pre.before(bar);
  }
}

function regionName(el: Element, fallback: string): string {
  const fig = el.closest('figure');
  const caption = fig?.querySelector('figcaption strong')?.textContent ?? fig?.querySelector('figcaption')?.textContent;
  return (caption ?? fallback).replace(/\s+/g, ' ').trim().slice(0, 80) || fallback;
}

/** A code frame that overflows must be reachable by keyboard; remove the stop again when it fits. */
function markCode(root: HTMLElement | null) {
  for (const pre of Array.from(root?.querySelectorAll<HTMLElement>('pre') ?? [])) {
    const overflows = pre.scrollWidth > pre.clientWidth + 1 || pre.scrollHeight > pre.clientHeight + 1;
    if (overflows && !pre.hasAttribute('tabindex') && !pre.closest('figure[tabindex]')) {
      pre.tabIndex = 0;
      pre.dataset.autoFocus = 'true';
      if (!pre.hasAttribute('aria-label') && !pre.hasAttribute('aria-labelledby')) {
        pre.setAttribute('role', 'region');
        pre.setAttribute('aria-label', `${regionName(pre, 'Code')}, scrollable`);
      }
    } else if (!overflows && pre.dataset.autoFocus) {
      pre.removeAttribute('tabindex');
      delete pre.dataset.autoFocus;
    }
  }
}

/**
 * Tables. A table inside a named wrapper (`.guide-table-wrap` or a labelled region) leaves the scrolling to that
 * wrapper, so there is one scroll surface. A bare table scrolls itself and is named and focusable only when it overflows.
 */
function markTables(root: HTMLElement | null) {
  for (const t of Array.from(root?.querySelectorAll<HTMLTableElement>('table') ?? [])) {
    const wrap = t.closest<HTMLElement>('.guide-table-wrap, [role="region"]');
    const caption = t.querySelector('caption')?.textContent?.trim();
    let prev: Element | null = t.previousElementSibling;
    while (prev && !/^H[1-6]$/.test(prev.tagName)) prev = prev.previousElementSibling;
    const name = caption || prev?.textContent?.trim() || t.closest('section')?.querySelector('h2')?.textContent?.trim();
    if (wrap) {
      t.removeAttribute('tabindex');
      if (wrap.scrollWidth > wrap.clientWidth + 1) {
        if (!wrap.hasAttribute('tabindex')) wrap.tabIndex = 0;
        if (!wrap.hasAttribute('role')) wrap.setAttribute('role', 'region');
        if (!wrap.hasAttribute('aria-label') && !wrap.hasAttribute('aria-labelledby') && name) wrap.setAttribute('aria-label', `${name}, table`);
      }
      continue;
    }
    if (!t.hasAttribute('role')) t.setAttribute('role', 'table');
    if (!t.hasAttribute('aria-label') && !t.hasAttribute('aria-labelledby') && name) t.setAttribute('aria-label', `${name}, table`);
    if (t.scrollWidth > t.clientWidth + 1) t.tabIndex = 0;
    else t.removeAttribute('tabindex');
  }
}

/** Walk a section in document order, splitting at h3 headings, to build searchable chunks. */
function chunksOf(section: HTMLElement, lesson: string): Hit[] {
  const h2 = section.querySelector('h2')?.textContent?.trim() ?? lesson;
  const chunks: { target: string; heading: string; text: string }[] = [];
  let cur = { target: section.id, heading: h2, text: '' };
  const walker = document.createTreeWalker(section, NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT);
  for (let n = walker.nextNode(); n; n = walker.nextNode()) {
    if (n.nodeType === Node.ELEMENT_NODE) {
      const e = n as HTMLElement;
      if (e.tagName === 'H3' && e.id) {
        chunks.push(cur);
        cur = { target: e.id, heading: e.textContent?.trim() ?? e.id, text: '' };
      }
    } else if (!n.parentElement?.closest('button,.guide-copybar,.guide-prevnext-host')) {
      cur.text += `${n.textContent ?? ''} `;
    }
  }
  chunks.push(cur);
  return chunks.map((c) => ({ target: c.target, lesson, heading: c.heading, excerpt: c.text.replace(/\s+/g, ' ').trim() }));
}

function search(root: HTMLElement | null, lessons: Lesson[], query: string): Hit[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
  if (terms.length === 0) return [];
  const labels = new Map(lessons.map((l) => [l.id, l.title]));
  const scored: { hit: Hit; rank: number }[] = [];
  for (const section of sectionEls(root)) {
    const lesson = labels.get(section.id) ?? section.id;
    for (const c of chunksOf(section, lesson)) {
      const text = c.excerpt.toLowerCase();
      const head = `${c.heading} ${lesson}`.toLowerCase();
      if (!terms.every((t) => head.includes(t) || text.includes(t))) continue;
      const first = terms.map((t) => text.indexOf(t)).filter((i) => i >= 0).sort((a, b) => a - b)[0] ?? 0;
      const from = Math.max(0, first - 60);
      const excerpt = `${from > 0 ? '… ' : ''}${c.excerpt.slice(from, from + 170)}${from + 170 < c.excerpt.length ? ' …' : ''}`;
      scored.push({ hit: { ...c, excerpt }, rank: terms.every((t) => head.includes(t)) ? 0 : 1 });
    }
  }
  return scored.sort((a, b) => a.rank - b.rank).slice(0, 30).map((s) => s.hit);
}

function hashTarget(hash: string): HTMLElement | null {
  try {
    return hash.length > 1 ? document.getElementById(decodeURIComponent(hash.slice(1))) : null;
  } catch {
    return null;
  }
}

/** The lesson and, for an h3, the contents entry that a destination belongs to. */
function placeOf(target: HTMLElement): { lesson: string; heading: string } | null {
  const lesson = target.closest<HTMLElement>('section.guide-section[id]')?.id;
  return lesson ? { lesson, heading: target.tagName === 'H3' ? target.id : '' } : null;
}

/** Reader input that takes over from a programmed hash hold. */
const INPUTS = ['wheel', 'touchstart', 'pointerdown', 'mousedown', 'keydown'] as const;

interface Pin {
  id: string;
  lesson: string;
  heading: string;
  until: number;
}

export function GuideApp() {
  const contentRef = useRef<HTMLDivElement>(null);
  const headerRef = useRef<HTMLElement>(null);
  const barRef = useRef<HTMLDivElement>(null);
  const browseRef = useRef<HTMLButtonElement>(null);
  const searchRef = useRef<HTMLInputElement>(null);
  const offsetRef = useRef(96);
  const pinRef = useRef<Pin | null>(null);
  const [lessons, setLessons] = useState<Lesson[]>([]);
  const [hosts, setHosts] = useState<Record<string, HTMLElement>>({});
  const [headings, setHeadings] = useState<Heading[]>([]);
  const [active, setActive] = useState('overview');
  const [activeHeading, setActiveHeading] = useState('');
  const [query, setQuery] = useState('');
  const [menuOpen, setMenuOpen] = useState(false);
  const [wide, setWide] = useState(() => window.matchMedia('(min-width: 1280px)').matches);

  useEffect(() => {
    const mq = window.matchMedia('(min-width: 1280px)');
    const on = () => setWide(mq.matches);
    mq.addEventListener('change', on);
    return () => mq.removeEventListener('change', on);
  }, []);

  // The drawer only exists below the wide layout.
  useEffect(() => {
    if (wide) setMenuOpen(false);
  }, [wide]);

  // Sticky heights are measured, not assumed, so anchors clear the masthead and the mobile bar at any wrap.
  // --guide-offset is what every heading, section and sticky rail clears (the stylesheet applies it with
  // !important so an author's inline scroll-margin cannot undercut it).
  const measure = useCallback(() => {
    const m = headerRef.current?.getBoundingClientRect().height ?? 0;
    const bar = barRef.current;
    const b = bar && getComputedStyle(bar).display !== 'none' ? bar.getBoundingClientRect().height : 0;
    const root = document.documentElement.style;
    root.setProperty('--guide-masthead-h', `${Math.ceil(m)}px`);
    root.setProperty('--guide-chrome-h', `${Math.ceil(m + b)}px`);
    root.setProperty('--guide-offset', `${Math.ceil(m + b) + 12}px`);
    offsetRef.current = Math.ceil(m + b) + 12;
  }, []);

  /** Record a destination as the current place now, ahead of the scroll that will reach it. */
  const arrive = useCallback((target: HTMLElement) => {
    const place = placeOf(target);
    if (!place) return;
    pinRef.current = { id: target.id, ...place, until: performance.now() + 1500 };
    setActive(place.lesson);
    setActiveHeading(place.heading);
  }, []);

  /**
   * Hold a hash destination at the sticky boundary while the document is still laying out. The first scroll
   * happens before modules render, fonts and images settle and the lessons above grow, so it lands short.
   * Each frame re-aligns until the page is loaded and its height has been steady for a run of frames, or until
   * the reader wheels, touches, presses a key or points at the page, which ends the hold at once (a frame
   * budget bounds it regardless). No timers: it follows the layout, not a guess at how long layout takes.
   */
  const holdRef = useRef<(() => void) | null>(null);
  /** Stop the frame loop only. Used when a hold is replaced or unmounted; the pin belongs to the new arrival. */
  const release = useCallback(() => {
    holdRef.current?.();
    holdRef.current = null;
  }, []);
  /** The reader has taken over: end any hold and forget the transient pin, even one whose hold has finished. */
  const takeover = useCallback(() => {
    release();
    pinRef.current = null;
  }, [release]);
  useEffect(() => {
    for (const type of INPUTS) window.addEventListener(type, takeover, { capture: true, passive: true });
    return () => {
      for (const type of INPUTS) window.removeEventListener(type, takeover, true);
    };
  }, [takeover]);
  const hold = useCallback(
    (target: HTMLElement) => {
      release();
      let frame = 0;
      let steady = 0;
      let frames = 0;
      let height = -1;
      const stop = () => {
        window.cancelAnimationFrame(frame);
        if (holdRef.current === stop) holdRef.current = null;
      };
      const step = () => {
        const drift = target.getBoundingClientRect().top - offsetRef.current;
        if (Math.abs(drift) > 1) window.scrollTo({ top: window.scrollY + drift, behavior: 'instant' });
        const pin = pinRef.current;
        if (pin && pin.id === target.id) pin.until = performance.now() + 1500;
        const h = document.documentElement.scrollHeight;
        steady = h === height ? steady + 1 : 0;
        height = h;
        const loaded =
          document.readyState === 'complete' &&
          (!document.fonts || document.fonts.status === 'loaded') &&
          Array.from(contentRef.current?.querySelectorAll('img') ?? []).every((i) => i.complete);
        if ((loaded && steady >= 30) || ++frames > 900) stop();
        else frame = window.requestAnimationFrame(step);
      };
      holdRef.current = stop;
      frame = window.requestAnimationFrame(step);
    },
    [release],
  );
  useEffect(() => release, [release]);

  // Mount: sizes, stable ids, lessons, then the initial hash (ids exist before it is resolved).
  useLayoutEffect(() => {
    measure();
    const root = contentRef.current;
    assignHeadingIds(root);
    addCopyBars(root);
    markCode(root);
    markTables(root);
    const found: Lesson[] = sectionEls(root).map((el) => {
      const meta = META[el.id];
      return {
        id: el.id,
        title: el.dataset.nav || meta?.nav || el.querySelector('h2')?.textContent?.trim() || el.id,
        group: el.dataset.group || meta?.group || el.closest<HTMLElement>('[data-group]')?.dataset.group || GROUPS[0],
      };
    });
    setLessons(found);
    const made: Record<string, HTMLElement> = {};
    for (const id of PATH.slice(0, -1)) {
      const section = document.getElementById(id);
      if (!section) continue;
      let host = section.querySelector<HTMLElement>(':scope > .guide-prevnext-host');
      if (!host) {
        host = document.createElement('div');
        host.className = 'guide-prevnext-host';
        section.append(host);
      }
      made[id] = host;
    }
    setHosts(made);
    const target = hashTarget(location.hash);
    if (target) {
      arrive(target);
      target.scrollIntoView();
      hold(target);
    }
  }, [measure, arrive, hold]);

  useEffect(() => {
    const ro = typeof ResizeObserver === 'undefined' ? null : new ResizeObserver(measure);
    if (headerRef.current) ro?.observe(headerRef.current);
    if (barRef.current) ro?.observe(barRef.current);
    window.addEventListener('resize', measure);
    return () => {
      ro?.disconnect();
      window.removeEventListener('resize', measure);
    };
  }, [measure]);

  // Modules re-render on their own; keep ids, copy bars and table and code semantics in step.
  useEffect(() => {
    const root = contentRef.current;
    if (!root) return;
    let frame = 0;
    const refresh = () => {
      window.cancelAnimationFrame(frame);
      frame = window.requestAnimationFrame(() => {
        assignHeadingIds(root);
        addCopyBars(root);
        markCode(root);
        markTables(root);
      });
    };
    const mo = new MutationObserver(refresh);
    mo.observe(root, { childList: true, subtree: true });
    window.addEventListener('resize', refresh);
    return () => {
      mo.disconnect();
      window.removeEventListener('resize', refresh);
      window.cancelAnimationFrame(frame);
    };
  }, []);

  // Current place: the lesson whose heading is the nearest one above the sticky boundary, and within it the
  // nearest h3 above it. A destination just navigated to wins until the scroll reaches it, so a lesson too
  // short to scroll to the boundary still reads as current.
  useEffect(() => {
    if (lessons.length === 0) return;
    let frame = 0;
    const update = () => {
      const line = offsetRef.current + 16;
      const pin = pinRef.current;
      if (pin) {
        const top = document.getElementById(pin.id)?.getBoundingClientRect().top ?? 0;
        const atEnd = window.innerHeight + window.scrollY >= document.documentElement.scrollHeight - 2;
        if (atEnd || Math.abs(top - offsetRef.current) <= 24) {
          setActive(pin.lesson);
          setActiveHeading(pin.heading);
          return;
        }
        if (performance.now() < pin.until) return;
        pinRef.current = null;
      }
      const sections = sectionEls(contentRef.current);
      let cur: HTMLElement | undefined;
      for (const el of sections) {
        if (el.getBoundingClientRect().top <= line) cur = el;
        else break;
      }
      cur ??= sections[0];
      let heading = '';
      for (const h of Array.from(cur?.querySelectorAll<HTMLElement>('h3[id]') ?? [])) {
        if (h.getBoundingClientRect().top <= line) heading = h.id;
        else break;
      }
      setActive(cur?.id ?? lessons[0]?.id ?? 'overview');
      setActiveHeading(heading);
    };
    const queue = () => {
      window.cancelAnimationFrame(frame);
      frame = window.requestAnimationFrame(update);
    };
    const onHash = () => {
      const target = hashTarget(location.hash);
      if (target) {
        arrive(target);
        hold(target);
      }
      queue();
    };
    update();
    window.addEventListener('scroll', queue, { passive: true });
    window.addEventListener('resize', queue);
    window.addEventListener('hashchange', onHash);
    return () => {
      window.removeEventListener('scroll', queue);
      window.removeEventListener('resize', queue);
      window.removeEventListener('hashchange', onHash);
      window.cancelAnimationFrame(frame);
    };
  }, [lessons, arrive, hold]);

  // On this page: the h3 headings of the current lesson.
  useEffect(() => {
    const el = sectionEls(contentRef.current).find((s) => s.id === active);
    setHeadings(
      Array.from(el?.querySelectorAll<HTMLElement>('h3[id]') ?? []).map((h) => ({ id: h.id, title: h.textContent?.trim() ?? h.id })),
    );
  }, [active, lessons]);

  const hits = useMemo(() => search(contentRef.current, lessons, query), [lessons, query]);
  const searching = query.trim().length > 0;

  const closeMenu = useCallback(() => {
    setMenuOpen(false);
    browseRef.current?.focus();
  }, []);

  useEffect(() => {
    if (!menuOpen) return;
    searchRef.current?.focus();
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') closeMenu();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [menuOpen, closeMenu]);

  /**
   * Follow an in-page link: focus the destination (its heading, for a lesson), scroll to it, record it as the
   * current place, then close the drawer. Focus is already on the destination when the drawer disappears.
   */
  const follow = (e: MouseEvent<HTMLAnchorElement>) => {
    const href = e.currentTarget.getAttribute('href') ?? '';
    const target = href.startsWith('#') ? hashTarget(href) : null;
    if (!target || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    e.preventDefault();
    const landing = target.matches('section') ? target.querySelector<HTMLElement>('h2') ?? target : target;
    if (!landing.hasAttribute('tabindex')) landing.setAttribute('tabindex', '-1');
    landing.focus({ preventScroll: true });
    history.pushState(null, '', href);
    arrive(target);
    target.scrollIntoView();
    setMenuOpen(false);
  };

  const skipToNavigation = (e: MouseEvent<HTMLAnchorElement>) => {
    if (wide) return;
    e.preventDefault();
    setMenuOpen(true);
  };

  const titles = useMemo(() => new Map(lessons.map((l) => [l.id, l.title])), [lessons]);
  const currentTitle = titles.get(active) ?? 'Overview';

  const contents = (
    <ul>
      {headings.map((h) => (
        <li key={h.id}>
          <a href={`#${h.id}`} aria-current={h.id === activeHeading ? 'location' : undefined} onClick={follow}>
            {h.title}
          </a>
        </li>
      ))}
    </ul>
  );

  return (
    <>
      <a className="skip" href="#guide-main">
        Skip to tutorial content
      </a>
      <a className="skip" href="#guide-lessons" onClick={skipToNavigation}>
        Skip to tutorial navigation
      </a>
      <header className="masthead" ref={headerRef}>
        <a className="wordmark" href="index.html">Moriarty</a>
        <span className="tagline">bounded financial contracts</span>
        <nav className="jump guide-jump" aria-label="Site">
          <a href="index.html">Home</a>
          <a href="tutorial.html" aria-current="page">Tutorials</a>
          <a href="kernel.html">Kernel<span className="guide-long"> explanation</span></a>
          <a href="docs/language.html"><span className="guide-long">Language reference</span><span className="guide-short">Reference</span></a>
        </nav>
      </header>

      <div className="guide-bar" ref={barRef}>
        <button
          type="button"
          ref={browseRef}
          className="guide-browse"
          aria-expanded={menuOpen}
          aria-controls="guide-lessons"
          onClick={() => setMenuOpen((o) => !o)}
        >
          Browse<span className="guide-long"> and search lessons</span>
        </button>
        <span className="guide-current">{currentTitle}</span>
      </div>

      <div className="guide-layout">
        <aside id="guide-lessons" tabIndex={-1} aria-label="Lesson navigation" className={`guide-side${menuOpen ? ' is-open' : ''}`}>
          <div className="guide-search">
            <label htmlFor="guide-search-input">Search lessons</label>
            <div className="guide-searchrow">
              <input
                id="guide-search-input"
                ref={searchRef}
                type="search"
                value={query}
                placeholder="atoms, nonce, LocalS0…"
                onChange={(e) => setQuery(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Escape' && query) {
                    e.stopPropagation();
                    setQuery('');
                  }
                }}
              />
              {searching ? (
                <button
                  type="button"
                  className="guide-clear"
                  onClick={() => {
                    setQuery('');
                    searchRef.current?.focus();
                  }}
                >
                  Clear search
                </button>
              ) : null}
            </div>
          </div>
          <p className="guide-count" role="status">
            {!searching
              ? ''
              : hits.length === 0
                ? `No heading or text matches “${query.trim()}”.`
                : `${hits.length} ${hits.length === 1 ? 'result' : 'results'}. The lessons are unchanged.`}
          </p>
          {searching ? (
            hits.length > 0 ? (
              <nav aria-label="Search results" className="guide-results">
                <ul>
                  {hits.map((h, i) => (
                    <li key={`${h.target}-${i}`}>
                      <a href={`#${h.target}`} onClick={follow}>
                        <span className="guide-hit-lesson">{h.lesson}</span>
                        <span className="guide-hit-heading">{h.heading}</span>
                        <span className="guide-hit-excerpt">{h.excerpt}</span>
                      </a>
                    </li>
                  ))}
                </ul>
              </nav>
            ) : null
          ) : (
            <>
              {headings.length > 0 ? (
                <nav aria-label="On this page" className="guide-toc-drawer">
                  <p className="guide-navhead">On this page</p>
                  {contents}
                </nav>
              ) : null}
              <nav aria-label="Lessons">
                {GROUPS.map((g) => {
                  const items = lessons.filter((l) => l.group === g);
                  if (items.length === 0) return null;
                  return (
                    <div key={g} className="guide-navgroup">
                      <p className="guide-navhead" id={`guide-group-${slug(g)}`}>{g}</p>
                      <ul aria-labelledby={`guide-group-${slug(g)}`}>
                        {items.map((l) => (
                          <li key={l.id}>
                            <a href={`#${l.id}`} aria-current={l.id === active ? 'location' : undefined} onClick={follow}>
                              {l.title}
                            </a>
                          </li>
                        ))}
                      </ul>
                    </div>
                  );
                })}
              </nav>
            </>
          )}
          <p className="guide-note">
            Reading position is a navigation aid. It is not a record of what you have run or of any
            financial qualification.
          </p>
        </aside>

        <main id="guide-main" className="guide-main" tabIndex={-1}>
          <header className="guide-head">
            <p className="guide-eyebrow">Tutorials · beta language {PROFILE}</p>
            <h1>Write a bounded financial agreement</h1>
            <p className="guide-lead">
              State the asset, domain, accounts, caps and rounds in one program, check it, and prepare a
              local candidate. The first lessons do not sign, prove, send or deploy anything.
            </p>
            <p className="guide-scope">
              <Status>LocalS0</Status> <Status>PreparedUnqualified</Status> Everything runnable here is a local
              check or preparation from a stipulated scenario, not an authorized or accepted transaction.
              Proof and ledger acceptance are <Status>Open</Status>.
            </p>
            <div className="guide-actions">
              <a className="guide-primary" href="#getting-started">Install and run the first check</a>
              <a className="guide-secondary" href="#overview-areas">Find a DeFi example</a>
              <a className="guide-secondary" href="#architecture">How it fits together</a>
            </div>
          </header>
          <div ref={contentRef}>
            <Module group="Start building">
              <Overview />
              <GettingStarted />
            </Module>
            <Module group="Language">
              <LanguagePatterns />
            </Module>
            <Module group="DeFi examples">
              <Markets />
              <RiskGovernance />
              <CrossChain />
            </Module>
            <Module group="Design and limits">
              <Architecture />
            </Module>
          </div>
          {Object.entries(hosts).map(([id, host]) => createPortal(<PrevNext id={id} titles={titles} />, host, id))}
        </main>

        <nav className="guide-toc" aria-label="On this page">
          <p className="guide-navhead">On this page</p>
          {contents}
        </nav>
      </div>
    </>
  );
}

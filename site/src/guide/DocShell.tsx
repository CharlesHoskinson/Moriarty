import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, type MouseEvent, type ReactNode } from 'react';
import { createPortal, flushSync } from 'react-dom';
import { PAGES, type PageKey } from './data';

/**
 * Shell shared by the five documentation pages: masthead with the page set, the page's section list and
 * in-section headings, search of the current page, previous/next links along a page's reading order, and
 * hash arrival that holds a deep link at its heading while the page settles.
 *
 * A page passes its sections as children. Each is a `section.guide-section[id]` with an `h2` (use the
 * `Section` component); `data-nav` gives its sidebar label and `data-group` its sidebar group.
 */

export interface DocShellProps {
  page: PageKey;
  eyebrow: string;
  title: string;
  lead?: ReactNode;
  /** Sidebar groups, in order. A section's `data-group` must be one of these. */
  groups: string[];
  /** Section ids in reading order. Each gets previous/next links; leave empty for pages read out of order. */
  path?: string[];
  children: ReactNode;
}

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

function PrevNext({ id, path, titles }: { id: string; path: string[]; titles: Map<string, string> }) {
  const i = path.indexOf(id);
  const prev = i > 0 ? path[i - 1] : undefined;
  const next = i >= 0 && i < path.length - 1 ? path[i + 1] : undefined;
  if (!prev && !next) return null;
  return (
    <nav className="guide-prevnext" aria-label={`Previous and next: ${titles.get(id) ?? id}`}>
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

/** A code region's name: its kind and title from the caption bar (not the copy control), numbered when a page repeats it. */
const regionNames = new Map<string, number>();
function regionName(el: Element, fallback: string): string {
  const fig = el.closest('figure');
  const tag = fig?.querySelector('.doc-code-tag')?.textContent?.trim();
  const title = fig?.querySelector('.doc-code-title')?.textContent?.trim();
  const base = ([tag, title].filter(Boolean).join(': ') || fallback).replace(/\s+/g, ' ').slice(0, 80);
  const n = (regionNames.get(base) ?? 0) + 1;
  regionNames.set(base, n);
  return n > 1 ? `${base} (${n})` : base;
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
    const wrap = t.closest<HTMLElement>('.doc-table-wrap, [role="region"]');
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
    } else if (!n.parentElement?.closest('button,.doc-code-bar,.guide-prevnext-host,[role="status"]')) {
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


export function DocShell({ page, eyebrow, title, lead, groups: groupsProp, path: pathProp, children }: DocShellProps) {
  // Pages pass literal arrays; key them by content so a re-render does not re-run the mount effect (and loop).
  const groupsKey = groupsProp.join('\n');
  const pathKey = (pathProp ?? []).join('\n');
  const groups = useMemo(() => groupsProp, [groupsKey]);
  const path = useMemo(() => pathProp ?? [], [pathKey]);
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
  const [active, setActive] = useState('');
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
    markCode(root);
    markTables(root);
    const found: Lesson[] = sectionEls(root).map((el) => ({
      id: el.id,
      title: el.dataset.nav || el.querySelector('h2')?.textContent?.trim() || el.id,
      group: el.dataset.group || el.closest<HTMLElement>('[data-group]')?.dataset.group || groups[0] || '',
    }));
    setLessons(found);
    const made: Record<string, HTMLElement> = {};
    for (const id of path) {
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
      // A reader who arrives on a link starts reading at its heading, so focus starts there too.
      const landing = target.matches('section') ? target.querySelector<HTMLElement>('h2') ?? target : target;
      if (!landing.hasAttribute('tabindex')) landing.setAttribute('tabindex', '-1');
      landing.focus({ preventScroll: true });
      arrive(target);
      target.scrollIntoView();
      hold(target);
    }
  }, [measure, arrive, hold, groups, path]);

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
      setActive(cur?.id ?? lessons[0]?.id ?? '');
      setActiveHeading(heading);
    };
    const queue = () => {
      window.cancelAnimationFrame(frame);
      frame = window.requestAnimationFrame(update);
    };
    const onHash = () => {
      const target = hashTarget(location.hash);
      if (target) {
        // Move focus with the reader (the heading, for a section), as the sidebar links do.
        const landing = target.matches('section') ? target.querySelector<HTMLElement>('h2') ?? target : target;
        if (!landing.hasAttribute('tabindex')) landing.setAttribute('tabindex', '-1');
        landing.focus({ preventScroll: true });
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

  // Keep the current entry visible in a sidebar taller than the window. The rail scrolls itself;
  // scrollIntoView would move the page as well.
  useEffect(() => {
    const rail = document.querySelector<HTMLElement>('.guide-side');
    const item = rail?.querySelector<HTMLElement>('a[aria-current="location"]');
    if (!rail || !item || rail.scrollHeight <= rail.clientHeight) return;
    const r = item.getBoundingClientRect();
    const b = rail.getBoundingClientRect();
    if (r.bottom > b.bottom - 24 || r.top < b.top + 16) rail.scrollTop += r.top - b.top - rail.clientHeight / 2;
  }, [active]);

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
    document.body.classList.add('guide-locked');
    searchRef.current?.focus();
    searchRef.current?.scrollIntoView({ block: 'nearest' });
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') closeMenu();
    };
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('keydown', onKey);
      document.body.classList.remove('guide-locked');
    };
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
    // Close the drawer first: while it is open the main column is inert and cannot take focus.
    flushSync(() => setMenuOpen(false));
    const landing = target.matches('section') ? target.querySelector<HTMLElement>('h2') ?? target : target;
    if (!landing.hasAttribute('tabindex')) landing.setAttribute('tabindex', '-1');
    landing.focus({ preventScroll: true });
    history.pushState(null, '', href);
    arrive(target);
    target.scrollIntoView();
  };

  const skipToNavigation = (e: MouseEvent<HTMLAnchorElement>) => {
    if (wide) return;
    e.preventDefault();
    setMenuOpen(true);
  };

  const titles = useMemo(() => new Map(lessons.map((l) => [l.id, l.title])), [lessons]);
  const currentTitle = titles.get(active) ?? title;

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
        Skip to content
      </a>
      <a className="skip" href="#guide-lessons" onClick={skipToNavigation}>
        Skip to page navigation
      </a>
      <header className="masthead" ref={headerRef}>
        <a className="wordmark" href="index.html">Moriarty</a>
        <nav className="jump guide-jump" aria-label="Documentation">
          {PAGES.map((p) => (
            <a key={p.key} href={p.href} aria-current={p.key === page ? 'page' : undefined}>
              <span className="guide-long">{p.label}</span>
              <span className="guide-short">{p.short}</span>
            </a>
          ))}
          <a href="kernel.html">Kernel</a>
        </nav>
      </header>

      <div className="guide-bar" ref={barRef} role="region" aria-label="Current section">
        <button
          type="button"
          ref={browseRef}
          className="guide-browse"
          aria-expanded={menuOpen}
          aria-controls="guide-lessons"
          onClick={() => setMenuOpen((o) => !o)}
        >
          Contents<span className="guide-long"> and search</span>
        </button>
        <span className="guide-current" aria-live="off">{currentTitle}</span>
      </div>

      <div className="guide-layout">
        <nav id="guide-lessons" tabIndex={-1} aria-label="Page contents" className={`guide-side${menuOpen ? ' is-open' : ''}`}>
          <div className="guide-search" role="search">
            <label htmlFor="guide-search-input">Search this page</label>
            <div className="guide-searchrow">
              <input
                id="guide-search-input"
                ref={searchRef}
                type="search"
                value={query}
                placeholder="atoms, nonce, scenario…"
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
                  Clear
                </button>
              ) : null}
            </div>
          </div>
          <p className="guide-count" role="status">
            {!searching
              ? ''
              : hits.length === 0
                ? `No heading or text on this page matches “${query.trim()}”.`
                : `${hits.length} ${hits.length === 1 ? 'result' : 'results'} on this page.`}
          </p>
          {searching ? (
            hits.length > 0 ? (
              <ul className="guide-results" aria-label="Search results">
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
            ) : null
          ) : (
            <>
              {headings.length > 0 ? (
                <div className="guide-toc-drawer">
                  <p className="guide-navhead">In this section</p>
                  {contents}
                </div>
              ) : null}
              {groups.map((g) => {
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
            </>
          )}
        </nav>

        <main id="guide-main" className="guide-main" tabIndex={-1} inert={menuOpen && !wide ? true : undefined}>
          <header className="guide-head">
            <p className="guide-eyebrow">{eyebrow}</p>
            <h1>{title}</h1>
            {lead ? <div className="guide-lead">{lead}</div> : null}
          </header>
          <div ref={contentRef}>{children}</div>
          {Object.entries(hosts).map(([id, host]) => createPortal(<PrevNext id={id} path={path} titles={titles} />, host, id))}
        </main>

        <nav className="guide-toc" aria-label="In this section">
          {headings.length > 0 ? <p className="guide-navhead">In this section</p> : null}
          {contents}
        </nav>
      </div>
    </>
  );
}


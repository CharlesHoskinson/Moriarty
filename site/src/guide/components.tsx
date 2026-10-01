import { useEffect, useRef, useState, type ReactNode } from 'react';
import { FAMILIES, MODE_LETTER, MODE_NAME, PAGE_HREF, type FamilyId, type Mode } from './data';

/**
 * The one set of building blocks every documentation page uses. A page never re-implements a code block,
 * a copy control, a status chip or a table wrapper; it imports these, so they look and behave the same
 * on all five pages.
 */

const COPY_FAILED = 'Copy failed. Select the text and copy it manually.';

/** Clipboard write that also works without navigator.clipboard (insecure origins); resolves false when nothing could copy. */
export async function copyText(text: string): Promise<boolean> {
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
export function CopyButton({ text, label = 'Copy', done = 'Copied.' }: { text: string; label?: string; done?: string }) {
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
    <>
      <button type="button" className="doc-copy" onClick={copy}>
        {state === 'ok' ? 'Copied' : label}
      </button>
      <span role="status" className={`doc-copy-state doc-copy-${state}`}>
        {state === 'ok' ? done : state === 'fail' ? COPY_FAILED : ''}
      </span>
    </>
  );
}

/**
 * What a code block holds. The kind decides the caption tag and whether a copy control is offered:
 * commands, complete files and excerpts can be pasted; output and proposed syntax cannot.
 *
 * - `command`: shell commands to run.
 * - `file`: a complete file, byte for byte.
 * - `excerpt`: part of a file; not runnable on its own.
 * - `output`: what a command prints. Never copied.
 * - `horizon`: proposed syntax that no beta tool accepts. Never copied.
 */
export type CodeKind = 'command' | 'file' | 'excerpt' | 'output' | 'horizon';

const KIND_TAG: Record<CodeKind, string> = {
  command: 'Shell',
  file: 'Complete file',
  excerpt: 'Excerpt',
  output: 'Output',
  horizon: 'Proposed syntax, not accepted by the beta',
};

const COPY_LABEL: Partial<Record<CodeKind, { label: string; done: string }>> = {
  command: { label: 'Copy', done: 'Copied the commands.' },
  file: { label: 'Copy', done: 'Copied the complete file.' },
  excerpt: { label: 'Copy', done: 'Copied the excerpt.' },
};

export interface CodeProps {
  kind: CodeKind;
  /** Short caption, usually a file name or what the command does. */
  title?: ReactNode;
  children: string;
  /** Optional link to the file in the repository. */
  href?: string;
}

/**
 * One code block: a caption row (kind tag, title, copy control) inside the frame, then the text. A block that
 * scrolls sideways is focusable so a keyboard reader can scroll it; the shell sets that after layout.
 */
export function Code({ kind, title, children, href }: CodeProps) {
  const copy = COPY_LABEL[kind];
  // A complete file is copied byte for byte, final newline included; the display drops it to avoid an empty last line.
  const text = children.replace(/\n$/, '');
  const copied = kind === 'file' ? children : text;
  return (
    <figure className="doc-code" data-kind={kind}>
      <figcaption className="doc-code-bar">
        <span className="doc-code-tag">{KIND_TAG[kind]}</span>
        {title ? <span className="doc-code-title">{href ? <a href={href}>{title}</a> : title}</span> : null}
        {copy ? <CopyButton text={copied} label={copy.label} done={copy.done} /> : null}
      </figcaption>
      <pre>
        <code>{text}</code>
      </pre>
    </figure>
  );
}

/** A long file shown collapsed, for reference. */
export function CodeDetails({ summary, ...code }: CodeProps & { summary: ReactNode }) {
  return (
    <details className="doc-details">
      <summary>{summary}</summary>
      <Code {...code} />
    </details>
  );
}

/** The support labels and result statuses a chip may show. */
export type StatusName =
  | 'LocalS0'
  | 'PreparedUnqualified'
  | 'SignedPreparedUnqualified'
  | 'SpecifiedOnly'
  | 'Unsupported'
  | 'Open'
  | 'CoreRejected'
  | 'SourceRejected'
  | 'AuthoringRejected'
  | 'FormationRejected';

/** How a label renders: runs locally, structure only, or not established. */
const TIER: Record<StatusName, 'runs' | 'structure' | 'open' | 'rejected'> = {
  LocalS0: 'runs',
  PreparedUnqualified: 'runs',
  SignedPreparedUnqualified: 'runs',
  SpecifiedOnly: 'structure',
  Unsupported: 'structure',
  Open: 'open',
  CoreRejected: 'rejected',
  SourceRejected: 'rejected',
  AuthoringRejected: 'rejected',
  FormationRejected: 'rejected',
};

/**
 * A status chip: the label name only, never a sentence. Chips belong in reference tables, the hub's example
 * list and family strips; prose says the meaning in words and links to the reference definition.
 */
export function Status({ name }: { name: StatusName }) {
  return (
    <a className="doc-status" data-tier={TIER[name]} href={`${PAGE_HREF.reference}#support-labels`} title={`${name}: see the support labels reference`}>
      {name}
    </a>
  );
}

/** A table that scrolls on its own when it is wider than the column. The caption names it for assistive technology. */
export function Table({ caption, children }: { caption: ReactNode; children: ReactNode }) {
  return (
    <div className="doc-table-wrap">
      <table className="doc-table">
        <caption>{caption}</caption>
        {children}
      </table>
    </div>
  );
}

/** A section: the unit the sidebar lists. `nav` is its short sidebar label, `group` its sidebar group. */
export function Section({
  id,
  title,
  nav,
  group,
  children,
}: {
  id: string;
  title: ReactNode;
  nav?: string;
  group?: string;
  children: ReactNode;
}) {
  return (
    <section className="guide-section" id={id} aria-labelledby={`${id}-h`} data-nav={nav} data-group={group}>
      <h2 id={`${id}-h`}>{title}</h2>
      {children}
    </section>
  );
}

/** A neutral note. `tone="caution"` is reserved for a point where the reader's result would otherwise mislead them. */
export function Note({ title, tone, children }: { title?: ReactNode; tone?: 'caution'; children: ReactNode }) {
  return (
    <div className="doc-note" role="note" data-tone={tone} aria-label={typeof title === 'string' ? title : undefined}>
      {title ? <p className="doc-note-title">{title}</p> : null}
      {children}
    </div>
  );
}

/** A worked answer the learner computes first, then opens to compare. */
export function Answer({ summary = 'Check your answer', children }: { summary?: ReactNode; children: ReactNode }) {
  return (
    <details className="doc-answer">
      <summary>{summary}</summary>
      <div>{children}</div>
    </details>
  );
}

/** A link that shows which kind of page it opens (Tutorial, How-to, Reference, Explanation). */
export function ModeLink({ mode, href, children }: { mode?: Mode; href: string; children: ReactNode }) {
  return (
    <a className="doc-modelink" href={href}>
      {mode ? (
        <span className="doc-mode" data-mode={mode} aria-label={`${MODE_NAME[mode]}:`} title={MODE_NAME[mode]}>
          {MODE_LETTER[mode]}
        </span>
      ) : null}
      {children}
    </a>
  );
}

export interface SeeAlsoItem {
  /** Omit for a link that is not one of the four kinds, such as the documentation hub. */
  mode?: Mode;
  href: string;
  label: ReactNode;
}

/** The cross-link footer at the end of a section. */
export function SeeAlso({ items, title = 'See also' }: { items: SeeAlsoItem[]; title?: string }) {
  return (
    <nav className="doc-seealso" aria-label={title}>
      <p className="doc-seealso-title">{title}</p>
      <ul>
        {items.map((i) => (
          <li key={i.href}>
            <ModeLink mode={i.mode} href={i.href}>
              {i.label}
            </ModeLink>
          </li>
        ))}
      </ul>
    </nav>
  );
}

/**
 * The three pages that cover one operation family, linked from the top of each: the explanation essay
 * (`explanation.html#<id>`), the reference entry (`reference.html#family-<id>`) and the how-to recipe
 * (`how-to.html#write-<id>`). Every family is SpecifiedOnly.
 */
export function FamilyStrip({ id, here }: { id: FamilyId; here?: Exclude<Mode, 'tutorial'> }) {
  const fam = FAMILIES.find((f) => f.id === id);
  return (
    <nav className="doc-familystrip" aria-label={`${fam?.name ?? id} pages`}>
      <Status name="SpecifiedOnly" />
      {here !== 'explanation' ? (
        <ModeLink mode="explanation" href={`${PAGE_HREF.explanation}#${id}`}>
          About the example
        </ModeLink>
      ) : null}
      {here !== 'reference' ? (
        <ModeLink mode="reference" href={`${PAGE_HREF.reference}#family-${id}`}>
          Reference entry
        </ModeLink>
      ) : null}
      {here !== 'howto' ? (
        <ModeLink mode="howto" href={`${PAGE_HREF.howto}#write-${id}`}>
          How to write one
        </ModeLink>
      ) : null}
    </nav>
  );
}

/** A numbered step list for tutorials and how-to guides. Each child is one step. */
export function Steps({ children }: { children: ReactNode }) {
  return <ol className="doc-steps">{children}</ol>;
}

/**
 * A figure with a light and a dark variant of one image. Both are in the page; CSS shows the one that matches
 * the theme (the media query and the manual data-theme switch). `name` is the asset stem in
 * public/tutorial-assets: `<name>-light.webp`, `<name>-dark.webp`, with PNG fallbacks.
 * Pass `alt=""` for decoration; the caption still describes what is drawn.
 */
/** Pixel size of each image in public/tutorial-assets, so the page reserves the right space before it loads. */
const IMAGE_SIZE: Record<string, [number, number]> = {
  hero: [1536, 768],
  pipeline: [1536, 640],
  'pipeline-mobile': [1024, 1536],
  quadrant: [640, 640],
};

/** Images that have a portrait variant for phones, where the wide one would make its text too small to read. */
const NARROW: Record<string, string> = { pipeline: 'pipeline-mobile' };

export function ThemedFigure({
  name,
  alt,
  caption,
  priority = false,
  className,
}: {
  name: string;
  alt: string;
  caption: ReactNode;
  priority?: boolean;
  className?: string;
}) {
  const [width, height] = IMAGE_SIZE[name] ?? [1536, 1024];
  const narrow = NARROW[name];
  const img = (theme: 'light' | 'dark') => (
    <picture className={`doc-themed doc-themed-${theme}`}>
      {narrow ? (
        <source
          media="(max-width: 640px)"
          type="image/webp"
          srcSet={`tutorial-assets/${narrow}-${theme}.webp`}
          width={IMAGE_SIZE[narrow]?.[0]}
          height={IMAGE_SIZE[narrow]?.[1]}
        />
      ) : null}
      <source type="image/webp" srcSet={`tutorial-assets/${name}-${theme}.webp`} width={width} height={height} />
      <img
        src={`tutorial-assets/${name}-${theme}.png`}
        width={width}
        height={height}
        alt={alt}
        decoding="async"
        loading={priority ? undefined : 'lazy'}
        fetchPriority={priority ? 'high' : undefined}
      />
    </picture>
  );
  return (
    <figure className={`doc-figure${className ? ` ${className}` : ''}${narrow ? ' doc-figure-narrow' : ''}`}>
      {img('light')}
      {img('dark')}
      <figcaption>{caption}</figcaption>
    </figure>
  );
}

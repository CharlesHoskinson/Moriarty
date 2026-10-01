import type { ReactNode } from 'react';
import { DesignMap } from '../DesignMap';
import { DocShell } from '../DocShell';
import { ModeLink, Section, ThemedFigure } from '../components';
import { BETA_PATH, FAMILIES, MODE_LETTER, PAGE_HREF, PROFILE, REPOSITORY, blob, type Mode } from '../data';

interface Quadrant {
  mode: Mode;
  kind: string;
  title: string;
  id: string;
  href: string;
  promise: ReactNode;
  links: { href: string; label: string }[];
}

const T = PAGE_HREF.tutorial;
const H = PAGE_HREF.howto;
const R = PAGE_HREF.reference;
const E = PAGE_HREF.explanation;

const QUADRANTS: Quadrant[] = [
  {
    mode: 'tutorial',
    kind: 'Tutorials',
    title: 'Learn by doing',
    id: 'start-tutorials',
    href: T,
    promise:
      'Install the command-line tool, write a transfer, check it, run it against a local scenario, then make a test fail. Start here if you have never used Moriarty.',
    links: [
      { href: T, label: 'Start the tutorial' },
      { href: `${T}#transfer`, label: 'Create, check and run a transfer' },
      { href: `${T}#repayment`, label: 'Repay a loan, interest first' },
      { href: `${T}#testing`, label: 'Make a test fail' },
    ],
  },
  {
    mode: 'howto',
    kind: 'How-to guides',
    title: 'Get a task done',
    id: 'start-howto',
    href: H,
    promise: 'Short recipes for a task you already know you want to do.',
    links: [
      { href: `${H}#write-test`, label: 'How to write a test' },
      { href: `${H}#sign-intent`, label: 'How to verify a signed intent' },
      { href: `${H}#diagnose`, label: 'How to diagnose a failed check or test' },
      { href: `${H}#check-family`, label: 'How to check a family example' },
    ],
  },
  {
    mode: 'reference',
    kind: 'Reference',
    title: 'Look something up',
    id: 'start-reference',
    href: R,
    promise: 'Commands, language rules, result codes, data formats and support by operation family.',
    links: [
      { href: `${R}#cli`, label: 'Command-line tool' },
      { href: `${R}#language`, label: 'Source language' },
      { href: `${R}#results`, label: 'Results and rejections' },
      { href: `${R}#support-matrix`, label: 'Support by operation family' },
      { href: `${R}#family-amm`, label: 'Operation family entries' },
    ],
  },
  {
    mode: 'explanation',
    kind: 'Explanation',
    title: 'Understand why',
    id: 'start-explanation',
    href: E,
    promise: 'What the language is for, how an intent becomes a candidate, and what a passing test does not show.',
    links: [
      { href: `${E}#purpose`, label: 'What Moriarty is for' },
      { href: `${E}#architecture`, label: 'How an intent becomes a candidate' },
      { href: `${E}#stipulation`, label: 'What a passing local run shows' },
      { href: 'kernel.html', label: 'The Federated DeFi Kernel' },
    ],
  },
];

const COMPASS: { need: string; mode: Mode; href: string; label: string }[] = [
  { need: 'I want to learn the language from the start.', mode: 'tutorial', href: T, label: 'Tutorials' },
  { need: 'I have a specific task to finish.', mode: 'howto', href: H, label: 'How-to guides' },
  { need: 'I need an exact fact: a command, a field, a code.', mode: 'reference', href: R, label: 'Reference' },
  { need: 'I want to know why it works this way.', mode: 'explanation', href: E, label: 'Explanation' },
];

const OTHER: { href: string; label: string; note: string }[] = [
  { href: 'kernel.html', label: 'The Federated DeFi Kernel', note: 'The optional federation layer and where it fits.' },
  {
    href: 'docs/language.html',
    label: 'Formal syntax and semantics',
    note: 'The successor source profile. It is a different language from the beta that these pages teach.',
  },
  { href: 'docs/requirements.html', label: 'Requirements', note: 'Proposed requirements for the native backend.' },
  { href: blob(`${BETA_PATH}/README.md`), label: 'Package README', note: `The ${BETA_PATH} package in the repository.` },
  { href: REPOSITORY, label: 'Source repository', note: 'CharlesHoskinson/Moriarty on GitHub.' },
];

function QuadrantCard({ q }: { q: Quadrant }) {
  return (
    <li className="doc-quadrant" data-mode={q.mode}>
      <p className="doc-quadrant-kind">
        <span className="doc-mode" data-mode={q.mode} aria-hidden="true">
          {MODE_LETTER[q.mode]}
        </span>
        {q.kind}
      </p>
      <h3 id={q.id}>
        <a href={q.href}>{q.title}</a>
      </h3>
      <p className="doc-quadrant-promise">{q.promise}</p>
      <ul className="doc-quadrant-links" aria-label={`${q.kind}: pages`}>
        {q.links.map((l) => (
          <li key={l.href}>
            <a href={l.href}>{l.label}</a>
          </li>
        ))}
      </ul>
    </li>
  );
}

export default function Documentation() {
  return (
    <DocShell
      page="documentation"
      eyebrow="Moriarty beta"
      title="Moriarty documentation"
      lead={
        <p>
          Moriarty is a language for writing bounded financial agreements. These pages cover the beta authoring language{' '}
          <code>{PROFILE}</code>.
        </p>
      }
      groups={['Start here', 'Examples', 'More']}
    >
      <Section id="start" title="Four kinds of page" nav="Four kinds of page" group="Start here">
        <ThemedFigure
          name="hero"
          alt=""
          priority
          className="doc-hero"
          caption="A written agreement, its parts kept in separate trays, and a gate that stays shut until the last step."
        />
        <ThemedFigure
          name="quadrant"
          alt=""
          priority
          className="doc-mark"
          caption="Each kind of page answers one need: learning, doing a task, looking something up, or understanding. The four cards below match the four kinds of page in the figure: tutorials, how-to guides, reference and explanation."
        />
        <ul className="doc-quadrants">
          {QUADRANTS.map((q) => (
            <QuadrantCard key={q.mode} q={q} />
          ))}
        </ul>
      </Section>

      <Section id="design" title="The design on one page" nav="The design on one page" group="Start here">
        <DesignMap />
      </Section>

      <Section id="compass" title="Which do I need?" nav="Which do I need?" group="Start here">
        <ul className="doc-compass">
          {COMPASS.map((c) => (
            <li key={c.mode}>
              <span className="doc-compass-need">{c.need}</span>
              <ModeLink mode={c.mode} href={c.href}>
                {c.label}
              </ModeLink>
            </li>
          ))}
        </ul>
      </Section>

      <Section id="examples" title="Examples by operation family" nav="Operation families" group="Examples">
        <p>
          All eight are <a href={`${R}#support-labels`}>specified only</a>: checked for structure and names, not run.
        </p>
        <ul className="doc-families">
          {FAMILIES.map((f) => (
            <li key={f.id} className="doc-family">
              <p className="doc-family-name">
                <strong>{f.name}</strong>
              </p>
              <p className="doc-family-scope">{f.scope}</p>
              <ul className="doc-family-links" aria-label={`${f.name} pages`}>
                <li>
                  <ModeLink mode="explanation" href={`${E}#${f.id}`}>
                    About<span className="doc-vh"> the {f.name} example</span>
                  </ModeLink>
                </li>
                <li>
                  <ModeLink mode="reference" href={`${R}#family-${f.id}`}>
                    Reference<span className="doc-vh"> entry for {f.name}</span>
                  </ModeLink>
                </li>
                <li>
                  <ModeLink mode="howto" href={`${H}#write-${f.id}`}>
                    How to write<span className="doc-vh"> {f.agreement}</span>
                  </ModeLink>
                </li>
              </ul>
            </li>
          ))}
        </ul>
      </Section>

      <Section id="status" title="What runs" nav="What runs" group="More">
        <p className="doc-statusline">
          Everything runnable here runs locally: nothing is proved, sent to a network or settled.{' '}
          <ModeLink mode="explanation" href={`${E}#stance`}>
            Why the limits are stated this way
          </ModeLink>{' '}
          <ModeLink mode="reference" href={`${R}#support-labels`}>
            Support labels
          </ModeLink>
        </p>
      </Section>

      <Section id="other" title="Other documents" nav="Other documents" group="More">
        <ul className="doc-other">
          {OTHER.map((o) => (
            <li key={o.href}>
              <a href={o.href}>{o.label}</a>
              <span>{o.note}</span>
            </li>
          ))}
        </ul>
      </Section>
    </DocShell>
  );
}

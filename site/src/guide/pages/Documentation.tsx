import type { ReactNode } from 'react';
import { DesignMap } from '../DesignMap';
import { DocShell } from '../DocShell';
import { ModeLink, Section, ThemedFigure } from '../components';
import { BETA_PATH, FAMILIES, MODE_LETTER, PAGE_HREF, PROFILE, REPOSITORY, blob, type Mode } from '../data';
import { term } from '../explanation/links';

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
      'Build mori from a clone, check and run a 10.00 USD transfer, repay part of a loan, then make a test fail. About 30 minutes. Start here if you have never used Moriarty.',
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
    promise: 'Recipes: install from an archive, write a test, diagnose a failed check, check a DeFi example, verify a signed intent.',
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
    promise: 'Why every term is written out, how a source file becomes a prepared result, what a passing local run shows, and how Moriarty relates to Midnight.',
    links: [
      { href: `${E}#purpose`, label: 'What Moriarty is for' },
      { href: `${E}#midnight`, label: 'Moriarty and Midnight' },
      { href: `${E}#from-solidity`, label: 'If you know Solidity' },
      { href: `${E}#architecture`, label: 'From source file to prepared result' },
      { href: `${E}#stipulation`, label: 'What a passing local run shows' },
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
  { href: 'kernel.html', label: 'The Federated DeFi Kernel', note: 'Optional services that may later coordinate evidence and routing. No page here needs them.' },
  {
    href: 'docs/language.html',
    label: 'Formal syntax and semantics',
    note: 'The successor source profile: a different language, which the beta tools reject.',
  },
  { href: 'docs/requirements.html', label: 'Requirements', note: 'Design requirements for compiling Moriarty to Midnight ZKIRv3 and proving it natively. They state obligations, not features.' },
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
        <>
          <p>
            Moriarty is a language for financial agreements on Midnight, for DeFi developers. A program states every
            asset, account, amount, cap and deadline, so an owner can sign exactly those terms.
          </p>
          <p>
            These pages cover the beta, <code>{PROFILE}</code>. Its command-line tool, <code>mori</code>, checks
            programs and runs a transfer or a loan repayment against a scenario you write. Everything runs on your
            computer. Nothing is signed by these tools, proved, sent or settled; the full limits are in{' '}
            <a href={`${E}#stance`}>what these pages do not claim</a>.
          </p>
        </>
      }
      groups={['Start here', 'Examples', 'More']}
    >
      <Section id="start" title="Four kinds of page" nav="Four kinds of page" group="Start here">
        <ThemedFigure
          name="hero"
          alt=""
          priority
          className="doc-hero"
          caption="A blank agreement, three trays of tokens, a route that splits and rejoins, an empty ledger, and a closed gate set apart as the last step."
        />
        <ThemedFigure
          name="quadrant"
          alt=""
          priority
          className="doc-mark"
          caption="Tutorials teach, how-to guides give recipes, reference lists facts, explanation gives reasons."
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
          The eight DeFi examples are <a href={term('specified-only')}>specified only</a>: <code>mori check</code>{' '}
          accepts them, and <code>mori expand</code> and <code>mori simulate</code> refuse to run them.
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

      <Section id="status" title="What works today" nav="What works today" group="More">
        <ul>
          <li>
            <strong>Transfer and loan repayment.</strong> <code>check</code>, <code>fmt</code>, <code>inspect</code>,{' '}
            <code>expand</code>, <code>simulate</code> and <code>test</code> run locally end to end, against a{' '}
            <a href={term('scenario')}>scenario</a> you write.
          </li>
          <li>
            <strong>The eight DeFi examples.</strong> <code>check</code> only. <code>expand</code> and{' '}
            <code>simulate</code> answer <code>Unsupported</code>.
          </li>
          <li>
            <strong>Signatures.</strong> <code>mori intent</code> prepares the bytes for an external signer, and{' '}
            <code>mori verify-intent</code> checks the signature with a native verifier that you build with Cargo.
            This works for the transfer and the repayment only.
          </li>
          <li>
            <strong>Not built.</strong> No compiler, no proof, no network connection and no ledger submission. The
            package is not on the npm registry; you build it from a clone.
          </li>
        </ul>
        <p className="doc-statusline">
          <ModeLink mode="reference" href={`${R}#support-labels`}>
            Support labels
          </ModeLink>{' '}
          <ModeLink mode="explanation" href={`${E}#stance`}>
            What these pages do not claim
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

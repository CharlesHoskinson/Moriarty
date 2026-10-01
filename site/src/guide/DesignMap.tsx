import { PAGE_HREF } from './data';

/**
 * The whole design on one page: what runs on your computer today, the separate signing check, the target that
 * no beta tool reaches, and the optional federation. Built as HTML rather than an image so every label is real
 * text that links to its reference entry, follows the colour scheme and reflows on a phone.
 *
 * Every label is a fact from reference.html; change the reference first, then this map.
 */

const R = PAGE_HREF.reference;
const E = PAGE_HREF.explanation;
const T = PAGE_HREF.tutorial;
const H = PAGE_HREF.howto;

interface Box {
  title: string;
  command?: string;
  body: string;
  result?: string;
  href: string;
}

const LOCAL: Box[] = [
  {
    title: 'Write the agreement',
    command: 'invoice.mori',
    body: 'Domain, accounts, asset and scale, then one intent per action with its caps and round window. A runnable intent writes out all twenty of its fields.',
    href: `${R}#language`,
  },
  {
    title: 'Check the source',
    command: 'mori check · mori inspect',
    body: 'Names, units, shapes and limits. inspect lists each intent with its terms and bounds.',
    result: 'AuthoringChecked or AuthoringRejected',
    href: `${R}#cli`,
  },
  {
    title: 'Generate the program',
    command: 'mori expand --scenario',
    body: 'One action plus a scenario becomes a Source/6 program.',
    result: 'Expanded',
    href: `${R}#scenario-format`,
  },
  {
    title: 'Run Core',
    command: 'mori simulate',
    body: 'Six judgments in order: stage, intent, effect, authority, history, failure. The first failure stops the run.',
    result: 'PreparedUnqualified (effects and post-state) or CoreRejected (one code)',
    href: `${R}#results`,
  },
  {
    title: 'Test your arithmetic',
    command: 'mori test',
    body: 'Each case in mori.tests.json gives the status you expect and, if you worked them out by hand, the effects and post-state.',
    result: 'TestsPassed or TestsFailed',
    href: `${R}#test-format`,
  },
];

const SIGN: Box[] = [
  {
    title: 'Prepare the statement',
    command: 'mori intent',
    body: 'A canonical intent frame that binds the source SHA-256, the action, the asset, the signer and the bounds.',
    result: 'OwnerIntentPrepared',
    href: `${H}#sign-intent`,
  },
  {
    title: 'Sign it',
    command: 'your own signer',
    body: 'Outside the tool. The scenario is not part of what is signed.',
    href: `${H}#sign-intent-own`,
  },
  {
    title: 'Verify the signature',
    command: 'mori verify-intent',
    body: 'A native verifier checks the signature, then Core prepares the candidate.',
    result: 'SignedPreparedUnqualified, with key authority Unverified',
    href: `${R}#results`,
  },
];

const TARGET: Box[] = [
  {
    title: 'Compile to ZKIRv3',
    body: 'The program as a Midnight circuit. No compiler exists in the beta.',
    href: `${E}#architecture`,
  },
  {
    title: 'Prove natively',
    body: "A proof from Midnight's own proof stack. No beta tool produces one.",
    href: `${E}#architecture`,
  },
  {
    title: 'Settle on Midnight',
    body: 'The ledger accepts the transaction. No example here reaches a ledger.',
    href: `${E}#stance`,
  },
];

function Step({ box, n }: { box: Box; n?: number }) {
  return (
    <li className="map-step">
      <a href={box.href}>
        <span className="map-step-title">
          {n ? <span className="map-n">{n}</span> : null}
          {box.title}
        </span>
        {box.command ? <code className="map-cmd">{box.command}</code> : null}
        <span className="map-body">{box.body}</span>
        {box.result ? <span className="map-result">{box.result}</span> : null}
      </a>
    </li>
  );
}

export function DesignMap() {
  return (
    <figure className="doc-map" aria-labelledby="design-map-caption">
      <div className="map-lane" data-lane="local">
        <p className="map-lane-head">
          <strong>Runs today, on your computer</strong>
          <span>
            The <a href={`${T}#transfer`}>transfer</a> and <a href={`${T}#repayment`}>repayment</a> examples run end to
            end. The scenario file states the starting balances, allowance and head; nothing reads a real ledger.
          </span>
        </p>
        <ol className="map-steps">
          {LOCAL.map((b, i) => (
            <Step key={b.title} box={b} n={i + 1} />
          ))}
        </ol>
        <p className="map-note">
          The eight DeFi examples (AMM, lending, stablecoins, options, oracles, governance, bridges, staking) stop after
          step 2: <code>check</code> accepts them, and <code>expand</code> and <code>simulate</code> answer{' '}
          <code>Unsupported</code>. <a href={`${R}#support-matrix`}>Support by family</a>
        </p>
      </div>

      <div className="map-lane" data-lane="sign">
        <p className="map-lane-head">
          <strong>Optional: check an owner's signature</strong>
          <span>
            Runs today with a native verifier you build. A valid signature shows that the key signed these terms, not that
            the key may act for the account.
          </span>
        </p>
        <ol className="map-steps">
          {SIGN.map((b) => (
            <Step key={b.title} box={b} />
          ))}
        </ol>
      </div>

      <div className="map-lane" data-lane="target">
        <p className="map-lane-head">
          <strong>Target, not built</strong>
          <span>Where a prepared candidate from step 4 is meant to go. Every step here is open.</span>
        </p>
        <ol className="map-steps">
          {TARGET.map((b) => (
            <Step key={b.title} box={b} />
          ))}
        </ol>
        <p className="map-note">
          Beside the target sits the optional <a href="kernel.html">Federated DeFi Kernel</a>: it would coordinate
          evidence and routing between programs. It never accepts a transaction, and no program needs it.{' '}
          <a href={`${E}#federation`}>About the federation</a>
        </p>
      </div>

      <figcaption id="design-map-caption">
        The Moriarty beta on one page. The first two lanes run today; the red lane is the target. Each box links to the
        page that documents it.
      </figcaption>
    </figure>
  );
}

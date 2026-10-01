import { useState, type ReactNode } from 'react';
import repaymentSource from '../../../packages/moriarty-beta/examples/local/repay/repayment.mori?raw';
import transferSource from '../../../packages/moriarty-beta/examples/signed-intent/transfer-schnorr-raw/program.mori?raw';
import lendingSource from '../../../packages/moriarty-beta/examples/lending.mori?raw';
import bridgeSource from '../../../packages/moriarty-beta/examples/bridge.mori?raw';
import ammSource from '../../../packages/moriarty-beta/examples/amm.mori?raw';

const REPO = 'https://github.com/CharlesHoskinson/Moriarty/blob/main';
const BETA = `${REPO}/packages/moriarty-beta`;
const HORIZON = `${REPO}/wiki-llm/beta-language-2026-09-30/FULL-LANGUAGE-HORIZON.md`;

type StatusName = 'LocalS0' | 'SpecifiedOnly' | 'Open' | 'PreparedUnqualified';

const STATUS_MEANING: Record<StatusName, string> = {
  LocalS0: 'executable locally in the beta package',
  SpecifiedOnly: 'checked beta structure only (names and quantities); not expanded or run',
  Open: 'not provided yet; proposed or unverified',
  PreparedUnqualified: 'a local candidate, not a signed, proved or settled transaction',
};

function Status({ name, note }: { name: StatusName; note?: string }) {
  return (
    <span className="guide-status" data-status={name}>
      <strong>{name}</strong>
      {' — '}
      {note ?? STATUS_MEANING[name]}
    </span>
  );
}

type Scope = 'complete' | 'excerpt' | 'horizon';

const SCOPE_LABEL: Record<Scope, string> = {
  complete: 'Complete beta file',
  excerpt: 'Beta excerpt (non-runnable)',
  horizon: 'Proposed horizon syntax (not accepted by beta parser)',
};

interface CodeProps {
  title: string;
  lang: 'mori' | 'json' | 'text';
  scope: Scope;
  status?: StatusName;
  source: string;
  href?: string;
  note?: string;
}

/**
 * One code block with an unambiguous presentation scope. Complete files keep
 * the imported bytes exactly; excerpts and horizon text are explanatory only.
 * Copy copies exactly what is displayed, and its outcome is announced.
 */
function Code({ title, lang, scope, status, source, href, note }: CodeProps) {
  const [copyMessage, setCopyMessage] = useState('');
  const shown = scope === 'complete' ? source : source.trimEnd();

  async function copy() {
    try {
      if (!navigator.clipboard || typeof navigator.clipboard.writeText !== 'function') {
        throw new Error('clipboard unavailable');
      }
      await navigator.clipboard.writeText(shown);
      setCopyMessage(`Copied ${title}.`);
    } catch {
      setCopyMessage(`Copy failed for ${title}: clipboard is unavailable. Select the text instead.`);
    }
  }

  return (
    <figure className="guide-code" data-kind={scope === 'horizon' ? 'horizon' : 'beta'} data-scope={scope}>
      <figcaption>
        <strong>{title}</strong> <em>{SCOPE_LABEL[scope]}.</em>{' '}
        {status ? <Status name={status} note={note} /> : note ? <span>{note}</span> : null}
        {href ? (
          <>
            {' '}
            <a href={href}>Source on GitHub</a>
          </>
        ) : null}
      </figcaption>
      <div className="guide-code-toolbar">
        <button type="button" className="guide-copy" onClick={copy} aria-label={`Copy ${title}`}>
          Copy
        </button>
        <span className="guide-copy-status" role="status" aria-live="polite">
          {copyMessage}
        </span>
      </div>
      <pre>
        <code className={`language-${lang}`}>{shown}</code>
      </pre>
    </figure>
  );
}

const TRANSFER_CORE = `// Gross debit = price + fee. From the Invoice example.
const price: Qty<USD> = 10.00 USD;   // 1000 atoms at scale 2
const fee = 0.10 USD;                // 10 atoms
intent Payment = {
  gross_cap: price + fee,            // 1010 atoms: the fee counts toward the cap
  fee_cap: fee,
  net_floor: price,                  // the recipient must receive at least 1000
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee),
  /* ...remaining signed terms as in the full file above... */
};`;

const REPAY_TEST_SHAPE = `{
  "profile": "moriarty-beta-tests/1",
  "cases": [
    {
      "name": "overpayment is rejected by Core",
      "source": "repayment.mori",
      "action": "repay_loan",
      "scenario": "overpay-scenario.json",
      "expect": { "status": "CoreRejected" }
    }
  ]
}`;

const HORIZON_HOLES = `// moriarty-horizon/0.1 — proposed syntax, excerpt of the Exchange agreement
intent Trade {
  fixed { owner: Alice; recipient: Alice; fee_to: Treasury; pool: Spot;
          input: 100.00 USD; output_asset: GOLD; net_floor: 0.900 GOLD; }
  holes { fee: QuantityHole<USD,31>(id: Fee, allowed: interval(0.00 USD,0.30 USD,0.01 USD));
          output: QuantityHole<GOLD,64>(id: Output, allowed: interval(0.900 GOLD,0.963 GOLD,0.001 GOLD)); }
  bounds { USD: budget(gross: 100.30 USD, fees: 0.30 USD);
           GOLD: budget(gross: 0.000 GOLD, fees: 0.000 GOLD); work: 1; }
}`;

const HORIZON_DUTY = `// moriarty-horizon/0.1 — proposed syntax, from the shared Horizon prelude
struct Duty<A> { claim: ClaimId<domain(A)>, owed: Qty<A>, terms: Digest,
                 continuation: Set<ActionId,16>, recovery: Set<ActionId,8> }
enum DebtStatus { Outstanding, Defaulted, Settled }`;

const HORIZON_RECOVERY = `// moriarty-horizon/0.1 — proposed syntax, from the Bridge library
enum ReceiptKnowledge { FinalReceipt, QualifiedNonreceipt, Unknown, Conflict }
relation Recover<A>(source: EscrowState<A>, nonreceipt: NonreceiptProof) -> EscrowState<A>;
// Recover: count=0 AND final nonreceipt/exclusion proof -> release 100 USD once.`;

const GRAMMAR_SKELETON = `profile "moriarty-beta/1";
agreement Name {
  domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
  account Payer = { domain: Preview, id: "Payer" };
  asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical" };
  const amount = 30.00 USD;                 // refers only to earlier declarations
  intent Repayment = { /* explicit signed terms */ };
  action repay_loan uses Repayment;         // the action selects an intent
}`;

function Term({ children }: { children: ReactNode }) {
  return <code>{children}</code>;
}

export function LanguagePatterns() {
  return (
    <>
      <section id="syntax" className="guide-section" aria-labelledby="syntax-h">
        <h2 id="syntax-h">Surface syntax</h2>
        <p>
          A beta program is a profile line followed by one agreement of brace-delimited declarations. Each
          declaration may refer only to declarations written before it, so the program is a finite chain of
          immutable bindings and a reader can follow every name upward to where it was defined. The
          current profile is <Term>moriarty-beta/1</Term>; this page adds no grammar beyond it.
        </p>
        <Code
          title="Grammar skeleton"
          lang="mori"
          scope="excerpt"
          source={GRAMMAR_SKELETON}
          note="Shape only. It contains an elided intent and is not an executable LocalS0 program."
        />
        <p>
          The first complete program is the shipped repayment example. It declares a domain, two accounts, an
          asset, an obligation, a constant and one intent, then names one action. It is the source behind the
          local repayment lesson.
        </p>
        <Code
          title="repayment.mori"
          lang="mori"
          scope="complete"
          status="LocalS0"
          source={repaymentSource}
          href={`${BETA}/examples/local/repay/repayment.mori`}
          note="executable locally; a successful run yields PreparedUnqualified"
        />

        <h3>Quantities, atoms and scales</h3>
        <p>
          A quantity is a decimal literal followed by an asset reference, for example <Term>30.00 USD</Term>.
          The asset declares its scale, and the quantity is stored as an integer count of atoms. At scale 2,{' '}
          <Term>10.00 USD</Term> is 1000 atoms and <Term>30.00 USD</Term> is 3000 atoms. JSON scenarios carry
          these counts as canonical decimal atom strings. There is no floating point, division, rounding or
          implicit conversion: <Term>10.00USD</Term> without a separator is rejected, and a literal needs the
          asset to give it meaning. The equivalent explicit form is{' '}
          <Term>atoms(asset: USD,value: 1000)</Term>.
        </p>
        <div className="guide-grid">
          <div>
            <h4>Ranges</h4>
            <p>
              Expression values are checked at every operation within 0 to 2^128−1. Signed monetary fields in
              S0 (values, fees, caps, floors, obligation amounts) must fit 2^127−1. Balances, counters and
              rounds may use the full unsigned 128-bit range. Scale is 0 to 18.
            </p>
          </div>
          <div>
            <h4>Operators</h4>
            <p>
              Checked <Term>+ - *</Term> on integers and same-asset quantities, associating left, with no unary
              minus. Comments and trailing commas are allowed. Names must follow{' '}
              <Term>[A-Za-z][A-Za-z0-9_]{'{0,63}'}</Term> when they become economic IDs.
            </p>
          </div>
        </div>

        <h3>Names, IDs and symbols are three different things</h3>
        <p>
          A source alias such as <Term>Buyer</Term> exists for navigation within one file. Its{' '}
          <Term>id</Term> field is the economic identity the scenario and Core use, so{' '}
          <Term>account Buyer = {'{'} domain: Preview, id: "Owner" {'}'}</Term> is the account{' '}
          <Term>Owner</Term> in domain <Term>Midnight</Term>. Renaming the alias leaves the identity
          unchanged. A display symbol such as <Term>symbol: "USD"</Term> is metadata only: it is neither an
          asset identity nor permission to convert. Assets and domains of different identity never mix. All
          of these are nominal claims; the language does not authenticate that an account is owned by anyone
          or that an asset is backed by anything.
        </p>
        <Code
          title="Invoice transfer (alias Buyer, id Owner; asset id A, symbol USD)"
          lang="mori"
          scope="complete"
          status="LocalS0"
          source={transferSource}
          href={`${BETA}/examples/signed-intent/transfer-schnorr-raw/program.mori`}
          note="executable locally and used by the signed-intent walkthrough"
        />

        <h3>Named arguments and bounded stages</h3>
        <p>
          Financial operations take named arguments only, such as{' '}
          <Term>transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee)</Term>. The beta
          registry fixes the argument names per operation, and unknown names or arguments are rejected. Only{' '}
          <Term>transfer</Term> and <Term>repay</Term> lower to the local evaluator today. An intent bounds
          its action with an inclusive round window (<Term>rounds(domain: Preview, from: 0, to: 10)</Term>),
          a gross cap that includes fees, a fee cap, a net floor, a nonce and a prior head. These are signed
          terms with no hidden defaults; the empty lists for observations, disclosures, retained effects and
          duties are written out because emptiness is itself a signed term. Source limits are finite: 65536
          bytes, 8192 tokens, depth 64 and 256 declarations.
        </p>

        <h3>Failures</h3>
        <p>
          Failure is reported at a named boundary rather than as a generic error. Formation, authoring and
          source errors mean the input was refused. A program can pass <Term>check</Term> and still be
          rejected by Core for a particular scenario, because caps, funds, authority, expiry and replay are
          decided there, in the order Stage, Intent, Effect, Authority, History, Failure. Tests assert one of{' '}
          <Term>PreparedUnqualified</Term>, <Term>CoreRejected</Term>, <Term>SourceRejected</Term>,{' '}
          <Term>AuthoringRejected</Term>, <Term>FormationRejected</Term> or <Term>Unsupported</Term>.
        </p>

        <h3>Current support</h3>
        <table>
          <thead>
            <tr>
              <th scope="col">Area</th>
              <th scope="col">Label</th>
              <th scope="col">Meaning today</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">Transfer with fee; repayment (AccrualFirst)</th>
              <td>
                <Status name="LocalS0" />
              </td>
              <td>Checked, simulated and tested against explicit local fixtures.</td>
            </tr>
            <tr>
              <th scope="row">Result of a local run</th>
              <td>
                <Status name="PreparedUnqualified" />
              </td>
              <td>A candidate. The scenario is a stipulation, not chain state.</td>
            </tr>
            <tr>
              <th scope="row">AMM, lending, stablecoin, derivatives, oracle, governance, bridge, staking</th>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>Names and quantities checked; expansion and simulation answer Unsupported.</td>
            </tr>
            <tr>
              <th scope="row">Key authority, authenticated state, proving, ledger settlement</th>
              <td>
                <Status name="Open" />
              </td>
              <td>Four premises and four bindings stay unverified in every local result.</td>
            </tr>
          </tbody>
        </table>
        <p>
          Reference: <a href={`${BETA}/GETTING-STARTED.md`}>Getting started</a>,{' '}
          <a href={`${BETA}/SIGNED-INTENT.md`}>signed intent walkthrough</a>,{' '}
          <a href={`${BETA}/examples`}>example directory</a>.
        </p>
      </section>

      <section id="patterns" className="guide-section" aria-labelledby="patterns-h">
        <h2 id="patterns-h">Reusable design patterns</h2>
        <p>
          Each pattern states what it protects, shows the smallest fragment that carries it, and says which
          label applies. Complete beta files are shown byte for byte from shipped sources. Beta excerpts are
          trimmed for explanation and are not runnable. Horizon fragments are proposed syntax that no beta
          parser accepts.
        </p>

        <h3>1. Gross debit includes the fee</h3>
        <p>
          The owner's cap must cover value plus fee, and the recipient floor is checked against value alone.
          With price 1000 atoms and fee 10, the debit is 1010: Owner 10000 becomes 8990, Recipient receives
          1000, Fee receives 10, and the allowance falls by 1010.
        </p>
        <Code
          title="Fee inside the cap"
          lang="mori"
          scope="excerpt"
          source={TRANSFER_CORE}
          note="Excerpt of the complete Invoice file above; the ellipsis comment makes this excerpt itself not runnable."
        />

        <h3>2. Accrue first, then principal</h3>
        <p>
          A funded repayment applies to accrued interest before principal. With principal 100000, accrued
          1000 and a payment of 3000 atoms, accrued becomes 0 and principal becomes 98000. The status stays{' '}
          <Term>Outstanding</Term>, and the intent requires a zero fee cap and zero net floor. The expected
          effects and post-state are written out in full in the{' '}
          <a href={`${BETA}/examples/local/repay/mori.tests.json`}>repayment test file</a> so the arithmetic is
          independent of what the tool prints.
        </p>
        <Status name="LocalS0" note="repayment.mori above; a successful run yields PreparedUnqualified" />

        <h3>3. Make invalid and rejected proposals explicit</h3>
        <p>
          A payment above the outstanding balance still produces a well-formed proposal, with the debt cells
          unchanged, and Core rejects it. The rejection publishes no new effects or post-state, and the
          proposal is never presented as a prepared result. Write the rejection as its own test so that
          failure is a checked behavior, and keep it separate from a test whose money expectation is wrong.
          Do not copy simulator output into an expectation to turn a test green.
        </p>
        <Code
          title="Shape of a rejection test case"
          lang="json"
          scope="excerpt"
          source={REPAY_TEST_SHAPE}
          href={`${BETA}/GETTING-STARTED.md`}
          note="Illustrative shape; overpay-scenario.json is a file you would write, not a shipped one."
        />

        <h3>4. Persistent duties</h3>
        <p>
          A liquidation or partial settlement should not erase what remains owed. The beta lending sketch
          records the intent in a string, and the horizon gives a duty a typed shape: a claim, an amount owed,
          terms, and the continuation and recovery actions that remain permitted.
        </p>
        <Code
          title="lending.mori"
          lang="mori"
          scope="complete"
          status="SpecifiedOnly"
          source={lendingSource}
          href={`${BETA}/examples/lending.mori`}
          note="checked for names and quantities; expansion answers Unsupported"
        />
        <Code
          title="Duty and debt status"
          lang="mori"
          scope="horizon"
          status="Open"
          source={HORIZON_DUTY}
          href={HORIZON}
          note="Duty is proposed; no beta parser, checker or evaluator accepts it."
        />

        <h3>5. Conditional settlement and recovery</h3>
        <p>
          Cross-domain movement is split into separate stages that share one claim identifier, and recovery is
          its own action rather than a side effect of waiting. The beta bridge sketch states the principle in
          its failure text: a timeout alone is not proof that the foreign side never received. The horizon
          makes that a type, so a release can require a qualified nonreceipt and an Unknown or Conflict outcome
          cannot release funds.
        </p>
        <Code
          title="bridge.mori"
          lang="mori"
          scope="complete"
          status="SpecifiedOnly"
          source={bridgeSource}
          href={`${BETA}/examples/bridge.mori`}
          note="Preview and Foreign are different domains; checked for names and quantities only"
        />
        <Code
          title="Recovery needs evidence"
          lang="mori"
          scope="horizon"
          status="Open"
          source={HORIZON_RECOVERY}
          href={HORIZON}
          note="Proposed; foreign verification, finality and nullifier proofs are not implemented."
        />

        <h3>6. Optional solver alternatives inside immutable bounds</h3>
        <p>
          The owner fixes recipients, assets, the net floor and the budgets. A solver may fill only declared
          holes, each a finite interval with a step. Whatever the solver picks, the fee hole cannot exceed the
          signed fee cap and the output cannot fall below the signed floor. Hard bounds are signed terms, so a
          hole offers choice without widening authority. In the beta AMM sketch below, the fee cap and net
          floor are fixed in the operation and nothing is chosen by a solver.
        </p>
        <Code
          title="amm.mori"
          lang="mori"
          scope="complete"
          status="SpecifiedOnly"
          source={ammSource}
          href={`${BETA}/examples/amm.mori`}
          note="checked for names and quantities; expansion answers Unsupported"
        />
        <Code
          title="Finite holes with fixed bounds"
          lang="mori"
          scope="horizon"
          status="Open"
          source={HORIZON_HOLES}
          href={HORIZON}
          note="Holes are proposed; the pool invariant and refinement proofs are also Open."
        />
        <p>
          Read the full proposal in the <a href={HORIZON}>full-language horizon</a>.
        </p>

        <h3>Signed intent versus candidate acceptance evidence</h3>
        <p>
          A signed intent is the owner's terms: signer and key reference, nonce, head, round window, gross cap,
          fee cap, net floor and the fixed operation. The scenario, balances, counters, proposed effects and
          post-state are not signed by the owner. They form a candidate that the local Core judges, and a
          changed scenario can keep the signature valid while producing a different candidate or a rejection.
          Candidate acceptance evidence, meaning authenticated state, native proof and ledger acceptance, is a
          separate layer that this page does not claim to exist. Acceptance of a candidate is distinct from
          optional federation, where several parties may later cross-check or relay results; neither is
          provided here. A source <Term>key: "key1"</Term> is an opaque
          reference. It is not a public key and it does not prove who owns the account. The native signature
          check on a prepared intent yields SignedPreparedUnqualified, with key authority Unverified, native
          proof NotChecked and ledger NotSubmitted.
        </p>
        <div className="guide-callout" role="note">
          <Status name="Open" note="authentication of IDs and ownership, proving, and financial settlement" />
          <p>
            Follow the <a href={`${BETA}/SIGNED-INTENT.md`}>signed intent walkthrough</a> for the exact
            commands and result table. This site does not reproduce a send, deploy or prove command because
            none exists for the beta package.
          </p>
        </div>
      </section>
    </>
  );
}

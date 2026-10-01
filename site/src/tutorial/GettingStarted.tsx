import { useState } from 'react';
import transferSource from './lessons/transfer.mori?raw';
import transferScenario from './lessons/transfer.scenario.json?raw';
import transferTests from './lessons/transfer.test.json?raw';
import repaySource from './lessons/repay.mori?raw';
import repayScenario from './lessons/repay.scenario.json?raw';
import repayTests from './lessons/repay.test.json?raw';

const PINNED = 'df8b155143e90450168af04fbfd20f7620043620';
const BLOB = `https://github.com/CharlesHoskinson/Moriarty/blob/${PINNED}/packages/moriarty-beta`;
const GUIDE = `${BLOB}/GETTING-STARTED.md`;
const SIGNED = `${BLOB}/SIGNED-INTENT.md`;

type Status = 'LocalS0' | 'SpecifiedOnly' | 'Open' | 'PreparedUnqualified';
type Kind = 'Complete file' | 'Commands' | 'Captured output';

interface TransferCase {
  name: string;
  expect: { status: string; code?: string; post?: { balances: { account: string; amount: string }[] } };
}
interface TestFile { profile: string; cases: TransferCase[] }
interface Scenario {
  balances: { account: string; amount: string }[];
  allowance: { owner: string; remaining: string; spent: string };
}

/** Derived from the shipped lesson files, so the failing and rejecting variants cannot drift from them. */
function wrongExpectation(): string {
  const tests = JSON.parse(transferTests) as TestFile;
  const first = tests.cases[0];
  const owner = first?.expect.post?.balances[0];
  if (first && owner) {
    first.name = 'owner balance after transfer (wrong)';
    owner.amount = '8991';
  }
  return JSON.stringify(tests, null, 2) + '\n';
}

function underfundedScenario(): string {
  const scenario = JSON.parse(transferScenario) as Scenario;
  const owner = scenario.balances[0];
  if (owner) owner.amount = '1009';
  scenario.allowance.remaining = '1009';
  return JSON.stringify(scenario, null, 2) + '\n';
}

function rejectionExpectation(): string {
  const tests = JSON.parse(transferTests) as TestFile;
  const first = tests.cases[0];
  if (first) {
    first.name = 'insufficient funds rejected by Core';
    first.expect = { status: 'CoreRejected', code: 'S0_EFFECT_RANGE' };
  }
  return JSON.stringify(tests, null, 2) + '\n';
}

function StatusLabel({ status }: { status: Status }) {
  return <span className="guide-status" data-status={status}>{status}</span>;
}

function dataUrl(text: string): string {
  return `data:text/plain;charset=utf-8,${encodeURIComponent(text)}`;
}

/** One Copy per artifact. The shell skips figures marked data-copy-ready, so this toolbar is the only one. */
function CopyButton({ text, label }: { text: string; label: string }) {
  const [state, setState] = useState<'idle' | 'ok' | 'fail'>('idle');
  const copy = () => {
    const write = typeof navigator !== 'undefined' ? navigator.clipboard?.writeText : undefined;
    if (!write) {
      setState('fail');
      return;
    }
    navigator.clipboard.writeText(text).then(
      () => setState('ok'),
      () => setState('fail'),
    );
  };
  return (
    <>
      <button type="button" className="guide-copy" onClick={copy} aria-label={`Copy ${label}`}>Copy</button>
      <span role="status" className={`guide-copy-state guide-copy-${state}`}>
        {state === 'ok' ? 'Copied.' : state === 'fail' ? 'Copy failed. Select the text and copy it manually.' : ''}
      </span>
    </>
  );
}

function Code({ label, kind, text, file }: { label: string; kind: Kind; text: string; file?: string }) {
  return (
    <figure className="guide-code" data-copy-ready="true">
      <figcaption className="guide-copybar">
        <strong>{label}</strong>
        <span className="guide-code-kind">{kind}</span>
        <CopyButton text={text} label={label} />
        {file ? <a href={dataUrl(text)} download={file}>Download {file}</a> : null}
      </figcaption>
      <pre tabIndex={0}><code>{text}</code></pre>
    </figure>
  );
}

/** Collapsed complete file. The Code toolbar (Copy and Download) is inside, so it is available once opened; the text is unchanged. */
function FileDetails({ summary, ...code }: { summary: string; label: string; kind: Kind; text: string; file: string }) {
  return (
    <details className="guide-details">
      <summary>{summary}</summary>
      <Code {...code} />
    </details>
  );
}

/** Provenance for a captured output: exact command, exit code, and the pinned source it was run against. */
function Capture({ command, exit }: { command: string; exit: number }) {
  return (
    <p className="guide-callout">
      <strong>Captured output.</strong> Exact stdout of <code>{command}</code>, exit {exit}, from
      the <code>dist/cli.js</code> already built in the maintainer worktree at commit{' '}
      <a href={`https://github.com/CharlesHoskinson/Moriarty/tree/${PINNED}/packages/moriarty-beta`}>{PINNED.slice(0, 8)}</a>{' '}
      on 2026-10-01, run against the file bytes shown above. No receipt is published for it;
      your build may differ, so re-run the command.
    </p>
  );
}

const CHECKOUT_SETUP = `git clone https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty/packages/moriarty-beta
npm ci
npm run build
MORI_PKG="$PWD"
mori() { node "$MORI_PKG/dist/cli.js" "$@"; }
mori --help`;

const TGZ_SETUP = `# from packages/moriarty-beta in your checkout, after the checkout steps above
MORI_TGZ="$PWD/$(npm pack --silent)"
echo "$MORI_TGZ"
mkdir -p /tmp/mori-project && cd /tmp/mori-project
npm install "$MORI_TGZ"
mori() { npx mori "$@"; }
mori --help`;

const TRANSFER_COMMANDS = `mori init /tmp/invoice
mori check /tmp/invoice/invoice.mori
mori fmt /tmp/invoice/invoice.mori
mori inspect /tmp/invoice/invoice.mori
mori expand /tmp/invoice/invoice.mori --action pay --scenario /tmp/invoice/scenario.json
mori simulate /tmp/invoice/invoice.mori --action pay --scenario /tmp/invoice/scenario.json
mori test /tmp/invoice`;

const REPAY_COMMANDS = `mori init /tmp/loan --template repay
mori check /tmp/loan/repayment.mori
mori simulate /tmp/loan/repayment.mori --action repay_loan --scenario /tmp/loan/scenario.json
mori test /tmp/loan`;

const WRONG_COMMANDS = `DL="$HOME/Downloads"   # replace if your browser saves downloads elsewhere
mkdir /tmp/invoice-wrong
cp /tmp/invoice/invoice.mori /tmp/invoice/scenario.json /tmp/invoice-wrong/
cp "$DL/transfer-wrong-expectation.mori.tests.json" /tmp/invoice-wrong/mori.tests.json
mori test /tmp/invoice-wrong`;

const REJECT_COMMANDS = `DL="$HOME/Downloads"   # replace if your browser saves downloads elsewhere
mkdir /tmp/invoice-underfunded
cp /tmp/invoice/invoice.mori /tmp/invoice-underfunded/
cp "$DL/transfer-underfunded.scenario.json" /tmp/invoice-underfunded/scenario.json
cp "$DL/transfer-core-rejection.mori.tests.json" /tmp/invoice-underfunded/mori.tests.json
mori test /tmp/invoice-underfunded`;

const TRANSFER_OUTPUT = `{
  "status": "TestsPassed",
  "qualification": "local-stipulation-only",
  "cases": [
    {
      "name": "literal fee payment",
      "passed": true,
      "status": "PreparedUnqualified",
      "code": null,
      "mismatches": []
    }
  ]
}`;

const REPAY_OUTPUT = `{
  "status": "TestsPassed",
  "qualification": "local-stipulation-only",
  "cases": [
    {
      "name": "interest first partial repayment",
      "passed": true,
      "status": "PreparedUnqualified",
      "code": null,
      "mismatches": []
    }
  ]
}`;

const WRONG_OUTPUT = `{
  "status": "TestsFailed",
  "qualification": "local-stipulation-only",
  "cases": [
    {
      "name": "owner balance after transfer (wrong)",
      "passed": false,
      "status": "PreparedUnqualified",
      "code": null,
      "mismatches": [
        {
          "pointer": "/post/balances/0/amount",
          "expected": "8991",
          "actual": "8990"
        }
      ]
    }
  ]
}`;

const REJECT_OUTPUT = `{
  "status": "TestsPassed",
  "qualification": "local-stipulation-only",
  "cases": [
    {
      "name": "insufficient funds rejected by Core",
      "passed": true,
      "status": "CoreRejected",
      "code": "S0_EFFECT_RANGE",
      "mismatches": []
    }
  ]
}`;

export function GettingStarted() {
  const wrongTests = wrongExpectation();
  const underfunded = underfundedScenario();
  const rejectionTests = rejectionExpectation();

  return (
    <>
      <section id="getting-started" className="guide-section" aria-labelledby="getting-started-h">
        <h2 id="getting-started-h">Getting started</h2>
        <p>
          The beta toolchain checks <code>.mori</code> financial source and prepares a local candidate
          for two operations: a transfer with a fee and a loan repayment. Both are <StatusLabel status="LocalS0" />.
          The preparation result is <StatusLabel status="PreparedUnqualified" />: a candidate computed from an
          explicit local scenario, not an authenticated, proved, signed or settled transaction. The other
          operation families in the examples check structure only (<StatusLabel status="SpecifiedOnly" />).
        </p>

        <h3 id="install">Install from a checkout</h3>
        <p>
          The CLI needs Node 24 or later and a Moriarty checkout; there is no npm registry release and no published
          download. The first block clones the repository, builds the package and defines a <code>mori</code> shell
          function that points at the build by its absolute path. The function lasts for this shell session, and
          every lesson command below uses <code>mori</code>. In a new shell, run the two lines that set
          <code> MORI_PKG</code> and <code>mori</code> again from <code>packages/moriarty-beta</code>.
        </p>
        <Code label="Clone, build and define mori" kind="Commands" text={CHECKOUT_SETUP} />

        <h3 id="install-archive">Alternative: build and install a local archive</h3>
        <p>
          To use the CLI from another project, pack the build you just made. <code>npm pack</code> writes
          the archive into the package directory and prints its name. The block records the absolute path in
          <code> MORI_TGZ</code> before changing directory, installs from that path and redefines <code>mori</code> to
          run the installed copy. It is an archive of your own build, not a downloadable release. The
          commands are taken from the <a href={GUIDE}>getting-started guide</a>, with the archive path captured by the shell; they were not run for this page.
        </p>
        <Code label="Pack the build and install it elsewhere" kind="Commands" text={TGZ_SETUP} />

        <h3 id="commands">Commands</h3>
        <p>
          <code>init DIR [--template transfer|repay]</code> writes a starter project and refuses a directory that
          already exists, so rerunning a lesson needs a new directory or the old one removed.
          <code> check FILE [--json]</code> parses and type-checks the source, <code>fmt FILE [--write]</code>
          prints or rewrites canonical layout, and <code>inspect FILE</code> lists the signed terms per action.
          <code> expand FILE --action NAME --scenario FILE</code> shows the generated Source/6 program and
          <code> simulate</code> takes the same arguments and runs actual local Core. <code>test DIR</code> runs
          <code>mori.tests.json</code> in the directory. <code>mori COMMAND --help</code> prints each command's arguments.
          Exit 0 means the requested local operation completed; exit 1 means a rejection, an unsupported execution,
          a test mismatch or a file error.
        </p>
        <p>
          Formatting changes bytes, so it changes the computed <code>sourceHash</code>; it never updates the
          claimed <code>source_hash</code> inside an intent. Source files must be UTF-8 without a byte order mark.
        </p>

        <h3 id="units">Units and identity</h3>
        <p>
          Three different things carry a name. The source alias (<code>Buyer</code>, <code>USD</code>) is a local
          label. The <code>id</code> field (<code>"Owner"</code>, <code>"A"</code>) is the economic identity that
          scenarios and Core effects use. A <code>symbol</code> such as USD is display metadata only. Asset scale 2 makes
          <code> 10.00 USD</code> equal to 1000 atoms, and every JSON amount is a canonical decimal atom string. There is
          no floating point, division, rounding or implicit conversion between assets or domains.
        </p>
        <p>
          Details are in the <a href={GUIDE}>getting-started guide</a> and the <a href={SIGNED}>signed intent walkthrough</a>,
          which covers external signatures checked by the native verifier.
        </p>

        <h3 id="open-premises">Premises and bindings that stay open</h3>
        <p>
          A local preparation rests on four external premises: the canonical intent signature, the snapshot-to-head
          relation, head extension, and the atomic ledger compare-and-consume. Four bindings are also unverified:
          the agreement identifier, the selected program, the asset scale and the authenticated predecessor. All eight
          are <StatusLabel status="Open" />. A matching hash or a passing local test closes none of them, and an empty
          premise list on a <StatusLabel status="SpecifiedOnly" /> operation means the S0 catalog does not apply, not that
          the operation is premise-free. Both lists are defined in the <a href={GUIDE}>reference guide</a>.
        </p>
      </section>

      <section id="transfer" className="guide-section" aria-labelledby="transfer-h">
        <h2 id="transfer-h">Transfer with a fee</h2>
        <p>
          <StatusLabel status="LocalS0" /> The starter agreement moves 10.00 USD from the owner to a recipient and pays
          a 0.10 USD fee to a treasury. The intent states every term the owner signs: the signer and key reference, the
          nonce, the head the operation extends, an inclusive round window of 0 to 10, a gross cap that includes the fee, the fee cap, and
          the net floor the recipient must receive. Nothing is defaulted.
        </p>
        <h3 id="transfer-derive">Derive the expected amounts first</h3>
        <ul>
          <li>Price 10.00 USD is 1000 atoms and the fee 0.10 USD is 10 atoms, so the gross debit is 1000 + 10 = 1010 atoms. The gross cap equals 1010 and the fee cap equals 10.</li>
          <li>Owner 10000 − 1010 = 8990. Recipient 0 + 1000 = 1000. Fee 0 + 10 = 10. The balances still sum to 10000.</li>
          <li>The allowance remaining 10000 − 1010 = 8990 with 1010 spent. Work remaining 10 − 1 = 9. The replay key <code>["Midnight","Owner","n1"]</code> is consumed and the head advances from <code>h0</code> to <code>h1</code>.</li>
        </ul>
        <p>
          The nonce appears as bare <code>n1</code> in the Source/6 expansion and as the domain, signer and nonce tuple in
          Core. The scenario round 1 is the observation round and is not advanced by the operation.
        </p>

        <h3 id="transfer-files">Files and commands</h3>
        <Code label="invoice.mori" kind="Complete file" text={transferSource} file="invoice.mori" />
        <p>
          The program above and the two files below are byte-for-byte what <code>mori init /tmp/invoice</code> writes (it also
          writes a README), so the downloads are for reading or comparing; you do not need them to run the lesson.
          The scenario is an untrusted stipulation of the state before the operation, with ledger identities
          <code> Owner</code>, <code>Recipient</code> and <code>Fee</code> and asset <code>A</code>. It holds the starting
          balances and allowance used in the derivation; the test file holds the expected effects and post state. Both are
          complete JSON, collapsed here; open one to copy or download it.
        </p>
        <FileDetails summary="scenario.json: complete starting-state JSON (open for Copy and Download)" label="scenario.json" kind="Complete file" text={transferScenario} file="scenario.json" />
        <FileDetails summary="mori.tests.json: complete expected-effects and post-state JSON (open for Copy and Download)" label="mori.tests.json" kind="Complete file" text={transferTests} file="mori.tests.json" />
        <Code label="Commands for the transfer lesson" kind="Commands" text={TRANSFER_COMMANDS} />
        <Code label="Output of mori test /tmp/invoice" kind="Captured output" text={TRANSFER_OUTPUT} />
        <Capture command="mori test /tmp/invoice" exit={0} />
      </section>

      <section id="repayment" className="guide-section" aria-labelledby="repayment-h">
        <h2 id="repayment-h">Loan repayment</h2>
        <p>
          <StatusLabel status="LocalS0" /> The repayment profile applies a payment to a loan under AccrualFirst:
          accrued interest is paid before principal. A funded repayment has a zero fee cap and a zero net floor, and
          a payment above the outstanding amount is rejected by Core at the Effect judgment.
        </p>
        <p>
          The obligation cell holds principal 100000, accrued 1000 and outstanding 101000 atoms.
        </p>

        <h3 id="repayment-derive">Derive the three repayment cases by hand</h3>
        <table>
          <thead>
            <tr><th scope="col">Payment</th><th scope="col">To interest</th><th scope="col">To principal</th><th scope="col">Principal after</th><th scope="col">Accrued after</th><th scope="col">Outstanding after</th></tr>
          </thead>
          <tbody>
            <tr><td>10.00 USD (1000 atoms), interest only</td><td>1000</td><td>0</td><td>100000</td><td>0</td><td>100000</td></tr>
            <tr><td>30.00 USD (3000 atoms), partial</td><td>1000</td><td>2000</td><td>98000</td><td>0</td><td>98000</td></tr>
            <tr><td>1010.00 USD (101000 atoms), full</td><td>1000</td><td>100000</td><td>0</td><td>0</td><td>0</td></tr>
          </tbody>
        </table>
        <p>
          For the shipped 30.00 USD case, Payer 200000 − 3000 = 197000 and Creditor 0 + 3000 = 3000. The allowance
          falls to 197000 with 3000 spent, work to 9, and the head moves from <code>h0</code> to <code>h1</code>. The
          obligation remains Outstanding with principal and outstanding 98000. Any payment above 101000 atoms, such as
          1010.01 USD, is rejected by Core.
        </p>

        <h3 id="repayment-files">Files and commands</h3>
        <Code label="repayment.mori" kind="Complete file" text={repaySource} file="repayment.mori" />
        <p>
          The program above and the two files below are what <code>mori init /tmp/loan --template repay</code> writes. If you
          save the downloads instead, use a directory of their own: the file names repeat those of the transfer lesson.
          Only the operative intent fields bind the request; the source constant <code>amount</code> is used by the intent, but a constant
          named principal or accrued would not constrain the scenario. To try another amount, change the <code>amount</code> constant and
          derive the new complete effects and post state before running the test. The scenario and test files are complete JSON,
          collapsed here; open one to copy or download it.
        </p>
        <FileDetails summary="scenario.json (repayment): complete starting-state JSON (open for Copy and Download)" label="scenario.json (repayment)" kind="Complete file" text={repayScenario} file="scenario.json" />
        <FileDetails summary="mori.tests.json (repayment): complete expected-effects and post-state JSON (open for Copy and Download)" label="mori.tests.json (repayment)" kind="Complete file" text={repayTests} file="mori.tests.json" />
        <Code label="Commands for the repayment lesson" kind="Commands" text={REPAY_COMMANDS} />
        <Code label="Output of mori test /tmp/loan" kind="Captured output" text={REPAY_OUTPUT} />
        <Capture command="mori test /tmp/loan" exit={0} />
        <p>
          <strong>Observation, not shown as output.</strong> On the same build, changing the <code>amount</code> constant to
          1010.00 USD and running <code>simulate</code> gave <code>PreparedUnqualified</code> with the obligation status{' '}
          <code>Settled</code> and principal, accrued and outstanding all 0; 1010.01 USD gave <code>CoreRejected</code> with code{' '}
          <code>S0_EFFECT_RANGE</code>. The <code>Settled</code> label concerns the local obligation cell and is not ledger settlement.
          The interest-only row of the table was not run.
        </p>
      </section>

      <section id="testing" className="guide-section" aria-labelledby="testing-h">
        <h2 id="testing-h">Testing</h2>
        <p>
          A test file is a closed record <code>{'{profile: "moriarty-beta-tests/1", cases: [...]}'}</code> with 1 to 64 cases.
          A case has exactly <code>name</code>, <code>source</code>, <code>action</code>, <code>scenario</code> and <code>expect</code>,
          with source and scenario paths inside the project directory. <code>expect</code> requires <code>status</code> and
          accepts <code>code</code>, <code>effects</code> and <code>post</code>, which compare the full ordered effect vector
          and the full post-state record. Asserting only the status, or status and code, checks less and does not show that the
          money moved correctly.
        </p>
        <p>
          The expected amounts have to come from your own arithmetic, as in the derivations above. Copying the effects that
          <code> simulate</code> printed into <code>expect</code> makes the test agree with the evaluator by construction, so
          it can no longer disagree with it.
        </p>

        <h3 id="wrong-expectation">A wrong money expectation fails</h3>
        <p>
          This variant of the transfer test claims the owner ends with 8991 atoms. The correct figure is 8990, so
          the test must fail and name the field. The scenario and source are unchanged: the preparation succeeds, and the
          expectation is what is wrong.
        </p>
        <p>
          Open the collapsed file below and download it. The commands assume your browser saved it to <code>~/Downloads</code> under the name shown;
          if it went elsewhere or the browser added a suffix such as <code>(1)</code>, edit the <code>DL</code> line or the file name
          first. The block also assumes the transfer lesson already created <code>/tmp/invoice</code>. The download is copied
          into a new directory as <code>mori.tests.json</code>, the name <code>test</code> reads.
        </p>
        <FileDetails summary="Negative case: transfer-wrong-expectation.mori.tests.json, complete JSON (open for Copy and Download)" label="transfer-wrong-expectation.mori.tests.json" kind="Complete file" text={wrongTests} file="transfer-wrong-expectation.mori.tests.json" />
        <Code label="Commands for the failing test" kind="Commands" text={WRONG_COMMANDS} />
        <Code label="Output of mori test /tmp/invoice-wrong" kind="Captured output" text={WRONG_OUTPUT} />
        <Capture command="mori test /tmp/invoice-wrong" exit={1} />

        <h3 id="core-rejection">An expected Core rejection passes</h3>
        <p>
          A different situation is a scenario that Core must refuse. With the owner holding 1009 atoms and an allowance of
          1009, the gross debit of 1010 exceeds both. <code>check</code> still succeeds, because Core decides funds
          and caps against the scenario. The test expects status <code>CoreRejected</code> with code <code>S0_EFFECT_RANGE</code> and passes.
          This case asserts only that status and code. It does not compare effects or a post state, so it says nothing about
          which amounts would have moved. It is a different lesson from the failing one: there the
          test disagreed with a successful preparation, and here the test agrees with a rejection.
        </p>
        <p>
          Open the two collapsed files below and download them. The commands assume they were saved to <code>~/Downloads</code> under the names shown, with
          the same <code>DL</code> and <code>/tmp/invoice</code> assumptions as above. Each is copied into a new directory under the
          name <code>test</code> expects, <code>scenario.json</code> or <code>mori.tests.json</code>.
        </p>
        <FileDetails summary="Negative case: transfer-underfunded.scenario.json, complete JSON (open for Copy and Download)" label="transfer-underfunded.scenario.json" kind="Complete file" text={underfunded} file="transfer-underfunded.scenario.json" />
        <FileDetails summary="Negative case: transfer-core-rejection.mori.tests.json, complete JSON (open for Copy and Download)" label="transfer-core-rejection.mori.tests.json" kind="Complete file" text={rejectionTests} file="transfer-core-rejection.mori.tests.json" />
        <Code label="Commands for the rejection test" kind="Commands" text={REJECT_COMMANDS} />
        <Code label="Output of mori test /tmp/invoice-underfunded" kind="Captured output" text={REJECT_OUTPUT} />
        <Capture command="mori test /tmp/invoice-underfunded" exit={0} />

        <h3 id="what-a-pass-means">What a pass means</h3>
        <p>
          Test results carry the qualification <code>local-stipulation-only</code>. A case with complete <code>effects</code> and
          <code> post</code> expectations that passes shows that the local Core, given your stipulated state, produced exactly those
          records; a case that asserts only a status and code shows that status and code. Neither
          authenticates the scenario, checks a signature, proves anything or touches a ledger, and the four premises and four
          bindings listed under <a href="#open-premises">Getting started</a> stay <StatusLabel status="Open" />.
          Signed preparation, with a native signature check before the same local Core, is described in
          the <a href={SIGNED}>signed intent walkthrough</a>.
        </p>
      </section>
    </>
  );
}

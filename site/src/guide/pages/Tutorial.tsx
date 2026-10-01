import { DocShell } from '../DocShell';
import { Answer, Code, CodeDetails, Section, SeeAlso, Steps, ThemedFigure } from '../components';
import { PACKAGE_VERSION, PAGE_HREF, PINNED, PINNED_SHORT, REPOSITORY } from '../data';
import transferSource from '../lessons/transfer.mori?raw';
import transferScenario from '../lessons/transfer.scenario.json?raw';
import repaySource from '../lessons/repay.mori?raw';
import repayScenario from '../lessons/repay.scenario.json?raw';
import transferInspect from '../tutorial/transfer-inspect.out.txt?raw';
import transferSimulate from '../tutorial/transfer-simulate.out.txt?raw';
import repaySimulate from '../tutorial/repay-simulate.out.txt?raw';

const H = PAGE_HREF.howto;
const R = PAGE_HREF.reference;
const E = PAGE_HREF.explanation;

/** The lines of `text` from the first line containing `from` through the first later line containing `to`. */
function linesFromTo(text: string, from: string, to: string): string {
  const lines = text.split('\n');
  const start = lines.findIndex((l) => l.includes(from));
  const end = lines.findIndex((l, i) => i >= start && l.includes(to));
  return start < 0 || end < 0 ? text : lines.slice(start, end + 1).join('\n');
}

/**
 * One JSON member of pretty-printed output, from the line holding `"key":` (the first one at or after the line
 * holding `after`, when given) through its closing bracket at the same indent.
 */
function jsonMember(text: string, key: string, after?: string): string {
  const lines = text.split('\n');
  const from = after ? Math.max(0, lines.findIndex((l) => l.includes(after))) : 0;
  const start = lines.findIndex((l, i) => i >= from && l.trimStart().startsWith(`"${key}":`));
  if (start < 0) return text;
  const indent = (lines[start] ?? '').length - (lines[start] ?? '').trimStart().length;
  const end = lines.findIndex((l, i) => {
    if (i <= start) return false;
    const t = l.trimStart();
    return l.length - t.length === indent && (t.startsWith('}') || t.startsWith(']'));
  });
  return lines.slice(start, end < 0 ? undefined : end + 1).join('\n');
}

/** The members of pretty-printed output from `"first":` through the end of the later member `"last":`. */
function jsonRange(text: string, first: string, last: string): string {
  const lines = text.split('\n');
  const start = lines.findIndex((l) => l.trimStart().startsWith(`"${first}":`));
  if (start < 0) return text;
  const tail = jsonMember(lines.slice(start).join('\n'), last);
  const before = lines.slice(start).findIndex((l) => l.trimStart().startsWith(`"${last}":`));
  return [...lines.slice(start, start + before), tail].join('\n');
}

/** `text` with the leading whitespace common to its non-empty lines removed, so an excerpt starts at column 0. */
function dedent(text: string): string {
  const lines = text.split('\n');
  const widths = lines.filter((l) => l.trim() !== '').map((l) => l.length - l.trimStart().length);
  const cut = widths.length ? Math.min(...widths) : 0;
  return lines.map((l) => l.slice(Math.min(cut, l.length - l.trimStart().length))).join('\n');
}

const NODE_OUTPUT = `v24.21.0`;

const CLONE = `git clone https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty
git switch --detach ${PINNED}
cd packages/moriarty-beta`;

const CLONE_OUTPUT = `Cloning into 'Moriarty'...
HEAD is now at df8b1551 Merge pull request #15 from CharlesHoskinson/feat/native-keygen-result-20261001`;

const NPM_CI_OUTPUT = `added 6 packages, and audited 7 packages in 1s

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   esbuild@0.28.2 (postinstall: node install.js)
npm warn install-scripts
npm warn install-scripts Run \`npm install-scripts ls\` to review, or \`npm install-scripts approve <pkg>\` to allow.`;

const BUILD_OUTPUT = `
> @moriarty-lang/beta@0.1.0-beta.1 build
> node build.mjs
`;

const DEFINE_MORI = `MORI_PKG="$PWD"
mori() { node "$MORI_PKG/dist/cli.js" "$@"; }`;

const HELP_OUTPUT = `Moriarty beta: init DIR [--template transfer|repay] | check FILE [--json] | fmt FILE [--write] | inspect FILE | expand/simulate FILE --action NAME --scenario FILE | test DIR | intent FILE --action NAME --scenario FILE --scheme SCHEME --public-key HEX --framing raw|midnight-sign-data --crypto-binary /ABS/BINARY [--review|--json] | verify-intent FILE --action NAME --scenario FILE --signature FILE --crypto-binary /ABS/BINARY [--review|--json] | lsp | mcp
Local financial results remain PreparedUnqualified.`;

const INIT_INVOICE_OUTPUT = `{
  "status": "Initialized",
  "directory": "/tmp/invoice",
  "qualification": "local-stipulation-only"
}`;

const LS_INVOICE_OUTPUT = `README.md
invoice.mori
mori.tests.json
scenario.json`;

const CHECK_INVOICE_OUTPUT = `AuthoringChecked Invoice
  pay: LocalS0
Local preparation remains PreparedUnqualified.`;

const SIMULATE_TRANSFER = `mori simulate /tmp/invoice/invoice.mori --action pay --scenario /tmp/invoice/scenario.json`;

const TEST_INVOICE_OUTPUT = `{
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

const INIT_LOAN_OUTPUT = `{
  "status": "Initialized",
  "directory": "/tmp/loan",
  "qualification": "local-stipulation-only"
}`;

const LS_LOAN_OUTPUT = `README.md
mori.tests.json
repayment.mori
scenario.json`;

const CHECK_LOAN_OUTPUT = `AuthoringChecked LoanAgreement
  repay_loan: LocalS0
Local preparation remains PreparedUnqualified.`;

const SIMULATE_REPAY = `mori simulate /tmp/loan/repayment.mori --action repay_loan --scenario /tmp/loan/scenario.json`;

const TEST_LOAN_OUTPUT = `{
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

const WRONG_SETUP = `mkdir /tmp/invoice-wrong
cp /tmp/invoice/invoice.mori /tmp/invoice/scenario.json /tmp/invoice-wrong/`;

const WRONG_AWK = `awk '!d && /"8990"/ {sub(/"8990"/, "\\"8991\\""); d=1} 1' /tmp/invoice/mori.tests.json > /tmp/invoice-wrong/mori.tests.json`;

const WRONG_DIFF_OUTPUT = `56c56
<               "amount": "8990"
---
>               "amount": "8991"`;

const WRONG_TEST_OUTPUT = `{
  "status": "TestsFailed",
  "qualification": "local-stipulation-only",
  "cases": [
    {
      "name": "literal fee payment",
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

const REJECT_SETUP = `mkdir /tmp/invoice-underfunded
cp /tmp/invoice/invoice.mori /tmp/invoice-underfunded/`;

const REJECT_SED = `sed 's/"10000"/"1009"/' /tmp/invoice/scenario.json > /tmp/invoice-underfunded/scenario.json`;

const REJECT_DIFF_OUTPUT = `12c12
<       "amount": "10000"
---
>       "amount": "1009"
25c25
<     "remaining": "10000",
---
>     "remaining": "1009",`;

const REJECT_TESTS = `cat > /tmp/invoice-underfunded/mori.tests.json <<'JSON'
{"profile":"moriarty-beta-tests/1","cases":[{"name":"insufficient funds rejected by Core","source":"invoice.mori","action":"pay","scenario":"scenario.json","expect":{"status":"CoreRejected","code":"S0_EFFECT_RANGE"}}]}
JSON`;

const REJECT_TEST_OUTPUT = `{
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

const NO_OUTPUT = 'These commands print nothing.';

export default function Tutorial() {
  const inspectTerms = dedent(jsonRange(transferInspect, 'gross_cap', 'net_floor'));
  const transferPost = dedent(jsonMember(transferSimulate, 'candidatePost'));
  const repayExcerpt = dedent(linesFromTo(repaySource, 'obligation Debt', 'operation: repay('));
  const repayPost = dedent(jsonMember(repaySimulate, 'candidatePost'));

  return (
    <DocShell
      page="tutorial"
      eyebrow="Tutorials"
      title="Write, check and test your first agreement"
      lead={
        <>
          In this tutorial you build the Moriarty beta command-line tool, check a transfer agreement, run it against a
          local scenario, repay a loan, and then make a test fail on purpose. It takes about 30 minutes.
        </>
      }
      groups={['Tutorial']}
      path={['overview', 'getting-started', 'transfer', 'repayment', 'testing', 'next-steps']}
    >
      <Section id="overview" title="Before you start" nav="Before you start" group="Tutorial">
        <p>
          Everything in this tutorial runs on your own computer, and nothing in it is signed, proved, sent to a ledger or
          settled (<a href={`${E}#stance`}>why the tool is limited this way</a>).
        </p>
        <ThemedFigure
          name="pipeline"
          alt=""
          caption={
            <>
              The tutorial covers the two local steps, Author and Propose. Signing is a{' '}
              <a href={`${PAGE_HREF.howto}#sign-intent`}>separate how-to guide</a>; Prove and Settle are not built.
            </>
          }
        />
        <p>You will:</p>
        <ul>
          <li>build the <code>mori</code> command from the Moriarty repository;</li>
          <li>create a transfer of 10.00 USD with a 0.10 USD fee, check it and run it;</li>
          <li>repay part of a loan and see interest paid before principal;</li>
          <li>break a test so that it fails, then write a test that expects a rejection.</li>
        </ul>
        <p>You need:</p>
        <ul>
          <li>Node.js 24 or later, with npm;</li>
          <li>git;</li>
          <li>a terminal running bash or zsh, with <code>awk</code>, <code>sed</code> and <code>diff</code>.</li>
        </ul>
        <p>
          The tutorial writes four directories: <code>/tmp/invoice</code>, <code>/tmp/loan</code>,{' '}
          <code>/tmp/invoice-wrong</code> and <code>/tmp/invoice-underfunded</code>. They must not exist yet, because the
          tool refuses to create a project in a directory that is already there. If you did this tutorial before, remove
          them first:
        </p>
        <Code kind="command" title="Remove the directories from an earlier run">
          {'rm -rf /tmp/invoice /tmp/loan /tmp/invoice-wrong /tmp/invoice-underfunded'}
        </Code>
        <p>You now know what you will build and what you need.</p>
      </Section>

      <Section id="getting-started" title="Step 1. Build the command-line tool" nav="1. Build the tool" group="Tutorial">
        <p>
          You build the tool from a clone of the whole repository and give it a short name in your shell. The package
          imports shared files from the repository's <code>experiments</code> directory, so a download of the package
          directory alone does not build.
        </p>
        <h3 id="install">Build the tool from a checkout</h3>
        <Steps>
          <li>
            <p>Check your Node.js version:</p>
            <Code kind="command" title="Check Node.js">
              {'node --version'}
            </Code>
            <p>You see a version that starts with <code>v24</code> or higher. The last two numbers can differ:</p>
            <Code kind="output">{NODE_OUTPUT}</Code>
          </li>
          <li>
            <p>Clone the repository, switch to the commit this tutorial was written against, and go to the package directory:</p>
            <Code kind="command" title="Clone the repository">
              {CLONE}
            </Code>
            <p>
              git prints the first line below, then download progress, then the commit it switched to. The repository is
              large, so the download can take a few minutes.
            </p>
            <Code kind="output">{CLONE_OUTPUT}</Code>
          </li>
          <li>
            <p>Install the build dependencies:</p>
            <Code kind="command" title="Install dependencies">
              {'npm ci'}
            </Code>
            <p>
              You see the lines below; the time can differ. The <code>install-scripts</code> warning about esbuild is
              expected, and the build in the next step works without that script.
            </p>
            <Code kind="output">{NPM_CI_OUTPUT}</Code>
          </li>
          <li>
            <p>Build the tool:</p>
            <Code kind="command" title="Build">
              {'npm run build'}
            </Code>
            <Code kind="output">{BUILD_OUTPUT}</Code>
          </li>
          <li>
            <p>
              Define <code>mori</code> as a shell function that runs the build. Keep this terminal open for the rest of the
              tutorial, because the function exists only in this shell.
            </p>
            <Code kind="command" title="Define mori">
              {DEFINE_MORI}
            </Code>
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Run the tool:</p>
            <Code kind="command" title="Show the commands">
              {'mori --help'}
            </Code>
            <p>You see one long line that lists every command, and a second line:</p>
            <Code kind="output">{HELP_OUTPUT}</Code>
          </li>
        </Steps>
        <p>You now have a working <code>mori</code> command.</p>
      </Section>

      <Section id="transfer" title="Step 2. Create, check and run a transfer" nav="2. Run a transfer" group="Tutorial">
        <p>
          The starter agreement moves 10.00 USD from a buyer to a seller and pays a 0.10 USD fee to a treasury. You
          create it, read it, check it, work out the result yourself, and then let the tool run it.
        </p>

        <h3 id="transfer-files">Create the transfer project and read the program</h3>
        <Steps>
          <li>
            <p>Create the project:</p>
            <Code kind="command" title="Create /tmp/invoice">
              {'mori init /tmp/invoice'}
            </Code>
            <Code kind="output">{INIT_INVOICE_OUTPUT}</Code>
          </li>
          <li>
            <p>List the files it wrote:</p>
            <Code kind="command" title="List the project">
              {'ls /tmp/invoice'}
            </Code>
            <Code kind="output">{LS_INVOICE_OUTPUT}</Code>
          </li>
          <li>
            <p>
              Open <code>/tmp/invoice/invoice.mori</code> in a text editor. It contains this program:
            </p>
            <Code kind="file" title="/tmp/invoice/invoice.mori">
              {transferSource}
            </Code>
            <p>Read these lines:</p>
            <ul>
              <li>
                <code>account Buyer = {'{'} domain: Preview, id: "Owner" {'}'};</code>: <code>Buyer</code> is the name in
                this file, and <code>"Owner"</code> is the id that the scenario and the results use. <code>Seller</code> is{' '}
                <code>"Recipient"</code> and <code>Treasury</code> is <code>"Fee"</code>.
              </li>
              <li>
                <code>asset USD = {'{'} … scale: 2 … {'}'};</code>: with scale 2, 10.00 USD is 1000 atoms, the
                whole units the tool counts in (<a href={`${E}#names-ids-symbols`}>why atoms and scales</a>).
              </li>
              <li>
                <code>const price: Qty&lt;USD&gt; = 10.00 USD;</code> and <code>const fee = 0.10 USD;</code>: the two
                amounts.
              </li>
              <li>
                <code>gross_cap: price + fee, fee_cap: fee, net_floor: price</code>: the buyer pays at most 10.10 USD in
                total, the fee is at most 0.10 USD, and the seller receives at least 10.00 USD.
              </li>
              <li>
                <code>operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee)</code>: the
                transfer itself.
              </li>
              <li>
                <code>action pay uses Payment;</code>: <code>pay</code> is the action name you give the tool when you run
                the agreement.
              </li>
            </ul>
          </li>
          <li>
            <p>Print the starting state the program runs against:</p>
            <Code kind="command" title="Show the scenario">
              {'cat /tmp/invoice/scenario.json'}
            </Code>
            <p>
              Look at <code>balances</code>, <code>allowance</code>, <code>replay</code> and <code>work_remaining</code>{' '}
              (<a href={`${R}#scenario-format`}>every scenario field</a>):
            </p>
            <Code kind="output">{transferScenario}</Code>
            <p>
              These fields describe the state before the transfer (
              <a href={`${E}#explicit-terms`}>why the nonce and the head are written out</a>):
            </p>
            <ul>
              <li>
                <code>head</code> names the state the transfer extends, <code>h0</code>. A run moves it to{' '}
                <code>post_head</code>, <code>h1</code>.
              </li>
              <li>
                <code>allowance</code> is how much of the owner's balance the transfer may spend.
              </li>
              <li>
                <code>replay</code> is <code>"unused"</code>: the transfer's one-time label, its nonce{' '}
                <code>"n1"</code>, has not been used yet. A run marks it as consumed.
              </li>
              <li>
                <code>work_remaining</code> is a local step budget. Each run spends one step.
              </li>
            </ul>
          </li>
        </Steps>

        <h3 id="transfer-check">Check the transfer program and list its terms</h3>
        <Steps>
          <li>
            <p>Check the program:</p>
            <Code kind="command" title="Check invoice.mori">
              {'mori check /tmp/invoice/invoice.mori'}
            </Code>
            <p>
              The check passes, and the action <code>pay</code> can run locally:
            </p>
            <Code kind="output">{CHECK_INVOICE_OUTPUT}</Code>
            <p>
              <code>pay: LocalS0</code> means the tool can run this action on your computer against a scenario. A run that
              succeeds produces <code>PreparedUnqualified</code>: a candidate result that is not signed, proved or settled (
              <a href={`${R}#support-labels`}>every support label</a>).
            </p>
          </li>
          <li>
            <p>List the terms the program states:</p>
            <Code kind="command" title="Inspect invoice.mori">
              {'mori inspect /tmp/invoice/invoice.mori'}
            </Code>
            <p>
              The output is long JSON. Find the three caps under <code>intents</code>. They are in atoms: 1010, 10 and 1000.
            </p>
            <Code kind="output" title="Part of the output: the caps">
              {inspectTerms}
            </Code>
            <CodeDetails kind="output" summary="The complete output of mori inspect">
              {transferInspect}
            </CodeDetails>
          </li>
        </Steps>

        <h3 id="transfer-derive">Work out the transfer result first</h3>
        <p>Before you run the transfer, write down your answers to these questions on paper:</p>
        <Steps>
          <li>How many atoms are 10.00 USD and 0.10 USD?</li>
          <li>How many atoms leave the owner's account in total?</li>
          <li>What are the balances of <code>Owner</code>, <code>Recipient</code> and <code>Fee</code> afterwards?</li>
          <li>What are the allowance's <code>remaining</code> and <code>spent</code> afterwards?</li>
        </Steps>
        <Answer>
          <ul>
            <li>10.00 USD is 1000 atoms and 0.10 USD is 10 atoms.</li>
            <li>The owner pays the price and the fee: 1000 + 10 = 1010 atoms. This equals the gross cap.</li>
            <li>
              <code>Owner</code> 10000 − 1010 = 8990, <code>Recipient</code> 0 + 1000 = 1000, <code>Fee</code> 0 + 10 = 10.
              The three balances still add up to 10000.
            </li>
            <li>
              The allowance has 10000 − 1010 = 8990 remaining and 1010 spent. The step budget,{' '}
              <code>work_remaining</code>, falls from 10 to 9.
            </li>
          </ul>
        </Answer>

        <h3 id="transfer-run">Run the transfer and compare</h3>
        <Steps>
          <li>
            <p>Run the action <code>pay</code> against the scenario:</p>
            <Code kind="command" title="Simulate the transfer">
              {SIMULATE_TRANSFER}
            </Code>
            <p>
              The output is long JSON with <code>"status": "PreparedUnqualified"</code> at the top. Find{' '}
              <code>candidatePost</code>, the state the tool computes after the transfer, and compare it with your
              answers. The output writes <code>workRemaining</code> for <code>work_remaining</code>, and the used nonce
              appears under <code>consumedReplay</code>:
            </p>
            <Code kind="output" title="Part of the output: candidatePost">
              {transferPost}
            </Code>
            <CodeDetails kind="output" summary="The complete output of mori simulate">
              {transferSimulate}
            </CodeDetails>
          </li>
          <li>
            <p>
              Run the test that came with the project. Its file, <code>mori.tests.json</code>, holds the same numbers you
              worked out:
            </p>
            <Code kind="command" title="Test /tmp/invoice">
              {'mori test /tmp/invoice'}
            </Code>
            <Code kind="output">{TEST_INVOICE_OUTPUT}</Code>
          </li>
        </Steps>
        <p>You now have a transfer that checks, runs and passes its test, and you predicted its result yourself.</p>
      </Section>

      <Section id="repayment" title="Step 3. Repay a loan, interest first" nav="3. Repay a loan" group="Tutorial">
        <p>
          The second starter agreement repays 30.00 USD of a loan. The payment goes to accrued interest first and then
          to principal (<a href={`${E}#patterns`}>why interest comes first</a>).
        </p>

        <h3 id="repayment-files">Create the loan project and read the program</h3>
        <Steps>
          <li>
            <p>Create the project from the repayment template:</p>
            <Code kind="command" title="Create /tmp/loan">
              {'mori init /tmp/loan --template repay'}
            </Code>
            <Code kind="output">{INIT_LOAN_OUTPUT}</Code>
          </li>
          <li>
            <p>List the files:</p>
            <Code kind="command" title="List the project">
              {'ls /tmp/loan'}
            </Code>
            <Code kind="output">{LS_LOAN_OUTPUT}</Code>
          </li>
          <li>
            <p>
              Open <code>/tmp/loan/repayment.mori</code> in your editor. These are the lines to read: the loan{' '}
              <code>Debt</code>, the payment <code>amount</code> of 30.00 USD, and the <code>repay</code> operation. The
              fee cap and the net floor are both zero.
            </p>
            <Code kind="excerpt" title="/tmp/loan/repayment.mori">
              {repayExcerpt}
            </Code>
            <p>
              The last line of the file, <code>action repay_loan uses Repayment;</code>, names the action you run.
            </p>
          </li>
          <li>
            <p>Check the program:</p>
            <Code kind="command" title="Check repayment.mori">
              {'mori check /tmp/loan/repayment.mori'}
            </Code>
            <Code kind="output">{CHECK_LOAN_OUTPUT}</Code>
          </li>
          <li>
            <p>Print the starting state:</p>
            <Code kind="command" title="Show the scenario">
              {'cat /tmp/loan/scenario.json'}
            </Code>
            <p>
              Look at the <code>obligation</code> at the end: principal 100000 atoms, accrued interest 1000 atoms,
              outstanding 101000 atoms. The payer holds 200000 atoms.
            </p>
            <Code kind="output">{repayScenario}</Code>
          </li>
        </Steps>

        <h3 id="repayment-derive">Work out the 30.00 USD repayment</h3>
        <p>Write down your answers before you run anything:</p>
        <Steps>
          <li>How many atoms is the payment?</li>
          <li>How much of it pays interest, and how much pays principal?</li>
          <li>What are the loan's principal, accrued and outstanding amounts afterwards?</li>
          <li>What are the balances of <code>Payer</code> and <code>Creditor</code> afterwards?</li>
        </Steps>
        <Answer>
          <ul>
            <li>30.00 USD at scale 2 is 3000 atoms.</li>
            <li>The first 1000 atoms pay all the accrued interest. The other 2000 atoms pay principal.</li>
            <li>Principal 100000 − 2000 = 98000, accrued 0, outstanding 98000. The loan is still outstanding.</li>
            <li>
              <code>Payer</code> 200000 − 3000 = 197000 and <code>Creditor</code> 0 + 3000 = 3000. The allowance has
              197000 remaining and 3000 spent.
            </li>
          </ul>
        </Answer>

        <h3 id="repayment-run">Run the repayment and compare</h3>
        <Steps>
          <li>
            <p>Run the action <code>repay_loan</code>:</p>
            <Code kind="command" title="Simulate the repayment">
              {SIMULATE_REPAY}
            </Code>
            <p>
              Find <code>candidatePost</code> and compare the balances, the allowance and the loan with your answers:
            </p>
            <Code kind="output" title="Part of the output: candidatePost">
              {repayPost}
            </Code>
            <CodeDetails kind="output" summary="The complete output of mori simulate">
              {repaySimulate}
            </CodeDetails>
          </li>
          <li>
            <p>Run the project's test:</p>
            <Code kind="command" title="Test /tmp/loan">
              {'mori test /tmp/loan'}
            </Code>
            <Code kind="output">{TEST_LOAN_OUTPUT}</Code>
          </li>
        </Steps>
        <p>You now have a loan repayment that you worked out by hand and that the tool confirms.</p>
      </Section>

      <Section
        id="testing"
        title="Step 4. Make a test fail, then expect a rejection"
        nav="4. Fail and reject"
        group="Tutorial"
      >
        <p>
          You make a test fail on purpose, then write a test that expects a rejection (
          <a href={`${E}#why-derive-expectations`}>why expected numbers come from your own arithmetic</a>).
        </p>

        <h3 id="wrong-expectation">Break the owner's expected balance</h3>
        <p>You copy the transfer project and change the expected owner balance from 8990 to 8991, which is wrong.</p>
        <Steps>
          <li>
            <p>Make a new directory with the same program and scenario:</p>
            <Code kind="command" title="Copy the transfer project">
              {WRONG_SETUP}
            </Code>
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>
              Write a copy of the test file with the first <code>"8990"</code>, the owner's balance after the transfer,
              changed to <code>"8991"</code>:
            </p>
            <Code kind="command" title="Write the wrong expectation">
              {WRONG_AWK}
            </Code>
            <p>This command prints nothing.</p>
          </li>
          <li>
            <p>Compare the two test files:</p>
            <Code kind="command" title="Show the change">
              {'diff /tmp/invoice/mori.tests.json /tmp/invoice-wrong/mori.tests.json'}
            </Code>
            <p>Only one line differs:</p>
            <Code kind="output">{WRONG_DIFF_OUTPUT}</Code>
          </li>
          <li>
            <p>Run the test:</p>
            <Code kind="command" title="Test /tmp/invoice-wrong">
              {'mori test /tmp/invoice-wrong'}
            </Code>
            <p>
              The test fails. The transfer itself still runs (<code>PreparedUnqualified</code>); the mismatch names the field
              and shows what you expected and what the tool computed:
            </p>
            <Code kind="output">{WRONG_TEST_OUTPUT}</Code>
          </li>
          <li>
            <p>Show the exit status of the last command:</p>
            <Code kind="command" title="Show the exit status">
              {'echo $?'}
            </Code>
            <p>A failed test exits with status 1:</p>
            <Code kind="output">{'1'}</Code>
          </li>
        </Steps>
        <p>You now have a failing test that points to the exact number that is wrong.</p>

        <h3 id="core-rejection">Expect Core to reject an underfunded payment</h3>
        <p>
          You give the owner only 1009 atoms and an allowance of 1009, one atom less than the 1010 the transfer needs.
          The program is unchanged. Core must refuse it (<a href={`${E}#three-questions`}>what Core decides</a>).
        </p>
        <Steps>
          <li>
            <p>Make a new directory with the same program:</p>
            <Code kind="command" title="Copy the program">
              {REJECT_SETUP}
            </Code>
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Write a scenario in which both the owner's balance and the allowance are 1009:</p>
            <Code kind="command" title="Write the underfunded scenario">
              {REJECT_SED}
            </Code>
            <p>This command prints nothing.</p>
          </li>
          <li>
            <p>Compare the two scenarios:</p>
            <Code kind="command" title="Show the change">
              {'diff /tmp/invoice/scenario.json /tmp/invoice-underfunded/scenario.json'}
            </Code>
            <p>Two lines differ, the balance and the allowance:</p>
            <Code kind="output">{REJECT_DIFF_OUTPUT}</Code>
          </li>
          <li>
            <p>
              Write a test that expects the status <code>CoreRejected</code> with the code <code>S0_EFFECT_RANGE</code>:
            </p>
            <Code kind="command" title="Write the rejection test">
              {REJECT_TESTS}
            </Code>
            <p>This command prints nothing.</p>
          </li>
          <li>
            <p>Run the test:</p>
            <Code kind="command" title="Test /tmp/invoice-underfunded">
              {'mori test /tmp/invoice-underfunded'}
            </Code>
            <p>
              Core rejects the transfer, the test expected exactly that, so the test passes:
            </p>
            <Code kind="output">{REJECT_TEST_OUTPUT}</Code>
          </li>
          <li>
            <p>Show the exit status:</p>
            <Code kind="command" title="Show the exit status">
              {'echo $?'}
            </Code>
            <p>A passing test exits with status 0:</p>
            <Code kind="output">{'0'}</Code>
          </li>
        </Steps>
        <p>
          This test checks only the status and the code, not which amounts would have moved. You now have one test that
          fails because its expectation is wrong, and one that passes because it expects a rejection.
        </p>
      </Section>

      <Section id="next-steps" title="Where to go next" nav="Next steps" group="Tutorial">
        <p>
          You have built the tool, run a transfer and a repayment, and written tests that fail and that expect a
          rejection. If you open a new terminal, <a href={`${H}#restore-shell`}>restore the mori function</a>. To use
          the tool from another project, <a href={`${H}#install-archive`}>pack and install a local archive</a>.
        </p>
        <SeeAlso
          title="Continue with"
          items={[
            { mode: 'howto', href: `${H}#write-test`, label: 'How to write a test with complete effects and post state' },
            { mode: 'reference', href: `${R}#test-format`, label: 'Reference: test file format' },
            { mode: 'explanation', href: `${E}#stipulation`, label: 'What a passing local run does and does not show' },
            { href: PAGE_HREF.documentation, label: 'Documentation home: every tutorial and guide' },
          ]}
        />
        <p className="doc-provenance">
          <small>
            The outputs on this page were produced with the moriarty-beta package, version {PACKAGE_VERSION}, at commit{' '}
            <a href={`${REPOSITORY}/tree/${PINNED}/packages/moriarty-beta`}>{PINNED_SHORT}</a>; your output may differ in
            whitespace.
          </small>
        </p>
      </Section>
    </DocShell>
  );
}

import type { ReactNode } from 'react';
import { DocShell } from '../DocShell';
import { Code, CodeDetails, FamilyStrip, ModeLink, Note, Section, SeeAlso, Steps, Table } from '../components';
import { BETA_PATH, FAMILIES, PAGE_HREF, SIGNED_INTENT, blob, type FamilyId } from '../data';
import { EXAMPLES, RUNS } from '../howto/runs';

const T = PAGE_HREF.tutorial;
const R = PAGE_HREF.reference;
const E = PAGE_HREF.explanation;

/** A recorded run: the command exactly as it ran on the pinned build, and what it printed. */
function run(key: string) {
  const r = RUNS[key];
  if (!r) throw new Error(`No recorded run named ${key}`);
  return r;
}

function lines(text: string): string[] {
  return text.split('\n');
}
function head(text: string, n: number): string {
  return lines(text).slice(0, n).join('\n');
}
function tail(text: string, n: number): string {
  return lines(text).slice(-n).join('\n');
}
/** Lines `from`..`to` of a file, 1-based and inclusive. */
function fileLines(text: string, from: number, to: number): string {
  return lines(text).slice(from - 1, to).join('\n');
}

/** The command block for a recorded run. */
function Cmd({ k, title }: { k: string; title: ReactNode }) {
  const c = run(k).cmd;
  if (c === null) throw new Error(`Run ${k} has no command`);
  return (
    <Code kind="command" title={title}>
      {c}
    </Code>
  );
}

/** The output block for a recorded run, optionally cut to its first or last lines. */
function Out({ k, first, last, title }: { k: string; first?: number; last?: number; title?: ReactNode }) {
  let text = run(k).out.replace(/^\n+/, '');
  if (first) text = head(text, first);
  if (last) text = tail(text, last);
  return (
    <Code kind="output" title={title}>
      {text}
    </Code>
  );
}

/** Goal and starting state: the two sentences every guide opens with. */
function Intro({ goal, start }: { goal: ReactNode; start: ReactNode }) {
  return (
    <>
      <p>
        <strong>Goal.</strong> {goal}
      </p>
      <p>
        <strong>Starting state.</strong> {start}
      </p>
    </>
  );
}

/** The closing check of a guide. */
function Worked({ children }: { children: ReactNode }) {
  return (
    <p>
      <strong>Check that it worked.</strong> {children}
    </p>
  );
}

function Background({ href, children }: { href: string; children: ReactNode }) {
  return (
    <p>
      Background:{' '}
      <ModeLink mode="explanation" href={href}>
        {children}
      </ModeLink>
    </p>
  );
}

const NO_OUTPUT = 'This prints nothing.';

const HAVE_MORI = (
  <>
    You have the <code>mori</code> function and the <code>MORI_PKG</code> variable from{' '}
    <a href={`${T}#install`}>the tutorial's install step</a>.
  </>
);

const NVIM_CONFIG = `vim.bo.filetype = 'moriarty'
dofile('/path/to/editor/nvim/moriarty.lua').start({'node', '/path/to/dist/cli.js', 'lsp'})`;

const SIGNATURE_SHAPE = `{"statement":{"profile":"moriarty-signed-intent/1","...":"replace with the full returned statement"},"signatureHex":"replace with 128 lowercase hex characters"}`;

/** One family recipe. Everything printed comes from the recorded runs; `excerpt` is a line range of the shipped file. */
interface Recipe {
  id: FamilyId;
  /** The example file's stem in examples/. */
  stem: string;
  goal: ReactNode;
  excerpt: [number, number];
  /** What the excerpt declares, in one or two sentences. */
  declare: ReactNode;
  /** Operation calls and their named arguments, as `check --json` lists them in operationSchemas. */
  calls: string[];
  /** What the sed command changes. */
  edit: ReactNode;
  /** The mistake whose real rejection is shown, and how to fix it. */
  mistake: ReactNode;
  fix: ReactNode;
}

const RECIPES: Recipe[] = [
  {
    id: 'amm',
    stem: 'amm',
    goal: 'Write a swap of a fixed input for at least a named output, plus liquidity mint and redeem, and check it.',
    excerpt: [7, 11],
    declare: (
      <>
        Declare both assets, then a <code>pool</code> that lists them. The swap intent names the pool, the owner, the
        exact input, the output asset, a net floor in the output asset and a fee cap.
      </>
    ),
    calls: [
      'amm.swap_exact_input(pool, owner, input, output_asset, net_floor, fee_cap)',
      'amm.mint(pool, owner, amounts, minimum_share_atoms)',
      'amm.redeem(pool, owner, share_atoms)',
    ],
    edit: (
      <>
        Change the swap to 50.00 USD in for at least 0.450 GOLD out:
      </>
    ),
    mistake: (
      <>
        a net floor written in the input asset (<code>net_floor: 0.45 USD</code>)
      </>
    ),
    fix: <>Write the net floor in the output asset, here <code>GOLD</code>.</>,
  },
  {
    id: 'lending',
    stem: 'lending',
    goal: 'Write a borrow, a roll to a later round and a liquidation, and check them.',
    excerpt: [9, 11],
    declare: (
      <>
        Declare an <code>obligation</code> in the debt asset. The borrow intent names it, the debtor, the creditor, the
        principal and the collateral.
      </>
    ),
    calls: [
      'lending.originate(obligation, debtor, creditor, principal, collateral)',
      'lending.roll_forward(obligation, to_round)',
      'lending.liquidate(obligation)',
    ],
    edit: <>Borrow 250.00 USD against 0.500 GOLD:</>,
    mistake: (
      <>
        a roll without its target round (<code>lending.roll_forward(obligation: Loan)</code>)
      </>
    ),
    fix: (
      <>
        Add the missing argument, here <code>to_round: 200</code>. Every named argument is required.
      </>
    ),
  },
  {
    id: 'stablecoins',
    stem: 'stablecoin',
    goal: 'Write a stablecoin mint against backing, a redemption and an emergency settlement, and check them.',
    excerpt: [9, 11],
    declare: (
      <>
        Declare an <code>instrument</code> with its own asset and its backing asset. The mint intent names the
        instrument, the owner, the supply and the backing.
      </>
    ),
    calls: [
      'stablecoin.mint(instrument, owner, supply, backing)',
      'stablecoin.redeem(instrument, owner, burn, minimum_backing)',
      'stablecoin.emergency_settle(instrument, owner, claim)',
    ],
    edit: <>Mint 50.00 USD of supply against 0.100 GOLD of backing:</>,
    mistake: (
      <>
        backing written in the supply asset (<code>backing: 0.10 USD</code>)
      </>
    ),
    fix: (
      <>
        Write the backing in the instrument's backing asset, here <code>GOLD</code>.
      </>
    ),
  },
  {
    id: 'options',
    stem: 'derivatives',
    goal: 'Write the fixing, exercise and settlement of a call option, and check them.',
    excerpt: [9, 16],
    declare: (
      <>
        Declare the option as an <code>instrument</code> (underlying, settlement asset, strike, exercise round,
        collateral) and the price it fixes against as an <code>observation</code>. Each intent then names the
        instrument; exercise and settlement also name the holder.
      </>
    ),
    calls: [
      'option.fix(instrument, observation)',
      'option.exercise(instrument, holder)',
      'option.settle(instrument, holder, payoff)',
    ],
    edit: <>Settle with a 50.00 USD payoff:</>,
    mistake: (
      <>
        an observation passed where an account is expected (<code>holder: Fixing</code>)
      </>
    ),
    fix: (
      <>
        Pass an <code>account</code> as the holder. The position in the message points at the declaration of the
        name you passed.
      </>
    ),
  },
  {
    id: 'oracles',
    stem: 'oracle',
    goal: 'Write an agreement that selects a named oracle observation, and check it.',
    excerpt: [9, 11],
    declare: (
      <>
        Declare an <code>observation</code> with its unit, source, round, maximum age and finality. The intent selects
        it. The file publishes no price.
      </>
    ),
    calls: ['oracle.select(observation)'],
    edit: <>Allow an observation at most 3 rounds old:</>,
    mistake: (
      <>
        an account passed where an observation is expected (<code>oracle.select(observation: Alice)</code>)
      </>
    ),
    fix: (
      <>
        Pass the name of an <code>observation</code> declaration.
      </>
    ),
  },
  {
    id: 'governance',
    stem: 'governance',
    goal: 'Write a queued, executed and vetoed policy change, and check it.',
    excerpt: [9, 11],
    declare: (
      <>
        Declare a <code>policy</code> with its id and epoch. The queue intent names the policy and the next epoch; the
        execute and veto intents name only the policy.
      </>
    ),
    calls: ['governance.queue(policy, next_epoch)', 'governance.execute(policy)', 'governance.veto(policy)'],
    edit: <>Queue the change for epoch 5:</>,
    mistake: (
      <>
        a quantity where an epoch number is expected (<code>next_epoch: 5.00 USD</code>)
      </>
    ),
    fix: <>Write the epoch as a bare unsigned number.</>,
  },
  {
    id: 'bridges',
    stem: 'bridge',
    goal: 'Write a bridge escrow, the matching claim on the other domain and a recovery, and check them.',
    excerpt: [9, 17],
    declare: (
      <>
        Declare the foreign domain, an account and an asset on it. Escrow and recovery spend the local asset; the
        claim spends the foreign one. All three share one <code>claim_id</code>.
      </>
    ),
    calls: [
      'bridge.escrow(owner, amount, destination, claim_id)',
      'bridge.claim(owner, amount, source, claim_id)',
      'bridge.recover(owner, amount, claim_id)',
    ],
    edit: <>Rename the claim in all three intents:</>,
    mistake: (
      <>
        an escrow amount in the foreign asset (<code>amount: 100.00 WrappedUSD</code>)
      </>
    ),
    fix: (
      <>
        Escrow from the owner's own domain: the owner and the amount must share a domain. Only the destination is
        foreign.
      </>
    ),
  },
  {
    id: 'staking',
    stem: 'staking',
    goal: 'Write a staking deposit, a reward, a slash, an unbond and a withdrawal, and check them.',
    excerpt: [9, 11],
    declare: (
      <>
        Declare a <code>share_class</code> with its backing asset. The deposit intent names the owner, the share class
        and the backing paid in.
      </>
    ),
    calls: [
      'staking.deposit(owner, shares, backing)',
      'staking.reward(shares, amount)',
      'staking.slash(shares, amount)',
      'staking.unbond(owner, shares, share_atoms)',
      'staking.withdraw(owner, shares, share_atoms, minimum_backing)',
    ],
    edit: <>Deposit 50.00 USD of backing:</>,
    mistake: (
      <>
        a quantity where a share count is expected (<code>share_atoms: 1.00 USD</code>)
      </>
    ),
    fix: <>Write share counts as bare unsigned numbers of share atoms.</>,
  },
];

function familyOf(id: FamilyId) {
  const f = FAMILIES.find((x) => x.id === id);
  if (!f) throw new Error(`Unknown family ${id}`);
  return f;
}

function RecipeSection({ r }: { r: Recipe }) {
  const fam = familyOf(r.id);
  const file = `${r.stem}.mori`;
  const source = EXAMPLES[file];
  if (!source) throw new Error(`No example ${file}`);
  const k = (step: string) => `fam-${r.id}-${step}`;
  return (
    <Section id={`write-${r.id}`} title={`How to write ${articleFor(fam.name)} agreement`} nav={fam.name} group="Author">
      <FamilyStrip id={r.id} here="howto" />
      <Intro goal={r.goal} start={HAVE_MORI} />
      <p>The beta checks this kind of agreement and does not run it.</p>
      <Steps>
        <li>
          <p>
            Start from the shipped example in a new directory:
          </p>
          <Cmd k={k('copy')} title={`Copy ${file}`} />
          <p>{NO_OUTPUT}</p>
        </li>
        <li>
          <p>{r.declare}</p>
          <Code kind="excerpt" title={`${file}, lines ${r.excerpt[0]} to ${r.excerpt[1]}`} href={blob(`${BETA_PATH}/${fam.file}`)}>
            {fileLines(source, r.excerpt[0], r.excerpt[1])}
          </Code>
          <p>
            The operations and their named arguments are:
          </p>
          <ul>
            {r.calls.map((c) => (
              <li key={c}>
                <code>{c}</code>
              </li>
            ))}
          </ul>
        </li>
        <li>
          <p>{r.edit}</p>
          <Cmd k={k('edit')} title={`Edit ${file}`} />
          <p>{NO_OUTPUT}</p>
        </li>
        <li>
          <p>Check the file:</p>
          <Cmd k={k('check')} title={`Check ${file}`} />
          <p>
            If <code>check</code> reports an error, read its code and position. For example, {r.mistake} gives:
          </p>
          <Out k={k('bad')} />
          <p>{r.fix}</p>
        </li>
      </Steps>
      <Worked>
        <code>check</code> exits 0 and lists every action as specified only:
      </Worked>
      <Out k={k('check')} />
      <p>
        Exact argument kinds: <a href={`${R}#family-${r.id}`}>Reference: {fam.name}</a>.
      </p>
    </Section>
  );
}

function articleFor(name: string): string {
  const lower: Record<string, string> = {
    AMM: 'an AMM',
    Lending: 'a lending',
    Stablecoins: 'a stablecoin',
    'Options and derivatives': 'an option',
    Oracles: 'an oracle',
    Governance: 'a governance',
    Bridges: 'a bridge',
    Staking: 'a staking',
  };
  return lower[name] ?? `a ${name.toLowerCase()}`;
}

/** Actions per shipped example, as `mori check` lists them. */
const FAMILY_ACTIONS: Record<FamilyId, string> = {
  amm: 'swap, mint_lp, redeem_lp',
  lending: 'borrow, roll, liquidate',
  stablecoins: 'mint, redeem, emergency',
  options: 'fix, exercise, settle',
  oracles: 'read_fixing',
  governance: 'queue, execute, veto',
  bridges: 'escrow, claim, recover',
  staking: 'deposit, reward, slash, unbond, withdraw',
};

/** A status cell: the status name in code, any exit note in plain text, a dash for none. */
function StatusCell({ text }: { text: string }) {
  const [name, rest] = text.split(' (');
  return (
    <span style={{ whiteSpace: 'nowrap' }}>
      {name === '—' ? '—' : <code style={{ wordBreak: 'normal' }}>{name}</code>}
      {rest ? ` (${rest}` : null}
    </span>
  );
}

interface Symptom {
  see: ReactNode;
  status: string;
  cause: ReactNode;
  fix: ReactNode;
}

const SYMPTOMS: Symptom[] = [
  {
    see: <code>mori: command not found</code>,
    status: '—',
    cause: 'A new shell, so the mori function is gone.',
    fix: <a href="#restore-shell">Restore the function</a>,
  },
  {
    see: <code>mori: ENOENT: no such file or directory, stat '…/mori.tests.json'</code>,
    status: '— (exit 1)',
    cause: (
      <>
        <code>mori test</code> was given a directory without a test file.
      </>
    ),
    fix: (
      <>
        Pass the project directory that holds <code>mori.tests.json</code>.
      </>
    ),
  },
  {
    see: <code>mori: simulate requires --action and --scenario</code>,
    status: '— (exit 1)',
    cause: 'An argument is missing.',
    fix: (
      <>
        Add both. Arguments for every command: <a href={`${R}#cli`}>Reference: command line</a>.
      </>
    ),
  },
  {
    see: <code>BETA_INIT_EXISTS</code>,
    status: 'FormationRejected',
    cause: (
      <>
        <code>init</code> refuses a directory that already exists.
      </>
    ),
    fix: 'Choose a new directory, or remove the old one.',
  },
  {
    see: <code>BETA_PROFILE at 1:9: Expected profile "moriarty-beta/1"</code>,
    status: 'AuthoringRejected',
    cause: 'The first line names another profile.',
    fix: (
      <>
        Start the file with <code>profile "moriarty-beta/1";</code>
      </>
    ),
  },
  {
    see: <code>BETA_QUANTITY_SEPARATOR: Quantity and asset need whitespace or a comment</code>,
    status: 'AuthoringRejected',
    cause: (
      <>
        An amount written as <code>10.00USD</code>.
      </>
    ),
    fix: (
      <>
        Write <code>10.00 USD</code>.
      </>
    ),
  },
  {
    see: <code>BETA_PRECISION: Quantity has more decimals than asset scale</code>,
    status: 'AuthoringRejected',
    cause: (
      <>
        <code>10.001 USD</code> for an asset of scale 2.
      </>
    ),
    fix: "Use at most as many decimals as the asset's scale.",
  },
  {
    see: <code>BETA_MISSING_FIELD: Missing field fee_cap</code>,
    status: 'AuthoringRejected',
    cause: 'A required intent field or named argument is left out.',
    fix: (
      <>
        Add it. Empty terms are written out too: <code>observations: []</code>, <code>delegation: None</code>.
      </>
    ),
  },
  {
    see: <code>BETA_REFERENCE: Unknown or forward reference Byer</code>,
    status: 'AuthoringRejected',
    cause: 'A misspelt name, or a name used before its declaration.',
    fix: 'Fix the spelling, or move the declaration above its first use.',
  },
  {
    see: (
      <>
        <code>BETA_ASSET_MISMATCH</code>, for example <code>Expected Qty&lt;GOLD&gt;, received Qty&lt;USD&gt;</code>
      </>
    ),
    status: 'AuthoringRejected',
    cause: 'Two different assets added together, or an amount in the wrong asset.',
    fix: 'Use the asset the field or operation expects. Nothing converts between assets.',
  },
  {
    see: <code>BETA_DOMAIN_MISMATCH: Transfer accounts have different domains</code>,
    status: 'AuthoringRejected',
    cause: 'Accounts or assets from two domains in one operation.',
    fix: 'Declare them in the same domain. Only a bridge names a foreign domain.',
  },
  {
    see: (
      <>
        <code>BETA_TYPE</code>, for example <code>Expected account reference</code>
      </>
    ),
    status: 'AuthoringRejected',
    cause: 'A declaration of the wrong kind passed to a named argument.',
    fix: 'Pass a declaration of the kind the argument names.',
  },
  {
    see: <code>BETA_ACTION_UNKNOWN: Unknown action payy</code>,
    status: 'AuthoringRejected',
    cause: (
      <>
        <code>--action</code> names no action in the file.
      </>
    ),
    fix: (
      <>
        Use an action name that <code>mori check</code> lists.
      </>
    ),
  },
  {
    see: <code>BETA_SCENARIO_IDENTITY: Expected asset ID A; got B (/asset)</code>,
    status: 'FormationRejected',
    cause: "The scenario's domain or asset id differs from the agreement's.",
    fix: (
      <>
        Use the economic ids (the <code>id</code> fields), not the source names.{' '}
        <a href={`${R}#scenario-format`}>Reference: scenario format</a>.
      </>
    ),
  },
  {
    see: (
      <>
        <code>BETA_PROFILE_UNSUPPORTED</code> from <code>simulate</code> or <code>expand</code>
      </>
    ),
    status: 'Unsupported',
    cause: 'The action is specified only. The beta checks it and does not run it.',
    fix: (
      <>
        Nothing to fix. Use <code>mori check</code> for these files (<a href="#check-family">how to check an example</a>).
      </>
    ),
  },
  {
    see: (
      <>
        <code>check</code> passes, then <code>S0_INTENT_SCOPE</code>
      </>
    ),
    status: 'CoreRejected',
    cause: 'The operation does not fit the signed bounds (gross cap, fee cap, net floor), or the round is outside the window.',
    fix: (
      <>
        Fix the bounds (<a href="#cap-with-fee">how to cap a payment</a>) or the scenario's <code>round</code>.
      </>
    ),
  },
  {
    see: <code>S0_EFFECT_RANGE</code>,
    status: 'CoreRejected',
    cause: "The owner's balance is below the gross debit, or a repayment is above the amount outstanding.",
    fix: 'Lower the amount or raise the scenario balance.',
  },
  {
    see: <code>S0_AUTH_SCOPE</code>,
    status: 'CoreRejected',
    cause: (
      <>
        The scenario's <code>allowance.remaining</code> is below the gross debit, or <code>work_remaining</code> is 0.
      </>
    ),
    fix: 'Raise the allowance or the work counter in the scenario.',
  },
  {
    see: <code>S0_HISTORY_REPLAY</code>,
    status: 'CoreRejected',
    cause: (
      <>
        The scenario says the nonce is already used (<code>"replay": "consumed"</code>).
      </>
    ),
    fix: (
      <>
        Set <code>"replay": "unused"</code>, or use a new nonce.
      </>
    ),
  },
  {
    see: <code>S0_HISTORY_STALE</code>,
    status: 'CoreRejected',
    cause: (
      <>
        The scenario's <code>head</code> differs from the intent's <code>pre_head</code>.
      </>
    ),
    fix: 'Make them the same.',
  },
  {
    see: (
      <>
        <code>TestsFailed</code> with a <code>pointer</code>, <code>expected</code> and <code>actual</code>
      </>
    ),
    status: 'TestsFailed (exit 1)',
    cause: 'The run differs from what the test expects at that JSON pointer.',
    fix: 'Redo the arithmetic for that value. Change the expectation only if your arithmetic was wrong.',
  },
  {
    see: <code>BETA_SIGNATURE_SOURCE_MISMATCH: Signed statement differs from current source</code>,
    status: 'FormationRejected',
    cause: 'The source bytes changed after the intent was prepared, even by a comment or by formatting.',
    fix: 'Prepare the intent again and get a new signature.',
  },
  {
    see: <code>SignatureRejected</code>,
    status: 'SignatureRejected (exit 1)',
    cause: 'The signature does not verify for the frame, key, scheme and framing.',
    fix: 'Sign the decoded signing-message bytes, once, with the stated key and scheme.',
  },
  {
    see: <code>BETA_CRYPTO_BINARY_UNAVAILABLE</code>,
    status: 'FormationRejected (exit 2)',
    cause: (
      <>
        <code>--crypto-binary</code> does not name a usable verifier.
      </>
    ),
    fix: 'Pass the absolute path of the built verifier.',
  },
];

export default function HowTo() {
  return (
    <DocShell
      page="howto"
      eyebrow="How-to guides"
      title="How-to guides"
      lead={
        <>
          Short recipes for tasks you already have in mind. Each one assumes you have done{' '}
          <a href={T}>the tutorial</a>. Results stay local and unqualified (
          <a href={`${E}#stance`}>why</a>).
        </>
      }
      groups={['Set up', 'Test and diagnose', 'Author', 'Check the examples', 'Sign']}
      path={[]}
    >
      {/* ------------------------------------------------------------------ Set up */}
      <Section id="install-archive" title="How to install the tool from a local archive" nav="Install from an archive" group="Set up">
        <Intro
          goal={<>Install the beta into a separate project, so <code>mori</code> runs without the checkout's build.</>}
          start={
            <>
              You have the built checkout from <a href={`${T}#install`}>the tutorial's install step</a>, and your shell
              is in <code>Moriarty/packages/moriarty-beta</code>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Pack the package. Packing runs the build again first.</p>
            <Cmd k="ia-pack" title="Pack the archive" />
            <Out k="ia-pack" />
          </li>
          <li>
            <p>Install the archive into a new project directory:</p>
            <Cmd k="ia-install" title="Install into /tmp/howto-project" />
            <p>The time can differ:</p>
            <Out k="ia-install" />
          </li>
          <li>
            <p>Define <code>mori</code> from the installed package:</p>
            <Cmd k="ia-define" title="Define mori and show the commands" />
            <Out k="ia-define" />
          </li>
        </Steps>
        <p>
          This is a local archive install. No release on the npm registry exists. Signing also needs the native
          verifier, which the archive does not contain (<a href="#sign-intent">how to verify a signed intent</a>).
        </p>
        <p>
          If you opened a new shell, <a href="#restore-shell">restore the function</a> first.
        </p>
        <Worked>Create and test a starter project from any directory:</Worked>
        <Cmd k="ia-check" title="Create and test a project" />
        <Out k="ia-check" />
      </Section>

      <Section id="restore-shell" title="How to restore the mori function in a new shell" nav="Restore mori" group="Set up">
        <Intro
          goal={<>Get <code>mori</code> back after you open a new terminal.</>}
          start={
            <>
              You built the tool in <a href={`${T}#install`}>the tutorial</a>, and the new shell says{' '}
              <code>mori: command not found</code>.
            </>
          }
        />
        <Steps>
          <li>
            <p>
              If you use the checkout, go to the package directory from the directory that holds{' '}
              <code>Moriarty</code>, and define the function again:
            </p>
            <Cmd k="rs-checkout" title="Restore mori from the checkout" />
          </li>
          <li>
            <p>
              If you installed from an archive (<a href="#install-archive">how to install from an archive</a>), define it
              from that project instead:
            </p>
            <Cmd k="rs-archive" title="Restore mori from the archive project" />
          </li>
        </Steps>
        <Worked>Both print the command list:</Worked>
        <Out k="rs-checkout" />
      </Section>

      <Section id="editor" title="How to set up the editor and language server" nav="Editor and language server" group="Set up">
        <Intro
          goal={
            <>
              Get diagnostics, completion, hover, go to definition, document symbols and formatting for{' '}
              <code>.mori</code> files in VS Code or Neovim.
            </>
          }
          start={
            <>
              You have the built checkout from <a href={`${T}#install`}>the tutorial</a>, your shell is in{' '}
              <code>Moriarty/packages/moriarty-beta</code>, and Node 24 or later is on your <code>PATH</code>.
            </>
          }
        />
        <h3 id="editor-vscode">VS Code</h3>
        <Steps>
          <li>
            <p>
              Build the extension package. The last command downloads the <code>vsce</code> packager from the npm
              registry.
            </p>
            <Cmd k="ed-vsix" title="Package the VS Code extension" />
            <p>The output ends with the path of the package in your checkout:</p>
            <Out k="ed-vsix" last={1} />
          </li>
          <li>
            <p>
              In VS Code, install the <code>.vsix</code> file from the Extensions view (Install from VSIX).
            </p>
          </li>
          <li>
            <p>
              If VS Code cannot start the server, set <code>moriarty.nodePath</code> to the absolute path of a Node 24
              executable.
            </p>
          </li>
        </Steps>
        <p>
          The package carries its own copy of the CLI as its server. Its tests cover packaging and the language server
          protocol, not activation inside VS Code.
        </p>
        <h3 id="editor-neovim">Neovim 0.11 or later</h3>
        <Steps>
          <li>
            <p>
              Load <code>editor/nvim/moriarty.lua</code> from your project configuration, with absolute paths:
            </p>
            <Code kind="excerpt" title="Neovim project configuration" href={blob(`${BETA_PATH}/editor/README.md`)}>
              {NVIM_CONFIG}
            </Code>
          </li>
          <li>
            <p>Run the headless client test from the package directory:</p>
            <Cmd k="ed-nvim" title="Run the Neovim smoke test" />
          </li>
        </Steps>
        <p>
          Other clients can run <code>mori lsp</code>: stdio, full document sync, UTF-16 positions. Activation in other
          clients is not tested.
        </p>
        <Worked>The Neovim test prints one line:</Worked>
        <Out k="ed-nvim" />
        <p>
          Canonical text: <a href={blob(`${BETA_PATH}/editor/README.md`)}>editor/README.md</a>.
        </p>
      </Section>

      {/* ------------------------------------------------------------------ Test and diagnose */}
      <Section id="write-test" title="How to write a test with complete effects and post state" nav="Write a complete test" group="Test and diagnose">
        <Intro
          goal="Add a test case that fixes every effect and the whole post state for a new scenario."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>tutorial step 2</a>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="wt-init" title="Create /tmp/howto-test" />
            <Out k="wt-init" />
          </li>
          <li>
            <p>
              Write a second scenario. Here the recipient already holds 500 atoms and the fee account 20:
            </p>
            <Cmd k="wt-scenario" title="Write scenario-held.json" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>
              Work out the result by hand, from the agreement and the scenario. The owner pays 1000 atoms plus a
              10-atom fee, so the effects are the same as in the first case. The post balances are 10000 − 1010 = 8990
              for the owner, 500 + 1000 = 1500 for the recipient and 20 + 10 = 30 for the fee account.
            </p>
          </li>
          <li>
            <p>
              Add a case with those numbers. This copies the first case, then sets its name, its scenario and the three
              post balances:
            </p>
            <Cmd k="wt-case" title="Add the case to mori.tests.json" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Run the tests:</p>
            <Cmd k="wt-run" title="Run the tests" />
          </li>
        </Steps>
        <p>
          If a case fails, its <code>mismatches</code> name the first differing value by JSON pointer, with expected and
          actual values. Check your arithmetic for that value (<a href="#diagnose">how to diagnose a failure</a>).
        </p>
        <p>
          Take expected numbers from your own arithmetic, never from <code>simulate</code> output. A case that asserts
          only <code>status</code> checks much less. The full case format:{' '}
          <a href={`${R}#test-format`}>Reference: test file format</a>.
        </p>
        <Worked>
          Both cases pass and <code>mori test</code> exits 0:
        </Worked>
        <Out k="wt-run" />
        <Background href={`${E}#why-derive-expectations`}>Why expected amounts must be derived independently</Background>
      </Section>

      <Section id="test-rejection" title="How to test that Core rejects a request" nav="Test a rejection" group="Test and diagnose">
        <Intro
          goal="Add a test case that passes only when Core rejects the request with a given code."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>tutorial step 2</a>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="tr-init" title="Create /tmp/howto-reject" />
          </li>
          <li>
            <p>Write a scenario that Core must reject. Here the nonce is already used:</p>
            <Cmd k="tr-scenario" title="Write scenario-replayed.json" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Find the rejection code:</p>
            <Cmd k="tr-find" title="Show the rejection" />
            <Out k="tr-find" />
          </li>
          <li>
            <p>
              Add a case that expects that status and code:
            </p>
            <Cmd k="tr-case" title="Add the case to mori.tests.json" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Run the tests:</p>
            <Cmd k="tr-run" title="Run the tests" />
          </li>
        </Steps>
        <p>
          A case with <code>expect: {'{ status, code }'}</code> asserts the status and the code only. A rejection
          publishes no effects and no post state.
        </p>
        <p>If the code in the case is wrong, the case fails and names the code it got:</p>
        <Out k="tr-wrong" title="Part of the output, with S0_EFFECT_RANGE expected" />
        <p>
          Other scenario changes and the codes they produce are in the <a href="#diagnose">diagnosis table</a>.
        </p>
        <Worked>
          Both cases pass and <code>mori test</code> exits 0:
        </Worked>
        <Out k="tr-run" />
        <Background href={`${E}#stipulation`}>What a passing local run does and does not show</Background>
      </Section>

      <Section id="change-amount" title="How to change the repayment amount" nav="Change the repayment" group="Test and diagnose">
        <Intro
          goal="Repay a different amount and keep the test exact."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the repayment project from{' '}
              <a href={`${T}#repayment`}>tutorial step 3</a>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="ca-init" title="Create /tmp/howto-loan" />
          </li>
          <li>
            <p>
              Change the amount constant from 30.00 USD to 20.00 USD. The intent's <code>gross_cap</code> is{' '}
              <code>amount</code>, so it follows.
            </p>
            <Cmd k="ca-edit" title="Edit repayment.mori" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Run the tests to see which expectations are now wrong:</p>
            <Cmd k="ca-fail" title="Run the tests" />
            <p>
              The case fails, and <code>mori test</code> exits 1. It reports the first difference in the effects and the
              first in the post state:
            </p>
            <Out k="ca-fail" />
          </li>
          <li>
            <p>
              Work out the new numbers. 20.00 USD is 2000 atoms. Interest comes first: all 1000 accrued atoms, then 1000
              of principal. Principal becomes 99000, accrued 0 and outstanding 99000. The payer goes from 200000 to
              198000 and the creditor receives 2000. The allowance has 198000 remaining and 2000 spent.
            </p>
          </li>
          <li>
            <p>
              Put those numbers in <code>mori.tests.json</code>. Every 3000 becomes 2000, 98000 becomes 99000 and 197000
              becomes 198000:
            </p>
            <Cmd k="ca-update" title="Edit mori.tests.json" />
            <p>{NO_OUTPUT}</p>
          </li>
        </Steps>
        <p>
          If the amount is above the 101000 atoms outstanding (for example 1010.01 USD), Core rejects it. Test that with a
          rejection case (<a href="#test-rejection">how to test a rejection</a>):
        </p>
        <Out k="ca-over" title="Part of the simulate output for 1010.01 USD" />
        <Worked>Run the tests again. The case passes:</Worked>
        <Cmd k="ca-run" title="Run the tests" />
        <Out k="ca-run" />
        <Background href={`${E}#patterns`}>Why repayment pays interest first</Background>
      </Section>

      <Section id="diagnose" title="How to diagnose a failed check, run or test" nav="Diagnose a failure" group="Test and diagnose">
        <Intro
          goal="Find the cause of a rejection or a failed test from what the tool prints."
          start={
            <>
              You ran <code>mori check</code>, <code>simulate</code>, <code>test</code> or <code>verify-intent</code>{' '}
              as in <a href={`${T}#testing`}>the tutorial</a>, and it did not give the result you expected.
            </>
          }
        />
        <Steps>
          <li>
            <p>
              Read the result <code>status</code> and the first diagnostic <code>code</code>. For <code>check</code>{' '}
              they are on the first two lines. For <code>simulate</code> a Core rejection's code is under{' '}
              <code>result.rejection</code>.
            </p>
          </li>
          <li>
            <p>Find the code or message in the table and apply the fix.</p>
          </li>
          <li>
            <p>Run the same command again.</p>
          </li>
        </Steps>
        <Table caption="Symptoms, the status they come with, likely causes and fixes">
          <thead>
            <tr>
              <th scope="col">What you see</th>
              <th scope="col">Status</th>
              <th scope="col">Likely cause</th>
              <th scope="col">Fix</th>
            </tr>
          </thead>
          <tbody>
            {SYMPTOMS.map((s, i) => (
              <tr key={i}>
                <td>{s.see}</td>
                <td>
                  <StatusCell text={s.status} />
                </td>
                <td>{s.cause}</td>
                <td>{s.fix}</td>
              </tr>
            ))}
          </tbody>
        </Table>
        <p>
          A passing <code>check</code> does not mean a scenario will succeed: Core decides caps, funds, authority, expiry
          and replay for each scenario. Every status and code:{' '}
          <a href={`${R}#results`}>Reference: results</a> and <a href={`${R}#diagnostics`}>Reference: diagnostic codes</a>.
        </p>
        <Worked>
          The command gives the status you expected, or the test prints <code>"status": "TestsPassed"</code> and exits
          0.
        </Worked>
      </Section>

      {/* ------------------------------------------------------------------ Author */}
      <Section id="cap-with-fee" title="How to cap a payment so the fee counts toward the gross debit" nav="Cap a payment with a fee" group="Author">
        <Intro
          goal="Bound a transfer so the owner never pays more than price plus fee, and the recipient never gets less than the price."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>tutorial step 2</a>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="cf-init" title="Create /tmp/howto-cap" />
          </li>
          <li>
            <p>
              Declare the price and the fee as constants, and write the three bounds from them:{' '}
              <code>gross_cap: price + fee</code>, <code>fee_cap: fee</code>, <code>net_floor: price</code>. The starter
              already does this. Change the price to 25.00 USD and the fee to 0.25 USD:
            </p>
            <Cmd k="cf-edit" title="Edit the price and fee, then show the lines" />
            <Out k="cf-edit" />
          </li>
          <li>
            <p>Check the file:</p>
            <Cmd k="cf-check" title="Check invoice.mori" />
            <Out k="cf-check" />
          </li>
          <li>
            <p>Confirm the bounds in atoms:</p>
            <Cmd k="cf-inspect" title="Show the three bounds" />
            <Out k="cf-inspect" />
          </li>
        </Steps>
        <p>
          If the gross cap leaves out the fee, <code>check</code> still passes, but Core rejects the payment:
        </p>
        <Cmd k="cf-nofee" title="Try a gross cap without the fee" />
        <Out k="cf-nofee" />
        <p>
          A fee cap below the fee, or a net floor above the price, is rejected with the same code. So always run the
          agreement, not only <code>check</code> it.
        </p>
        <Worked>Run it against the starter scenario. The result starts:</Worked>
        <Cmd k="cf-sim" title="Run the payment" />
        <Out k="cf-sim" />
        <Background href={`${E}#explicit-terms`}>Why bounds are signed instead of results</Background>
      </Section>

      <Section id="format-and-inspect" title="How to format a file and list its signed terms" nav="Format and inspect" group="Author">
        <Intro
          goal="Lay out a source file in the standard format, and list the intent terms it carries."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>tutorial step 2</a>.
            </>
          }
        />
        <Note title="Formatting changes the source hash" tone="caution">
          <p>
            <code>fmt --write</code> changes the file's bytes, so its SHA-256 changes. A signed intent binds that
            SHA-256, so format before you prepare an intent for signing, never after.
          </p>
        </Note>
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="fi-init" title="Create /tmp/howto-fmt" />
          </li>
          <li>
            <p>
              Preview the formatted file. Without <code>--write</code>, <code>fmt</code> prints the whole file and changes
              nothing:
            </p>
            <Cmd k="fi-preview" title="Preview the format" />
            <Out k="fi-preview" />
          </li>
          <li>
            <p>Write the format in place, and compare the source hash before and after:</p>
            <Cmd k="fi-write" title="Format in place" />
            <Out k="fi-write" />
            <p>
              <code>fmt --write</code> prints nothing. Comments and spelling are kept.
            </p>
          </li>
          <li>
            <p>
              List each intent's terms. <code>inspect</code> prints them as JSON; this prints one line per term, with
              amounts in atoms:
            </p>
            <Cmd k="fi-terms" title="List the intent terms" />
          </li>
        </Steps>
        <Worked>Every term of the intent is listed, including the empty ones:</Worked>
        <Out k="fi-terms" />
        <p>
          <code>inspect</code> also prints identity claims, action support and open premises:{' '}
          <a href={`${R}#cli`}>Reference: command line</a>.
        </p>
      </Section>

      {RECIPES.map((r) => (
        <RecipeSection key={r.id} r={r} />
      ))}

      {/* ------------------------------------------------------------------ Check the examples */}
      <Section id="check-family" title="How to check an example from the eight DeFi families" nav="Check a DeFi example" group="Check the examples">
        <Intro goal="Check one of the shipped DeFi examples and read the result." start={HAVE_MORI} />
        <p>
          These eight examples are specified only: the beta checks their structure, names and quantities, and does not
          run them (<a href={`${R}#support-labels`}>Reference: support labels</a>).
        </p>
        <Steps>
          <li>
            <p>Check the file. Here, the AMM example:</p>
            <Cmd k="cfam-check" title="Check amm.mori" />
          </li>
          <li>
            <p>
              For machine-readable output, add <code>--json</code>. Its <code>operationSchemas</code> list each
              operation's named arguments.
            </p>
          </li>
          <li>
            <p>
              If you try to run one of these actions, the tool refuses and does not read the scenario:
            </p>
            <Cmd k="cfam-sim" title="Try to run the swap" />
            <Out k="cfam-sim" />
            <p>
              <code>simulate</code> exits 1 with <code>Unsupported</code>.
            </p>
          </li>
        </Steps>
        <p>
          If you installed from an archive, <code>MORI_PKG</code> already points at the installed package, and the
          same commands work (<a href="#install-archive">how to install from an archive</a>).
        </p>
        <Table caption="The eight example files, under packages/moriarty-beta/examples">
          <thead>
            <tr>
              <th scope="col">Family</th>
              <th scope="col">File</th>
              <th scope="col">Actions</th>
              <th scope="col">How to write one</th>
            </tr>
          </thead>
          <tbody>
            {FAMILIES.map((f) => (
              <tr key={f.id}>
                <td>{f.name}</td>
                <td>
                  <a href={blob(`${BETA_PATH}/${f.file}`)}>
                    <code>{f.file}</code>
                  </a>
                </td>
                <td>
                  <code>{FAMILY_ACTIONS[f.id]}</code>
                </td>
                <td>
                  <a href={`#write-${f.id}`}>{`How to write ${articleFor(f.name)} agreement`}</a>
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
        <CodeDetails
          summary="Check all eight files at once"
          kind="command"
          title="Check every example"
        >
          {run('cfam-all').cmd ?? ''}
        </CodeDetails>
        <Worked>
          <code>check</code> exits 0, and each action says it is specified only:
        </Worked>
        <Out k="cfam-check" />
      </Section>

      {/* ------------------------------------------------------------------ Sign */}
      <Section id="sign-intent" title="How to verify a signed intent locally" nav="Verify a signed intent" group="Sign">
        <Intro
          goal={
            <>
              Prepare an owner intent for an external signer, then check the signature with the native verifier before
              local preparation.
            </>
          }
          start={
            <>
              You have the built checkout from <a href={`${T}#install`}>the tutorial</a>, your shell is in{' '}
              <code>Moriarty/packages/moriarty-beta</code>, and Rust with Cargo is installed.
            </>
          }
        />
        <p>
          The signature is over a canonical intent frame. The frame binds the SHA-256 of the exact source bytes, the
          agreement and action, the asset, the signer and key reference, the bounds and the operation. It is not a
          signature over the source file itself. A success is <code>SignedPreparedUnqualified</code>: the key signed
          those terms. Whether the key may act for the account, the current state, a proof and ledger acceptance stay
          open (<a href={`${E}#three-questions`}>what is and is not signed</a>).
        </p>

        <h3 id="sign-intent-install">Install the package and build the verifier</h3>
        <Steps>
          <li>
            <p>
              Build the native verifier from the pinned lockfile, and keep its absolute path. The first build compiles
              many crates and needs network access to fetch them; add <code>--offline</code> only when they are already
              cached.
            </p>
            <Cmd k="si-verifier" title="Build the verifier" />
            <p>The last line of the build reads (the time differs):</p>
            <Out k="si-verifier" />
            <p>
              To use the binary elsewhere, copy it to a trusted absolute path and set <code>MORI_VERIFIER</code> to it. The
              tool never searches <code>PATH</code> for it.
            </p>
          </li>
          <li>
            <p>Pack the package, install it into a new project, and define <code>mori</code> from it:</p>
            <Cmd k="si-install" title="Install into /tmp/howto-signed" />
            <Out k="si-install" />
          </li>
        </Steps>

        <h3 id="sign-intent-example">Verify a ready public example</h3>
        <Steps>
          <li>
            <p>Run the example's own test:</p>
            <Cmd k="si-test" title="Test the Schnorr example" />
            <Out k="si-test" />
          </li>
          <li>
            <p>Verify its signature and review the result:</p>
            <Cmd k="si-review" title="Verify the Schnorr example" />
            <p>
              It exits 0. The review starts with the signature check and the source match:
            </p>
            <Out k="si-review" first={11} title="Start of the review" />
            <CodeDetails summary="The complete review" kind="output">
              {run('si-review').out}
            </CodeDetails>
          </li>
          <li>
            <p>
              For the other two examples, use directory <code>transfer-ecdsa-wallet</code> with action <code>pay</code>,
              or <code>repay-ecdsa-raw</code> with action <code>repay_loan</code>.
            </p>
          </li>
        </Steps>
        <p>
          The example keys are throwaway test identities. They do not authorize the source's <code>Owner</code> or{' '}
          <code>Payer</code> account.
        </p>

        <h3 id="sign-intent-own">Prepare and verify your own external signature</h3>
        <Steps>
          <li>
            <p>Copy a program and a scenario into the project:</p>
            <Cmd k="si-copy" title="Copy the program and scenario" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>
              Set your public key in its raw encoding, as lowercase hex without <code>0x</code>: 32 bytes for BIP340
              Schnorr, a compressed 33-byte key for secp256k1 ECDSA. To follow along without a key of your own, use the
              example's key:
            </p>
            <Cmd k="si-key" title="Set the public key" />
          </li>
          <li>
            <p>
              Prepare the intent. The result holds the full <code>statement</code>, <code>frame_hex</code>,{' '}
              <code>frame_sha256</code> and <code>signing_message_hex</code>; this prints the status, the frame's
              SHA-256 and the signing message length in bytes:
            </p>
            <Cmd k="si-intent" title="Prepare the intent" />
            <Out k="si-intent" />
            <p>
              <code>OwnerIntentPrepared</code> proves no one holds the key. Read the statement before you sign.
            </p>
          </li>
          <li>
            <p>Decode the signing message into the exact bytes to sign:</p>
            <Cmd k="si-export" title="Write signing-message.bin" />
            <Out k="si-export" />
            <p>
              With <code>raw</code> framing the signing message is the frame itself, so its SHA-256 equals{' '}
              <code>frame_sha256</code>. With <code>midnight-sign-data</code> it is{' '}
              <code>midnight_signed_message:&lt;frame byte length&gt;:</code> followed by the frame; that prefix is
              already in <code>signing_message_hex</code>, so do not add it again.
            </p>
          </li>
          <li>
            <p>
              Sign <code>signing-message.bin</code> with your own signer, and keep the 64-byte raw signature as 128
              lowercase hex characters in <code>SIGNATURE_HEX</code>. This step needs your signing system; the tool takes
              no private key. Sign the bytes, not the hex text or <code>frame_sha256</code>. Both supported schemes hash
              the message with SHA-256 internally, so do not hash it first. Convert a DER ECDSA signature to raw{' '}
              <code>r || s</code> (low S) outside this tool.
            </p>
            <p>
              To follow along with the example's key, use the example's signature. It fits because the statement you
              prepared is the same as the example's:
            </p>
            <Cmd k="si-sample-sig" title="Use the example signature" />
          </li>
          <li>
            <p>
              Write <code>signature.json</code> with exactly the returned statement and the signature. Its shape is:
            </p>
            <Code kind="excerpt" title="Shape only, not a valid file">
              {SIGNATURE_SHAPE}
            </Code>
            <Cmd k="si-sigfile" title="Write signature.json" />
            <p>{NO_OUTPUT}</p>
          </li>
          <li>
            <p>Verify, then print the main fields of the result:</p>
            <Cmd k="si-verify" title="Verify your signature" />
          </li>
        </Steps>
        <p>
          If you change the source after preparing the intent, even by a comment or by formatting, the SHA-256 no longer
          matches and verification stops before the signature check:
        </p>
        <Cmd k="si-edited" title="Edit the source, then verify again" />
        <Out k="si-edited" />

        <h3 id="sign-intent-results">Read the signed result</h3>
        <Table caption="Results of intent and verify-intent, with exit codes">
          <thead>
            <tr>
              <th scope="col">Result</th>
              <th scope="col">Exit</th>
              <th scope="col">Meaning</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <code>OwnerIntentPrepared</code>
              </td>
              <td>0</td>
              <td>Signable bytes prepared. Possession of the key is not checked.</td>
            </tr>
            <tr>
              <td>
                <code>SignedPreparedUnqualified</code>
              </td>
              <td>0</td>
              <td>Signature valid, and a local Core candidate prepared.</td>
            </tr>
            <tr>
              <td>
                <code>SignatureRejected</code>
              </td>
              <td>1</td>
              <td>The signature checked false. Local preparation is skipped.</td>
            </tr>
            <tr>
              <td>
                <code>SignedCoreRejected</code>
              </td>
              <td>1</td>
              <td>Signature valid, and local Core rejected the scenario.</td>
            </tr>
            <tr>
              <td>Formation, source or schema rejection</td>
              <td>1</td>
              <td>No signed preparation.</td>
            </tr>
            <tr>
              <td>Verifier binary, timeout or transport failure</td>
              <td>2</td>
              <td>Signature validity unknown. Nothing is accepted as a fallback.</td>
            </tr>
          </tbody>
        </Table>
        <p>
          The scenario (balances, allowance and work counters, proposed effects, post state) is not signed. A changed
          scenario can keep the signature valid and give a different candidate or a Core rejection.
        </p>
        <Worked>
          The verification exits 0, the source matches, and the open items stay open:
        </Worked>
        <Out k="si-verify" />
        <p>
          Canonical text: <a href={SIGNED_INTENT}>SIGNED-INTENT.md</a>.
        </p>
        <SeeAlso
          items={[
            { mode: 'explanation', href: `${E}#three-questions`, label: 'Authorization, acceptance and coordination (Explanation)' },
            { mode: 'reference', href: `${R}#premises-bindings`, label: 'Reference: local premises and unverified bindings' },
          ]}
        />
      </Section>
    </DocShell>
  );
}

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

/** A term's first use on this page, linked to its glossary entry. */
function Term({ slug, children }: { slug: string; children: ReactNode }) {
  return <a href={`${R}#term-${slug}`}>{children}</a>;
}

/** Only the lines of a recorded output that contain one of `keep`, in their original order. */
function Pick({ k, keep, title }: { k: string; keep: string[]; title: ReactNode }) {
  const text = lines(run(k).out)
    .filter((l) => keep.some((x) => l.includes(x)))
    .join('\n');
  return (
    <Code kind="output" title={title}>
      {text}
    </Code>
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
    goal: 'Write a swap that pays a fixed input amount and requires a minimum output amount. Add a liquidity mint and a redeem.',
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
    fix: <>Write the net floor in the output asset. In this example the output asset is <code>GOLD</code>.</>,
  },
  {
    id: 'lending',
    stem: 'lending',
    goal: 'Describe a collateralised loan from origination to liquidation, including a roll to a later round.',
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
        Add the missing argument. In this example it is <code>to_round: 200</code>. Every named argument is
        required.
      </>
    ),
  },
  {
    id: 'stablecoins',
    stem: 'stablecoin',
    goal: 'Mint stablecoin supply against backing in another asset. Then write its redemption and an emergency settlement.',
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
        Write the backing in the instrument's backing asset. In this example the backing asset is{' '}
        <code>GOLD</code>.
      </>
    ),
  },
  {
    id: 'options',
    stem: 'derivatives',
    goal: 'Define a call option, then the intents that fix it against an observation, exercise it and settle it with a stated payoff.',
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
        Pass an <code>account</code> as the holder. The position in the message, <code>10:22</code>, is the
        declaration of the name you passed, not the call.
      </>
    ),
  },
  {
    id: 'oracles',
    stem: 'oracle',
    goal: 'Make an intent select one named observation. The example selects a USD-per-GOLD fixing with a maximum age.',
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
    goal: 'Queue a policy change for a later epoch, and write the intents that execute or veto it.',
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
    fix: <>Write the epoch as a whole number with no asset, such as <code>5</code>.</>,
  },
  {
    id: 'bridges',
    stem: 'bridge',
    goal: 'Lock funds in escrow on the local domain and claim them on a foreign domain. Add a recovery under the same claim id.',
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
    goal: 'Cover the life of a staking position, from the deposit through rewards and slashing to unbond and withdrawal.',
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
    fix: <>Write a share count as a whole number of share atoms with no asset, such as <code>100</code>.</>,
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
    <Section id={`write-${r.id}`} title={`How to write ${fam.agreement}`} nav={fam.name} group="Author">
      <FamilyStrip id={r.id} here="howto" />
      <Intro goal={r.goal} start={HAVE_MORI} />
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
          <Out k={k('check')} />
          <p>
            If <code>check</code> reports an error, read its code and position. For example, {r.mistake} gives:
          </p>
          <Out k={k('bad')} />
          <p>{r.fix}</p>
        </li>
      </Steps>
      <Worked>
        <code>mori check {file}</code> exits 0 and lists every action as <code>SpecifiedOnly</code>.
      </Worked>
      <p>
        Exact argument kinds: <a href={`${R}#family-${r.id}`}>Reference: {fam.name}</a>.
      </p>
    </Section>
  );
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

interface Symptom {
  see: ReactNode;
  /** The result status and exit code, only where they tell the symptoms apart. */
  result?: ReactNode;
  cause: ReactNode;
  fix: ReactNode;
}

const SYMPTOMS: Symptom[] = [
  {
    see: <code>mori: command not found</code>,
    cause: 'You opened a new shell. A shell function exists only in the shell where you defined it.',
    fix: <a href="#restore-shell">Restore the function</a>,
  },
  {
    see: <code>mori: ENOENT: no such file or directory, stat '…/mori.tests.json'</code>,
    result: <>Exit code 1</>,
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
    result: <>Exit code 1</>,
    cause: 'An argument is missing.',
    fix: (
      <>
        Add both. Arguments for every command: <a href={`${R}#cli`}>Reference: command line</a>.
      </>
    ),
  },
  {
    see: <code>BETA_INIT_EXISTS</code>,
    cause: (
      <>
        <code>init</code> refuses a directory that already exists.
      </>
    ),
    fix: 'Choose a new directory, or remove the old one.',
  },
  {
    see: <code>BETA_PROFILE at 1:9: Expected profile "moriarty-beta/1"</code>,
    cause: 'The first line names another profile.',
    fix: (
      <>
        Start the file with <code>profile "moriarty-beta/1";</code>
      </>
    ),
  },
  {
    see: <code>BETA_QUANTITY_SEPARATOR: Quantity and asset need whitespace or a comment</code>,
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
    cause: (
      <>
        <code>10.001 USD</code> for an asset of scale 2.
      </>
    ),
    fix: "Use at most as many decimals as the asset's scale.",
  },
  {
    see: <code>BETA_MISSING_FIELD: Missing field fee_cap</code>,
    cause: 'A required intent field or named argument is left out.',
    fix: (
      <>
        Add it. Empty terms are written out too: <code>observations: []</code>, <code>delegation: None</code>.
      </>
    ),
  },
  {
    see: <code>BETA_REFERENCE: Unknown or forward reference Byer</code>,
    cause: 'A misspelt name, or a name used before its declaration.',
    fix: 'Fix the spelling, or move the declaration above its first use.',
  },
  {
    see: (
      <>
        <code>BETA_ASSET_MISMATCH</code>, for example <code>Expected Qty&lt;GOLD&gt;, received Qty&lt;USD&gt;</code>
      </>
    ),
    cause: 'Two different assets added together, or an amount in the wrong asset.',
    fix: 'Use the asset the field or operation expects. Nothing converts between assets.',
  },
  {
    see: <code>BETA_DOMAIN_MISMATCH: Transfer accounts have different domains</code>,
    cause: (
      <>
        Accounts or assets from two <Term slug="domain">domains</Term> in one operation.
      </>
    ),
    fix: 'Declare them in the same domain. Only a bridge names a foreign domain.',
  },
  {
    see: (
      <>
        <code>BETA_TYPE</code>, for example <code>Expected account reference</code>
      </>
    ),
    cause: 'A declaration of the wrong kind passed to a named argument.',
    fix: 'Pass a declaration of the kind the argument names.',
  },
  {
    see: <code>BETA_ACTION_UNKNOWN: Unknown action payy</code>,
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
    cause: (
      <>
        The action is <code>SpecifiedOnly</code>. The beta checks it and does not run it.
      </>
    ),
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
    cause: 'The operation does not fit the signed bounds (gross cap, fee cap, net floor), or the round is outside the window.',
    fix: (
      <>
        Fix the bounds (<a href="#cap-with-fee">how to cap a payment</a>) or the scenario's <code>round</code>.
      </>
    ),
  },
  {
    see: <code>S0_EFFECT_RANGE</code>,
    cause: "The owner's balance is below the gross debit, or a repayment is above the amount outstanding.",
    fix: 'Lower the amount or raise the scenario balance.',
  },
  {
    see: <code>S0_AUTH_SCOPE</code>,
    cause: (
      <>
        The scenario's <code>allowance.remaining</code> is below the gross debit, or <code>work_remaining</code> (the{' '}
        <Term slug="work-counter">work counter</Term>) is 0.
      </>
    ),
    fix: 'Raise the allowance or the work counter in the scenario.',
  },
  {
    see: <code>S0_HISTORY_REPLAY</code>,
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
    cause: (
      <>
        The scenario's <code>head</code> differs from the intent's <code>pre_head</code> (
        <Term slug="head">head</Term>).
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
    result: <>Exit code 1</>,
    cause: 'The run differs from what the test expects at that JSON pointer.',
    fix: 'Redo the arithmetic for that value. Change the expectation only if your arithmetic was wrong.',
  },
  {
    see: <code>BETA_SIGNATURE_SOURCE_MISMATCH: Signed statement differs from current source</code>,
    cause: 'The source bytes changed after the intent was prepared, even by a comment or by formatting.',
    fix: 'Prepare the intent again and get a new signature.',
  },
  {
    see: <code>SignatureRejected</code>,
    result: <>Exit code 1</>,
    cause: 'The signature does not verify for the frame, key, scheme and framing.',
    fix: 'Sign the decoded signing-message bytes, once, with the stated key and scheme.',
  },
  {
    see: <code>BETA_CRYPTO_BINARY_UNAVAILABLE</code>,
    result: (
      <>
        <code>FormationRejected</code>, exit code 2
      </>
    ),
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
      title="How to do common tasks with mori"
      lead={
        <>
          Recipes for installing from an archive, setting up an editor, writing and diagnosing tests, writing the eight
          DeFi examples and verifying a signed <Term slug="intent">intent</Term>. Each recipe assumes you have finished{' '}
          <a href={T}>the tutorial</a>. The eight DeFi examples are{' '}
          <Term slug="specified-only">specified only</Term>: <code>mori check</code> accepts them, and{' '}
          <code>mori simulate</code> does not run them.
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
          The package is not published on the npm registry, so this recipe installs a local <code>.tgz</code> archive.
          The archive does not contain the native verifier that signature checks need (
          <a href="#sign-intent">how to verify a signed intent</a>).
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
            <Out k="rs-checkout" />
          </li>
          <li>
            <p>
              If you installed from an archive (<a href="#install-archive">how to install from an archive</a>), define it
              from that project instead:
            </p>
            <Cmd k="rs-archive" title="Restore mori from the archive project" />
          </li>
        </Steps>
        <Worked>
          Either way, <code>mori --help</code> prints the command list that starts{' '}
          <code>Moriarty beta: init DIR</code>.
        </Worked>
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
          The extension package includes its own copy of the CLI and runs it as the language server. Its tests cover
          packaging and the language server protocol. They do not test activation inside VS Code.
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
            <Out k="ed-nvim" />
          </li>
        </Steps>
        <p>
          Other editors can run <code>mori lsp</code>. It uses stdio, full document sync and UTF-16 positions.
          Activation in other editors is not tested.
        </p>
        <Worked>
          The Neovim test prints one line that starts <code>Neovim Moriarty stdio LSP smoke passed</code>.
        </Worked>
        <p>
          Full instructions: <a href={blob(`${BETA_PATH}/editor/README.md`)}>editor/README.md</a>.
        </p>
      </Section>

      {/* ------------------------------------------------------------------ Test and diagnose */}
      <Section id="write-test" title="How to write a test with complete effects and post state" nav="Write a complete test" group="Test and diagnose">
        <Intro
          goal="Add a test case that fixes every effect and the whole post state for a new scenario."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>the tutorial's transfer step</a>.
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
              Write a second <Term slug="scenario">scenario</Term>. In it the recipient already holds 500{' '}
              <Term slug="atom">atoms</Term> and the fee account holds 20:
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
            <Out k="wt-run" />
            <p>
              <code>PreparedUnqualified</code> means that <Term slug="core">Core</Term>, the reference evaluator, prepared a <Term slug="candidate">candidate</Term> result
              that no authentication, proof or ledger has qualified. The <Term slug="qualification">qualification</Term>{' '}
              field, <code>local-stipulation-only</code>, says what the result rests on.
            </p>
          </li>
        </Steps>
        <p>
          If a case fails, its <code>mismatches</code> name the first differing value by JSON pointer, with expected and
          actual values. Check your arithmetic for that value (<a href="#diagnose">how to diagnose a failure</a>).
        </p>
        <p>
          Take expected numbers from your own arithmetic, never from <code>simulate</code> output. A case that asserts
          only <code>status</code> does not check any amount. The full case format:{' '}
          <a href={`${R}#test-format`}>Reference: test file format</a>.
        </p>
        <Worked>
          <code>mori test /tmp/howto-test</code> prints <code>"status": "TestsPassed"</code>, both cases show{' '}
          <code>"passed": true</code>, and the command exits 0.
        </Worked>
        <Background href={`${E}#why-derive-expectations`}>Why expected amounts must be derived independently</Background>
      </Section>

      <Section id="test-rejection" title="How to test that Core rejects a request" nav="Test a rejection" group="Test and diagnose">
        <Intro
          goal={
            <>
              Add a test case that passes only when Core rejects the request with a given code.
            </>
          }
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>the tutorial's transfer step</a>.
            </>
          }
        />
        <Steps>
          <li>
            <p>Create the project and go into it:</p>
            <Cmd k="tr-init" title="Create /tmp/howto-reject" />
          </li>
          <li>
            <p>
              Write a scenario that Core must reject. In this scenario the nonce is already used, so the{' '}
              <Term slug="replay-key">replay key</Term> is spent:
            </p>
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
            <Out k="tr-run" />
          </li>
        </Steps>
        <p>
          A case with <code>expect: {'{ status, code }'}</code> asserts the status and the code only. A rejected run has
          no effects and no post state.
        </p>
        <p>
          If the code in the case is wrong, the case fails and names the code it got. To see this, change the expected
          code to <code>S0_EFFECT_RANGE</code> and run the tests again:
        </p>
        <Cmd k="tr-wrong" title="Expect the wrong code and run the tests" />
        <Out k="tr-wrong" title="Part of the output" />
        <p>
          With the wrong code, <code>mori test</code> exits 1. Put the correct expectation back:
        </p>
        <Cmd k="tr-restore" title="Restore mori.tests.json" />
        <p>{NO_OUTPUT}</p>
        <p>
          Other scenario changes and the codes they produce are in the <a href="#diagnose">diagnosis table</a>.
        </p>
        <Worked>
          <code>mori test /tmp/howto-reject</code> prints <code>"status": "TestsPassed"</code>. The second case shows{' '}
          <code>"status": "CoreRejected"</code> and <code>"code": "S0_HISTORY_REPLAY"</code>, and the command exits 0.
        </Worked>
        <Background href={`${E}#stipulation`}>What a passing local run does and does not show</Background>
      </Section>

      <Section id="change-amount" title="How to change the repayment amount" nav="Change the repayment" group="Test and diagnose">
        <Intro
          goal="Repay a different amount and keep the test exact."
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the repayment project from{' '}
              <a href={`${T}#repayment`}>the tutorial's repayment step</a>.
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
          A payment of exactly the 101000 atoms outstanding (1010.00 USD) repays the loan in full. The candidate result
          sets principal, accrued and outstanding to 0 and the obligation's status to <code>Settled</code>:
        </p>
        <Cmd k="ca-full" title="Run a repayment of 1010.00 USD" />
        <Out k="ca-full" title="The obligation in the effects" />
        <p>
          If the amount is above the 101000 atoms outstanding, Core rejects it at the effect judgment with{' '}
          <code>S0_EFFECT_RANGE</code>. To see the rejection, write a copy that repays 1010.01 USD and run it:
        </p>
        <Cmd k="ca-over" title="Run a repayment of 1010.01 USD" />
        <Out k="ca-over" />
        <p>
          Test that with a rejection case (<a href="#test-rejection">how to test a rejection</a>).
        </p>
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
        <Table caption="Messages and result statuses, with cause and fix">
          <thead>
            <tr>
              <th scope="col">What you see</th>
              <th scope="col">Likely cause and fix</th>
            </tr>
          </thead>
          <tbody>
            {SYMPTOMS.map((s, i) => (
              <tr key={i}>
                <td>
                  {s.see}
                  {s.result ? <span style={{ display: 'block', marginTop: '0.25rem' }}>{s.result}</span> : null}
                </td>
                <td>
                  {s.cause}
                  <span style={{ display: 'block', marginTop: '0.25rem' }}>
                    <strong>Fix:</strong> {s.fix}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
        <p>
          <code>check</code> reads only the program. A scenario can still be rejected, because Core tests the program
          against the scenario (<a href={`${R}#results`}>Reference: results and judgment order</a>). Every code:{' '}
          <a href={`${R}#diagnostics`}>Reference: diagnostic codes</a>.
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
              <a href={`${T}#transfer`}>the tutorial's transfer step</a>.
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
          A fee cap below the fee, or a net floor above the price, is rejected with the same code. Run{' '}
          <code>simulate</code> as well as <code>check</code>: only <code>simulate</code> shows these rejections.
        </p>
        <Worked>Run it against the starter scenario. The result starts:</Worked>
        <Cmd k="cf-sim" title="Run the payment" />
        <Out k="cf-sim" />
        <Background href={`${E}#explicit-terms`}>Why bounds are signed instead of results</Background>
      </Section>

      <Section id="format-and-inspect" title="How to format a file and list its signed terms" nav="Format and inspect" group="Author">
        <Intro
          goal={
            <>
              Lay out a source file in the standard format, and list the terms of each intent in it.
            </>
          }
          start={
            <>
              {HAVE_MORI} The steps use a fresh copy of the transfer project from{' '}
              <a href={`${T}#transfer`}>the tutorial's transfer step</a>.
            </>
          }
        />
        <Note title="Formatting changes the source hash" tone="caution">
          <p>
            <code>fmt --write</code> changes the file's bytes, so its SHA-256 changes. A signed intent contains that
            SHA-256. Format the file before you prepare an intent for signing. Do not format it after.
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
            <Out k="fi-terms" />
          </li>
        </Steps>
        <Worked>
          The term list has one line for each of the intent's 20 fields, including empty ones such as{' '}
          <code>observations</code> and <code>delegation</code>.
        </Worked>
        <p>
          <code>inspect</code> also prints identity claims, action support and the premises that a local run does not
          establish:{' '}
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
          These eight examples are <code>SpecifiedOnly</code>: the beta checks their structure, names and quantities,
          and does not run them (<a href={`${R}#support-labels`}>Reference: support labels</a>).
        </p>
        <Steps>
          <li>
            <p>Check the file. Here, the AMM example:</p>
            <Cmd k="cfam-check" title="Check amm.mori" />
            <Out k="cfam-check" />
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
          If you installed from an archive, <code>MORI_PKG</code> already holds the path of the installed package, and
          the same commands work (<a href="#install-archive">how to install from an archive</a>).
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
                  <a href={`#write-${f.id}`}>{`How to write ${f.agreement}`}</a>
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
          <code>check</code> exits 0 for each file and lists each action as{' '}
          <code>SpecifiedOnly (execution unsupported; financial relations open)</code>.
        </Worked>
      </Section>

      {/* ------------------------------------------------------------------ Sign */}
      <Section id="sign-intent" title="How to verify a signed intent locally" nav="Verify a signed intent" group="Sign">
        <Intro
          goal={
            <>
              Prepare an owner intent for an external signer, sign it with your own key, and check the signature with
              the native verifier.
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
          The signature covers the intent frame. The frame contains the source file's SHA-256 and the owner's terms. It
          does not contain the scenario (
          <ModeLink mode="explanation" href={`${E}#three-questions`}>
            Explanation: authorization, acceptance and coordination
          </ModeLink>
          ).
        </p>

        <h3 id="sign-intent-install">Install the package and build the verifier</h3>
        <Steps>
          <li>
            <p>
              Build the native verifier with the dependency versions in the lockfile (<code>--locked</code>), and keep
              its absolute path. The first build compiles many crates and downloads them, so it needs network access.
              Add <code>--offline</code> only if the crates are already in your Cargo cache.
            </p>
            <Cmd k="si-verifier" title="Build the verifier" />
            <p>The last line of the build reads (the time differs):</p>
            <Out k="si-verifier" />
            <p>
              The verifier checks signatures with the Schnorr and ECDSA code of <code>midnight-base-crypto</code>, from
              the <code>midnight-ledger</code> repository at the revision that <code>Cargo.toml</code> names.
            </p>
            <p>
              <code>mori</code> runs the binary at the path you give and does not check its hash. The binary alone
              decides whether a signature is valid, so use one that you built from this lockfile, and keep it in a
              directory that only you can write to. To use it from another place, copy it there and set{' '}
              <code>MORI_VERIFIER</code> to the new absolute path. The tool never searches <code>PATH</code> for it.
            </p>
          </li>
          <li>
            <p>Pack the package, install it into a new project, and define <code>mori</code> from it:</p>
            <Cmd k="si-install" title="Install into /tmp/howto-signed" />
            <Out k="si-install" />
          </li>
        </Steps>

        <h3 id="sign-intent-example">Verify a ready public example</h3>
        <p>
          The package ships three signed examples. Their keys are test keys. A passing verification with them shows that
          the tooling works. It does not show that anyone authorized the source's <code>Owner</code> or{' '}
          <code>Payer</code> account.
        </p>
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
              For the other two examples, use directory <code>transfer-ecdsa-wallet</code> (ECDSA,{' '}
              <code>midnight-sign-data</code> framing) with action <code>pay</code>, or <code>repay-ecdsa-raw</code>{' '}
              (ECDSA, <code>raw</code> framing) with action <code>repay_loan</code>.
            </p>
          </li>
        </Steps>

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
              Choose the framing, which says which bytes are signed. With <code>raw</code>, the signer signs the frame
              itself. With <code>midnight-sign-data</code>, it signs the ASCII prefix{' '}
              <code>midnight_signed_message:&lt;frame byte length&gt;:</code> followed by the frame. The framing is a
              signed term, and verification tries no other framing. SIGNED-INTENT.md names no wallet or signer product
              for either framing, and no live wallet has been tested with this flow. This recipe uses{' '}
              <code>raw</code>.
            </p>
          </li>
          <li>
            <p>
              Prepare the intent. The result holds the full <code>statement</code>, <code>frame_hex</code>,{' '}
              <code>frame_sha256</code> and <code>signing_message_hex</code>. This command prints the status, the
              frame's SHA-256 and the length of the signing message in bytes:
            </p>
            <Cmd k="si-intent" title="Prepare the intent" />
            <Out k="si-intent" />
            <p>
              <code>OwnerIntentPrepared</code> means that the bytes to sign are in <code>signing_message_hex</code>. The
              tool has not checked a signature, so this result does not show that anyone holds the key.
            </p>
          </li>
          <li>
            <p>Before you sign, print the statement in readable form:</p>
            <Cmd k="si-intent-review" title="Review the terms before signing" />
            <Pick
              k="si-intent-review"
              title="Part of the output"
              keep={[
                'OwnerIntentPrepared',
                '[agreementId]',
                '[actionName]',
                '[sourceSha256]',
                '[domain/',
                '[asset/id]',
                '[asset/scale]',
                '[signature/',
                '[intent/signer]',
                '[intent/nonce]',
                '[intent/preHead]',
                '[intent/notBefore]',
                '[intent/notAfter]',
                '[intent/grossCap]',
                '[intent/feeCap]',
                '[intent/netFloor]',
                '[intent/operation/',
                'Unsigned scenario context',
              ]}
            />
            <p>Read these fields and confirm each one:</p>
            <ul>
              <li>
                <code>sourceSha256</code> is the hash of the file you mean to sign. The review prints{' '}
                <code>(match)</code> when it equals the hash of the file on disk.
              </li>
              <li>
                <code>agreementId</code>, <code>actionName</code> and <code>selectedActionId</code> name the agreement
                and the action you mean.
              </li>
              <li>
                <code>domain/id</code>, <code>domain/chain</code> and <code>domain/network</code> are claims copied from
                the source. The network <code>preview</code> is a label, not a connection to Midnight's Preview
                network.
              </li>
              <li>
                <code>asset/id</code> and <code>asset/scale</code>: every amount is in atoms of this asset.
              </li>
              <li>
                <code>signature/scheme</code>, <code>signature/publicKeyHex</code> and <code>signature/framing</code> are
                your scheme, your key and your framing.
              </li>
              <li>
                <code>intent/signer</code> and the accounts in <code>intent/operation</code>: <code>from</code>,{' '}
                <code>recipient</code> and <code>feeRecipient</code> for a transfer, or <code>payer</code> and{' '}
                <code>obligationId</code> for a repayment.
              </li>
              <li>
                <code>intent/grossCap</code>, <code>intent/feeCap</code>, <code>intent/netFloor</code> and the{' '}
                <code>amount</code> and <code>fee</code> of the operation, in atoms.
              </li>
              <li>
                <code>intent/nonce</code>, <code>intent/preHead</code>, <code>intent/notBefore</code> and{' '}
                <code>intent/notAfter</code>: the replay label, the head label and the round window.
              </li>
            </ul>
            <p>
              The part under <code>Unsigned scenario context</code> lists the balances, the allowance and the head. They
              are not signed.
            </p>
          </li>
          <li>
            <p>Decode the signing message into the exact bytes to sign:</p>
            <Cmd k="si-export" title="Write signing-message.bin" />
            <Out k="si-export" />
            <p>
              With <code>raw</code> framing the signing message is the frame itself, so its SHA-256 equals{' '}
              <code>frame_sha256</code>. With <code>midnight-sign-data</code> the prefix is already in{' '}
              <code>signing_message_hex</code>, so do not add it again.
            </p>
          </li>
          <li>
            <Note title="What a valid signature shows" tone="caution">
              <p>
                A valid signature shows only that this key signed these bytes. It does not show that the key controls
                the <code>Owner</code> account, that the key is not revoked, or that any ledger has seen the intent. The
                result records this as <code>"keyAuthority": "Unverified"</code>.
              </p>
            </Note>
            <p>
              Sign with your own signing system. The tool takes no private key. Follow these rules:
            </p>
            <ul>
              <li>
                Sign the bytes in <code>signing-message.bin</code>. Do not sign the hex text or{' '}
                <code>frame_sha256</code>.
              </li>
              <li>
                If you chose <code>midnight-sign-data</code> and your signer adds the{' '}
                <code>midnight_signed_message</code> prefix itself, give it the decoded <code>frame_hex</code> bytes
                instead, so that the prefix appears once.
              </li>
              <li>
                Do not hash the message first. Both supported schemes hash it with SHA-256 inside the signer. If your
                signer accepts only a prehash, give it the SHA-256 of <code>signing-message.bin</code>, once.
              </li>
              <li>
                Keep the 64-byte raw signature as 128 lowercase hex characters in <code>SIGNATURE_HEX</code>.
              </li>
              <li>
                For ECDSA, convert a DER-encoded signature to raw <code>r || s</code> outside this tool, with{' '}
                <code>s</code> in low-S form. Low S: if <code>(r, s)</code> is a valid ECDSA signature, so is{' '}
                <code>(r, n − s)</code>, where <code>n</code> is the order of the secp256k1 group. The low-S form is
                the one with <code>s</code> at most <code>n / 2</code>.
              </li>
            </ul>
            <p>
              To follow along with the example's key, use the example's signature. It is valid for your statement
              because your statement is the same as the example's statement:
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
            <Note title="The scenario is not signed" tone="caution">
              <p>
                The same signature verifies against any well-formed scenario for this source and action. A changed
                balance, allowance or head gives a different candidate or a Core
                rejection under the same signature. Check the scenario separately.
              </p>
            </Note>
            <p>Verify, then print the main fields of the result:</p>
            <Cmd k="si-verify" title="Verify your signature" />
            <Out k="si-verify" />
            <p>
              <code>SignedPreparedUnqualified</code> means that the signature verified and Core prepared a candidate
              result that no authentication, proof or ledger has qualified.
            </p>
          </li>
        </Steps>
        <p>
          To see that the scenario is not signed, lower the owner's balance from 10000 to 500 atoms and verify again
          with the same signature:
        </p>
        <Cmd k="si-scenario" title="Change the scenario, then verify again" />
        <Out k="si-scenario" />
        <p>
          The signature is still valid. Core rejects the run because 500 atoms is less than the 1010-atom gross debit.
        </p>
        <p>
          If you change the source after preparing the intent, even by a comment or by formatting, the SHA-256 no longer
          matches and verification stops before the signature check:
        </p>
        <Cmd k="si-edited" title="Edit the source, then verify again" />
        <Out k="si-edited" />
        <p>
          <code>verify-intent</code> exits 0 when the signature verifies and a local candidate is prepared, 1 when the
          signature, the source or Core rejects, and 2 when the verifier itself fails (
          <a href={`${R}#results`}>Reference: results</a>, <a href={`${R}#cli`}>Reference: command line</a>).
        </p>
        <Worked>
          The verification of your signature exits 0 and prints <code>"sourceMatched": true</code>,{' '}
          <code>"keyAuthority": "Unverified"</code>, <code>"state": "LocalStipulationOnly"</code>,{' '}
          <code>"nativeProof": "NotChecked"</code> and <code>"ledger": "NotSubmitted"</code>.
        </Worked>
        <p>
          Full protocol: <a href={SIGNED_INTENT}>SIGNED-INTENT.md</a>.
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

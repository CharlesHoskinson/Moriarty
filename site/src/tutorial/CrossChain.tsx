import { useEffect, useRef, useState } from 'react';
import bridgeSource from '../../../packages/moriarty-beta/examples/bridge.mori?raw';
import stakingSource from '../../../packages/moriarty-beta/examples/staking.mori?raw';

const REPO = 'https://github.com/CharlesHoskinson/Moriarty/blob/main';
const BETA = `${REPO}/packages/moriarty-beta`;
const BRIDGE_FILE = `${BETA}/examples/bridge.mori`;
const STAKING_FILE = `${BETA}/examples/staking.mori`;

type StatusName = 'LocalS0' | 'SpecifiedOnly' | 'Open' | 'PreparedUnqualified';

const STATUS_TEXT: Record<StatusName, string> = {
  LocalS0: 'local preparation exists for transfer and repayment',
  SpecifiedOnly: 'structure checked; execution unsupported; financial relations open',
  Open: 'not closed by this authoring check',
  PreparedUnqualified: 'a local candidate, not an authenticated, proved or settled transaction',
};

function Status({ name }: { name: StatusName }) {
  return (
    <span className="guide-status" data-status={name}>
      <strong>{name}</strong>
      <span> — {STATUS_TEXT[name]}</span>
    </span>
  );
}

const COPY_FAILED = 'Copy failed. Select the text and copy it manually.';

type CodeKind = 'program' | 'command' | 'output';

const COPY_LABEL: Record<CodeKind, string> = {
  program: 'Copy program',
  command: 'Copy command',
  output: 'Copy recorded output',
};

const REGION_KIND: Record<CodeKind, string> = {
  program: 'complete source',
  command: 'command',
  output: 'recorded output',
};

function CopyButton({ text, label }: { text: string; label: string }) {
  const [state, setState] = useState<'idle' | 'ok' | 'fail'>('idle');
  const timer = useRef<number | undefined>(undefined);
  useEffect(() => () => window.clearTimeout(timer.current), []);
  const copy = async () => {
    try {
      const clipboard = navigator.clipboard;
      if (!clipboard || typeof clipboard.writeText !== 'function') {
        throw new Error('clipboard unavailable');
      }
      await clipboard.writeText(text);
      setState('ok');
    } catch {
      setState('fail');
    }
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setState('idle'), 4000);
  };
  return (
    <span className="guide-copybar">
      <button
        type="button"
        className="guide-copy"
        style={{ minWidth: '44px', minHeight: '44px' }}
        onClick={copy}
      >
        {label}
      </button>
      <span role="status" className={`guide-copy-state guide-copy-${state}`}>
        {state === 'ok' ? 'Copied.' : state === 'fail' ? COPY_FAILED : ''}
      </span>
    </span>
  );
}

function Code({
  title,
  status,
  text,
  note,
  href,
  kind,
}: {
  title: string;
  status: StatusName;
  text: string;
  note?: string;
  href?: string;
  kind: CodeKind;
}) {
  return (
    <figure className="guide-code" data-copy-owned="true" tabIndex={0} aria-label={`${title}, ${REGION_KIND[kind]}`}>
      <figcaption>
        <strong>{title}</strong> <Status name={status} />
        {note ? <span> {note}</span> : null}
        {href ? (
          <>
            {' '}
            <a href={href}>Source</a>
          </>
        ) : null}{' '}
        <CopyButton text={text} label={COPY_LABEL[kind]} />
      </figcaption>
      <pre>
        <code>{text}</code>
      </pre>
    </figure>
  );
}

const BRIDGE_CHECK = `AuthoringChecked BridgeDemo
  escrow: SpecifiedOnly (execution unsupported; financial relations open)
  claim: SpecifiedOnly (execution unsupported; financial relations open)
  recover: SpecifiedOnly (execution unsupported; financial relations open)
`;

const BRIDGE_EXPAND = `{
  "qualification": "local-stipulation-only",
  "status": "Unsupported",
  "diagnostics": [
    {
      "code": "BETA_PROFILE_UNSUPPORTED",
      "message": "Recognized authoring profile has no local execution; scenario was not schema-checked or applied",
      "span": {
        "start": 854,
        "end": 880
      }
    }
  ],
  "publishedPost": null,
  "publishedEffects": null,
  "sourceHash": "ed1899d32b244e5f2d7cab829baa4135c13722cc3340fbe90febaa08aaf12c27",
  "scenarioHash": "ca3d163bab055381827226140568f3bef7eaac187cebd76878e0b63e9e442356",
  "scenarioValidation": "NotAppliedUnsupported"
}
`;

const STAKING_CHECK = `AuthoringChecked StakingDemo
  deposit: SpecifiedOnly (execution unsupported; financial relations open)
  reward: SpecifiedOnly (execution unsupported; financial relations open)
  slash: SpecifiedOnly (execution unsupported; financial relations open)
  unbond: SpecifiedOnly (execution unsupported; financial relations open)
  withdraw: SpecifiedOnly (execution unsupported; financial relations open)
`;

const STAKING_SIMULATE = `{
  "qualification": "local-stipulation-only",
  "status": "Unsupported",
  "diagnostics": [
    {
      "code": "BETA_PROFILE_UNSUPPORTED",
      "message": "Recognized authoring profile has no local execution; scenario was not schema-checked or applied",
      "span": {
        "start": 1270,
        "end": 1300
      }
    }
  ],
  "publishedPost": null,
  "publishedEffects": null,
  "sourceHash": "301cfa8c15f1a848db7b6a23aa3b246fd646a970b8212458d5990e1da8a9b385",
  "scenarioHash": "ca3d163bab055381827226140568f3bef7eaac187cebd76878e0b63e9e442356",
  "scenarioValidation": "NotAppliedUnsupported"
}
`;

const CHECK_COMMAND = `printf '%s\\n' '{}' > /tmp/mori-unused-scenario.json
node dist/cli.js check examples/bridge.mori
node dist/cli.js expand examples/bridge.mori --action escrow --scenario /tmp/mori-unused-scenario.json`;

const STAKE_COMMAND = `printf '%s\\n' '{}' > /tmp/mori-unused-scenario.json
node dist/cli.js check examples/staking.mori
node dist/cli.js simulate examples/staking.mori --action withdraw --scenario /tmp/mori-unused-scenario.json`;

export function CrossChain() {
  return (
    <>
      <section id="bridges" className="guide-section" aria-labelledby="bridges-h">
        <h2 id="bridges-h">Bridge escrow, claim and recovery</h2>
        <p>
          A bridge names one claim on two domains: a local escrow, a foreign claim, and a recovery of the
          escrow. In the shipped <code>bridge.mori</code> example, the escrow intent names Alice and 100.00 USD
          on Midnight, the claim intent names ForeignAlice and 100.00 of a wrapped asset on a second domain,
          and the recover intent names Alice and the original escrow under the same claim id. The file records
          those accounts, assets, domains and amounts. Observation of the other domain, finality and settlement
          are outside the file.
        </p>
        <p>
          <Status name="SpecifiedOnly" /> accompanies every action below. <code>check</code> accepts the
          agreement. <code>expand</code> and <code>simulate</code> stop with <code>BETA_PROFILE_UNSUPPORTED</code>{' '}
          and publish no effects. The only <Status name="LocalS0" /> operations in this package are the
          transfer and repayment lessons.
        </p>

        <h3>Use</h3>
        <p>
          Escrow names the source-side lock. The owner is Alice, whose economic id is <code>Alice</code> on the
          domain whose economic id is <code>Midnight</code> (<code>chain: &quot;midnight&quot;</code>,{' '}
          <code>network: &quot;preview&quot;</code>). The amount is 100.00 USD. That asset&apos;s economic id is{' '}
          <code>USDCanonical</code>, its scale is 2, and its representation is <code>canonical</code>, so
          100.00 USD is 10000 atoms. The destination domain&apos;s economic id is <code>Foreign</code>. The
          claim id is the string <code>claim-1</code>.
        </p>
        <p>
          Claim names the destination-side amount. The owner is ForeignAlice, economic id <code>ForeignAlice</code>{' '}
          on domain <code>Foreign</code>. The amount is 100.00 WrappedUSD: economic id <code>WrappedUSD</code>,
          scale 2, representation <code>bridge-claim</code>, which is also 10000 atoms. The source domain named
          by the call is Preview. Recover names the return of those 10000 atoms of <code>USDCanonical</code> to
          Alice under the same <code>claim-1</code>.
        </p>
        <div className="guide-grid">
          <div>
            <h4>Source lock</h4>
            <p>
              10000 atoms of <code>USDCanonical</code> on <code>Midnight</code>, held by account <code>Alice</code>.
              Symbol <code>USD</code> is display metadata.
            </p>
          </div>
          <div>
            <h4>Foreign claim</h4>
            <p>
              10000 atoms of <code>WrappedUSD</code> on <code>Foreign</code>, held by account{' '}
              <code>ForeignAlice</code>. The equal atom count does not make this the same asset.
            </p>
          </div>
        </div>
        <p>
          Bob and GOLD are declared in the file and used by none of the three actions. Chain and network
          strings are recorded claims. <code>inspect</code> marks every identity <code>authenticated: false</code>.
        </p>

        <h3>Intent and kernel</h3>
        <p>
          The intent is the operation the author wrote: <code>bridge.escrow</code>, <code>bridge.claim</code>,
          or <code>bridge.recover</code>, with its owner, amount, domain argument and claim id. Recover also
          stores the failure string{' '}
          <code>timeout alone is not proof of foreign nonreceipt</code>. That string is authoring data. The
          checker stores it and does not treat it as a proved duty, a policy, or a finality observation.
        </p>
        <p>
          Execution still has to admit an authentic observation of the foreign result and bind what finality
          that observation has. The beta checker does not do that work. <code>check --json</code> reports{' '}
          <code>evidence: &quot;AuthoringOnly&quot;</code> and open gates <code>authentication</code>,{' '}
          <code>nativeProof</code>, <code>financialCorrespondence</code> and <code>atomicLedgerAcceptance</code>.
          Coverage on each action is syntax, names and quantities checked, <code>financialRelations: Open</code>,{' '}
          <code>localPreparation: Unsupported</code>. A kernel, or a Midnight program that does not use one,
          would still have to supply that missing evidence. This example supplies none of it.
        </p>

        <h3>Pending claim</h3>
        <p>
          The design pattern is one pending claim, <code>claim-1</code>, with one terminal consumption.
          Escrow names the local lock: Alice and 10000 atoms of <code>USDCanonical</code>, destination{' '}
          <code>Foreign</code>. Claim names the foreign payment: ForeignAlice and 10000 atoms of{' '}
          <code>WrappedUSD</code>, source Preview. Recover names the return of the original escrow to Alice.
          The specified rule admits the claim when an authentic observation says the source lock finalized,
          and admits the recovery when a separate observation says the foreign side did not receive. Those
          two terminals are alternatives for the same claim id.
        </p>
        <p>
          The beta calls do not carry that observation. <code>bridge.claim</code> takes owner, amount, source
          and claim id. <code>bridge.recover</code> takes owner, amount and claim id, and the failure string
          is the only record of the nonreceipt rule. The shipped agreement contains all three actions, and
          the checker accepts that text. It does not consume <code>claim-1</code>.
        </p>
        <Code
          title="examples/bridge.mori"
          status="SpecifiedOnly"
          text={bridgeSource}
          href={BRIDGE_FILE}
          note="Package example. AuthoringChecked BridgeDemo."
          kind="program"
        />
        <Code
          title="Commands from packages/moriarty-beta"
          status="SpecifiedOnly"
          text={CHECK_COMMAND}
          note="Run from packages/moriarty-beta. The printf line writes the scenario file used below. Exit 0 on check. Exit 1 on expand."
          kind="command"
        />
        <Code
          title="Readable check of examples/bridge.mori"
          status="SpecifiedOnly"
          text={BRIDGE_CHECK}
          note="Stdout from the primary dist at commit df8b155. A Node color warning on stderr is omitted."
          kind="output"
        />
        <Code
          title="expand --action escrow"
          status="Open"
          text={BRIDGE_EXPAND}
          note="The scenario file was the bytes {} plus a newline. scenarioValidation says those bytes were not applied. publishedEffects is null."
          kind="output"
        />

        <h3>Late result and refund</h3>
        <p>
          A timeout records that a round bound has passed. It leaves the foreign execution unknown: the claim
          may have finalized after the last observation, it may still be queued, or it may never have been
          submitted. Recover becomes eligible only with a separate nonreceipt fact. The failure string on the
          recover intent is that rule, written as text. Running recover because the clock expired, while the
          foreign claim may already have succeeded, spends the escrow and the claim against the same{' '}
          <code>claim-1</code>.
        </p>
        <p>
          A result that arrives after a refund attempt is a different transition from the refund. Whichever
          transition is authenticated consumes <code>claim-1</code> once. The other cannot pay again and
          cannot erase the effects already committed. A committed debit stays committed. A compensating
          payment is a new authorized action with its own fees and residual duties. Midnight phase rules are
          not a global rollback, and this authoring file does not encode one. The same distinction is drawn
          for conditional settlement on the <a href="kernel.html#unknown">kernel page</a>.
        </p>
        <p>
          The checker does reject one concrete mis-statement of custody. An escrow whose owner account is on
          one domain and whose amount asset is on the other is <code>AuthoringRejected</code> with{' '}
          <code>BETA_DOMAIN_MISMATCH</code>, <code>Quantity asset has different domain</code>. Owner and amount
          stay on the local domain. Only the named <code>source</code> or <code>destination</code> argument
          may name the foreign domain. That rejection is a nominal domain check. It is not a finality proof
          and it does not decide the refund race.
        </p>
      </section>

      <section id="staking" className="guide-section" aria-labelledby="staking-h">
        <h2 id="staking-h">Staking deposit, reward, slash and exit</h2>
        <p>
          Staking in this package is a share class backed by one asset, plus named changes to that backing
          and a two-step exit. <code>staking.mori</code> deposits 100.00 USD into share class <code>VaultShare</code>,
          adds a reward of 5.00 USD, removes a slash of 3.00 USD, unbonds 100 share atoms, and withdraws those
          same 100 share atoms with a minimum of 90.00 USD. Each of those figures is an exact quantity in the
          source. The checker does not compute a share price, a reward index, or a redemption.
        </p>

        <h3>Use</h3>
        <p>
          The share class economic id is <code>VaultShare</code> on domain <code>Midnight</code>. Its backing
          asset is <code>USDCanonical</code>, scale 2, representation <code>canonical</code>. Alice, economic
          id <code>Alice</code> on that same domain, is the owner of deposit, unbond and withdraw. Reward and
          slash name the share class and an amount. They do not name an owner.
        </p>
        <table tabIndex={0} aria-labelledby="stake-quantities-caption">
          <caption id="stake-quantities-caption">
            Quantities <code>inspect</code> reports for <code>StakingDemo</code>. Display amounts and atom
            strings are the same fact.
          </caption>
          <thead>
            <tr>
              <th scope="col">Action</th>
              <th scope="col">What the intent names</th>
              <th scope="col">Atoms</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">deposit</th>
              <td>backing 100.00 USD of USDCanonical, for Alice</td>
              <td>10000</td>
            </tr>
            <tr>
              <th scope="row">reward</th>
              <td>5.00 USD added to VaultShare</td>
              <td>500</td>
            </tr>
            <tr>
              <th scope="row">slash</th>
              <td>3.00 USD removed from VaultShare</td>
              <td>300</td>
            </tr>
            <tr>
              <th scope="row">unbond</th>
              <td>100 share atoms of VaultShare, for Alice</td>
              <td>scalar 100, not a USD quantity</td>
            </tr>
            <tr>
              <th scope="row">withdraw</th>
              <td>the same 100 share atoms, minimum backing 90.00 USD</td>
              <td>share scalar 100; minimum 9000 atoms of USDCanonical</td>
            </tr>
          </tbody>
        </table>
        <p>
          GOLD is declared at scale 3 and is not the backing asset. Bob is declared and is not an owner of
          these actions. A reward, slash, deposit or minimum written in GOLD would be a different asset from{' '}
          <code>USDCanonical</code>. The backing argument has to be a quantity of the share class&apos;s
          declared backing asset.
        </p>

        <h3>Intent and kernel</h3>
        <p>
          Each intent fixes one operation and its arguments. Deposit&apos;s rounding string is{' '}
          <code>share mint floor benefits vault</code>. Slash&apos;s loss string is{' '}
          <code>signed slash priority; pending withdrawal remains explicit</code>. Both are prose hints. The
          checker accepts text here and does not derive a share balance, a rounding remainder, a slash
          order, or a surviving duty from them.
        </p>
        <p>
          The kernel work that remains is the financial relation: how many shares 10000 atoms of backing
          mint, who is credited the 500-atom reward, whose claim the 300-atom slash reduces, and whether 100
          share atoms may later leave with at least 9000 atoms of <code>USDCanonical</code>.{' '}
          <code>inspect</code> reports <code>financialRelations: Open</code>, <code>localPreparation: Unsupported</code>,
          empty premise and binding arrays, <code>authentication: open</code> and <code>ledgerCommit: open</code>.
          Empty arrays mean the local transfer catalog does not apply. They do not mean the action is
          qualified.
        </p>

        <h3>Share class and a later exit</h3>
        <p>
          The pattern separates backing changes from the exit. Deposit, reward and slash speak in atoms of
          the backing asset. Unbond speaks in share atoms and names no payout. Withdraw repeats the share-atom
          count and adds an explicit minimum in the backing asset. An unbond is a request. The withdrawal is
          the later action that would deliver backing, and only if its minimum is met. The file keeps those
          as two actions so the request is not itself delivery of 90.00 USD.
        </p>
        <Code
          title="examples/staking.mori"
          status="SpecifiedOnly"
          text={stakingSource}
          href={STAKING_FILE}
          note="Package example. AuthoringChecked StakingDemo."
          kind="program"
        />
        <Code
          title="Commands from packages/moriarty-beta"
          status="SpecifiedOnly"
          text={STAKE_COMMAND}
          note="Run from packages/moriarty-beta. Exit 0 on check. Exit 1 on simulate. Stderr may show a Node color warning; it is not a mori diagnostic."
          kind="command"
        />
        <Code
          title="Readable check of examples/staking.mori"
          status="SpecifiedOnly"
          text={STAKING_CHECK}
          note="Stdout from the primary dist at commit df8b155."
          kind="output"
        />
        <Code
          title="simulate --action withdraw"
          status="Open"
          text={STAKING_SIMULATE}
          note="Same unused scenario bytes as the bridge expand. No post-state is published, so the 9000-atom minimum is not tested."
          kind="output"
        />

        <h3>Slash and the pending exit</h3>
        <p>
          The loss hint says a slash keeps a pending withdrawal explicit. The five actions are separate, so
          a slash of 300 atoms does not delete the unbond of 100 share atoms, and the unbond does not pay
          the withdraw minimum. The checker does not order those actions or preserve a duty across them. The
          hint is the specified scope, and the support label on each action is <Status name="SpecifiedOnly" />.
        </p>
        <p>
          A deposit whose owner is on a different domain from the share class is rejected at authoring with{' '}
          <code>BETA_DOMAIN_MISMATCH</code>, <code>Operation argument domains differ</code>. Owner, share
          class and backing quantity share the declared domain. That check fixes the identity of the
          position. It does not decide slash priority, and it does not pay the exit.
        </p>
      </section>

      <section id="trust" className="guide-section" aria-labelledby="trust-h">
        <h2 id="trust-h">Support, identity and federation</h2>
        <p>
          The closed authoring catalog is the family examples in the table. <code>check</code> on each shipped file returns{' '}
          <code>AuthoringChecked</code> and <Status name="SpecifiedOnly" /> for every action.{' '}
          <code>financialRelations</code> stays <Status name="Open" />. Expansion and simulation of those
          actions return <code>Unsupported</code>, code <code>BETA_PROFILE_UNSUPPORTED</code>, with{' '}
          <code>publishedEffects: null</code>. Local preparation that can reach{' '}
          <Status name="PreparedUnqualified" /> is the <a href="#transfer">transfer</a> and{' '}
          <a href="#repayment">repayment</a> path, marked <Status name="LocalS0" />. A prepared result there
          is still a candidate under a local stipulation.
        </p>
        <table tabIndex={0} aria-labelledby="support-matrix-caption">
          <caption id="support-matrix-caption">
            Support observed with <code>node dist/cli.js check</code> on the eight example files, primary
            dist, commit df8b155. Section links use the sibling module ids. The lending row links LocalS0
            repayment separately from the specified family.
          </caption>
          <thead>
            <tr>
              <th scope="col">Area</th>
              <th scope="col">Example</th>
              <th scope="col">Actions</th>
              <th scope="col">Authoring</th>
              <th scope="col">Relation</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">
                <a href="#amm">AMM</a>
              </th>
              <td>
                <a href={`${BETA}/examples/amm.mori`}>amm.mori</a>
              </td>
              <td>swap, mint_lp, redeem_lp</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#lending">Lending</a>
              </th>
              <td>
                <a href={`${BETA}/examples/lending.mori`}>lending.mori</a>
              </td>
              <td>
                borrow, roll, liquidate — specified origination, roll and liquidation in lending.mori.{' '}
                <a href="#lending-repay">LocalS0 repayment</a>
              </td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#stablecoins">Stablecoin</a>
              </th>
              <td>
                <a href={`${BETA}/examples/stablecoin.mori`}>stablecoin.mori</a>
              </td>
              <td>mint, redeem, emergency</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#options">Derivatives</a>
              </th>
              <td>
                <a href={`${BETA}/examples/derivatives.mori`}>derivatives.mori</a>
              </td>
              <td>fix, exercise, settle</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#oracles">Oracle</a>
              </th>
              <td>
                <a href={`${BETA}/examples/oracle.mori`}>oracle.mori</a>
              </td>
              <td>read_fixing</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#governance">Governance</a>
              </th>
              <td>
                <a href={`${BETA}/examples/governance.mori`}>governance.mori</a>
              </td>
              <td>queue, execute, veto</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#bridges">Bridge</a>
              </th>
              <td>
                <a href={BRIDGE_FILE}>bridge.mori</a>
              </td>
              <td>escrow, claim, recover</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
            <tr>
              <th scope="row">
                <a href="#staking">Staking</a>
              </th>
              <td>
                <a href={STAKING_FILE}>staking.mori</a>
              </td>
              <td>deposit, reward, slash, unbond, withdraw</td>
              <td>
                <Status name="SpecifiedOnly" />
              </td>
              <td>
                <Status name="Open" />
              </td>
            </tr>
          </tbody>
        </table>

        <h3>Account, key, state, custody and asset</h3>
        <p>
          An account is a domain plus an economic id. The source name <code>Alice</code> is the declaration
          a reader navigates. The id field is the economic identity. In this example those strings match, and
          they still are different roles: renaming the declaration would not by itself retarget the id.
          Duplicate economic ids are rejected. <code>inspect</code> returns <code>authenticated: false</code>{' '}
          for Preview, Foreign, Alice, ForeignAlice, USD, GOLD and WrappedUSD.
        </p>
        <p>
          These bridge and staking intents carry no <code>key</code>, nonce, head or round window. A key
          string on a local transfer intent is an opaque reference, not a public key and not proof of
          account ownership. Nothing in these two files supplies a key to check. State is likewise unnamed:
          there is no predecessor and no signed head. Custody is the <code>owner</code> argument. That
          account and the amount, or the share class, share one domain. On a bridge, <code>source</code> and{' '}
          <code>destination</code> are the arguments that may name the other domain.
        </p>
        <p>
          An asset is its domain, economic id, scale and representation. <code>USDCanonical</code> at scale 2
          on <code>Midnight</code> is not <code>WrappedUSD</code> at scale 2 on <code>Foreign</code>. The
          checker uses scale to turn <code>100.00 USD</code> into the atom string <code>10000</code>. That
          is nominal arithmetic inside authoring. It does not authenticate the asset, and it does not
          convert one asset into the other. The symbol is display text.
        </p>

        <h3>Local premises and unverified bindings</h3>
        <p>
          Local transfer preparation carries four external premises and four unverified bindings. They are
          the S0 catalog documented in{' '}
          <a href={`${BETA}/GETTING-STARTED.md`}>GETTING-STARTED.md</a> and listed again beside the{' '}
          <a href="#open-premises">transfer lesson</a>. <code>inspect</code> of <code>bridge.mori</code> and{' '}
          <code>staking.mori</code> returns <code>requiredPremises: []</code> and{' '}
          <code>unverifiedBindings: []</code> both on the agreement and on each action. The empty arrays
          mean this catalog is not the one being applied. Authentication, native proof, financial
          correspondence and ledger acceptance stay open, which is also the <code>openGates</code> list on{' '}
          <code>check --json</code>.
        </p>
        <div className="guide-grid">
          <div>
            <h4>
              Premises <Status name="Open" />
            </h4>
            <ul>
              <li>
                <code>canonical-intent-signature</code>
              </li>
              <li>
                <code>snapshot-to-head</code>
              </li>
              <li>
                <code>head-extension</code>
              </li>
              <li>
                <code>atomic-ledger-compare-and-consume</code>
              </li>
            </ul>
          </div>
          <div>
            <h4>
              Bindings <Status name="Open" />
            </h4>
            <ul>
              <li>
                <code>agreement-id</code>
              </li>
              <li>
                <code>selected-program</code>
              </li>
              <li>
                <code>asset-scale</code>
              </li>
              <li>
                <code>authenticated-predecessor</code>
              </li>
            </ul>
          </div>
        </div>
        <p className="guide-callout">
          Signing for a local transfer or repayment is the separate flow in{' '}
          <a href={`${BETA}/SIGNED-INTENT.md`}>SIGNED-INTENT.md</a>. A successful native signature check
          there is <code>SignedPreparedUnqualified</code>, with ledger acceptance still open. Intent
          preparation expands the action first and stops unless that expansion succeeds. Bridge and staking
          actions expand as <code>Unsupported</code>, so that flow does not prepare a signature for them.
        </p>

        <h3>Federation is optional</h3>
        <p>
          An agreement whose effects stay on Midnight is judged by Midnight&apos;s proof and state
          transition. The Federated DeFi Kernel is an optional coordinator: it can discover solvers, obtain
          evidence, hold threshold custody, route a cross-chain effect, observe finality and run recovery.
          A supported program does not need that federation, a hosted solver, or a project approval in order
          to be authored for Midnight. Asset-owner authorization still applies where the agreement uses it.
          The direct path and the optional kernel are both described on the{' '}
          <a href="kernel.html#agreement">kernel page</a>.
        </p>
        <p>
          The <a href={`${REPO}/ROADMAP.md`}>roadmap</a> places conditional settlement, including a late
          success racing a refund, in U3, and places the federation itself in U5. U3 does not depend on U5.
          Neither milestone is closed by these example files. The same roadmap records scoped local
          evaluation, finite K comparisons, and scoped Preview loan and swap results as foundations. It
          records general source-to-ledger correspondence, native recursive history and private composition
          as open. This page does not present a recursive proof, a history certificate, or a network
          settlement of the bridge or the stake.
        </p>
        <p>
          Further reading stays on the product documents:{' '}
          <a href={`${REPO}/docs/MORIARTY-PRODUCT-CONTRACT.md`}>product contract</a>,{' '}
          <a href={`${REPO}/docs/MORIARTY-CONSOLIDATED-DESIGN.md`}>consolidated design</a>, the{' '}
          <a href="kernel.html#status">implementation status</a> on the kernel page, and the{' '}
          <a href="#syntax">surface syntax</a> used by every example above. The commands a reader runs first
          are the <a href="#getting-started">local install and transfer</a>.
        </p>
      </section>
    </>
  );
}

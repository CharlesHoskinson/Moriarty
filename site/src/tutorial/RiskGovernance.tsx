import { useId, useState, type CSSProperties, type ReactNode } from 'react';
import derivativesSource from '../../../packages/moriarty-beta/examples/derivatives.mori?raw';
import governanceSource from '../../../packages/moriarty-beta/examples/governance.mori?raw';
import oracleSource from '../../../packages/moriarty-beta/examples/oracle.mori?raw';

const REV = 'df8b155143e90450168af04fbfd20f7620043620';
const REPO = 'https://github.com/CharlesHoskinson/Moriarty';

const DERIVATIVES_SHA256 = '17b0e9ea666e0867b032c40605a5a89b02beddc043affa9a1d1d6256cf187449';
const ORACLE_SHA256 = '2da41606e65fe6385df8e148bc4ade41a802d377245323d8bc7838944372ac04';
const GOVERNANCE_SHA256 = '1a9eb7c670f957429d5a4c82d89ed0f3dd1e8147210c3e524bbfd34885b2af4f';

const sectionStyle: CSSProperties = { scrollMarginTop: 'var(--guide-offset)', minWidth: 0 };
const headingStyle: CSSProperties = { scrollMarginTop: 'var(--guide-offset)' };
const proseStyle: CSSProperties = { fontSize: '1.0625rem', lineHeight: 1.6, maxWidth: '72ch' };
const noteStyle: CSSProperties = { ...proseStyle, color: 'var(--ink-muted, CanvasText)' };
const toolbarStyle: CSSProperties = {
  display: 'flex',
  flexWrap: 'wrap',
  gap: '0.5rem',
  alignItems: 'center',
  margin: '0.75rem 0',
};
const buttonStyle: CSSProperties = {
  font: 'inherit',
  fontSize: '1rem',
  lineHeight: 1.3,
  minBlockSize: '2.75rem',
  padding: '0.5rem 0.9rem',
  background: 'var(--paper-raised, canvas)',
  color: 'var(--ink, CanvasText)',
  border: '1px solid var(--rule-strong, CanvasText)',
  borderRadius: '3px',
  cursor: 'pointer',
};
const preStyle: CSSProperties = {
  fontSize: '0.9375rem',
  lineHeight: 1.65,
  overflow: 'auto',
  maxInlineSize: '100%',
  minWidth: 0,
  margin: 0,
  padding: '0.75rem 1rem',
};
const codeStyle: CSSProperties = { fontSize: 'inherit', lineHeight: 'inherit' };
const tableWrapStyle: CSSProperties = {
  overflowX: 'auto',
  maxInlineSize: '100%',
  minWidth: 0,
  margin: '1rem 0',
};
const tableStyle: CSSProperties = {
  display: 'table',
  overflow: 'visible',
  maxInlineSize: 'none',
  borderCollapse: 'collapse',
  fontSize: '1rem',
  lineHeight: 1.45,
};
const cellStyle: CSSProperties = {
  textAlign: 'left',
  verticalAlign: 'top',
  padding: '0.4rem 0.75rem',
  borderBottom: '1px solid var(--rule, CanvasText)',
  whiteSpace: 'nowrap',
};

function blob(path: string): string {
  return `${REPO}/blob/${REV}/${path}`;
}

function mustSlice(source: string, start: string, end: string, label: string): string {
  const from = source.indexOf(start);
  const to = from < 0 ? -1 : source.indexOf(end, from);
  if (from < 0 || to < 0) throw new Error(`Missing ${label} excerpt in shipped example`);
  return source.slice(from, to + end.length);
}

const optionExcerpt = mustSlice(
  derivativesSource,
  'instrument Call = ',
  'action settle uses Settle;',
  'options',
);
const oracleExcerpt = mustSlice(
  oracleSource,
  'observation Fixing = ',
  'action read_fixing uses Read;',
  'oracle',
);
const governanceExcerpt = mustSlice(
  governanceSource,
  'policy Terms = ',
  'action veto uses Veto;',
  'governance',
);

function DocLink({ path, children }: { path: string; children: ReactNode }) {
  return <a href={blob(path)}>{children}</a>;
}

function StatusLine({ children }: { children: ReactNode }) {
  return (
    <p className="guide-status" style={{ ...noteStyle, fontFamily: 'var(--mono, ui-monospace, monospace)' }}>
      {children}
    </p>
  );
}

function ScrollTable({ label, caption, children }: { label: string; caption: string; children: ReactNode }) {
  return (
    <div className="guide-table-wrap" tabIndex={0} role="region" aria-label={label} style={tableWrapStyle}>
      <table style={tableStyle}>
        <caption style={{ captionSide: 'top', textAlign: 'left', fontSize: '1rem', padding: '0 0 0.5rem', color: 'var(--ink, CanvasText)' }}>
          {caption}
        </caption>
        {children}
      </table>
    </div>
  );
}

function SourceFrame({
  title,
  filename,
  role,
  source,
  copyName,
  downloadName,
  note,
}: {
  title: string;
  filename: string;
  role: string;
  source: string;
  copyName: string;
  downloadName?: string;
  note?: string;
}) {
  const statusId = useId();
  const [copyMessage, setCopyMessage] = useState('');

  async function onCopy(): Promise<void> {
    try {
      await navigator.clipboard.writeText(source);
      setCopyMessage(`Copied ${copyName}.`);
    } catch {
      setCopyMessage('Copy failed. Select the text in the box below.');
    }
  }

  function onDownload(): void {
    if (!downloadName) return;
    const url = URL.createObjectURL(new Blob([source], { type: 'text/plain;charset=utf-8' }));
    const link = document.createElement('a');
    link.href = url;
    link.download = downloadName;
    document.body.appendChild(link);
    link.click();
    link.remove();
    URL.revokeObjectURL(url);
  }

  return (
    <figure style={{ margin: '1rem 0', minWidth: 0 }}>
      <figcaption style={{ ...proseStyle, marginBottom: '0.5rem' }}>
        {title}. {filename}. {role}. Profile <code>moriarty-beta/1</code>.
      </figcaption>
      <div style={toolbarStyle}>
        <button type="button" style={buttonStyle} onClick={() => void onCopy()}>
          {`Copy ${copyName}`}
        </button>
        {downloadName ? (
          <button type="button" style={buttonStyle} onClick={onDownload}>
            {`Download ${downloadName}`}
          </button>
        ) : null}
      </div>
      <p id={statusId} role="status" style={noteStyle}>{copyMessage}</p>
      {note ? <p style={noteStyle}>{note}</p> : null}
      <pre className="guide-code" tabIndex={0} aria-label={`${filename}, ${role}`} style={preStyle}>
        <code style={codeStyle}>{source}</code>
      </pre>
    </figure>
  );
}

function CheckCommands({ file, headingId, derived }: { file: string; headingId: string; derived: string }) {
  const commands = `node dist/cli.js check examples/${file}\nnode dist/cli.js check examples/${file} --json`;
  return (
    <>
      <h3 id={headingId} style={headingStyle}>Check {file}</h3>
      <p style={proseStyle}>
        Run both commands from <code>packages/moriarty-beta</code>
        {' '}after <code>npm run build</code>
        {' '}in that package.
        The readable command prints authoring status. <code>--json</code>
        {' '}is the same check for tools.
      </p>
      <SourceFrame
        title="Command"
        filename={file}
        role="Command. Copy excludes shell prompts and output"
        source={commands}
        copyName={`check commands for ${file}`}
      />
      <p style={proseStyle}>
        Illustrative readable result, derived from the formatter in <code>packages/moriarty-beta/src/cli.ts</code>, not a capture.
        A successful readable check is the agreement status, then one line per action: name, support, and for SpecifiedOnly the parenthesis
        &quot;execution unsupported; financial relations open&quot;. The formatter adds &quot;Local preparation remains PreparedUnqualified.&quot;
        only when an action is LocalS0. The lines below apply that formatter to the action names in this file
        and to the assertions in <code>tests/examples.test.mjs</code>.
      </p>
      <pre
        className="guide-code"
        data-copy="none"
        tabIndex={0}
        aria-label={`Illustrative readable result for ${file}, derived from the formatter, not a capture`}
        style={preStyle}
      >
        <code style={codeStyle}>{derived}</code>
      </pre>
    </>
  );
}

function Boundary({ action }: { action: string }) {
  return (
    <div className="guide-callout">
      <StatusLine>SpecifiedOnly · Open · Unsupported</StatusLine>
      <p style={proseStyle}>
        Authoring of {action} is checked. <code>financialRelations</code>
        {' '}stays Open. <code>localPreparation</code>
        {' '}stays Unsupported. Evidence on that report is AuthoringOnly.
        Expand and simulate return status Unsupported, diagnostic <code>BETA_PROFILE_UNSUPPORTED</code>,
        message &quot;Recognized authoring profile has no local execution; scenario was not schema-checked or applied&quot;,
        with <code>publishedEffects</code>
        {' '}null, <code>publishedPost</code>
        {' '}null, <code>scenarioValidation</code>
        {' '}NotAppliedUnsupported, and qualification <code>local-stipulation-only</code>.
        That qualification labels the local refusal. It does not qualify the financial action.
        The package test calls both with scenario text <code>{'{}'}</code>.
        LocalS0 and PreparedUnqualified name transfer and repayment preparation. They do not apply to {action}.
        {' '}Authentication, native proof, financial correspondence and ledger acceptance are in <a href="#trust">Trust</a>.
      </p>
    </div>
  );
}

function Split({ id, children }: { id: string; children: ReactNode }) {
  return (
    <>
      <h3 id={id} style={headingStyle}>Intent and kernel</h3>
      <p style={proseStyle}>
        The intent is the authored operation. The checker binds nominal domain, asset and quantity shape, and these actions stop before Core.
        These files declare no <code>signer</code>, no key and no signature.
        On the signed transfer and repayment path, a native signature check leaves <code>keyAuthority</code>
        {' '}Unverified, state LocalStipulationOnly, native proof NotChecked, ledger NotSubmitted, and <code>ledger_accepted</code>
        {' '}false. Account authority stays separate from that signature.
        {' '}The shared premises, optional kernel and maintainer boundary are in <a href="#trust">Trust</a>.
      </p>
      {children}
    </>
  );
}

export function RiskGovernance() {
  return (
    <>
      <section id="options" className="guide-section" aria-labelledby="options-title" style={sectionStyle}>
        <h2 id="options-title" style={headingStyle}>Options</h2>
        <p style={proseStyle}>
          The options example is <DocLink path="packages/moriarty-beta/examples/derivatives.mori">derivatives.mori</DocLink>,
          profile <code>moriarty-beta/1</code>, agreement <code>DerivativesDemo</code>, in package{' '}
          <code>@moriarty-lang/beta</code> at version 0.1.0-beta.1. The file is the current authoring syntax.
          Fixing, exercise and settlement are three intents in that file. Their financial execution remains open.
        </p>
        <Boundary action="fix, exercise and settle" />
        <SourceFrame
          title="Excerpt"
          filename="derivatives.mori"
          role="Excerpt, not a runnable file. The prelude with domain, accounts and assets is in the complete file"
          source={optionExcerpt}
          copyName="options excerpt"
          note="Copy excerpt copies only these lines. Download the complete file from the details block so the trailing newline and prelude stay intact."
        />

        <h3 id="options-use" style={headingStyle}>Economic use</h3>
        <p style={proseStyle}>
          The sketch names Alice as holder of a call named Call. The underlying asset is GOLD: declaration alias GOLD, economic id
          GoldCanonical, scale 3, symbol GOLD, representation canonical, domain Preview. Settlement is USD:
          alias USD, economic id USDCanonical, scale 2, symbol USD, the same domain. The strike field is the
          string <code>100.00</code> and <code>strike_units</code> is the string <code>USD per GOLD</code>.
          Collateral is the quantity <code>500.00 USD</code>. At scale 2 that spelling is 50000 atoms.{' '}
          <code>exercise_round</code> is the scalar 150. Settlement names holder Alice and payoff{' '}
          <code>100.00 USD</code>, which is 10000 atoms of USDCanonical.
        </p>
        <p style={proseStyle}>
          The use is a cash-settled call. <code>option.fix</code> names the observation that would record a
          USD-per-GOLD value. <code>option.exercise</code> names Alice and carries the continuation string
          "missing fixing retains reserve and exercise duty". <code>option.settle</code> names the same holder,
          the 10000-atom payoff, and the rounding string "payoff floor benefits reserve". The file records that
          sequence. Check accepts the record. It does not multiply a fixing by a notional, and it does not move
          the 50000 atoms of collateral.
        </p>
        <p style={proseStyle}>
          A quantity literal needs whitespace or a comment before the asset alias. <code>500.00USD</code> is
          rejected. Symbol GOLD is display metadata. It does not convert GOLD into USD. Bob is declared as
          account id Bob on Preview and is not an argument of fix, exercise or settle.
        </p>

        <h3 id="options-amounts" style={headingStyle}>Identities and amounts</h3>
        <ScrollTable
          label="Option identities and amounts"
          caption="Identities and amounts in derivatives.mori. Atoms are the checker's scale conversion. Strike is a string, so it has no atom count."
        >
          <thead>
            <tr>
              <th scope="col" style={cellStyle}>Field</th>
              <th scope="col" style={cellStyle}>Spelling</th>
              <th scope="col" style={cellStyle}>Alias</th>
              <th scope="col" style={cellStyle}>Economic id</th>
              <th scope="col" style={cellStyle}>Scale</th>
              <th scope="col" style={cellStyle}>Atoms or scalar</th>
              <th scope="col" style={cellStyle}>Checked relation</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row" style={cellStyle}>Domain Preview</th>
              <td style={cellStyle}>id, chain, network</td>
              <td style={cellStyle}>Preview</td>
              <td style={cellStyle}>Midnight</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>chain midnight, network preview</td>
              <td style={cellStyle}>Authored domain claim. The network string is not a Preview connection.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Holder</th>
              <td style={cellStyle}>Alice</td>
              <td style={cellStyle}>Alice</td>
              <td style={cellStyle}>Alice</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>account</td>
              <td style={cellStyle}>Account argument of exercise and settle. Same domain as Call.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Declared account</th>
              <td style={cellStyle}>Bob</td>
              <td style={cellStyle}>Bob</td>
              <td style={cellStyle}>Bob</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>account</td>
              <td style={cellStyle}>Declared. Not an operation argument in this file.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Underlying</th>
              <td style={cellStyle}>GOLD</td>
              <td style={cellStyle}>GOLD</td>
              <td style={cellStyle}>GoldCanonical</td>
              <td style={cellStyle}>3</td>
              <td style={cellStyle}>no GOLD quantity in this file</td>
              <td style={cellStyle}>Instrument underlying. Domain must match Call.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Settlement and collateral</th>
              <td style={cellStyle}>500.00 USD</td>
              <td style={cellStyle}>USD</td>
              <td style={cellStyle}>USDCanonical</td>
              <td style={cellStyle}>2</td>
              <td style={cellStyle}>50000 atoms</td>
              <td style={cellStyle}>Qty on the instrument. Domain must match Call.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Payoff</th>
              <td style={cellStyle}>100.00 USD</td>
              <td style={cellStyle}>USD</td>
              <td style={cellStyle}>USDCanonical</td>
              <td style={cellStyle}>2</td>
              <td style={cellStyle}>10000 atoms</td>
              <td style={cellStyle}>option.settle requires Qty of the settlement asset, here USD.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Strike</th>
              <td style={cellStyle}>"100.00"</td>
              <td style={cellStyle}>none</td>
              <td style={cellStyle}>none</td>
              <td style={cellStyle}>none</td>
              <td style={cellStyle}>string, not converted</td>
              <td style={cellStyle}>Stored with strike_units "USD per GOLD".</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Exercise round</th>
              <td style={cellStyle}>150</td>
              <td style={cellStyle}>Call</td>
              <td style={cellStyle}>Call</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>scalar 150</td>
              <td style={cellStyle}>Instrument scalar. This intent has no rounds() window.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Fixing round</th>
              <td style={cellStyle}>150</td>
              <td style={cellStyle}>Fixing</td>
              <td style={cellStyle}>Fixing</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>scalar 150</td>
              <td style={cellStyle}>Observation scalar. Unit and source are strings.</td>
            </tr>
          </tbody>
        </ScrollTable>
        <p style={proseStyle}>
          Scale 2 appends two fractional digits, so <code>500.00</code> becomes the integer 50000 and{' '}
          <code>100.00</code> becomes 10000. The same characters in the strike string stay a string. 10000 is less
          than 50000, and the checker does not compare those two quantities.
        </p>

        <Split id="options-split">
          <p style={proseStyle}>
            There is no fee quantity and no fee account in this file. A LocalS0 transfer, by contrast, requires
            three distinct accounts on one domain, a value and a fee of the same asset, and a signer equal to the{' '}
            <code>from</code> account. Fee counts in the gross debit on that path. <code>option.settle</code>'s
            schema is <code>instrument</code>, <code>holder</code> and <code>payoff</code>. The sketch names Alice
            as that holder, economic id Alice, domain Preview. That argument names an account. The settle schema has no credit
            effect and no fee account. Optional intent fields <code>gross_cap</code>, <code>fee_cap</code> and{' '}
            <code>net_floor</code> are absent. If an author adds <code>fee_cap</code> and also names an intent asset,
            check requires <code>fee_cap</code> to be a quantity of that asset. This intent names no asset, so the
            file has no fee bound. The action remains SpecifiedOnly in either case.
          </p>
        </Split>

        <h3 id="options-pattern" style={headingStyle}>Authoring pattern</h3>
        <p style={proseStyle}>
          Declare the domain, the accounts and both assets before the instrument. The instrument needs{' '}
          <code>domain</code>, <code>id</code>, <code>underlying</code> and <code>settlement</code>. This file also
          stores strike text, <code>exercise_round</code> and collateral. The observation needs <code>id</code> and{' '}
          <code>domain</code>. Then write three intents and three actions, in source order, each intent referring
          only to earlier declarations.
        </p>
        <ScrollTable label="Option operation schemas" caption="Required arguments from the closed schema map in frontend.ts.">
          <thead>
            <tr>
              <th scope="col" style={cellStyle}>Action</th>
              <th scope="col" style={cellStyle}>Intent</th>
              <th scope="col" style={cellStyle}>Operation</th>
              <th scope="col" style={cellStyle}>Required arguments</th>
              <th scope="col" style={cellStyle}>Extra fields this file stores</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row" style={cellStyle}>fix</th>
              <td style={cellStyle}>Fix</td>
              <td style={cellStyle}>option.fix</td>
              <td style={cellStyle}>instrument, observation</td>
              <td style={cellStyle}>none</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>exercise</th>
              <td style={cellStyle}>Exercise</td>
              <td style={cellStyle}>option.exercise</td>
              <td style={cellStyle}>instrument, holder</td>
              <td style={cellStyle}>continuation string</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>settle</th>
              <td style={cellStyle}>Settle</td>
              <td style={cellStyle}>option.settle</td>
              <td style={cellStyle}>instrument, holder, payoff</td>
              <td style={cellStyle}>rounding string</td>
            </tr>
          </tbody>
        </ScrollTable>
        <p style={proseStyle}>
          The pattern separates fixing, the holder's exercise, and settlement so each can carry its own authority
          later. In this file the separation is already visible, and the later authority is not filled in: no{' '}
          <code>valid</code> rounds window, no signer, and no retained-duty value other than the continuation string.
          The observation used by fix is declared here with source <code>feed</code> and unit <code>USD per GOLD</code>.
          The oracle section shows a different observation record under the same alias shape.
        </p>

        <h3 id="options-hostile" style={headingStyle}>Hostile case</h3>
        <div className="guide-callout">
          <p style={proseStyle}>
            The payoff spelling <code>100.00 USD</code> and the strike spelling <code>100.00</code> are different
            fields. Check requires the payoff's asset alias to be the settlement asset. A GOLD payoff would be{' '}
            <code>BETA_ASSET_MISMATCH</code> with the message shape <code>Expected Qty&lt;USD&gt;, received Qty&lt;GOLD&gt;</code>.
            An instrument without underlying or settlement is <code>BETA_MISSING_FIELD</code>, message
            "Option instrument requires underlying and settlement assets". Different domains among the instrument,
            observation and holder are <code>BETA_DOMAIN_MISMATCH</code>, message "Operation argument domains differ"
            or "Intent header and operation domains differ" when a header is present. Those are checker rules in{' '}
            <DocLink path="packages/moriarty-beta/src/frontend.ts">frontend.ts</DocLink>. This package test does not
            ship a rejected option fixture, and this page does not show a rejection transcript.
          </p>
          <p style={proseStyle}>
            The amount 10000 atoms is not tied to the strike string, to observed round 150, or to the 50000-atom
            collateral. The continuation string does not create a reserve liability or an exercise duty. The rounding
            string does not select a floor beneficiary. <code>exercise_round</code> 150 and the observation's{' '}
            <code>observed_round</code> 150 happen to be the same scalar. Check does not require them to match, and
            this intent has no <code>rounds</code> window that would authorize exercise only at that round.
            A signature on the file bytes would still leave key authority unverified. It would not make Alice the
            ledger holder, and it would not make the source string <code>feed</code> an issuer.
          </p>
        </div>

        <CheckCommands
          headingId="options-command"
          file="derivatives.mori"
          derived={'AuthoringChecked DerivativesDemo\n  fix: SpecifiedOnly (execution unsupported; financial relations open)\n  exercise: SpecifiedOnly (execution unsupported; financial relations open)\n  settle: SpecifiedOnly (execution unsupported; financial relations open)\n'}
        />

        <h3 id="options-file" style={headingStyle}>Complete file</h3>
        <details>
          <summary>Complete derivatives.mori, 1244 bytes</summary>
          <p style={proseStyle}>
            SHA-256 of the shipped file bytes, computed from the checkout copy, is <code style={{ overflowWrap: 'anywhere' }}>{DERIVATIVES_SHA256}</code>.{' '}
            <code>check</code> defines <code>sourceHash</code> as SHA-256 of the source text. That digest identifies
            these bytes. It does not establish the strike, the fixing or the payoff. Copy and download use this
            imported text, including the final newline. Comments and whitespace are part of the digest.
          </p>
          <SourceFrame
            title="Complete program"
            filename="derivatives.mori"
            role="Complete program"
            source={derivativesSource}
            copyName="derivatives.mori program"
            downloadName="derivatives.mori"
          />
        </details>
        <p style={proseStyle}>
          Repository evidence: <DocLink path="packages/moriarty-beta/examples/derivatives.mori">derivatives.mori</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/examples/README.md">examples README</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/GETTING-STARTED.md">GETTING-STARTED.md</DocLink> (SpecifiedOnly authoring hints),{' '}
          <DocLink path="packages/moriarty-beta/src/frontend.ts">frontend.ts</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/src/bridge.ts">bridge.ts</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/src/cli.ts">cli.ts</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/tests/examples.test.mjs">examples.test.mjs</DocLink>.
          The fixing observation continues in <a href="#oracles">Oracles</a>.
        </p>

        <h3 id="options-horizon" style={headingStyle}>Proposed lifecycle</h3>
        <details data-copy="none">
          <summary>Proposed option lifecycle. Not moriarty-beta/1 and not a runnable file</summary>
          <p style={proseStyle}>
            <DocLink path="wiki-llm/beta-language-2026-09-30/FULL-LANGUAGE-HORIZON.md">FULL-LANGUAGE-HORIZON.md</DocLink> section
            "Derivatives" specifies a typed call with phases Fund, Fix, Exercise and Settle. That notation is{' '}
            <code>moriarty-horizon/0.1</code> proposal text. The beta parser does not accept it. The page does not
            offer that text as a copyable program.
          </p>
          <p style={proseStyle}>
            The horizon illustration uses strike <code>Price&lt;USD,GOLD,2&gt;(10000)</code>, which that document
            reads as 100 USD per GOLD, notional 1.000 GOLD, premium 5.00 USD and collateral 500.00 USD. It then
            states a fixing of 140 USD per GOLD, payoff 40 USD to Alice, residual reserve 460 USD to Bob, and
            Alice's result before other costs of 35 USD. The arithmetic in that illustration is 140 minus 100
            equals 40, times notional 1 equals 40 USD, 500 minus 40 equals 460 USD, and 40 minus the separate 5 USD
            premium equals 35 USD. Missing or disputed fixing, in that proposal, retains the 500 USD reserve and
            the exercise and settlement duty. The beta file's payoff is 100.00 USD (10000 atoms), Bob is unused,
            and there is no premium quantity. Those horizon figures are not a result from <code>derivatives.mori</code>.
          </p>
        </details>
      </section>

      <section id="oracles" className="guide-section" aria-labelledby="oracles-title" style={sectionStyle}>
        <h2 id="oracles-title" style={headingStyle}>Oracles</h2>
        <p style={proseStyle}>
          The oracle example is <DocLink path="packages/moriarty-beta/examples/oracle.mori">oracle.mori</DocLink>,
          profile <code>moriarty-beta/1</code>, agreement <code>OracleDemo</code>. One action,{' '}
          <code>read_fixing</code>, selects a named observation. The selection is authoring. It does not publish
          a price.
        </p>
        <Boundary action="read_fixing" />
        <SourceFrame
          title="Excerpt"
          filename="oracle.mori"
          role="Excerpt, not a runnable file"
          source={oracleExcerpt}
          copyName="oracle excerpt"
          note="Copy excerpt copies only these lines. The complete file is in the details block."
        />

        <h3 id="oracles-use" style={headingStyle}>Economic use</h3>
        <p style={proseStyle}>
          A later payment may need a GOLD price quoted in USD. This file names that input <code>Fixing</code> on
          domain Preview (economic id Midnight, chain midnight, network preview). The unit string is{' '}
          <code>USD per GOLD</code>. The source string is <code>approved-feed</code>. <code>observed_round</code> is
          the scalar 140 and <code>maximum_age</code> is the scalar 5. Finality is the string <code>finalized</code>.
          Provenance is the string <code>provider binding open</code>. The intent <code>Read</code> calls{' '}
          <code>oracle.select</code> on that observation. Its failure string is{' '}
          <code>ObservationStale or ObservationDisputed</code>. Its continuation string is{' '}
          <code>Pending retains dependent duty</code>.
        </p>
        <p style={proseStyle}>
          The use is to name the issuer, the domain, the freshness bound and the trust assumption beside the
          observation, so a dependent duty can stay pending while the observation is missing, stale or disputed.
          In this file the issuer is not an account. The source field is a string. The domain is the observation's
          domain. Freshness is two scalars stored side by side. Trust is the finality string and the provenance
          string. Check stores those fields. It does not subtract 140 from a head, and it does not accept or reject
          the feed.
        </p>

        <h3 id="oracles-amounts" style={headingStyle}>Identities and amounts</h3>
        <ScrollTable
          label="Oracle identities"
          caption="Declarations in oracle.mori. The select operation's only argument is the observation. USD and GOLD are declared and are not arguments."
        >
          <thead>
            <tr>
              <th scope="col" style={cellStyle}>Declaration</th>
              <th scope="col" style={cellStyle}>Alias</th>
              <th scope="col" style={cellStyle}>Economic id</th>
              <th scope="col" style={cellStyle}>Scale or scalar</th>
              <th scope="col" style={cellStyle}>Role in oracle.select</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row" style={cellStyle}>Domain</th>
              <td style={cellStyle}>Preview</td>
              <td style={cellStyle}>Midnight</td>
              <td style={cellStyle}>chain midnight, network preview</td>
              <td style={cellStyle}>Domain of Fixing. Network string is an authored claim.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Account</th>
              <td style={cellStyle}>Alice</td>
              <td style={cellStyle}>Alice</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>Declared. Not an argument.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Account</th>
              <td style={cellStyle}>Bob</td>
              <td style={cellStyle}>Bob</td>
              <td style={cellStyle}>n/a</td>
              <td style={cellStyle}>Declared. Not an argument.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Asset</th>
              <td style={cellStyle}>USD</td>
              <td style={cellStyle}>USDCanonical</td>
              <td style={cellStyle}>scale 2, symbol USD</td>
              <td style={cellStyle}>Declared. The unit string does not reference this alias.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Asset</th>
              <td style={cellStyle}>GOLD</td>
              <td style={cellStyle}>GoldCanonical</td>
              <td style={cellStyle}>scale 3, symbol GOLD</td>
              <td style={cellStyle}>Declared. The unit string does not reference this alias.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Observation</th>
              <td style={cellStyle}>Fixing</td>
              <td style={cellStyle}>Fixing</td>
              <td style={cellStyle}>observed_round 140, maximum_age 5</td>
              <td style={cellStyle}>The only operation argument.</td>
            </tr>
          </tbody>
        </ScrollTable>
        <p style={proseStyle}>
          There is no fee, no recipient and no quantity on <code>oracle.select</code>. The schema is{' '}
          <code>observation</code> only. USD at scale 2 and GOLD at scale 3 are available for a later money intent.
          This action does not spend them. A document digest is not a field of this observation. The SHA-256 of the
          program file, below, identifies the program bytes. It is not a price of GOLD, and it is not evidence that{' '}
          <code>approved-feed</code> issued round 140.
        </p>

        <Split id="oracles-split">
          <p style={proseStyle}>
            The observation's domain must agree with any other entity argument. Here the only entity argument is
            Fixing, whose domain is Preview. An intent header <code>domain</code>, <code>asset</code> or{' '}
            <code>signer</code>, if added, must agree with that domain. This intent has none of those headers.
            A hash written into <code>provenance</code>, or a policy <code>source_hash</code> on another agreement,
            remains an authored string. <DocLink path="packages/moriarty-beta/GETTING-STARTED.md">GETTING-STARTED.md</DocLink> treats
            source <code>source_hash</code> and <code>policy_digest</code> as unverified claims. Matching hashes do
            not establish authority. The product contract treats an external observation as a named assumption.
            It does not substitute for the program and transition proof.
          </p>
        </Split>

        <h3 id="oracles-pattern" style={headingStyle}>Authoring pattern</h3>
        <p style={proseStyle}>
          Declare the observation before the intent. Required observation fields are <code>id</code> and{' '}
          <code>domain</code>. Optional stored fields used here are <code>unit</code>, <code>source</code>,{' '}
          <code>observed_round</code>, <code>maximum_age</code>, <code>finality</code> and <code>provenance</code>.
          Rounds and age are scalars. The other four are strings. The intent's operation is{' '}
          <code>oracle.select(observation: Fixing)</code>. Failure and continuation are optional strings on the
          intent, which this file fills in. One action, <code>read_fixing</code>, uses that intent.
        </p>
        <p style={proseStyle}>
          The pattern keeps the price input as an observation record rather than a number inside a payment.
          Dependent work can name the same observation. This file's failure and continuation strings are the
          author's statement of stale, disputed and pending behavior. They are not a case split the checker runs.
          The options file has a separate Fixing observation: source <code>feed</code>, observed round 150, and no
          maximum age. Same declaration shape, different claims. See <a href="#options">Options</a>.
        </p>

        <h3 id="oracles-hostile" style={headingStyle}>Hostile case</h3>
        <div className="guide-callout">
          <p style={proseStyle}>
            <code>finalized</code> and <code>provider binding open</code> are strings. Check accepts them for any
            other strings of the same type. <code>observed_round</code> 140 and <code>maximum_age</code> 5 are
            independent scalars. The checker does not compute an age and does not compare either scalar with a
            ledger round. The failure string names ObservationStale and ObservationDisputed. Expand and simulate
            still return Unsupported before a scenario is applied, so those names are not rejection codes produced
            by this action.
          </p>
          <p style={proseStyle}>
            There is no <code>issuer</code> field on an observation. A grant record can name an issuer account, and
            this file declares no grant. Writing a document hash into provenance would identify bytes the author
            claimed. It would not make the unit string a USD-per-GOLD quote of GoldCanonical, and it would not
            discharge the continuation's dependent duty. No fee recipient exists to redirect, because no fee is declared.
          </p>
        </div>

        <CheckCommands
          headingId="oracles-command"
          file="oracle.mori"
          derived={'AuthoringChecked OracleDemo\n  read_fixing: SpecifiedOnly (execution unsupported; financial relations open)\n'}
        />

        <h3 id="oracles-file" style={headingStyle}>Complete file</h3>
        <details>
          <summary>Complete oracle.mori, 897 bytes</summary>
          <p style={proseStyle}>
            SHA-256 of the shipped file bytes is <code style={{ overflowWrap: 'anywhere' }}>{ORACLE_SHA256}</code>.
            That digest is the program text. It is not the observation's value and it is not the string{' '}
            <code>approved-feed</code>.
          </p>
          <SourceFrame
            title="Complete program"
            filename="oracle.mori"
            role="Complete program"
            source={oracleSource}
            copyName="oracle.mori program"
            downloadName="oracle.mori"
          />
        </details>
        <p style={proseStyle}>
          Repository evidence: <DocLink path="packages/moriarty-beta/examples/oracle.mori">oracle.mori</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/src/frontend.ts">frontend.ts</DocLink> observation fields,{' '}
          <DocLink path="packages/moriarty-beta/README.md">package README</DocLink>,{' '}
          <DocLink path="docs/MORIARTY-PRODUCT-CONTRACT.md">product contract</DocLink>.
        </p>

        <h3 id="oracles-horizon" style={headingStyle}>Proposed lifecycle</h3>
        <details data-copy="none">
          <summary>Proposed observation qualification. Not moriarty-beta/1 and not a runnable file</summary>
          <p style={proseStyle}>
            The horizon section "Oracles and observations" specifies a typed select with Final, Missing, Stale and
            Disputed results, a requested round, and a maximum age compared with a current round. Its concrete
            candidate uses <code>price&lt;USD,GOLD,2&gt;(14000)</code>. Because that document reads{' '}
            <code>Price&lt;USD,GOLD,2&gt;(10000)</code> as 100 USD per GOLD, the same scale reads 14000 as 140.00 USD
            per GOLD. The candidate's observed round is 150 and received round is 151. The document says age 1 at
            round 151 meets maximum age 5, and round 156 is Stale (156 minus 150 equals 6). It also says the value
            and time remain claims until qualification, a policy record does not compute a median, and no timestamp
            authenticates a feed. A selector digest in that proposal binds the query identity. It does not by itself
            make the price true.
          </p>
          <p style={proseStyle}>
            The beta observation is a different record: observed round 140, maximum age 5, no current round, and no
            mantissa. The horizon age arithmetic does not apply to <code>oracle.mori</code>.
          </p>
        </details>
      </section>

      <section id="governance" className="guide-section" aria-labelledby="governance-title" style={sectionStyle}>
        <h2 id="governance-title" style={headingStyle}>Governance</h2>
        <p style={proseStyle}>
          The governance example is <DocLink path="packages/moriarty-beta/examples/governance.mori">governance.mori</DocLink>,
          profile <code>moriarty-beta/1</code>, agreement <code>GovernanceDemo</code>. Three actions share one policy:
          queue, execute and veto. The file is an authoring sketch of a delayed policy change. Maintainer review
          stays outside this agreement.
        </p>
        <Boundary action="queue, execute and veto" />
        <SourceFrame
          title="Excerpt"
          filename="governance.mori"
          role="Excerpt, not a runnable file"
          source={governanceExcerpt}
          copyName="governance excerpt"
          note="Copy excerpt copies only these lines. The complete file is in the details block."
        />

        <h3 id="governance-use" style={headingStyle}>Economic use</h3>
        <p style={proseStyle}>
          Policy Terms is at epoch scalar 3. Its id string is <code>Terms</code>. <code>source_hash</code> is the
          authored string <code>terms-v3</code>. <code>duty_preservation</code> is the string <code>required</code>.
          Queue asks for <code>next_epoch</code> 4, with <code>earliest_round</code> 180 and <code>veto_before</code> 179.
          The relation string is <code>existing signed duties retain terms</code>. Execute and veto each name the
          same policy and add no round fields.
        </p>
        <p style={proseStyle}>
          The intended policy change would become eligible at round 180. These authored integers and strings do not enforce that behavior.
          The sketch names a veto scalar 179 on the queue intent. The relation string says existing signed duties retain the terms they were signed under.
          180 minus 179 equals 1. Those are adjacent authored integers. Check requires each of epoch, next_epoch, earliest_round
          and veto_before to be an unsigned scalar when present. It does not order them, and it does not load a clock.
        </p>

        <h3 id="governance-amounts" style={headingStyle}>Identities and amounts</h3>
        <ScrollTable
          label="Governance identities and round scalars"
          caption="GovernanceDemo declarations. No asset quantity is an argument of queue, execute or veto. terms-v3 is a claim string, not the file digest."
        >
          <thead>
            <tr>
              <th scope="col" style={cellStyle}>Field</th>
              <th scope="col" style={cellStyle}>Spelling</th>
              <th scope="col" style={cellStyle}>Where it sits</th>
              <th scope="col" style={cellStyle}>What check does with it</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row" style={cellStyle}>Policy id</th>
              <td style={cellStyle}>Terms</td>
              <td style={cellStyle}>policy Terms</td>
              <td style={cellStyle}>Stored string. Argument of all three operations.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Epoch</th>
              <td style={cellStyle}>3</td>
              <td style={cellStyle}>policy Terms</td>
              <td style={cellStyle}>Scalar. Not compared with next_epoch.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Next epoch</th>
              <td style={cellStyle}>4</td>
              <td style={cellStyle}>governance.queue argument</td>
              <td style={cellStyle}>Required scalar argument of queue only.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Earliest round</th>
              <td style={cellStyle}>180</td>
              <td style={cellStyle}>Queue intent field</td>
              <td style={cellStyle}>Optional scalar on the intent. Not an argument of execute.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Veto before</th>
              <td style={cellStyle}>179</td>
              <td style={cellStyle}>Queue intent field</td>
              <td style={cellStyle}>Optional scalar on the intent. Not an argument of veto.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Source hash claim</th>
              <td style={cellStyle}>terms-v3</td>
              <td style={cellStyle}>policy string</td>
              <td style={cellStyle}>Stored string. Not compared with the computed file digest.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Duty preservation</th>
              <td style={cellStyle}>required</td>
              <td style={cellStyle}>policy string</td>
              <td style={cellStyle}>Stored string. Does not preserve a duty.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Relation</th>
              <td style={cellStyle}>existing signed duties retain terms</td>
              <td style={cellStyle}>Queue intent string</td>
              <td style={cellStyle}>Stored string.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>USD and GOLD</th>
              <td style={cellStyle}>USDCanonical scale 2, GoldCanonical scale 3</td>
              <td style={cellStyle}>asset declarations</td>
              <td style={cellStyle}>Declared. No fee or payoff quantity uses them here.</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>Alice and Bob</th>
              <td style={cellStyle}>account ids Alice and Bob</td>
              <td style={cellStyle}>account declarations on Preview</td>
              <td style={cellStyle}>Declared. Not signers and not operation arguments.</td>
            </tr>
          </tbody>
        </ScrollTable>

        <Split id="governance-split">
          <p style={proseStyle}>
            Queue, execute and veto are separate intents, so a signature on one action's bytes would not be a
            signature on the other two. This file has no signer on any of them. The hard constraint that is present
            is nominal: one policy reference, and a scalar next epoch on queue. The file names no recipient and no fee.
            Domain Preview is declared with economic id Midnight. The policy record has no domain field, and queue,
            execute and veto do not take a domain argument, so the operation-argument domain comparison has no second
            domain to compare. A signer added together with an intent domain header must be an account on that domain.
            This file names neither a signer nor an intent domain.
          </p>
        </Split>

        <h3 id="governance-pattern" style={headingStyle}>Authoring pattern</h3>
        <p style={proseStyle}>
          A policy record may be empty and still parse. This file fills <code>id</code>, <code>epoch</code>,{' '}
          <code>source_hash</code> and <code>duty_preservation</code>. Queue's operation arguments are exactly{' '}
          <code>policy</code> and <code>next_epoch</code>. The timelock scalars sit on the intent, beside the call,
          which is why they can be present on Queue and absent on Execute and Veto. Execute and veto take only{' '}
          <code>policy</code>. Each action uses its own intent.
        </p>
        <ScrollTable label="Governance operation schemas" caption="Required arguments from the closed schema map in frontend.ts.">
          <thead>
            <tr>
              <th scope="col" style={cellStyle}>Action</th>
              <th scope="col" style={cellStyle}>Operation</th>
              <th scope="col" style={cellStyle}>Required arguments</th>
              <th scope="col" style={cellStyle}>Round fields on that intent</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row" style={cellStyle}>queue</th>
              <td style={cellStyle}>governance.queue</td>
              <td style={cellStyle}>policy, next_epoch</td>
              <td style={cellStyle}>earliest_round 180, veto_before 179</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>execute</th>
              <td style={cellStyle}>governance.execute</td>
              <td style={cellStyle}>policy</td>
              <td style={cellStyle}>none</td>
            </tr>
            <tr>
              <th scope="row" style={cellStyle}>veto</th>
              <td style={cellStyle}>governance.veto</td>
              <td style={cellStyle}>policy</td>
              <td style={cellStyle}>none</td>
            </tr>
          </tbody>
        </ScrollTable>
        <p style={proseStyle}>
          The pattern is a queued change, a separate veto, and a separate execute, with the delay written on the
          queue intent. Empty <code>retained_duties</code> is not how this file states the duty. It uses the relation
          string and the policy string <code>required</code>. Getting started says those prose hints, including an
          empty policy, are stored authoring data. They do not establish duty preservation.
        </p>

        <h3 id="governance-hostile" style={headingStyle}>Hostile case</h3>
        <div className="guide-callout">
          <p style={proseStyle}>
            Execute's schema does not include <code>earliest_round</code>. Veto's schema does not include{' '}
            <code>veto_before</code>. Authoring the three actions in one agreement does not attach round 180 or
            round 179 to execute or veto. A reader can queue epoch 4 and, in a separate intent, execute Terms with
            no timelock field at all. Check accepts that shape. The string <code>terms-v3</code> is not the SHA-256
            of this file and is not compared with it. Replacing it with another short label still type-checks.
          </p>
          <p style={proseStyle}>
            No grant, signer list or quorum is declared. Alice and Bob are not given amend, veto or execute authority.
            A later signature would show that a key signed bytes. It would not verify that the key is the policy
            authority, and it would not admit anyone as a Moriarty maintainer. The horizon document says the public
            compiler requires no governance quorum or maintainer permission: those roles, when specified, are
            agreement roles. The product contract puts reviewer identity and campaign process in internal project
            work, outside the public toolchain prerequisite.
          </p>
        </div>

        <CheckCommands
          headingId="governance-command"
          file="governance.mori"
          derived={'AuthoringChecked GovernanceDemo\n  queue: SpecifiedOnly (execution unsupported; financial relations open)\n  execute: SpecifiedOnly (execution unsupported; financial relations open)\n  veto: SpecifiedOnly (execution unsupported; financial relations open)\n'}
        />

        <h3 id="governance-file" style={headingStyle}>Complete file</h3>
        <details>
          <summary>Complete governance.mori, 971 bytes</summary>
          <p style={proseStyle}>
            SHA-256 of the shipped file bytes is <code style={{ overflowWrap: 'anywhere' }}>{GOVERNANCE_SHA256}</code>.
            The policy field <code>source_hash</code> is the different string <code>terms-v3</code>. The two values
            sit next to each other so a hash of the program is not mistaken for a hash of the terms, and neither
            value is treated as proof that existing duties kept their old terms.
          </p>
          <SourceFrame
            title="Complete program"
            filename="governance.mori"
            role="Complete program"
            source={governanceSource}
            copyName="governance.mori program"
            downloadName="governance.mori"
          />
        </details>
        <p style={proseStyle}>
          Repository evidence: <DocLink path="packages/moriarty-beta/examples/governance.mori">governance.mori</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/GETTING-STARTED.md">GETTING-STARTED.md</DocLink>,{' '}
          <DocLink path="packages/moriarty-beta/SIGNED-INTENT.md">SIGNED-INTENT.md</DocLink>,{' '}
          <DocLink path="docs/MORIARTY-PRODUCT-CONTRACT.md">product contract</DocLink>,{' '}
          <DocLink path="wiki-llm/beta-language-2026-09-30/CONVERGENCE.md">CONVERGENCE.md</DocLink>.
          Related authoring: <a href="#options">Options</a> and <a href="#oracles">Oracles</a>.
        </p>

        <h3 id="governance-horizon" style={headingStyle}>Proposed lifecycle</h3>
        <details data-copy="none">
          <summary>Proposed queue, veto and execute timelock. Not moriarty-beta/1 and not a runnable file</summary>
          <p style={proseStyle}>
            The horizon section "Governance" specifies Queue, Veto and Execute as separate stages. Its illustration
            uses next epoch 4, earliest round 180 and veto through round 179. The proposal text requires a veto only
            while the current round is at most 179, and execute only when the round is at least 180, with a final
            no-veto fact. Enact changes which policy authorizes new commitments. The same section says it does not
            rewrite a stored debt, redemption or withdrawal duty. The beta relation string says the same thing in
            authoring prose and does not supply that proof.
          </p>
          <p style={proseStyle}>
            The horizon fixed terms use <code>digest("terms-v4")</code> as a next commitment. The beta policy string
            is <code>terms-v3</code>. They are different claims. Neither is the file digest{' '}
            <code style={{ overflowWrap: 'anywhere' }}>{GOVERNANCE_SHA256}</code>.
          </p>
        </details>
      </section>
    </>
  );
}

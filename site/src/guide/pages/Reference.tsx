import type { ReactNode } from 'react';
import { DocShell } from '../DocShell';
import { CodeDetails, FamilyStrip, SeeAlso, Section, Status, Table, type StatusName } from '../components';
import {
  BETA_PATH,
  FAMILIES,
  GETTING_STARTED,
  PACKAGE_VERSION,
  PAGE_HREF,
  PINNED,
  PINNED_SHORT,
  PRODUCT_CONTRACT,
  PROFILE,
  REPOSITORY,
  SIGNED_INTENT,
  blob,
} from '../data';
import { FAMILY_ENTRIES, REGISTRY, REGISTRY_MAP, type FamilyEntry } from '../reference/families';
import { CORE_CODES, DIAGNOSTIC_GROUPS, SOURCE6_CODES } from '../reference/diagnostics';
import transferScenario from '../lessons/transfer.scenario.json?raw';
import repayScenario from '../lessons/repay.scenario.json?raw';
import transferTests from '../lessons/transfer.test.json?raw';
import repayTests from '../lessons/repay.test.json?raw';

const E = PAGE_HREF.explanation;
const H = PAGE_HREF.howto;
const pkg = (path: string) => blob(`${BETA_PATH}/${path}`);

const GROUPS = ['Tool', 'Language', 'Results and support', 'Data formats', 'Operation families', 'Release'];

/** The contents line at the top: one link per part of the page, in page order. The sidebar lists every section. */
const CONTENTS: [string, string][] = [
  ['cli', 'Command line'],
  ['language', 'Source language'],
  ['results', 'Results'],
  ['diagnostics', 'Diagnostic codes'],
  ['support-labels', 'Support labels'],
  ['support-matrix', 'Support matrix'],
  ['premises-bindings', 'Premises and bindings'],
  ['scenario-format', 'Scenario format'],
  ['test-format', 'Test format'],
  ['family-amm', 'Operation families'],
  ['example-files', 'Example files'],
  ['release', 'Release'],
];

const NOWRAP = { whiteSpace: 'nowrap' } as const;

/**
 * Line breaking for code in narrow table cells. Lines break only at spaces, so a word such as `--write` or
 * `verify-intent` is never split at its hyphen. A word of 28 or more characters also gets break opportunities after
 * `_`, `/`, `,` and `-`, and between a lower-case and an upper-case letter. A <wbr> adds no character, so copied
 * text is unchanged.
 */
function breakable(s: string): ReactNode {
  return s.split(' ').map((word, i) => (
    <span key={i}>
      {i > 0 ? ' ' : null}
      {word.length < 28 ? (
        <span style={NOWRAP}>{word}</span>
      ) : (
        word.split(/(?<=[_/,-])|(?<=[a-z])(?=[A-Z])/).map((part, j) => (
          <span key={j} style={NOWRAP}>
            {j > 0 ? <wbr /> : null}
            {part}
          </span>
        ))
      )}
    </span>
  ));
}

function C({ children }: { children: ReactNode }) {
  return <code>{typeof children === 'string' ? breakable(children) : children}</code>;
}

/** A SHA-256 digest with a break opportunity every 16 hexadecimal characters. */
function Digest({ hex }: { hex: string }) {
  return (
    <code>
      {hex.match(/.{1,16}/g)!.map((part, i) => (
        <span key={i}>
          {i > 0 ? <wbr /> : null}
          {part}
        </span>
      ))}
    </code>
  );
}

/** A label chip. Inside the support-labels and support-matrix sections it is rendered without a link to itself. */
function Chip({ name, link = true }: { name: StatusName; link?: boolean }) {
  return <Status name={name} link={link} />;
}

/** A comma-separated list of code spans. */
function Codes({ items }: { items: string[] }) {
  if (items.length === 0) return <>none</>;
  return (
    <>
      {items.map((x, i) => (
        <span key={x + i}>
          {i > 0 ? ', ' : null}
          <C>{x}</C>
        </span>
      ))}
    </>
  );
}

function Contents() {
  return (
    <nav aria-label="Contents of this page">
      <p>
        Facts about the <C>mori</C> tool and the source language <C>{PROFILE}</C>, at commit{' '}
        <a href={`${REPOSITORY}/tree/${PINNED}`}>{PINNED_SHORT}</a>. What a local result does and does not establish is
        explained in <a href={`${E}#stance`}>Explanation: what these pages do not claim</a>.
      </p>
      <p className="doc-contents">
        {CONTENTS.map(([id, label], i) => (
          <span key={id}>
            {i > 0 ? ' · ' : null}
            <a href={`#${id}`}>{label}</a>
          </span>
        ))}
      </p>
    </nav>
  );
}

/* ------------------------------------------------------------------ Tool */

function Cli() {
  /** [command, synopsis, effect, output, exit] */
  const rows: [string, string, ReactNode, ReactNode, ReactNode][] = [
    [
      'init',
      'init DIR [--template transfer|repay]',
      <>
        Creates <C>DIR</C> and writes <C>invoice.mori</C> (template <C>transfer</C>, the default) or{' '}
        <C>repayment.mori</C> (template <C>repay</C>), plus <C>scenario.json</C>, <C>mori.tests.json</C> and{' '}
        <C>README.md</C>. Refuses an existing <C>DIR</C> with <C>BETA_INIT_EXISTS</C>.
      </>,
      <>
        JSON <C>{'{status: "Initialized", directory, qualification: "local-stipulation-only"}'}</C>
      </>,
      '0, 1',
    ],
    [
      'check',
      'check FILE [--json]',
      'Parses and checks the source.',
      <>
        Text: <C>STATUS AGREEMENT</C>, one line <C>CODE at LINE:COLUMN: MESSAGE</C> per diagnostic, one line{' '}
        <C>ACTION: LABEL</C> per action. With <C>--json</C>: the analysis record.
      </>,
      <>
        0 <C>AuthoringChecked</C>
        <br />1 <C>AuthoringRejected</C>
      </>,
    ],
    [
      'fmt',
      'fmt FILE [--write]',
      <>
        Formats the source. Prints it, or rewrites <C>FILE</C> with <C>--write</C>.
      </>,
      <>
        Formatted text; on rejection JSON <C>{'{status: "AuthoringRejected", diagnostics}'}</C>
      </>,
      '0, 1',
    ],
    [
      'inspect',
      'inspect FILE',
      'Lists identities, actions with support and bounds, intents with their terms, coverage, premises and bindings.',
      'JSON',
      <>
        0 <C>AuthoringChecked</C>
        <br />1 <C>AuthoringRejected</C>, <C>InspectionRejected</C>
      </>,
    ],
    [
      'expand',
      'expand FILE --action NAME --scenario FILE',
      'Generates the Source/6 program for one action from the source and the scenario.',
      <>
        JSON with <C>status</C>, <C>sourceHash</C>, <C>scenarioHash</C>, <C>source6</C>, <C>fieldMap</C>,{' '}
        <C>origins</C>, <C>proposalNote</C>, <C>qualification</C>
      </>,
      <>
        0 <C>Expanded</C>
        <br />1 otherwise
      </>,
    ],
    [
      'simulate',
      'simulate FILE --action NAME --scenario FILE',
      'Expands the action, then runs Core on the generated program.',
      <>
        JSON with <C>status</C>, <C>sourceHash</C>, <C>scenarioHash</C>, <C>result</C>, <C>qualification</C>
      </>,
      <>
        0 <C>PreparedUnqualified</C>
        <br />1 otherwise
      </>,
    ],
    [
      'test',
      'test DIR',
      <>
        Runs every case in <C>DIR/mori.tests.json</C>.
      </>,
      <>
        JSON report; see <a href="#test-format">test file format</a>
      </>,
      <>
        0 <C>TestsPassed</C>
        <br />1 otherwise
      </>,
    ],
    [
      'intent',
      'intent FILE --action NAME --scenario FILE --scheme SCHEME --public-key HEX --framing raw|midnight-sign-data --crypto-binary /ABS/BINARY [--review|--json]',
      'Builds the canonical intent statement and the exact signing message through the native binary.',
      <>
        JSON with status <C>OwnerIntentPrepared</C>, or review text with <C>--review</C>
      </>,
      <>
        0 <C>OwnerIntentPrepared</C>
        <br />1 otherwise
        <br />2 <C>BETA_CRYPTO_*</C>
      </>,
    ],
    [
      'verify-intent',
      'verify-intent FILE --action NAME --scenario FILE --signature FILE --crypto-binary /ABS/BINARY [--review|--json]',
      'Checks the external signature through the native binary, then runs local preparation.',
      <>
        JSON with status <C>SignedPreparedUnqualified</C>, <C>SignatureRejected</C> or <C>SignedCoreRejected</C>, or
        review text
      </>,
      <>
        0 <C>SignedPreparedUnqualified</C>
        <br />1 otherwise
        <br />2 <C>BETA_CRYPTO_*</C>
      </>,
    ],
    ['lsp', 'lsp', 'Runs the language server on standard input and output. Takes no arguments.', 'LSP messages', '1 if given arguments'],
    ['mcp', 'mcp', 'Runs the MCP server on standard input and output. Takes no arguments.', 'MCP messages', '1 if given arguments'],
    ['help', 'help | --help | (no command)', 'Prints the usage line for all commands.', 'Text', '0'],
    [
      'COMMAND --help',
      'COMMAND --help | COMMAND -h',
      'Prints the synopsis of one command.',
      <>
        Text <C>mori SYNOPSIS</C>
      </>,
      <>
        0
        <br />1 unknown command
      </>,
    ],
  ];
  return (
    <Section id="cli" title="Command-line tool" nav="Command-line tool" group="Tool">
      <p>
        <C>mori</C> is the <C>bin</C> entry of package <C>@moriarty-lang/beta</C> (<C>dist/cli.js</C>). It requires
        Node 24 or later. Source: <a href={pkg('src/cli.ts')}>src/cli.ts</a>. Every help text ends with the line{' '}
        <C>Local financial results remain PreparedUnqualified.</C>
      </p>
      <Table caption="Commands. Each command is written after mori.">
        <thead>
          <tr>
            <th scope="col">Command</th>
            <th scope="col">Synopsis, effect and output</th>
            <th scope="col">Exit</th>
          </tr>
        </thead>
        <tbody>
          {rows.map(([cmd, synopsis, effect, out, exit]) => (
            <tr key={cmd}>
              <th scope="row">
                <C>{cmd}</C>
              </th>
              <td>
                <C>{synopsis}</C>
                <br />
                {effect}
                <br />
                Output: {out}
              </td>
              <td>{exit}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Option values">
        <thead>
          <tr>
            <th scope="col">Option</th>
            <th scope="col">Commands</th>
            <th scope="col">Value</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="row"><C>--template</C></th>
            <td><C>init</C></td>
            <td><C>transfer</C> (default) or <C>repay</C></td>
          </tr>
          <tr>
            <th scope="row"><C>--json</C></th>
            <td><C>check</C>, <C>intent</C>, <C>verify-intent</C></td>
            <td>Flag. JSON output. For <C>intent</C> and <C>verify-intent</C> it is the default.</td>
          </tr>
          <tr>
            <th scope="row"><C>--write</C></th>
            <td><C>fmt</C></td>
            <td>Flag. Rewrites the file in place.</td>
          </tr>
          <tr>
            <th scope="row"><C>--action</C></th>
            <td><C>expand</C>, <C>simulate</C>, <C>intent</C>, <C>verify-intent</C></td>
            <td>An action name declared in the file. Required.</td>
          </tr>
          <tr>
            <th scope="row"><C>--scenario</C></th>
            <td><C>expand</C>, <C>simulate</C>, <C>intent</C>, <C>verify-intent</C></td>
            <td>Path to a <a href="#scenario-format">scenario file</a>. Required.</td>
          </tr>
          <tr>
            <th scope="row"><C>--scheme</C></th>
            <td><C>intent</C></td>
            <td><C>schnorr_bip340</C> or <C>ecdsa_secp256k1_sha256</C></td>
          </tr>
          <tr>
            <th scope="row"><C>--public-key</C></th>
            <td><C>intent</C></td>
            <td>Lowercase hexadecimal without <C>0x</C>: 64 characters for <C>schnorr_bip340</C>, 66 characters (compressed key) for <C>ecdsa_secp256k1_sha256</C></td>
          </tr>
          <tr>
            <th scope="row"><C>--framing</C></th>
            <td><C>intent</C></td>
            <td><C>raw</C> or <C>midnight-sign-data</C></td>
          </tr>
          <tr>
            <th scope="row"><C>--signature</C></th>
            <td><C>verify-intent</C></td>
            <td>Path to a JSON file with exactly <C>statement</C> and <C>signatureHex</C> (128 lowercase hexadecimal characters)</td>
          </tr>
          <tr>
            <th scope="row"><C>--crypto-binary</C></th>
            <td><C>intent</C>, <C>verify-intent</C></td>
            <td>Absolute path to the native verifier. Never looked up on <C>PATH</C>. Required.</td>
          </tr>
          <tr>
            <th scope="row"><C>--review</C></th>
            <td><C>intent</C>, <C>verify-intent</C></td>
            <td>Flag. Readable review text. Cannot be combined with <C>--json</C>.</td>
          </tr>
        </tbody>
      </Table>
      <Table caption="Argument and file rules">
        <thead>
          <tr>
            <th scope="col">Rule</th>
            <th scope="col">On violation</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Exactly one path argument (none for <C>lsp</C> and <C>mcp</C>).</td>
            <td><C>mori: COMMAND requires one path</C></td>
          </tr>
          <tr>
            <td>Each option at most once, and only the options listed for the command.</td>
            <td><C>mori: Unknown or duplicate option OPTION</C></td>
          </tr>
          <tr>
            <td>A value option is followed by a value that does not start with <C>--</C>.</td>
            <td><C>mori: Missing value for OPTION</C></td>
          </tr>
          <tr>
            <td>Input files are at most 65536 bytes.</td>
            <td><C>mori: File exceeds 65536 bytes</C></td>
          </tr>
          <tr>
            <td>Input files are valid UTF-8 without a byte order mark.</td>
            <td><C>BETA_BOM</C> or <C>BETA_UTF8</C>, status <C>FormationRejected</C></td>
          </tr>
          <tr>
            <td>Relative paths resolve against the current directory.</td>
            <td>File system error, for example <C>mori: ENOENT: no such file or directory, stat 'PATH'</C></td>
          </tr>
        </tbody>
      </Table>
      <Table caption="Exit codes">
        <thead>
          <tr>
            <th scope="col">Code</th>
            <th scope="col">Meaning</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="row">0</th>
            <td>The requested local operation completed. A preparation remains unqualified.</td>
          </tr>
          <tr>
            <th scope="row">1</th>
            <td>Rejection, unsupported execution, test mismatch, or a command or file error.</td>
          </tr>
          <tr>
            <th scope="row">2</th>
            <td>
              <C>intent</C> or <C>verify-intent</C> failed at the native verifier: any code starting with{' '}
              <C>BETA_CRYPTO_</C>.
            </td>
          </tr>
        </tbody>
      </Table>
      <p>
        A diagnostic failure outside source analysis prints JSON to standard output:{' '}
        <C>{'{status: "FormationRejected", diagnostics: [{code, message}], publishedEffects: null, publishedPost: null}'}</C>.
        Any other error prints <C>mori: MESSAGE</C> to standard error. <C>fmt --write</C> changes the file's bytes and
        therefore its computed <C>sourceHash</C>; it never changes the <C>source_hash</C> or <C>policy_digest</C> claims
        written in an intent.
      </p>
      <SeeAlso
        items={[
          { mode: 'howto', href: `${H}#format-and-inspect`, label: 'How to format a file and list its signed terms' },
          { mode: 'howto', href: `${H}#diagnose`, label: 'How to diagnose a failed check or test' },
        ]}
      />
    </Section>
  );
}

/* ------------------------------------------------------------------ Language */

const ROLE_MEANING: [string, string][] = [
  ['account, asset, domain, obligation, pool, instrument, observation, policy, share_class', 'A reference to an earlier declaration of that kind.'],
  ['qty', 'A quantity: a literal such as 10.00 USD, an atoms(...) call, or an expression of one asset.'],
  ['qty[]', 'An array of quantities.'],
  ['scalar', 'An unsigned integer.'],
  ['numeric', 'A scalar or a quantity. Both arguments of min and max have the same type and asset.'],
  ['string', 'A string literal.'],
];

const SOURCE6_RESERVED =
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head';

function Language() {
  return (
    <Section id="language" title={`Source language: ${PROFILE}`} nav="Source language" group="Language">
      <p>
        Source: <a href={pkg('src/frontend.ts')}>src/frontend.ts</a>. A file is one program:{' '}
        <C>{'profile "moriarty-beta/1";'}</C>, then <C>{'agreement NAME { DECLARATIONS }'}</C>, then end of file.
        A declaration is <C>KIND NAME = EXPRESSION;</C> or <C>KIND NAME: TYPE = EXPRESSION;</C>. An action is{' '}
        <C>action NAME uses INTENT;</C>. An expression refers only to declarations written above it. Declaration and
        action names are unique within the agreement.
      </p>
      <Table caption="Lexical elements">
        <thead>
          <tr>
            <th scope="col">Element</th>
            <th scope="col">Form</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row">Whitespace</th><td>Space, tab, carriage return, line feed.</td></tr>
          <tr><th scope="row">Comment</th><td><C>//</C> to end of line, or <C>/* ... */</C> (not nested).</td></tr>
          <tr><th scope="row">String</th><td>JSON string syntax and escapes. Decoded length at most 1024 bytes. No lone surrogates.</td></tr>
          <tr>
            <th scope="row">Number</th>
            <td>
              Decimal digits with an optional fraction. <C>_</C> may separate digit groups (<C>1_000</C>). No leading
              zero, sign or exponent.
            </td>
          </tr>
          <tr><th scope="row">Identifier</th><td>An ASCII letter, then ASCII letters, digits or <C>_</C>; at most 64 characters.</td></tr>
          <tr><th scope="row">Punctuation</th><td><C>{'{ } [ ] ( ) : ; , = < > . + - *'}</C></td></tr>
          <tr>
            <th scope="row">Other</th>
            <td>Any other character outside a string or comment, including <C>/</C>, is rejected with <C>BETA_CHARACTER</C>.</td>
          </tr>
        </tbody>
      </Table>
      <Table caption="Declaration kinds and their fields">
        <thead>
          <tr>
            <th scope="col">Kind</th>
            <th scope="col">Required fields</th>
            <th scope="col">Optional fields</th>
            <th scope="col">Checks</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>domain</C></th><td><Codes items={['id', 'chain', 'network']} /></td><td>none</td><td>All strings. <C>id</C> is an economic id, unique among domains.</td></tr>
          <tr><th scope="row"><C>account</C></th><td><Codes items={['domain', 'id']} /></td><td>none</td><td><C>id</C> is an economic id, unique per domain.</td></tr>
          <tr><th scope="row"><C>asset</C></th><td><Codes items={['domain', 'id', 'scale', 'representation']} /></td><td><Codes items={['symbol']} /></td><td><C>scale</C> 0 to 18. <C>id</C> unique per domain. <C>symbol</C> is display text.</td></tr>
          <tr><th scope="row"><C>obligation</C></th><td><Codes items={['domain', 'id', 'asset']} /></td><td>none</td><td>The asset is on the same domain. <C>id</C> unique per domain.</td></tr>
          <tr><th scope="row"><C>const</C></th><td colSpan={2}>Any expression.</td><td>Optional type annotation is checked.</td></tr>
          <tr><th scope="row"><C>intent</C></th><td colSpan={2}>See <a href="#intent-fields">intent fields</a>.</td><td>Requires an <C>operation</C> call.</td></tr>
          <tr><th scope="row"><C>pool</C></th><td><Codes items={['domain', 'id', 'assets']} /></td><td>none</td><td><C>assets</C> is a nonempty array of distinct assets on the pool's domain.</td></tr>
          <tr><th scope="row"><C>share_class</C></th><td><Codes items={['domain', 'id', 'backing']} /></td><td>none</td><td><C>id</C> unique per domain.</td></tr>
          <tr><th scope="row"><C>instrument</C></th><td><Codes items={['domain', 'id']} /></td><td><Codes items={['asset', 'backing', 'underlying', 'settlement', 'strike_units', 'strike', 'exercise_round', 'collateral']} /></td><td>Asset fields reference assets on the same domain.</td></tr>
          <tr><th scope="row"><C>observation</C></th><td><Codes items={['id', 'domain']} /></td><td><Codes items={['unit', 'source', 'observed_round', 'maximum_age', 'finality', 'provenance']} /></td><td>Rounds and age are scalars; the rest are strings.</td></tr>
          <tr><th scope="row"><C>policy</C></th><td>none</td><td><Codes items={['id', 'epoch', 'source_hash', 'duty_preservation']} /></td><td><C>epoch</C> is a scalar; the rest are strings.</td></tr>
          <tr><th scope="row"><C>grant</C></th><td>none</td><td><Codes items={['issuer', 'scope', 'expiry', 'revocation_epoch', 'gross_limit', 'work_limit', 'signers', 'evidence']} /></td><td><C>issuer</C> is an account; <C>signers</C> an account array.</td></tr>
          <tr><th scope="row"><C>party</C></th><td><Codes items={['id', 'account']} /></td><td>none</td><td><C>account</C> is an account.</td></tr>
          <tr>
            <th scope="row"><C>stage</C></th>
            <td><Codes items={['domain']} /></td>
            <td><Codes items={['id', 'trigger', 'paired_claim', 'authority', 'reads', 'writes', 'duty', 'status', 'amount', 'evidence', 'relation', 'signed_floor', 'failure', 'operation', 'completion', 'rounding', 'retained_duties']} /></td>
            <td><C>signed_floor</C> is a string or a quantity on the stage domain.</td>
          </tr>
          <tr><th scope="row"><C>episode</C></th><td><Codes items={['id', 'stages']} /></td><td><Codes items={['pending', 'timeout', 'recovery', 'failure', 'duty', 'authority']} /></td><td><C>stages</C> is an array of stages.</td></tr>
          <tr><th scope="row"><C>action</C></th><td colSpan={2}><C>action NAME uses INTENT;</C></td><td>Support is <C>LocalS0</C> when the intent's operation is <C>transfer</C> or <C>repay</C>, else <C>SpecifiedOnly</C>.</td></tr>
        </tbody>
      </Table>
      <Table caption="Expression forms">
        <thead>
          <tr>
            <th scope="col">Form</th>
            <th scope="col">Example</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row">String</th><td><C>"Midnight"</C></td></tr>
          <tr><th scope="row">Boolean</th><td><C>true</C>, <C>false</C></td></tr>
          <tr><th scope="row">Scalar</th><td><C>10</C></td></tr>
          <tr><th scope="row">Quantity</th><td><C>10.00 USD</C>, <C>atoms(asset: USD, value: 1000)</C></td></tr>
          <tr><th scope="row">Tag</th><td><C>None</C>, <C>SuccessOnly</C></td></tr>
          <tr><th scope="row">Array</th><td><C>[USD, GOLD]</C>; a trailing comma is allowed</td></tr>
          <tr><th scope="row">Record</th><td><C>{'{ domain: Preview, id: "A" }'}</C>; at most 64 fields; a trailing comma is allowed</td></tr>
          <tr><th scope="row">Call</th><td><C>rounds(domain: Preview, from: 0, to: 10)</C>; named arguments only</td></tr>
          <tr><th scope="row">Reference</th><td><C>Preview</C>: the value of an earlier declaration</td></tr>
          <tr><th scope="row">Arithmetic</th><td><C>price + fee</C>; see <a href="#quantities">quantities</a></td></tr>
          <tr><th scope="row">Parentheses</th><td><C>(a + b) * 2</C></td></tr>
        </tbody>
      </Table>
      <Table caption="Type annotations">
        <thead>
          <tr>
            <th scope="col">Annotation</th>
            <th scope="col">Accepts</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>{'Qty<ASSET>'}</C></th><td>A quantity of that asset.</td></tr>
          <tr><th scope="row"><C>Scalar</C>, <C>UInt128</C></th><td>An unsigned integer.</td></tr>
          <tr><th scope="row"><C>String</C>, <C>Bool</C></th><td>A string; a boolean.</td></tr>
          <tr>
            <th scope="row"><Codes items={['Domain', 'Account', 'Asset', 'Obligation', 'Pool', 'Instrument', 'Observation', 'Policy', 'Grant', 'Stage', 'Episode', 'Party', 'ShareClass']} /></th>
            <td>A declaration of the matching kind.</td>
          </tr>
          <tr><th scope="row"><C>{'Account<DOMAIN>'}</C>, <C>{'Asset<DOMAIN>'}</C>, <C>{'Obligation<DOMAIN>'}</C></th><td>A declaration of that kind on that domain.</td></tr>
        </tbody>
      </Table>
      <p>
        Reserved words cannot name a declaration: the fifteen declaration kinds, <C>profile</C>, <C>agreement</C>,{' '}
        <C>action</C>, <C>uses</C>, <C>true</C>, <C>false</C>, <C>None</C>, <C>SuccessOnly</C>. The agreement name,
        action names and every economic <C>id</C> of a domain, account, asset or obligation must match{' '}
        <C>{'[A-Za-z][A-Za-z0-9_]{0,63}'}</C> and must not be a Source/6 word (<C>BETA_TRANSPORT_ID</C>).
      </p>
      <details className="doc-details">
        <summary>Source/6 words</summary>
        <p>
          <C>{SOURCE6_RESERVED}</C>
        </p>
      </details>

      <h3 id="quantities">Quantities, atoms and arithmetic</h3>
      <Table caption="Quantity and arithmetic rules">
        <thead>
          <tr>
            <th scope="col">Rule</th>
            <th scope="col">Detail</th>
            <th scope="col">Code on violation</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Literal form</td>
            <td><C>NUMBER ASSET</C> with whitespace or a comment between them: <C>30.00 USD</C>. <C>10.00USD</C> is rejected.</td>
            <td><C>BETA_QUANTITY_SEPARATOR</C></td>
          </tr>
          <tr>
            <td>Atoms</td>
            <td>A quantity is stored as integer atoms: the number times 10^scale. At scale 2, <C>10.00 USD</C> is 1000 atoms and <C>1.5 USD</C> is 150 atoms.</td>
            <td>none</td>
          </tr>
          <tr>
            <td>Decimals</td>
            <td>At most <C>scale</C> fraction digits.</td>
            <td><C>BETA_PRECISION</C></td>
          </tr>
          <tr>
            <td>Decimal without asset</td>
            <td>A fraction needs an asset: <C>1.5</C> alone is rejected.</td>
            <td><C>BETA_DECIMAL_SCALAR</C></td>
          </tr>
          <tr>
            <td>Explicit atoms</td>
            <td><C>atoms(asset: USD, value: 1000)</C> is 1000 atoms of <C>USD</C>.</td>
            <td>none</td>
          </tr>
          <tr>
            <td>Operators</td>
            <td><C>*</C> binds tighter than <C>+</C> and <C>-</C>; all associate left. No unary minus, no division.</td>
            <td><C>BETA_SYNTAX</C>, <C>BETA_CHARACTER</C></td>
          </tr>
          <tr>
            <td><C>+</C> and <C>-</C></td>
            <td>Both operands scalars, or both quantities of the same asset.</td>
            <td><C>BETA_ASSET_MISMATCH</C></td>
          </tr>
          <tr>
            <td><C>*</C></td>
            <td>Scalar times scalar, or scalar times quantity. Quantity times quantity is rejected.</td>
            <td><C>BETA_TYPE</C></td>
          </tr>
          <tr>
            <td><C>min</C>, <C>max</C></td>
            <td><C>min(a: X, b: Y)</C>: both scalars, or both quantities of one asset.</td>
            <td><C>BETA_ASSET_MISMATCH</C></td>
          </tr>
          <tr>
            <td>Range</td>
            <td>Every literal and every intermediate result lies in 0 to 2^128−1. <C>2 - 3</C> is rejected.</td>
            <td><C>BETA_UINT128_BOUND</C></td>
          </tr>
          <tr>
            <td>S0 signed fields</td>
            <td>In a <C>LocalS0</C> intent, <C>gross_cap</C>, <C>fee_cap</C>, <C>net_floor</C> and the operation amounts fit 2^127−1.</td>
            <td><C>BETA_S127_BOUND</C></td>
          </tr>
          <tr>
            <td>No conversion</td>
            <td>Different assets and domains never mix. There is no floating point, rounding or implicit conversion.</td>
            <td><C>BETA_ASSET_MISMATCH</C>, <C>BETA_DOMAIN_MISMATCH</C></td>
          </tr>
          <tr>
            <td>JSON amounts</td>
            <td>Scenarios, tests and results carry atoms as canonical decimal strings: <C>"1010"</C>.</td>
            <td><C>BETA_SCENARIO_INTEGER</C></td>
          </tr>
        </tbody>
      </Table>

      <h3 id="named-arguments">Named-argument registry</h3>
      <p>
        Calls take named arguments only. The registry is closed: an unknown call is rejected with{' '}
        <C>BETA_UNKNOWN_CALL</C>, an unknown argument with <C>BETA_UNKNOWN_FIELD</C> and a missing one with{' '}
        <C>BETA_MISSING_FIELD</C>. Every listed argument is required. <C>mori check --json</C> reports the same
        registry as <C>operationSchemas</C>. Only <C>transfer</C> and <C>repay</C> lower to the local evaluator.
      </p>
      <Table caption="Calls and their arguments (argument: role)">
        <thead>
          <tr>
            <th scope="col">Call</th>
            <th scope="col">Arguments</th>
            <th scope="col">Use</th>
          </tr>
        </thead>
        <tbody>
          {REGISTRY.map(([name, args]) => (
            <tr key={name}>
              <th scope="row"><C>{name}</C></th>
              <td>
                <Codes items={args.map(([a, role]) => `${a}: ${role}`)} />
              </td>
              <td>
                {name === 'transfer' || name === 'repay' ? (
                  <>Intent operation; <Chip name="LocalS0" /></>
                ) : name.includes('.') ? (
                  <>Intent operation; <Chip name="SpecifiedOnly" /></>
                ) : (
                  'Value'
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Argument roles">
        <thead>
          <tr>
            <th scope="col">Role</th>
            <th scope="col">Accepted value</th>
          </tr>
        </thead>
        <tbody>
          {ROLE_MEANING.map(([role, meaning]) => (
            <tr key={role}>
              <th scope="row"><Codes items={role.split(', ')} /></th>
              <td>{meaning}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Checks across the arguments of one call">
        <thead>
          <tr>
            <th scope="col">Call</th>
            <th scope="col">Check</th>
            <th scope="col">Code</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>rounds</C></th><td><C>from</C> is at most <C>to</C>.</td><td><C>BETA_ROUND_WINDOW</C></td></tr>
          <tr><th scope="row"><C>transfer</C></th><td><C>from</C>, <C>to</C>, <C>fee_to</C> are three distinct accounts on one domain; <C>fee</C> is in the asset of <C>value</C>, on that domain.</td><td><C>BETA_ENDPOINT_ALIAS</C>, <C>BETA_DOMAIN_MISMATCH</C>, <C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>repay</C></th><td><C>amount</C> is in the obligation's asset; <C>payer</C> is on the obligation's domain.</td><td><C>BETA_ASSET_MISMATCH</C>, <C>BETA_DOMAIN_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>amm.*</C></th><td>Pool and owner on one domain. Swap: <C>net_floor</C> in <C>output_asset</C>, <C>fee_cap</C> in the input asset. Input, output and minted assets are pool members.</td><td><C>BETA_DOMAIN_MISMATCH</C>, <C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>lending.originate</C></th><td><C>principal</C> is in the obligation's asset.</td><td><C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>stablecoin.*</C></th><td>The instrument has <C>asset</C> and <C>backing</C>. <C>supply</C>, <C>burn</C>, <C>claim</C> are in <C>asset</C>; <C>backing</C>, <C>minimum_backing</C> in <C>backing</C>.</td><td><C>BETA_MISSING_FIELD</C>, <C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>option.*</C></th><td>The instrument has <C>underlying</C> and <C>settlement</C>. <C>payoff</C> is in <C>settlement</C>.</td><td><C>BETA_MISSING_FIELD</C>, <C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row"><C>staking.*</C></th><td><C>backing</C>, <C>amount</C>, <C>minimum_backing</C> are in the share class's <C>backing</C> asset.</td><td><C>BETA_ASSET_MISMATCH</C></td></tr>
          <tr><th scope="row">All dotted calls</th><td>All arguments that carry a domain share one domain, and quantities are of assets on it. Exception: <C>source</C> and <C>destination</C> of <C>bridge.*</C>.</td><td><C>BETA_DOMAIN_MISMATCH</C></td></tr>
        </tbody>
      </Table>

      <h3 id="intent-fields">Intent fields</h3>
      <p>
        An intent whose operation is <C>transfer</C> or <C>repay</C> has exactly these twenty fields, all required. No
        field has a default.
      </p>
      <Table caption="Fields of a LocalS0 intent, in the order the tool lists them">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Value</th>
            <th scope="col">Rule</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>domain</C></th><td>domain</td><td>The asset and the signer are on this domain.</td></tr>
          <tr><th scope="row"><C>asset</C></th><td>asset</td><td>Caps, floor and operation amounts are in this asset.</td></tr>
          <tr><th scope="row"><C>signer</C></th><td>account</td><td>Equals <C>from</C> of <C>transfer</C>, or <C>payer</C> of <C>repay</C> (<C>BETA_SIGNER</C>).</td></tr>
          <tr><th scope="row"><C>key</C></th><td>nonempty string</td><td>An opaque key reference, not a public key.</td></tr>
          <tr><th scope="row"><C>nonce</C></th><td>nonempty string</td><td>With domain and signer, forms the replay key.</td></tr>
          <tr><th scope="row"><C>pre_head</C></th><td>nonempty string</td><td>Core compares it with the scenario <C>head</C>.</td></tr>
          <tr><th scope="row"><C>valid</C></th><td><C>rounds(domain:, from:, to:)</C></td><td>On the intent's domain. Inclusive window; <C>from</C> at most <C>to</C>.</td></tr>
          <tr><th scope="row"><C>gross_cap</C></th><td>quantity</td><td>Upper bound on the gross debit, fee included.</td></tr>
          <tr><th scope="row"><C>fee_cap</C></th><td>quantity</td><td>Upper bound on the fee. Zero for <C>repay</C> (Core).</td></tr>
          <tr><th scope="row"><C>net_floor</C></th><td>quantity</td><td>Lower bound on the transferred value. Zero for <C>repay</C> (Core).</td></tr>
          <tr><th scope="row"><C>operation</C></th><td><C>transfer(...)</C> or <C>repay(...)</C></td><td>Accounts and obligation on the intent's domain and asset.</td></tr>
          <tr><th scope="row"><C>source_hash</C></th><td>nonempty string</td><td>A claim. Not compared with any computed hash.</td></tr>
          <tr><th scope="row"><C>policy_digest</C></th><td>nonempty string</td><td>A claim. Not compared with any computed digest.</td></tr>
          <tr><th scope="row"><C>failure</C></th><td><C>SuccessOnly</C></td><td><C>BETA_FAILURE_POLICY</C> otherwise.</td></tr>
          <tr><th scope="row"><C>observations</C></th><td><C>[]</C></td><td>Empty array.</td></tr>
          <tr><th scope="row"><C>disclosures</C></th><td><C>[]</C></td><td>Empty array.</td></tr>
          <tr><th scope="row"><C>retained_effects</C></th><td><C>[]</C></td><td>Empty array.</td></tr>
          <tr><th scope="row"><C>retained_duties</C></th><td><C>[]</C></td><td>Empty array.</td></tr>
          <tr><th scope="row"><C>delegation</C></th><td><C>None</C></td><td><C>BETA_FAILURE_POLICY</C> otherwise.</td></tr>
          <tr><th scope="row"><C>recovery</C></th><td><C>None</C></td><td><C>BETA_FAILURE_POLICY</C> otherwise.</td></tr>
        </tbody>
      </Table>
      <p>
        Core decides the economic bounds for a given scenario (<C>S0_INTENT_SCOPE</C>): transfer value above zero, fee
        at most <C>fee_cap</C>, value plus fee at most <C>gross_cap</C>, value at least <C>net_floor</C>, and the
        scenario round inside <C>valid</C>; repayment amount above zero and at most <C>gross_cap</C>.
      </p>
      <Table caption="Fields of a SpecifiedOnly intent (operation is a dotted call)">
        <thead>
          <tr>
            <th scope="col">Fields</th>
            <th scope="col">Value</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>operation</C></th><td>Required. A registry call whose name contains a dot.</td></tr>
          <tr><th scope="row"><Codes items={['domain', 'asset', 'signer']} /></th><td>A domain, an asset, an account. When present they agree with the operation's local domain.</td></tr>
          <tr><th scope="row"><Codes items={['nonce', 'key', 'pre_head', 'source_hash', 'policy_digest', 'kernel', 'completion', 'rounding', 'relation', 'status', 'observation', 'continuation', 'loss', 'fixing']} /></th><td>Strings.</td></tr>
          <tr><th scope="row"><C>valid</C></th><td><C>rounds(...)</C> on the intent's domain, or an array of two scalars.</td></tr>
          <tr><th scope="row"><Codes items={['gross_cap', 'fee_cap', 'net_floor']} /></th><td>Quantities in <C>asset</C> (for a bridge, the local amount's asset). For <C>amm.swap_exact_input</C>, <C>net_floor</C> is in <C>output_asset</C>.</td></tr>
          <tr><th scope="row"><Codes items={['earliest_round', 'veto_before']} /></th><td>Scalars.</td></tr>
          <tr><th scope="row"><Codes items={['reads', 'writes']} /></th><td>String arrays.</td></tr>
          <tr><th scope="row"><C>policy</C></th><td>A policy.</td></tr>
          <tr><th scope="row"><C>authority</C></th><td>A grant or a string.</td></tr>
          <tr><th scope="row"><C>retained_duties</C></th><td>A string or a string array.</td></tr>
          <tr><th scope="row"><Codes items={['failure', 'observations', 'disclosures', 'retained_effects', 'delegation', 'recovery']} /></th><td>Accepted and stored; the value is not type-checked.</td></tr>
        </tbody>
      </Table>

      <h3 id="limits">Source limits</h3>
      <Table caption="Limits on one source file">
        <thead>
          <tr>
            <th scope="col">Limit</th>
            <th scope="col">Value</th>
            <th scope="col">Code</th>
          </tr>
        </thead>
        <tbody>
          <tr><td>Source size</td><td>65536 bytes (UTF-8)</td><td><C>BETA_SOURCE_BOUND</C></td></tr>
          <tr><td>Tokens, counting comments and end of file</td><td>8192</td><td><C>BETA_TOKEN_BOUND</C></td></tr>
          <tr><td>Identifier length</td><td>64 characters</td><td><C>BETA_IDENTIFIER_BOUND</C></td></tr>
          <tr><td>Decoded string</td><td>1024 bytes</td><td><C>BETA_STRING_BOUND</C></td></tr>
          <tr><td>Nesting depth</td><td>64</td><td><C>BETA_DEPTH_BOUND</C></td></tr>
          <tr><td>Analysis nodes</td><td>8192</td><td><C>BETA_NODE_BOUND</C></td></tr>
          <tr><td>Expression edges</td><td>32768</td><td><C>BETA_EDGE_BOUND</C></td></tr>
          <tr><td>Fields in one record or call; type arguments</td><td>64</td><td><C>BETA_FIELD_BOUND</C></td></tr>
          <tr><td>Declarations and actions in one agreement</td><td>256</td><td><C>BETA_DECLARATION_BOUND</C></td></tr>
          <tr><td>Asset scale</td><td>0 to 18</td><td><C>BETA_SCALE</C></td></tr>
          <tr><td>Value range</td><td>0 to 2^128−1</td><td><C>BETA_UINT128_BOUND</C></td></tr>
          <tr><td>S0 signed monetary fields</td><td>at most 2^127−1</td><td><C>BETA_S127_BOUND</C></td></tr>
          <tr><td>Formatted output</td><td>65536 bytes</td><td><C>BETA_FORMAT_BOUND</C></td></tr>
        </tbody>
      </Table>

      <h3 id="price-orientation">Price orientation</h3>
      <p>
        <C>{PROFILE}</C> has no price type. A <C>Price</C> annotation is rejected with <C>BETA_ANNOTATION</C>. Price
        units in the examples are strings: <C>strike_units: "USD per GOLD"</C>, <C>unit: "USD per GOLD"</C>.
      </p>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#explicit-terms`, label: 'Why every term is stated' },
          { mode: 'howto', href: `${H}#cap-with-fee`, label: 'How to cap a payment so the fee counts toward the gross debit' },
          { mode: 'explanation', href: `${E}#horizon-price`, label: 'Proposed price orientation' },
        ]}
      />
    </Section>
  );
}

/* ------------------------------------------------------------------ Results and support */

function Results() {
  /** [row key, status shown, reported by, meaning] */
  const statuses: [string, ReactNode, string, ReactNode][] = [
    ['Initialized', <C>Initialized</C>, 'init', 'The project directory and its four files were written.'],
    ['AuthoringChecked', <C>AuthoringChecked</C>, 'check, inspect', 'The source passed every source check.'],
    ['AuthoringRejected', <C>AuthoringRejected</C>, 'check, fmt, inspect, expand, simulate', 'A source diagnostic, or an unknown action name.'],
    ['InspectionRejected', <C>InspectionRejected</C>, 'inspect', 'Inspection exceeded its bounds. Carries the original authoringStatus; not a source error.'],
    ['Expanded', <C>Expanded</C>, 'expand', 'The Source/6 program was generated.'],
    ['PreparedUnqualified', <C>PreparedUnqualified</C>, 'simulate', 'Core produced a candidate: ordered effects and a candidate post-state.'],
    ['CoreRejected', <C>CoreRejected</C>, 'simulate', 'Core rejected the request at one judgment.'],
    ['SourceRejected', <C>SourceRejected</C>, 'simulate', 'The Source/6 parser rejected the generated program during preparation.'],
    ['FormationRejected', <C>FormationRejected</C>, 'expand, simulate, test, intent, verify-intent, init', 'A scenario, test, signature or command input was refused before Core.'],
    [
      'FormationRejected-crypto',
      <>
        <C>FormationRejected</C> with a <C>BETA_CRYPTO_*</C> code
      </>,
      'intent, verify-intent',
      'The native verifier gave no valid answer: binary, timeout, transport or response failure. Exit 2. Signature validity unknown. Nothing is accepted as a fallback.',
    ],
    ['Unsupported', <C>Unsupported</C>, 'expand, simulate', 'The action is SpecifiedOnly. No effects or post-state are published.'],
    ['TestsPassed', <C>TestsPassed</C>, 'test', 'Every case matched its expectation.'],
    ['TestsFailed', <C>TestsFailed</C>, 'test', 'At least one case did not match.'],
    ['OwnerIntentPrepared', <C>OwnerIntentPrepared</C>, 'intent', 'The canonical statement and signing message were built. No signature was checked.'],
    [
      'SignedPreparedUnqualified',
      <C>SignedPreparedUnqualified</C>,
      'verify-intent',
      <>
        The native verifier accepted the owner's signature and local preparation produced a candidate. The signature
        is over the canonical intent statement, not over the source file. The statement binds the SHA-256 of the
        exact source bytes, the agreement and action, the chain and network claims, the asset representation, scale
        and symbol, the signer and key reference, the signature metadata, and the owner's operation and bounds. The
        scenario is not signed.
      </>,
    ],
    ['SignedCoreRejected', <C>SignedCoreRejected</C>, 'verify-intent', 'The signature verified and Core rejected the request.'],
    ['SignatureRejected', <C>SignatureRejected</C>, 'verify-intent', 'The native verifier rejected the signature. Local preparation is skipped.'],
  ];
  const judgments: [string, string, string][] = [
    ['stage', 'The state and request have the S0 shape.', 'S0_STAGE_UNSUPPORTED'],
    ['intent', 'Round window, caps, floor, distinct endpoints, signed action equals submitted action.', 'S0_INTENT_SCOPE, S0_INTENT_ALIAS'],
    ['effect', 'Funds and credit ranges; proposed effects equal computed effects.', 'S0_EFFECT_RANGE, S0_EFFECT_MISMATCH'],
    ['authority', 'Allowance and work counters cover the gross debit and one step.', 'S0_AUTH_SCOPE'],
    ['history', 'pre_head equals head; replay key unused; post_head is a new head.', 'S0_HISTORY_STALE, S0_HISTORY_REPLAY, S0_HISTORY_SUCCESSOR'],
    ['failure', 'The requested outcome is terminal success with nothing retained.', 'S0_FAILURE_UNSUPPORTED'],
  ];
  return (
    <Section id="results" title="Results, rejections and judgment order" nav="Results" group="Results and support">
      <Table caption="Result statuses">
        <thead>
          <tr>
            <th scope="col">Status</th>
            <th scope="col">Reported by</th>
            <th scope="col">Meaning</th>
          </tr>
        </thead>
        <tbody>
          {statuses.map(([key, s, by, m]) => (
            <tr key={key}>
              <th scope="row">{s}</th>
              <td><Codes items={by.split(', ')} /></td>
              <td>{m}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <p>
        Every <C>SignedPreparedUnqualified</C>, <C>SignedCoreRejected</C> and <C>SignatureRejected</C> result carries
        four state values: <C>keyAuthority: "Unverified"</C>, <C>state: "LocalStipulationOnly"</C>,{' '}
        <C>nativeProof: "NotChecked"</C> and <C>ledger: "NotSubmitted"</C>, with <C>ledger_accepted: false</C>.
      </p>
      <p>
        A test case may expect one of six statuses: <C>PreparedUnqualified</C>, <C>CoreRejected</C>,{' '}
        <C>SourceRejected</C>, <C>AuthoringRejected</C>, <C>FormationRejected</C>, <C>Unsupported</C>.
      </p>
      <Table caption="Core judgment order. Core reports the first failing judgment only.">
        <thead>
          <tr>
            <th scope="col">Order</th>
            <th scope="col"><C>judgment</C></th>
            <th scope="col">Decides</th>
            <th scope="col">Codes</th>
          </tr>
        </thead>
        <tbody>
          {judgments.map(([j, what, codes], i) => (
            <tr key={j}>
              <td>{i + 1}</td>
              <th scope="row"><C>{j}</C></th>
              <td>{what}</td>
              <td><Codes items={codes.split(', ')} /></td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Where the parts of a simulate result are">
        <thead>
          <tr>
            <th scope="col">JSON path</th>
            <th scope="col">Present when</th>
            <th scope="col">Content</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>result.candidate.effects</C></th><td><C>PreparedUnqualified</C></td><td>Ordered effects: <C>Debit</C>, <C>Credit</C>, <C>SetObligation</C>, <C>UseAllowance</C>, <C>UseReplay</C>, <C>AdvanceHead</C>.</td></tr>
          <tr><th scope="row"><C>result.candidate.candidatePost</C></th><td><C>PreparedUnqualified</C></td><td>Fields <Codes items={['core', 'domain', 'asset', 'head', 'round', 'workRemaining', 'workSpent', 'balances', 'allowances', 'obligations', 'consumedReplay']} />.</td></tr>
          <tr><th scope="row"><C>result.candidate.requiredPremises</C></th><td><C>PreparedUnqualified</C></td><td>See <a href="#premises-bindings">premises and bindings</a>.</td></tr>
          <tr><th scope="row"><C>result.unverifiedBindings</C></th><td><C>PreparedUnqualified</C></td><td>See <a href="#premises-bindings">premises and bindings</a>.</td></tr>
          <tr><th scope="row"><C>result.rejection</C></th><td><C>CoreRejected</C></td><td><C>{'{status: "Rejected", judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null}'}</C></td></tr>
          <tr><th scope="row"><C>result.code</C>, <C>result.offset</C></th><td><C>SourceRejected</C></td><td>Source/6 code and byte offset in the generated program.</td></tr>
          <tr><th scope="row"><C>diagnostics</C></th><td><C>AuthoringRejected</C>, <C>FormationRejected</C>, <C>Unsupported</C></td><td>One diagnostic; <C>publishedEffects</C> and <C>publishedPost</C> are <C>null</C>.</td></tr>
        </tbody>
      </Table>
      <Table caption="Scope fields">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Where</th>
            <th scope="col">Values</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>qualification</C></th><td><C>init</C>, <C>expand</C>, <C>simulate</C>, <C>test</C></td><td><C>local-stipulation-only</C>. A missing <C>qualification</C> never means qualification.</td></tr>
          <tr><th scope="row"><C>qualification</C></th><td>native verifier responses in <C>intent</C>, <C>verify-intent</C></td><td><C>signature-protocol-only</C></td></tr>
          <tr><th scope="row"><C>evidence</C></th><td><C>check --json</C></td><td><C>AuthoringOnly</C></td></tr>
          <tr><th scope="row"><C>openGates</C></th><td><C>check --json</C></td><td><Codes items={['authentication', 'nativeProof', 'financialCorrespondence', 'atomicLedgerAcceptance']} /></td></tr>
          <tr><th scope="row"><C>actions[].coverage.financialRelations</C></th><td><C>check --json</C>, <C>inspect</C></td><td><C>DelegatedToCoreDuringLocalPreparation</C> (LocalS0) or <C>Open</C> (SpecifiedOnly)</td></tr>
          <tr><th scope="row"><C>actions[].coverage.localPreparation</C></th><td><C>check --json</C>, <C>inspect</C></td><td><C>AvailableUnqualified</C> (LocalS0) or <C>Unsupported</C> (SpecifiedOnly)</td></tr>
          <tr><th scope="row"><C>scenarioValidation</C></th><td><C>Unsupported</C> results</td><td><C>NotAppliedUnsupported</C>: the scenario was not schema-checked or applied.</td></tr>
        </tbody>
      </Table>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#architecture`, label: 'How an intent becomes a candidate' },
          { mode: 'howto', href: `${H}#test-rejection`, label: 'How to test that Core rejects a request' },
        ]}
      />
    </Section>
  );
}

function Diagnostics() {
  return (
    <Section id="diagnostics" title="Diagnostic codes" nav="Diagnostic codes" group="Results and support">
      <p>
        A diagnostic is <C>{'{code, message, span: {start, end}}'}</C>; the span is a byte range in the UTF-8 source.
        Source analysis stops at the first failure, so a result carries one diagnostic. Readable <C>check</C> prints{' '}
        <C>CODE at LINE:COLUMN: MESSAGE</C> with 1-based line and column. In the message column, <C>{'<x>'}</C> is filled
        in by the tool.
      </p>
      {DIAGNOSTIC_GROUPS.map((g) => (
        <Table key={g.key} caption={`${g.caption} Status: ${g.status}.`}>
          <thead>
            <tr>
              <th scope="col">Code</th>
              <th scope="col">Message</th>
            </tr>
          </thead>
          <tbody>
            {g.codes.map((c) => (
              <tr key={c.code}>
                <th scope="row"><C>{c.code}</C></th>
                <td>
                  {c.messages.map((m, i) => (
                    <span key={m}>
                      {i > 0 ? <br /> : null}
                      {m}
                    </span>
                  ))}
                </td>
              </tr>
            ))}
          </tbody>
        </Table>
      ))}
      <Table caption="Core codes (status CoreRejected). They carry a judgment and no message.">
        <thead>
          <tr>
            <th scope="col">Code</th>
            <th scope="col"><C>judgment</C></th>
            <th scope="col">Raised when</th>
          </tr>
        </thead>
        <tbody>
          {CORE_CODES.map((c) => (
            <tr key={c.code}>
              <th scope="row"><C>{c.code}</C></th>
              <td><C>{c.judgment}</C></td>
              <td>{c.when}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <p>
        Codes of the generated Source/6 parser appear under <C>FormationRejected</C> with a message of the form{' '}
        <C>Generated Source/6 formation: MESSAGE; field FIELD; byte N; scenario POINTERS</C>, and under{' '}
        <C>SourceRejected</C> as <C>result.code</C>: <Codes items={SOURCE6_CODES} />.
      </p>
      <SeeAlso
        items={[
          { mode: 'howto', href: `${H}#diagnose`, label: 'How to diagnose a failed check or test' },
          { mode: 'explanation', href: `${E}#stipulation`, label: 'What a passing local run does and does not show' },
        ]}
      />
    </Section>
  );
}

const LABELS: [StatusName, ReactNode, ReactNode][] = [
  [
    'LocalS0',
    'Runs locally against a stipulated scenario. The scenario is a stipulation, not a fact about any ledger.',
    <>Action support in <C>check</C> and <C>inspect</C>, for intents whose operation is <C>transfer</C> or <C>repay</C>.</>,
  ],
  [
    'PreparedUnqualified',
    <>
      A candidate result exists. Its limits are in{' '}
      <a href={`${E}#stance`}>Explanation: what these pages do not claim</a>.
    </>,
    <>Status of <C>simulate</C> and of a test case.</>,
  ],
  [
    'SignedPreparedUnqualified',
    <>
      A native verifier accepted the owner's signature and local preparation produced a candidate. What the statement
      binds and the four state values the result carries: <a href="#results">results</a>.
    </>,
    <>Status of <C>verify-intent</C>.</>,
  ],
  [
    'SpecifiedOnly',
    'The source is checked for structure, names and quantities. Execution is refused as Unsupported.',
    <>Action support in <C>check</C> and <C>inspect</C>, for intents whose operation is a dotted call.</>,
  ],
  [
    'Unsupported',
    'The status expand and simulate return for a SpecifiedOnly action. No effects or post-state are published, and the scenario is not checked.',
    <>Status of <C>expand</C> and <C>simulate</C>, code <C>BETA_PROFILE_UNSUPPORTED</C>.</>,
  ],
  [
    'Open',
    'A capability or relation the example names but nothing yet establishes.',
    <><C>financialRelations: "Open"</C> in <C>check --json</C>; every premise and binding.</>,
  ],
];

function SupportLabels() {
  return (
    <Section id="support-labels" title="Support labels" nav="Support labels" group="Results and support">
      <Table caption="The six support labels">
        <thead>
          <tr>
            <th scope="col">Label</th>
            <th scope="col">Definition</th>
            <th scope="col">Where the tool reports it</th>
          </tr>
        </thead>
        <tbody>
          {LABELS.map(([name, def, where]) => (
            <tr key={name}>
              <th scope="row"><Chip name={name} link={false} /></th>
              <td>{def}</td>
              <td>{where}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#stance`, label: 'Why labels and limits are stated the way they are' },
          { mode: 'explanation', href: `${E}#three-questions`, label: 'Authorization, acceptance and coordination' },
        ]}
      />
    </Section>
  );
}

function SupportMatrix() {
  const local: [string, string, string, string][] = [
    ['Transfer with fee', 'mori init DIR', 'invoice.mori', 'pay'],
    ['Repayment, interest first', 'mori init DIR --template repay', 'repayment.mori', 'repay_loan'],
  ];
  return (
    <Section id="support-matrix" title="Support by operation family" nav="Support matrix" group="Results and support">
      <Table caption={`Support observed with check, expand and simulate on the shipped files at commit ${PINNED_SHORT}`}>
        <thead>
          <tr>
            <th scope="col">Operation</th>
            <th scope="col">File and actions</th>
            <th scope="col">Label</th>
            <th scope="col">expand, simulate</th>
          </tr>
        </thead>
        <tbody>
          {local.map(([name, init, file, action]) => (
            <tr key={name}>
              <th scope="row">{name}</th>
              <td>
                <C>{file}</C> from <C>{init}</C>
                <br />
                <C>{action}</C>
              </td>
              <td><Chip name="LocalS0" link={false} /></td>
              <td>
                <C>Expanded</C>
                <br />
                <Chip name="PreparedUnqualified" link={false} /> or <Chip name="CoreRejected" link={false} />
              </td>
            </tr>
          ))}
          {FAMILY_ENTRIES.map((f) => {
            const fam = FAMILIES.find((x) => x.id === f.id)!;
            return (
              <tr key={f.id}>
                <th scope="row"><a href={`#family-${f.id}`}>{fam.name}</a></th>
                <td>
                  <a href={pkg(f.file)}><C>{f.file.replace('examples/', '')}</C></a>
                  <br />
                  <Codes items={f.operations.map((o) => o.action)} />
                </td>
                <td><Chip name="SpecifiedOnly" link={false} /></td>
                <td>
                  <Chip name="Unsupported" link={false} />
                  <br />
                  <C>BETA_PROFILE_UNSUPPORTED</C>
                </td>
              </tr>
            );
          })}
        </tbody>
      </Table>
      <p>
        In <C>check --json</C> and <C>inspect</C>, a <Chip name="LocalS0" link={false} /> action reports{' '}
        <C>financialRelations: "DelegatedToCoreDuringLocalPreparation"</C> and{' '}
        <C>localPreparation: "AvailableUnqualified"</C>; a <Chip name="SpecifiedOnly" link={false} /> action reports{' '}
        <C>financialRelations: "Open"</C> and <C>localPreparation: "Unsupported"</C> (
        <a href="#results">scope fields</a>).
      </p>
      <p>
        <C>verify-intent</C> can reach <Chip name="SignedPreparedUnqualified" link={false} /> for the transfer and
        repayment actions only. <C>intent</C> on a SpecifiedOnly action stops with <C>BETA_AUTH_SOURCE</C>, because its
        expansion is <C>Unsupported</C>.
      </p>
      <SeeAlso
        items={[
          { mode: 'howto', href: `${H}#check-family`, label: 'How to check an example from the eight DeFi families' },
          { mode: 'explanation', href: `${E}#stance`, label: 'Why labels and limits are stated the way they are' },
        ]}
      />
    </Section>
  );
}

function PremisesBindings() {
  const premises: [string, string][] = [
    ['canonical-intent-signature', 'The owner signed the canonical intent statement for this action.'],
    ['snapshot-to-head', 'The scenario state is the state at the scenario head.'],
    ['head-extension', 'The successor head extends the predecessor head.'],
    ['atomic-ledger-compare-and-consume', 'A ledger applies the effects and consumes the replay key in one atomic step against the expected head.'],
  ];
  const bindings: [string, string][] = [
    ['agreement-id', 'The agreement name is bound to an authenticated agreement.'],
    ['selected-program', 'The selected Source/6 program (TransferLiteralFee or RepayAccrualFirst) is the approved one.'],
    ['asset-scale', 'The scale used to compute atoms is the asset’s real scale.'],
    ['authenticated-predecessor', 'The scenario predecessor is authenticated. It is echoed into Source/6 and is not a Core equality check.'],
  ];
  return (
    <Section id="premises-bindings" title="Local premises and unverified bindings" nav="Premises and bindings" group="Results and support">
      <p>
        Every <Chip name="LocalS0" /> preparation lists four external premises (<C>requiredPremises</C>) and four
        unverified bindings (<C>unverifiedBindings</C>), in <C>simulate</C>, <C>inspect</C> and{' '}
        <C>verify-intent</C>. A matching hash or a passing local test closes none of them. Source:{' '}
        <a href={GETTING_STARTED}>GETTING-STARTED.md</a>.
      </p>
      <Table caption="Premises and bindings of a LocalS0 preparation">
        <thead>
          <tr>
            <th scope="col">Identifier</th>
            <th scope="col">List</th>
            <th scope="col">What it names</th>
            <th scope="col">Status</th>
          </tr>
        </thead>
        <tbody>
          {premises.map(([id, what]) => (
            <tr key={id}>
              <th scope="row"><C>{id}</C></th>
              <td><C>requiredPremises</C></td>
              <td>{what}</td>
              <td><Chip name="Open" /></td>
            </tr>
          ))}
          {bindings.map(([id, what]) => (
            <tr key={id}>
              <th scope="row"><C>{id}</C></th>
              <td><C>unverifiedBindings</C></td>
              <td>{what}</td>
              <td><Chip name="Open" /></td>
            </tr>
          ))}
        </tbody>
      </Table>
      <p>
        On a <Chip name="SpecifiedOnly" /> action, <C>inspect</C> reports <C>requiredPremises: []</C> and{' '}
        <C>unverifiedBindings: []</C>. The empty lists mean that the S0 catalog does not apply. Authentication and
        financial relations remain <C>Open</C>.
      </p>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#stipulation`, label: 'What a passing local run does and does not show' },
          { mode: 'howto', href: `${H}#sign-intent`, label: 'How to verify a signed intent locally' },
        ]}
      />
    </Section>
  );
}

/* ------------------------------------------------------------------ Data formats */

function ScenarioFormat() {
  const fields: [string, string, ReactNode][] = [
    ['profile', 'string', <><C>"moriarty-local-scenario/1"</C></>],
    ['kind', 'string', <><C>"local-stipulation"</C></>],
    ['domain', 'economic id', <>Equals the <C>id</C> of the intent's domain.</>],
    ['asset', 'economic id', <>Equals the <C>id</C> of the intent's asset.</>],
    ['head', 'nonempty text', <>Core compares it with the intent's <C>pre_head</C>.</>],
    ['predecessor', 'nonempty text', 'Echoed into Source/6. Not authenticated and not compared by Core.'],
    ['round', 'integer string', <>Core requires it inside the intent's <C>valid</C> window. Not advanced automatically.</>],
    ['balances', 'array of {account, amount}', 'Exact ordered cells. Transfer: signer, recipient (to), fee recipient (fee_to). Repayment: payer, creditor. No duplicates.'],
    ['allowance', '{owner, remaining, spent}', 'owner is the signer. remaining + spent fits 2^128−1.'],
    ['replay', 'string', <><C>"unused"</C> or <C>"consumed"</C></>],
    ['work_remaining', 'integer string', 'work_remaining + work_spent fits 2^128−1.'],
    ['work_spent', 'integer string', 'See work_remaining.'],
    ['post_head', 'nonempty text', 'Core requires a head different from head.'],
    ['obligation', 'record', 'Required for repayment. Forbidden for transfer. Fields below.'],
    ['candidate_effects', 'array, at most 16', 'Optional. Replaces the computed proposal. Checked through Source/6 and Core.'],
  ];
  const effects: [string, string][] = [
    ['Debit', 'kind, account, asset, amount'],
    ['Credit', 'kind, account, asset, amount'],
    ['SetObligation', 'kind, id, principal, accrued, outstanding, status'],
    ['UseAllowance', 'kind, owner, amount'],
    ['UseReplay', 'kind, key'],
    ['AdvanceHead', 'kind, predecessor, successor'],
  ];
  return (
    <Section id="scenario-format" title="Local scenario format: moriarty-local-scenario/1" nav="Scenario format" group="Data formats">
      <p>
        Source: <a href={pkg('src/bridge.ts')}>src/bridge.ts</a>; canonical text in{' '}
        <a href={GETTING_STARTED}>GETTING-STARTED.md</a>, “Local scenario reference”. A JSON record. Every field below
        is required unless marked optional. Unknown fields reject. The file follows the{' '}
        <a href="#scenario-format-json-limits">JSON limits</a>. Errors in shape, identity or accounting are{' '}
        <C>FormationRejected</C>; caps, funds, authority, expiry and replay are decided by Core.
      </p>
      <Table caption="Scenario fields">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Shape</th>
            <th scope="col">Rule</th>
          </tr>
        </thead>
        <tbody>
          {fields.map(([f, shape, rule]) => (
            <tr key={f}>
              <th scope="row"><C>{f}</C></th>
              <td>{shape}</td>
              <td>{rule}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Obligation fields (repayment)">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Rule</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>id</C></th><td>Equals the <C>id</C> of the source obligation.</td></tr>
          <tr><th scope="row"><C>debtor</C></th><td>Equals the payer's <C>id</C>.</td></tr>
          <tr><th scope="row"><C>creditor</C></th><td>Differs from the payer.</td></tr>
          <tr><th scope="row"><C>asset</C></th><td>Equals the intent's asset <C>id</C>.</td></tr>
          <tr><th scope="row"><C>principal</C>, <C>accrued</C>, <C>outstanding</C></th><td>Integer strings at most 2^127−1. principal + accrued equals outstanding.</td></tr>
          <tr><th scope="row"><C>status</C></th><td><C>"Outstanding"</C></td></tr>
        </tbody>
      </Table>
      <Table caption="Effect variants in candidate_effects (closed field sets)">
        <thead>
          <tr>
            <th scope="col"><C>kind</C></th>
            <th scope="col">Fields</th>
          </tr>
        </thead>
        <tbody>
          {effects.map(([k, f]) => (
            <tr key={k}>
              <th scope="row"><C>{k}</C></th>
              <td><Codes items={f.split(', ')} /></td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption="Value rules">
        <thead>
          <tr>
            <th scope="col">Value</th>
            <th scope="col">Rule</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row">Economic id</th><td><C>{'[A-Za-z][A-Za-z0-9_]{0,63}'}</C></td></tr>
          <tr><th scope="row">Integer string</th><td><C>0</C> or a nonzero digit followed by digits; no sign, leading zero, exponent or decimal point; at most 2^128−1.</td></tr>
          <tr><th scope="row">Text</th><td>Nonempty; at most 1024 bytes.</td></tr>
          <tr><th scope="row">Debit and Credit asset</th><td>Equals the scenario asset.</td></tr>
          <tr><th scope="row">SetObligation status</th><td><C>"Outstanding"</C> or <C>"Settled"</C></td></tr>
          <tr><th scope="row">Replay key</th><td>In <C>candidate_effects</C>, <C>UseReplay.key</C> is the bare nonce (<C>"n1"</C>). In Core results it is the tuple of domain, signer and nonce encoded as a JSON string: <C>{'"[\\"Midnight\\",\\"Owner\\",\\"n1\\"]"'}</C>.</td></tr>
        </tbody>
      </Table>
      <h3 id="scenario-format-json-limits">JSON limits for scenario, test and signature files</h3>
      <Table caption="Bounded JSON (src/json.ts)">
        <thead>
          <tr>
            <th scope="col">Limit</th>
            <th scope="col">Value</th>
            <th scope="col">Code</th>
          </tr>
        </thead>
        <tbody>
          <tr><td>File size</td><td>65536 bytes</td><td><C>BETA_JSON_BOUND</C></td></tr>
          <tr><td>Depth</td><td>32</td><td><C>BETA_JSON_DEPTH</C></td></tr>
          <tr><td>Nodes</td><td>4096</td><td><C>BETA_JSON_NODES</C></td></tr>
          <tr><td>Fields per object</td><td>64</td><td><C>BETA_JSON_FIELDS</C></td></tr>
          <tr><td>Decoded string</td><td>1024 bytes</td><td><C>BETA_JSON_STRING</C></td></tr>
          <tr><td>Duplicate keys, including escaped spellings of one key</td><td>rejected</td><td><C>BETA_JSON_DUPLICATE</C></td></tr>
          <tr><td>Comments</td><td>not allowed</td><td><C>BETA_JSON_SYNTAX</C></td></tr>
        </tbody>
      </Table>
      <CodeDetails summary="Transfer scenario written by mori init" kind="file" title="scenario.json (transfer template)">
        {transferScenario}
      </CodeDetails>
      <CodeDetails summary="Repayment scenario written by mori init --template repay" kind="file" title="scenario.json (repay template)">
        {repayScenario}
      </CodeDetails>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#stipulation`, label: 'What a passing local run does and does not show' },
          { mode: 'howto', href: `${H}#change-amount`, label: 'How to change the repayment amount' },
        ]}
      />
    </Section>
  );
}

function TestFormat() {
  return (
    <Section id="test-format" title="Test file format: moriarty-beta-tests/1" nav="Test file format" group="Data formats">
      <p>
        Source: <a href={pkg('src/cli.ts')}>src/cli.ts</a> (<C>runCases</C>). <C>mori test DIR</C> reads{' '}
        <C>DIR/mori.tests.json</C>. Every record is closed: unknown or missing fields reject with{' '}
        <C>BETA_CASE_SCHEMA</C>. The file follows the <a href="#scenario-format-json-limits">JSON limits</a>.
      </p>
      <Table caption="Test file fields">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Required</th>
            <th scope="col">Rule</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>profile</C></th><td>yes</td><td><C>"moriarty-beta-tests/1"</C></td></tr>
          <tr><th scope="row"><C>cases</C></th><td>yes</td><td>Array of 1 to 64 cases.</td></tr>
          <tr><th scope="row"><C>cases[].name</C></th><td>yes</td><td>Nonempty string, unique in the file.</td></tr>
          <tr><th scope="row"><C>cases[].source</C></th><td>yes</td><td>Relative path to a <C>.mori</C> file inside <C>DIR</C>, also after resolving symbolic links (<C>BETA_CASE_PATH</C>).</td></tr>
          <tr><th scope="row"><C>cases[].action</C></th><td>yes</td><td>Action name in the source.</td></tr>
          <tr><th scope="row"><C>cases[].scenario</C></th><td>yes</td><td>Relative path to a scenario file inside <C>DIR</C>.</td></tr>
          <tr><th scope="row"><C>cases[].expect.status</C></th><td>yes</td><td>One of the six test statuses (see <a href="#results">results</a>).</td></tr>
          <tr><th scope="row"><C>cases[].expect.code</C></th><td>no</td><td>Compared with the Core code for <C>CoreRejected</C>, the Source/6 code for <C>SourceRejected</C>, the first diagnostic code for other rejections, and <C>null</C> for <C>PreparedUnqualified</C>.</td></tr>
          <tr><th scope="row"><C>cases[].expect.effects</C></th><td>no</td><td>The full ordered effect vector (<C>result.candidate.effects</C>).</td></tr>
          <tr><th scope="row"><C>cases[].expect.post</C></th><td>no</td><td>The full candidate post-state (<C>result.candidate.candidatePost</C>), with no omitted or extra fields.</td></tr>
        </tbody>
      </Table>
      <Table caption="Test report fields">
        <thead>
          <tr>
            <th scope="col">Field</th>
            <th scope="col">Value</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>status</C></th><td><C>TestsPassed</C> or <C>TestsFailed</C> (exit 1)</td></tr>
          <tr><th scope="row"><C>qualification</C></th><td><C>local-stipulation-only</C></td></tr>
          <tr><th scope="row"><C>cases[].name</C></th><td>The case name.</td></tr>
          <tr><th scope="row"><C>cases[].passed</C></th><td><C>true</C> when there are no mismatches.</td></tr>
          <tr><th scope="row"><C>cases[].status</C></th><td>The actual status.</td></tr>
          <tr><th scope="row"><C>cases[].code</C></th><td>The actual code, or <C>null</C>.</td></tr>
          <tr><th scope="row"><C>cases[].mismatches[]</C></th><td>At most four: the first difference for each asserted key, in the order status, code, effects, post.</td></tr>
          <tr><th scope="row"><C>mismatches[].pointer</C></th><td>RFC 6901 pointer, for example <C>/post/balances/0/amount</C>. Arrays of different length give <C>…/length</C>.</td></tr>
          <tr><th scope="row"><C>mismatches[].expected</C>, <C>mismatches[].actual</C></th><td>The value, or a summary: <C>{'{type: "missing"}'}</C>, <C>{'{type: "array", length}'}</C>, <C>{'{type: "record", fields}'}</C>, or <C>{'{type: "string", length, prefix}'}</C> for strings over 256 characters.</td></tr>
        </tbody>
      </Table>
      <p>The report is at most 524288 bytes (<C>BETA_CASE_RESULT_BOUND</C>).</p>
      <CodeDetails summary="Transfer tests written by mori init" kind="file" title="mori.tests.json (transfer template)">
        {transferTests}
      </CodeDetails>
      <CodeDetails summary="Repayment tests written by mori init --template repay" kind="file" title="mori.tests.json (repay template)">
        {repayTests}
      </CodeDetails>
      <SeeAlso
        items={[
          { mode: 'explanation', href: `${E}#why-derive-expectations`, label: 'Why expected amounts must be derived independently' },
          { mode: 'howto', href: `${H}#write-test`, label: 'How to write a test with complete effects and post state' },
        ]}
      />
    </Section>
  );
}

/* ------------------------------------------------------------------ Operation families */

function FamilyEntrySection({ entry }: { entry: FamilyEntry }) {
  const fam = FAMILIES.find((f) => f.id === entry.id)!;
  const name = entry.file.replace('examples/', '');
  return (
    <Section id={`family-${entry.id}`} title={`${fam.name}: ${name}`} nav={fam.name} group="Operation families">
      <FamilyStrip id={entry.id} here="reference" />
      <Table caption={`${fam.name}: file`}>
        <tbody>
          <tr>
            <th scope="row">File</th>
            <td>
              <a href={pkg(entry.file)}><C>{`${BETA_PATH}/${entry.file}`}</C></a> (size and SHA-256:{' '}
              <a href="#example-files">example files</a>)
            </td>
          </tr>
          <tr><th scope="row">Agreement</th><td><C>{entry.agreement}</C></td></tr>
          <tr><th scope="row">Support</th><td>See <a href="#support-matrix">support by operation family</a>.</td></tr>
        </tbody>
      </Table>
      <Table caption={`${fam.name}: operations and required named arguments`}>
        <thead>
          <tr>
            <th scope="col">Action and intent</th>
            <th scope="col">Operation</th>
            <th scope="col">Required arguments</th>
            <th scope="col">Other intent fields</th>
          </tr>
        </thead>
        <tbody>
          {entry.operations.map((o) => (
            <tr key={o.action}>
              <th scope="row">
                <C>{o.action}</C>
                <br />
                <span style={{ fontWeight: 400 }}>uses</span> <C>{o.intent}</C>
              </th>
              <td><C>{o.operation}</C></td>
              <td><Codes items={(REGISTRY_MAP.get(o.operation) ?? []).map(([a, role]) => `${a}: ${role}`)} /></td>
              <td><Codes items={o.other} /></td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption={`${fam.name}: declarations and economic identities`}>
        <thead>
          <tr>
            <th scope="col">Name</th>
            <th scope="col">Kind</th>
            <th scope="col">Economic id</th>
            <th scope="col">Domain</th>
            <th scope="col">Other fields</th>
          </tr>
        </thead>
        <tbody>
          {entry.declarations.map((d) => (
            <tr key={d.name}>
              <th scope="row"><C>{d.name}</C></th>
              <td><C>{d.kind}</C></td>
              <td>{d.id ? <C>{d.id}</C> : 'none'}</td>
              <td>{d.domain ? <C>{d.domain}</C> : 'none'}</td>
              <td>{d.fields ? <C>{d.fields}</C> : 'none'}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <Table caption={`${fam.name}: quantities, scalars and atoms`}>
        <thead>
          <tr>
            <th scope="col">Written</th>
            <th scope="col">Where</th>
            <th scope="col">Type</th>
            <th scope="col">Scale</th>
            <th scope="col">Atoms or value</th>
          </tr>
        </thead>
        <tbody>
          {entry.values.map((v) => (
            <tr key={v.where}>
              <th scope="row"><C>{v.literal}</C></th>
              <td><C>{v.where}</C></td>
              <td><C>{v.type}</C></td>
              <td>{v.scale || 'none'}</td>
              <td>{v.value}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <CodeDetails summary={`Complete file: ${name}`} kind="file" title={name} href={pkg(entry.file)}>
        {entry.source}
      </CodeDetails>
    </Section>
  );
}

/* ------------------------------------------------------------------ Release */

const EXAMPLE_FILES: [string, number, string, StatusName | string][] = [
  ['examples/amm.mori', 1095, '6e1c3790202964bd4202be9fa67cd7f81beb559c25918055e023442ba9c0ef11', 'SpecifiedOnly'],
  ['examples/lending.mori', 1062, '2a85e347e953120c070982dd844c65c7afbe2899ec198c242e0aceea09995bb3', 'SpecifiedOnly'],
  ['examples/stablecoin.mori', 1138, 'a9695866e7bdbc3474735c632cef207682d1c50a7be25694489ebe36a6ce7400', 'SpecifiedOnly'],
  ['examples/derivatives.mori', 1244, '17b0e9ea666e0867b032c40605a5a89b02beddc043affa9a1d1d6256cf187449', 'SpecifiedOnly'],
  ['examples/oracle.mori', 897, '2da41606e65fe6385df8e148bc4ade41a802d377245323d8bc7838944372ac04', 'SpecifiedOnly'],
  ['examples/governance.mori', 971, '1a9eb7c670f957429d5a4c82d89ed0f3dd1e8147210c3e524bbfd34885b2af4f', 'SpecifiedOnly'],
  ['examples/bridge.mori', 1227, 'ed1899d32b244e5f2d7cab829baa4135c13722cc3340fbe90febaa08aaf12c27', 'SpecifiedOnly'],
  ['examples/staking.mori', 1303, '301cfa8c15f1a848db7b6a23aa3b246fd646a970b8212458d5990e1da8a9b385', 'SpecifiedOnly'],
  ['examples/local/repay/repayment.mori', 916, '1fc1749eba918c220bcf28e10564f6532787a00282ea68e96e7e8c058d4a9158', 'LocalS0'],
  ['examples/local/repay/scenario.json', 717, '00bfbfaac6d28d7d13c0450e7204634e26a3768901be79c54f80eb9fba19fe8e', 'scenario'],
  ['examples/local/repay/mori.tests.json', 2229, '53b0c1ae8bff2bec3809313fac4d5a7b6878b9f806a896b48ec96f59fb475155', 'tests'],
  ['examples/signed-intent/transfer-schnorr-raw/program.mori', 960, 'fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c', 'LocalS0'],
  ['examples/signed-intent/transfer-schnorr-raw/scenario.json', 556, '12928a795bba2435140fc22c0d33f039da3b04c8de6c244d0c4da338900f0430', 'scenario'],
  ['examples/signed-intent/transfer-schnorr-raw/mori.tests.json', 1914, '85fe2024f3eb781bb80fd674bf04d5e68c93c68275a08955cb800fe6ad96493a', 'tests'],
  ['examples/signed-intent/transfer-schnorr-raw/signature.json', 1773, 'c959c5f0fff7489edd1787e68fb452b9a335ee6b1164ad3329fc4e151e3e3b12', 'signature'],
  ['examples/signed-intent/transfer-schnorr-raw/expected.json', 1234, '5ce7952b751ea263b026dc148974678fb27228fb8131ec99f28cb66440261565', 'expected result'],
  ['examples/signed-intent/transfer-ecdsa-wallet/program.mori', 960, 'fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c', 'LocalS0'],
  ['examples/signed-intent/transfer-ecdsa-wallet/scenario.json', 556, '12928a795bba2435140fc22c0d33f039da3b04c8de6c244d0c4da338900f0430', 'scenario'],
  ['examples/signed-intent/transfer-ecdsa-wallet/mori.tests.json', 1914, '85fe2024f3eb781bb80fd674bf04d5e68c93c68275a08955cb800fe6ad96493a', 'tests'],
  ['examples/signed-intent/transfer-ecdsa-wallet/signature.json', 1798, '116c238a8b18638dc282bc8efb323926ee440fe506aa2eaff0a37d50bf02969f', 'signature'],
  ['examples/signed-intent/transfer-ecdsa-wallet/expected.json', 1234, '5ce7952b751ea263b026dc148974678fb27228fb8131ec99f28cb66440261565', 'expected result'],
  ['examples/signed-intent/repay-ecdsa-raw/program.mori', 915, 'cda0142e991033fba9eb1551e9188e1bb2b3b953513d2893473810ef5e1b1c66', 'LocalS0'],
  ['examples/signed-intent/repay-ecdsa-raw/scenario.json', 717, '00bfbfaac6d28d7d13c0450e7204634e26a3768901be79c54f80eb9fba19fe8e', 'scenario'],
  ['examples/signed-intent/repay-ecdsa-raw/mori.tests.json', 2225, 'da428ac85b5b53e43e90809f6e614cb4837ddba5cfb67b6a02635d8b97a3461f', 'tests'],
  ['examples/signed-intent/repay-ecdsa-raw/signature.json', 1768, 'a8c52b3b6a8357a75ab22e35cc8f199cc5a7a6e6d99f9c339ec67291b97d0e8c', 'signature'],
  ['examples/signed-intent/repay-ecdsa-raw/expected.json', 1484, '9cb8aa4b7740b79e18fd9f3195a8582a5307f32b122cd3d0c8367b8baccc39c0', 'expected result'],
];

const STATUS_LABELS: string[] = ['LocalS0', 'SpecifiedOnly'];

function Examples() {
  return (
    <Section id="example-files" title="Example files" nav="Example files" group="Release">
      <p>
        Files under <a href={`${REPOSITORY}/tree/${PINNED}/${BETA_PATH}/examples`}><C>{`${BETA_PATH}/examples`}</C></a>{' '}
        at commit {PINNED_SHORT}. Sizes and digests are of the committed bytes. Every <C>mori test</C> on the four
        directories with a <C>mori.tests.json</C> reports <C>TestsPassed</C>.
      </p>
      <Table caption="Shipped example files">
        <thead>
          <tr>
            <th scope="col">File</th>
            <th scope="col">Bytes</th>
            <th scope="col">SHA-256</th>
            <th scope="col">Label or content</th>
          </tr>
        </thead>
        <tbody>
          {EXAMPLE_FILES.map(([file, bytes, sha, label]) => {
            const fam = FAMILY_ENTRIES.find((f) => f.file === file);
            return (
              <tr key={file}>
                <th scope="row"><a href={pkg(file)}><C>{file.replace('examples/', '')}</C></a></th>
                <td>{bytes}</td>
                <td><Digest hex={sha} /></td>
                <td>
                  {STATUS_LABELS.includes(label) ? <Chip name={label as StatusName} /> : label}
                  {fam ? <> (<a href={`#family-${fam.id}`}>entry</a>)</> : null}
                </td>
              </tr>
            );
          })}
        </tbody>
      </Table>
      <Table caption="Files written by mori init. Digests are in the table above.">
        <thead>
          <tr>
            <th scope="col">Template</th>
            <th scope="col">Source file</th>
            <th scope="col">Bytes</th>
            <th scope="col">Same bytes as</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="row"><C>transfer</C></th>
            <td><C>invoice.mori</C></td>
            <td>960</td>
            <td><C>signed-intent/transfer-*/program.mori</C></td>
          </tr>
          <tr>
            <th scope="row"><C>repay</C></th>
            <td><C>repayment.mori</C></td>
            <td>915</td>
            <td><C>signed-intent/repay-ecdsa-raw/program.mori</C>. <C>local/repay/repayment.mori</C> adds a final newline.</td>
          </tr>
        </tbody>
      </Table>
      <Table caption="Signed-intent example directories">
        <thead>
          <tr>
            <th scope="col">Directory</th>
            <th scope="col">Action</th>
            <th scope="col">Scheme</th>
            <th scope="col">Framing</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><C>transfer-schnorr-raw</C></th><td><C>pay</C></td><td><C>schnorr_bip340</C></td><td><C>raw</C></td></tr>
          <tr><th scope="row"><C>transfer-ecdsa-wallet</C></th><td><C>pay</C></td><td><C>ecdsa_secp256k1_sha256</C></td><td><C>midnight-sign-data</C></td></tr>
          <tr><th scope="row"><C>repay-ecdsa-raw</C></th><td><C>repay_loan</C></td><td><C>ecdsa_secp256k1_sha256</C></td><td><C>raw</C></td></tr>
        </tbody>
      </Table>
      <SeeAlso
        items={[
          { mode: 'howto', href: `${H}#check-family`, label: 'How to check an example from the eight DeFi families' },
          { mode: 'explanation', href: `${E}#purpose`, label: 'What Moriarty is for' },
        ]}
      />
    </Section>
  );
}

function Release() {
  return (
    <Section id="release" title="Release and sources" nav="Release and sources" group="Release">
      <Table caption="Release">
        <tbody>
          <tr><th scope="row">Repository</th><td><a href={REPOSITORY}>CharlesHoskinson/Moriarty</a></td></tr>
          <tr><th scope="row">Package</th><td><C>@moriarty-lang/beta</C>, directory <C>{BETA_PATH}</C></td></tr>
          <tr><th scope="row">Version</th><td><C>{PACKAGE_VERSION}</C></td></tr>
          <tr><th scope="row">Language profile</th><td><C>{PROFILE}</C></td></tr>
          <tr><th scope="row">Command</th><td><C>mori</C> (<C>dist/cli.js</C>)</td></tr>
          <tr><th scope="row">Runtime</th><td>Node 24 or later</td></tr>
          <tr><th scope="row">Distribution</th><td>Built from a checkout; not published to npm. <C>npm pack</C> produces <C>moriarty-lang-beta-{PACKAGE_VERSION}.tgz</C> for a local install.</td></tr>
          <tr><th scope="row">Commit of these pages</th><td><a href={`${REPOSITORY}/tree/${PINNED}`}><Digest hex={PINNED} /></a></td></tr>
        </tbody>
      </Table>
      <Table caption="Canonical texts at the pinned commit">
        <thead>
          <tr>
            <th scope="col">Document</th>
            <th scope="col">Covers</th>
          </tr>
        </thead>
        <tbody>
          <tr><th scope="row"><a href={GETTING_STARTED}>GETTING-STARTED.md</a></th><td>Install, program shape, scenario and test formats, diagnostics.</td></tr>
          <tr><th scope="row"><a href={SIGNED_INTENT}>SIGNED-INTENT.md</a></th><td>The <C>intent</C> and <C>verify-intent</C> flow and the native verifier.</td></tr>
          <tr><th scope="row"><a href={PRODUCT_CONTRACT}>MORIARTY-PRODUCT-CONTRACT.md</a></th><td>The product contract.</td></tr>
          <tr><th scope="row"><a href={pkg('README.md')}>README.md</a></th><td>The package overview.</td></tr>
        </tbody>
      </Table>
      <p>
        <a href="docs/language.html">docs/language.html</a> documents a different, successor profile,{' '}
        <C>moriarty-financial-agreement-source/5</C>, and its sibling profiles. It is not the reference for{' '}
        <C>{PROFILE}</C>; its samples are not accepted by the beta tools.
      </p>
      <SeeAlso
        items={[
          { mode: 'howto', href: `${H}#install-archive`, label: 'How to pack and install a local archive' },
          { mode: 'explanation', href: `${E}#horizon`, label: 'The proposed horizon language' },
        ]}
      />
    </Section>
  );
}

function ReadingCodeBlocks() {
  const kinds: [string, string, string][] = [
    ['Shell', 'Commands to run in a shell.', 'yes'],
    ['Complete file', 'A whole file, byte for byte.', 'yes'],
    ['Excerpt', 'Part of a file. Not runnable on its own.', 'yes'],
    ['Output', 'What a command printed.', 'no'],
    ['Proposed syntax, not accepted by the beta', 'Syntax proposed for a later profile. No beta tool accepts it.', 'no'],
  ];
  return (
    <Section id="reading-code-blocks" title="How code blocks are labelled" nav="Code block labels" group="Release">
      <Table caption="Code block tags">
        <thead>
          <tr>
            <th scope="col">Tag</th>
            <th scope="col">Content</th>
            <th scope="col">Copy control</th>
          </tr>
        </thead>
        <tbody>
          {kinds.map(([tag, content, copy]) => (
            <tr key={tag}>
              <th scope="row">{tag}</th>
              <td>{content}</td>
              <td>{copy}</td>
            </tr>
          ))}
        </tbody>
      </Table>
      <SeeAlso items={[{ mode: 'explanation', href: `${E}#horizon`, label: 'The proposed horizon language' }]} />
    </Section>
  );
}

export default function Reference() {
  return (
    <DocShell page="reference" eyebrow="Reference" title="Reference" lead={<Contents />} groups={GROUPS} path={[]}>
      <Cli />
      <Language />
      <Results />
      <Diagnostics />
      <SupportLabels />
      <SupportMatrix />
      <PremisesBindings />
      <ScenarioFormat />
      <TestFormat />
      {FAMILY_ENTRIES.map((e) => (
        <FamilyEntrySection key={e.id} entry={e} />
      ))}
      <Examples />
      <Release />
      <ReadingCodeBlocks />
    </DocShell>
  );
}

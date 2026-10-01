import { DocShell } from '../DocShell';
import { Code, Section, SeeAlso, Table, ThemedFigure } from '../components';
import { DesignMap } from '../DesignMap';
import { GETTING_STARTED, PRODUCT_CONTRACT, PROFILE, SIGNED_INTENT } from '../data';
import { Families } from '../explanation/Families';
import { Horizon } from '../explanation/Horizon';
import {
  CONSOLIDATED_DESIGN,
  H,
  MIDNIGHT_LEDGER_INTENT,
  R,
  ROADMAP,
  T,
  VERIFIER_CRATE,
  term,
} from '../explanation/links';

const PIPELINE_CAPTION = 'The five steps. A rejected proposal returns to Propose.';

const STARTER_INTENT = `intent Payment = {
  domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
  valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: price + fee, fee_cap: fee, net_floor: price,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee),
  source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };`;

const NAMES_EXCERPT = `domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
account Buyer = { domain: Preview, id: "Owner" };`;

const FEE_CAP_EXCERPT = `gross_cap: price + fee,   // 1010 atoms: the fee counts toward the cap
fee_cap: fee,             // 10 atoms
net_floor: price,         // the recipient must receive at least 1000`;

export default function Explanation() {
  return (
    <DocShell
      page="explanation"
      eyebrow="Explanation"
      title="Why Moriarty works the way it does"
      lead={
        <p>
          A Moriarty program writes out every term of a financial agreement, and the owner signs those terms. This
          page explains that choice, how one local run works, what a pass shows, what each of the eight DeFi
          examples specifies, and what is only proposed. The full limits are in{' '}
          <a href="#stance">what these pages do not claim</a>.
        </p>
      }
      groups={['Ideas', 'How it works', 'Limits', 'The eight examples', 'Proposed directions']}
      path={[]}
    >
      {/* ------------------------------------------------------------------ Ideas */}
      <Section id="purpose" title="What Moriarty is for" nav="What it is for" group="Ideas">
        <p>
          A Moriarty program names the asset, the domain, the paying and receiving accounts, the amount, the caps
          and the round window of an agreement. The part that holds these terms is the{' '}
          <a href={term('intent')}>intent</a>. Here is the intent of the starter transfer:
        </p>
        <Code kind="excerpt" title="The intent of the starter transfer">
          {STARTER_INTENT}
        </Code>
        <p>
          In a contract written as code, the financial meaning is whatever the code does. In Moriarty the source
          states the terms, and the tools check them. <code>mori check</code> reads the whole program. For a
          transfer or a loan repayment, <code>mori simulate</code> works out what the agreement does to a starting
          position that you state. A reader can check the terms by reading them. The cost is length: the starter
          transfer is 18 lines for one payment of 10.00 USD.
        </p>
        <p>
          You can write, check and simulate programs with the local <code>mori</code> tool alone. You need no
          account, registry or hosted service. The Federated DeFi Kernel (the{' '}
          <a href={term('kernel')}>kernel</a>) is a separate, optional set of services that may later coordinate
          evidence and routing for such programs. No beta command calls it; <a href="#federation">the federation
          section</a> says where it fits.
        </p>
        <p>
          These pages cover the beta profile <code>{PROFILE}</code>. In it, two operations, <code>transfer</code>{' '}
          and <code>repay</code>, run locally, and the eight DeFi examples are checked only. The{' '}
          <a href="docs/language.html">formal language specification</a> describes a richer successor profile. It
          has different syntax, and the beta tools reject its samples. It shows where the design is headed; this
          package has no tool for it.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'tutorial', href: `${T}#overview`, label: 'Tutorial: check, run and test your first agreement' },
            { mode: 'reference', href: `${R}#release`, label: 'Reference: release and sources' },
          ]}
        />
      </Section>

      <Section id="midnight" title="Moriarty and Midnight" nav="Moriarty and Midnight" group="Ideas">
        <p>
          Moriarty is meant to compile to Midnight's ZKIRv3 and run on Midnight. The beta has no compiler, makes no
          proof and never connects to a Midnight node. Today, only a few labels and one signing format are
          specific to Midnight.
        </p>
        <h3>The target</h3>
        <p>
          The <a href={PRODUCT_CONTRACT}>product contract</a> sets the target. A compiled program is a{' '}
          <a href={term('zkir')}>ZKIRv3</a> artifact that runs on Midnight. Midnight's native proof stack proves and
          verifies it; that stack derives from Halo2 and uses PLONK with KZG commitments. Generated Compact source
          can be an intermediate step only. It counts only when a pinned ZKIRv3 artifact exists and Midnight accepts
          it. A result from the local evaluator is not the target either.
        </p>
        <h3>What is specific to Midnight in the beta</h3>
        <ul>
          <li>
            Every example declares{' '}
            <code>{'domain Preview = { id: "Midnight", chain: "midnight", network: "preview" }'}</code>. These are
            labels. <code>Preview</code> is the source name, <code>Midnight</code> is the economic id that scenarios
            and results use, and <code>chain</code> and <code>network</code> are strings. Nothing connects to
            Midnight's Preview network.
          </li>
          <li>
            <code>mori intent --framing midnight-sign-data</code> puts the prefix{' '}
            <code>{'midnight_signed_message:<frame byte length>:'}</code> before the intent frame. No test with a
            live wallet has been done (<a href={SIGNED_INTENT}>SIGNED-INTENT.md</a>).
          </li>
          <li>
            The native verifier that you build, <a href={VERIFIER_CRATE}>experiments/midnight-crypto</a>, checks
            BIP340 Schnorr and secp256k1 ECDSA signatures. It uses the <code>midnight-base-crypto</code> crate from
            the midnight-ledger repository, at a pinned revision.
          </li>
        </ul>
        <h3>Words that mean something different on the ledger</h3>
        <Table caption="Terms in moriarty-beta/1 that also exist on Midnight">
          <thead>
            <tr>
              <th scope="col">Term</th>
              <th scope="col">In the beta</th>
              <th scope="col">What it is not</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">intent</th>
              <td>A source declaration of the owner's terms: signer, nonce, caps, round window and one operation.</td>
              <td>
                The Midnight ledger also has an <a href={MIDNIGHT_LEDGER_INTENT}>Intent</a> type, a part of a
                transaction with guaranteed and fallible offers. The beta maps nothing to it.
              </td>
            </tr>
            <tr>
              <th scope="row">domain</th>
              <td>A declaration with an economic id and two strings, <code>chain</code> and <code>network</code>.</td>
              <td>Not a network connection, a contract address or a ledger.</td>
            </tr>
            <tr>
              <th scope="row">head</th>
              <td>
                A string in the scenario. <code>pre_head</code> is where the operation starts (<code>h0</code>);
                a successful run sets <code>h1</code>.
              </td>
              <td>Not a block hash or a commitment to contract state.</td>
            </tr>
            <tr>
              <th scope="row">replay key</th>
              <td>
                The domain, signer and nonce. A run marks it as used in the candidate post-state. Nothing is saved
                between runs.
              </td>
              <td>Not a nullifier or a nonce stored on a ledger.</td>
            </tr>
            <tr>
              <th scope="row">round</th>
              <td>An integer in the scenario, compared with the intent's <code>valid</code> window.</td>
              <td>Not the block height or the chain time.</td>
            </tr>
            <tr>
              <th scope="row">proof</th>
              <td>The beta produces none.</td>
              <td>The target is a Midnight native proof of the compiled program.</td>
            </tr>
          </tbody>
        </Table>
        <p>
          Balances in the beta are plain account records in a JSON file. The beta has no notion of shielded or
          unshielded tokens. In a transfer or repayment intent, <code>disclosures</code> must be empty.
        </p>
        <SeeAlso
          items={[
            { mode: 'howto', href: `${H}#sign-intent`, label: 'How to verify a signed intent locally' },
            { mode: 'reference', href: `${R}#scenario-format`, label: 'Reference: local scenario format' },
          ]}
        />
      </Section>

      <Section id="from-solidity" title="If you know Solidity" nav="If you know Solidity" group="Ideas">
        <p>
          A Moriarty agreement is data, not code. It has no function bodies, loops, conditionals or storage. Each
          row below is an analogy to help you read the beta, not an equivalence.
        </p>
        <Table caption="Solidity ideas and their nearest counterpart in moriarty-beta/1">
          <thead>
            <tr>
              <th scope="col">In Solidity</th>
              <th scope="col">Nearest counterpart in the beta</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">A contract with functions</th>
              <td>
                An agreement of declarations. An <code>action</code> names an intent, and the intent names one
                operation, <code>transfer</code> or <code>repay</code>, with named arguments.
              </td>
            </tr>
            <tr>
              <th scope="row">
                <code>msg.sender</code>
              </th>
              <td>
                The intent's <code>signer</code>. For a transfer it must be the <code>from</code> account, or{' '}
                <code>check</code> reports <code>BETA_SIGNER</code>. Nothing authenticates it locally.
              </td>
            </tr>
            <tr>
              <th scope="row">
                <code>uint256</code> amounts and <code>decimals</code>
              </th>
              <td>
                <a href={term('atom')}>Atoms</a>: whole numbers from 0 to 2<sup>128</sup> − 1, with the asset's{' '}
                <code>scale</code>. There is no division.
              </td>
            </tr>
            <tr>
              <th scope="row">ERC-20 allowance</th>
              <td>
                The scenario's <code>allowance</code>. A transfer uses the gross debit, 1010 atoms in the starter. A
                debit above <code>remaining</code> gives <code>CoreRejected</code> with <code>S0_AUTH_SCOPE</code>.
              </td>
            </tr>
            <tr>
              <th scope="row">Account nonce</th>
              <td>
                The intent's <code>nonce</code>, part of the <a href={term('replay-key')}>replay key</a>. A scenario
                with <code>replay: "consumed"</code> gives <code>S0_HISTORY_REPLAY</code>.
              </td>
            </tr>
            <tr>
              <th scope="row">
                <code>require</code> and revert
              </th>
              <td>
                <code>CoreRejected</code> with a judgment and a code, for example <code>S0_EFFECT_RANGE</code> for
                too small a balance. A compile error is closer to <code>AuthoringRejected</code> with a{' '}
                <code>BETA_</code> code.
              </td>
            </tr>
            <tr>
              <th scope="row">A deadline on block number</th>
              <td>
                <code>valid: rounds(domain: Preview, from: 0, to: 10)</code>, compared with the scenario's{' '}
                <code>round</code>. Outside the window: <code>S0_INTENT_SCOPE</code>.
              </td>
            </tr>
            <tr>
              <th scope="row">Contract storage</th>
              <td>
                The <a href={term('scenario')}>scenario</a> file you write. Nothing checks it against a ledger, and
                nothing is saved between runs.
              </td>
            </tr>
            <tr>
              <th scope="row">Gas limit</th>
              <td>
                <code>work_remaining</code> and <code>work_spent</code>, a local step budget. The starter transfer
                spends 1. No fee is paid for it.
              </td>
            </tr>
            <tr>
              <th scope="row">Deploy, then send a transaction</th>
              <td>No counterpart. The beta has no compiler, proof, network or ledger.</td>
            </tr>
          </tbody>
        </Table>
        <SeeAlso
          items={[
            { mode: 'tutorial', href: `${T}#transfer`, label: 'Tutorial: check and run the starter transfer' },
            { mode: 'reference', href: `${R}#diagnostics`, label: 'Reference: diagnostic codes' },
          ]}
        />
      </Section>

      <Section id="names-ids-symbols" title="Names, identities, symbols and atoms" nav="Names and atoms" group="Ideas">
        <p>
          A Moriarty program has three kinds of name, and they are easy to confuse. A <em>source name</em> such as{' '}
          <code>Buyer</code> or <code>USD</code> is a label inside one file. The <code>id</code> field is the{' '}
          <em>economic identity</em> that scenarios, results and the evaluator use. A <code>symbol</code> such as{' '}
          <code>"USD"</code> is display text only.
        </p>
        <Code kind="excerpt" title="From the starter transfer">
          {NAMES_EXCERPT}
        </Code>
        <p>
          In these lines the account is <code>Owner</code> in the domain whose id is <code>Midnight</code>; its
          source name is <code>Preview</code>. Renaming <code>Buyer</code> leaves the identity unchanged. Two
          declarations with the same economic id are rejected. In some examples the source name and the id are the
          same string, such as <code>Alice</code>. Renaming the declaration still does not change the id.
        </p>
        <h3>Accounts, keys, state and custody</h3>
        <p>
          An account has two parts: a domain and an economic id. A <code>key</code> on an intent, such as{' '}
          <code>key: "key1"</code>, is only a label. It is not a public key, and it does not show who owns the
          account. The <code>inspect</code> command reports <code>authenticated: false</code> for every identity,
          because the language records names and checks no ownership or backing.
        </p>
        <p>
          The <code>owner</code> argument of an operation says who holds the asset. The owner and the quantity must
          be in the same domain. A bridge operation also names a second domain, and only its <code>source</code>{' '}
          or <code>destination</code> argument may do so. Only the transfer and repayment intents state a starting
          state (<code>pre_head</code>).
        </p>
        <h3>Why quantities are counted in atoms</h3>
        <p>
          An asset is its domain, economic id, scale and representation. An atom is the smallest unit of an asset.
          Amounts are whole numbers of atoms; at scale 2, 10.00 USD is 1000 atoms. At scale 3,{' '}
          <code>0.900 GOLD</code> is 900 atoms. Every amount in a scenario or a result is a decimal string of atoms.
        </p>
        <p>
          The language has no floating point, no division, no rounding and no implicit conversion. Two parties
          that compute the same transfer get the same integers. The cost is that a rate, a percentage fee or a
          price ratio cannot be written: an interest amount or a fee is a literal quantity that you supply. For
          terms that need rounding, the proposed horizon language makes the direction and the beneficiary part of
          the agreement (<a href="#horizon-price">proposed price orientation</a>).
        </p>
        <p>
          Equal atom counts do not make equal assets. In the bridge example, 10000 atoms of{' '}
          <code>USDCanonical</code> on <code>Preview</code> and 10000 atoms of <code>WrappedUSD</code> on{' '}
          <code>Foreign</code> are different assets, and nothing converts one into the other. A shared symbol would
          not change that.
        </p>
        <SeeAlso
          items={[
            { mode: 'tutorial', href: `${T}#transfer-derive`, label: 'Tutorial: derive the transfer amounts in atoms' },
            { mode: 'reference', href: `${R}#language`, label: 'Reference: source language, quantities and name rules' },
          ]}
        />
      </Section>

      <Section id="explicit-terms" title="Why every term is stated" nav="Explicit terms" group="Ideas">
        <p>
          A transfer or repayment intent has no defaults: all twenty fields are required (
          <a href={`${R}#intent-fields`}>Reference: intent fields</a>). The starter writes out its signer, key
          reference, nonce, prior head, round window, caps, floor, failure policy and operation. It also writes{' '}
          <code>observations: []</code>, <code>disclosures: []</code>, <code>retained_effects: []</code> and{' '}
          <code>retained_duties: []</code>. In the beta these four lists must be empty, and any entry is rejected
          with <code>BETA_FAILURE_POLICY</code>. They are written so a reader can see them. An owner who signs{' '}
          <code>disclosures: []</code> agrees to disclose nothing.
        </p>
        <p>
          The eight DeFi example intents are different. They are specified only, and only <code>operation</code>{' '}
          is required in them; the rule above applies to the two operations that run.
        </p>
        <p>
          Two of these terms need an explanation. The <em>nonce</em> is a one-time label. The nonce, the domain and
          the signer together make the <a href={term('replay-key')}>replay key</a>. A run marks the replay key as
          used, so the same intent cannot run twice from that state. The <a href={term('head')}>head</a>{' '}
          (<code>pre_head</code>) is the state the operation starts from. In the starter it is <code>h0</code>,
          and a successful run moves it to <code>h1</code>.
        </p>
        <p>
          The same idea shapes the rest of the language. Operations take named arguments only, from a closed
          registry, so an unknown or misspelt argument is rejected, not ignored. A declaration can use only names
          declared earlier in the file, so a reader finds every definition by reading up the file. The language
          has no loops or recursion, and source files have size and depth limits, so a check always ends.
        </p>
        <p>
          The owner signs three bounds. <code>gross_cap</code> limits everything that leaves the owner's account,
          the fee included. <code>fee_cap</code> limits the fee. <code>net_floor</code> sets the least the
          recipient receives. The signature covers any result inside the bounds and no result outside them,
          whoever computes the result.
        </p>
        <p>
          Explicit terms make programs long and small edits noisy. Moriarty accepts that, because a financial
          agreement is read and relied on more often than it is written.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'howto', href: `${H}#cap-with-fee`, label: 'How to cap a payment so the fee counts toward the gross debit' },
            { mode: 'reference', href: `${R}#language`, label: 'Reference: intent fields, named arguments and limits' },
          ]}
        />
      </Section>

      <Section id="three-questions" title="Authorization, acceptance and coordination" nav="Three questions" group="Ideas">
        <p>
          A result can answer three separate questions. The beta answers only the first, and only in part. A result
          that answers one question says nothing about the other two.
        </p>
        <ul>
          <li>
            <strong>Owner authorization:</strong> did the owner agree to these terms? The beta checks one thing:
            that a signature verifies under the stated public key over the prepared statement. It does not check
            that the key controls the account or that the key is unrevoked.
          </li>
          <li>
            <strong>Objective acceptance:</strong> did the transaction really happen? That needs a proof of the
            state change and acceptance by the ledger. The beta produces neither.
          </li>
          <li>
            <strong>Optional coordination:</strong> who passes evidence, routing and recovery information between
            parties? That is the federation's role. The federation never accepts a transaction for a party, and no
            program needs it.
          </li>
        </ul>
        <h3>What the owner signs and what stays unsigned</h3>
        <p>
          The local authoring tools never produce a signature. In the separate signed-intent flow,{' '}
          <code>mori intent</code> prepares a canonical intent frame, an external signer signs it, and{' '}
          <code>mori verify-intent</code> checks the signature with a native verifier. Then it runs the same local
          preparation. The signed statement contains the SHA-256 of the source bytes, the agreement, the action and
          the owner's terms (<a href={`${R}#results-signed-statement`}>Reference: the signed statement</a> has the full list). The signature
          covers the statement, not the source file directly. If you change a comment or a space, the SHA-256
          changes and you need a new signature. The formatter never changes values written inside an intent, such
          as its <code>source_hash</code>.
        </p>
        <p>
          The scenario is not signed. The person who runs the check writes its balances, allowance, work counters
          and replay state. <a href={term('core')}>Core</a>, the reference evaluator, tests the proposed operation
          against them: caps, funds, allowance, round window and replay. If you change the scenario, the signature
          stays valid, but the result can be a different candidate or a rejection. A local round or replay check
          says nothing about the current chain time or the replay state of any ledger.
        </p>
        <p>
          A successful check gives <code>SignedPreparedUnqualified</code>. The beta has no authenticated state, no
          native proof and no ledger acceptance. Signed preparation begins by expanding the action, so it works
          only for the transfer and the repayment. The eight DeFi examples expand as <code>Unsupported</code>, and
          the flow prepares no signature for them.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'howto', href: `${H}#sign-intent`, label: 'How to verify a signed intent locally' },
            { mode: 'reference', href: `${R}#results`, label: 'Reference: results, rejections and judgment order' },
          ]}
        />
      </Section>

      {/* ------------------------------------------------------------------ How it works */}
      <Section id="architecture" title="From source file to prepared result" nav="Architecture" group="How it works">
        <p>
          A result passes through up to five steps, and passing one says nothing about the next. Two steps run
          locally today, a third is an optional external check, and the last two have no tool.
        </p>
        <h3 id="architecture-map">The whole design</h3>
        <DesignMap />
        <h3 id="architecture-steps">The five steps</h3>
        <ThemedFigure name="pipeline" alt="" caption={PIPELINE_CAPTION} />
        <ol>
          <li>
            <strong>Author.</strong> A <code>.mori</code> program declares domains, accounts, assets, intents and
            actions. <code>mori check</code> reads its names, units and shapes. Runs locally.
          </li>
          <li>
            <strong>Authorize.</strong> Optional. <code>mori intent</code> prepares a canonical intent frame that
            contains the SHA-256 of the exact source bytes. An external signer signs it, and{' '}
            <code>mori verify-intent</code> checks the signature with a native verifier. The local tools do not
            sign. A valid signature does not show that the key has authority over the account.
          </li>
          <li>
            <strong>Propose.</strong> <code>mori simulate</code> applies a scenario and returns ordered effects and
            a candidate post-state, or a rejection. <code>mori expand</code> shows the generated intermediate
            program, <a href={term('source-6')}>Source/6</a>, that the evaluator runs. Runs locally, for the
            transfer and the repayment only.
          </li>
          <li>
            <strong>Prove.</strong> The target. No beta tool produces a native proof on Midnight.
          </li>
          <li>
            <strong>Settle.</strong> The target. No example here reaches ledger acceptance.
          </li>
        </ol>
        <p>
          The steps are not one fixed sequence. <code>mori simulate</code> goes from Author to Propose and skips
          Authorize. <code>mori verify-intent</code> runs Authorize and then the same local preparation. A check
          can pass while a scenario is later rejected: <code>check</code> reads only the program, and Core tests
          the program against a stated state. A rejected proposal publishes no effects or post-state; you revise
          it and propose again. <a href={`${R}#phases`}>Reference: phases</a> gives the order of checks for
          each command.
        </p>
        <p>
          Author and Propose are separate commands. That is why the eight DeFi examples can be checked although
          none can run, and why a passing check is never a payment result.
        </p>
        <h3 id="architecture-kernel">Where the kernel fits</h3>
        <p>
          The optional federation coordinates evidence and routing around these steps. It does not provide the
          ledger acceptance that Settle needs, and it stays separate from the owner's authorization and from the
          ledger's own judgment. Authoring, checking and preparation never call it. The{' '}
          <a href="kernel.html">kernel page</a> describes how it is meant to work.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'tutorial', href: `${T}#transfer`, label: 'Tutorial: run the transfer through check and simulate' },
            { mode: 'reference', href: `${R}#cli`, label: 'Reference: command-line reference' },
          ]}
        />
      </Section>

      <Section
        id="why-derive-expectations"
        title="Why expected amounts must be derived independently"
        nav="Deriving expectations"
        group="How it works"
      >
        <p>
          Do not copy expected values from <code>simulate</code>. A test built that way agrees with the evaluator by
          construction, so it cannot catch an evaluator mistake. The beta test format lets a case state the full
          ordered effect list and the full post-state, and the tutorial asks you to work those out by hand first.
        </p>
        <p>
          If your arithmetic says the owner ends with 8990 atoms and the evaluator says 8991, the test names the
          field, and you find out whether your model of the agreement or the evaluator is wrong. The shipped
          examples follow the same rule: their expectations come from the economics of the example and the stated
          fixture state, not from the output.
        </p>
        <p>
          A test that asserts only a status, or a status and a code, is weaker. It shows that the evaluator reached
          that status. It says nothing about which amounts moved. A rejection test needs only the status and code.
          A test of a successful payment should state every effect and the whole post-state.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'tutorial', href: `${T}#testing`, label: 'Tutorial: make a test fail on purpose' },
            { mode: 'howto', href: `${H}#write-test`, label: 'How to write a test with complete effects and post state' },
            { mode: 'reference', href: `${R}#test-format`, label: 'Reference: test file format' },
          ]}
        />
      </Section>

      <Section id="patterns" title="Six recurring design choices" nav="Design choices" group="How it works">
        <p>
          Six design choices recur in the examples. Each one stops a specific misreading or abuse of an agreement.
          The how-to guides show how to apply them.
        </p>
        <h3>Why the fee sits inside the cap</h3>
        <p>
          The owner's gross cap must cover the value plus the fee. The recipient's floor is checked against the
          value alone. In the starter transfer, a price of 1000 atoms and a fee of 10 give a gross debit of 1010.
          The owner goes from 10000 to 8990, the recipient receives 1000, the fee account receives 10, and the
          allowance falls by 1010.
        </p>
        <Code kind="excerpt" title="Fee inside the cap, from the starter transfer">
          {FEE_CAP_EXCERPT}
        </Code>
        <p>
          If the fee sat outside the cap, an owner who signed a cap of 1000 could lose 1010, or more if the fee
          field allowed it. With the fee inside, <code>gross_cap</code> is the most the owner can lose: here, 1010
          atoms.
        </p>
        <h3>Why interest is paid before principal</h3>
        <p>
          The <code>repay</code> operation applies a payment to <code>accrued</code> first and to{' '}
          <code>principal</code> second. With principal 100000, accrued 1000 and a payment of 3000 atoms, accrued
          becomes 0 and principal 98000. Principal first would leave 1000 atoms of interest unpaid and cut principal
          by 3000, so any later interest would run on a smaller balance. The order is part of the operation, so
          both parties read the same allocation from the same source.
        </p>
        <p>
          The beta computes no interest. It has no rate, day count or accrual date: <code>accrued</code> is a number
          you write in the scenario. The effects show one debit from the payer and one credit to the creditor; the
          split between interest and principal appears only in the obligation's new values. A payment of 101000
          atoms clears the loan and sets its status to <code>Settled</code>.
        </p>
        <h3>Why a rejection is tested as behaviour</h3>
        <p>
          A repayment above the outstanding balance is well formed, so <code>check</code> passes. Core rejects it
          with <code>S0_EFFECT_RANGE</code>, and the rejection publishes no effects or post-state. A rejection
          test states that status and code. A test with a wrong amount fails against a successful preparation; a
          rejection test passes when Core refuses.
        </p>
        <h3>Why a duty must outlive the step that creates it</h3>
        <p>
          A liquidation or a partial settlement should not erase what remains owed. If the step that sells the
          collateral also deletes the obligation, a shortfall disappears with it. The beta lending example can only
          record this as a continuation string. The proposed language gives a duty a typed shape: a claim, an
          amount owed, terms, and the actions that may continue or recover it.
        </p>
        <h3>Why recovery needs evidence, not a timeout</h3>
        <p>
          Cross-domain movement is split into separate stages that share one claim identifier. Recovery is an
          action of its own, not a side effect of waiting. A timeout alone does not show that the other side never
          received. The beta bridge states that rule as text. The proposed language makes it a type, so an unknown
          or conflicting outcome cannot release funds. The <a href="#bridges">bridge example</a> works through the
          race.
        </p>
        <h3>Why a solver chooses only inside signed bounds</h3>
        <p>
          Some agreements leave part of the result to be filled in later, for example the exact fee and output of a
          swap. Filling in those values is a <em>completion</em>, and whoever does it is a solver. The owner fixes
          recipients, assets, the net floor and the budgets, and a solver may choose only inside them. Whatever the
          solver picks, the fee cannot exceed the signed fee cap and the output cannot fall below the signed floor.
          In the beta AMM example nothing is chosen by a solver. The proposed language adds declared choice points
          with finite ranges.
        </p>
        <p>
          The proposed forms for duties, recovery and solver choice are in <a href="#horizon">the proposed horizon
          language</a>. No beta tool accepts that syntax.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'howto', href: `${H}#cap-with-fee`, label: 'How to cap a payment so the fee counts toward the gross debit' },
            { mode: 'howto', href: `${H}#test-rejection`, label: 'How to test that Core rejects a request' },
            { mode: 'tutorial', href: `${T}#repayment`, label: 'Tutorial: the accrual-first repayment' },
          ]}
        />
      </Section>

      <Section id="federation" title="The optional federation and where it fits" nav="Federation" group="How it works">
        <p>
          No program needs the federation, and no part of it ships in the beta package. An agreement whose effects
          stay on Midnight is meant to be judged by Midnight's own proof and state transition. The Federated DeFi
          Kernel is an optional coordinator around that. It is designed to discover solvers, obtain evidence, hold
          threshold custody, route a cross-chain effect, observe finality and run recovery.
        </p>
        <p>
          A required coordinator could block any program. So authoring, checking and simulation never call the
          federation, and a program needs neither it nor a project's approval. Asset-owner authorization still
          applies where the agreement uses it.
        </p>
        <p>
          The project <a href={ROADMAP}>roadmap</a> has one delivery sequence, U0 to U7. Milestone U3 covers
          conditional settlement, partial progress and recovery, including a late success that races a refund. U5
          covers the federated kernel and solvers. U3 does not depend on U5. The roadmap also lists earlier, scoped
          results outside this package, including a loan payment and a swap on Midnight's Preview network. They do
          not come from <code>{PROFILE}</code>, and these pages do not rely on them.
        </p>
        <p>
          The <a href="kernel.html#agreement">kernel page</a> describes the direct path and the optional kernel,
          with their <a href="kernel.html#status">implementation status</a>. The design behind both is in the{' '}
          <a href={CONSOLIDATED_DESIGN}>consolidated design</a> and the <a href={PRODUCT_CONTRACT}>product
          contract</a>.
        </p>
        <SeeAlso
          items={[
            { mode: 'explanation', href: 'kernel.html', label: 'The Federated DeFi Kernel' },
            { mode: 'reference', href: `${R}#support-matrix`, label: 'Reference: support by operation family' },
          ]}
        />
      </Section>

      {/* ------------------------------------------------------------------ Limits */}
      <Section id="stipulation" title="What a passing local run does and does not show" nav="What a pass shows" group="Limits">
        <p>
          A pass shows that Core, given the state you stated, produced exactly the records you expected. It does not
          show that the state is true. A local run starts from a <a href={term('scenario')}>scenario</a>: a JSON
          file with the balances, allowance, work counters, replay state and head before the operation. The
          scenario is a <a href={term('stipulation')}>stipulation</a>. It says "suppose the world looks like this",
          and nothing checks that the world does.
        </p>
        <p>
          The status <code>PreparedUnqualified</code> means that Core prepared a candidate result that no
          authentication, proof or ledger has qualified. The <a href={term('qualification')}>qualification</a>{' '}
          field, <code>local-stipulation-only</code>, says what the result rests on: your scenario. A case that
          asserts only a status and code shows that status and code, and nothing about amounts.
        </p>
        <h3>Premises and bindings a local result leaves open</h3>
        <p>
          A local preparation of the transfer or repayment rests on four external premises and four unverified
          bindings (<a href={`${R}#premises-bindings`}>Reference: local premises and unverified bindings</a>). Each
          names something a real system would still have to establish.
        </p>
        <p>
          The same is true of the claims a source makes about itself. <code>source_hash</code> and{' '}
          <code>policy_digest</code> are claims written in the intent. The computed <code>sourceHash</code> is a
          digest of the actual input text. Nothing compares the two for you.
        </p>
        <p>
          The premise and binding lists belong to <a href={term('s0')}>S0</a>, the small set of operations the
          local evaluator implements (the transfer and the repayment). They are defined in the package's{' '}
          <a href={GETTING_STARTED}>GETTING-STARTED.md</a>. On an action that is{' '}
          <a href={term('specified-only')}>specified only</a>, the lists are empty because the S0 catalog does not
          apply. The action's premises are unknown, not absent.
        </p>
        <SeeAlso
          items={[
            { mode: 'reference', href: `${R}#premises-bindings`, label: 'Reference: local premises and unverified bindings' },
            { mode: 'reference', href: `${R}#scenario-format`, label: 'Reference: local scenario format' },
          ]}
        />
      </Section>

      <Section id="stance" title="What these pages do not claim" nav="What is not claimed" group="Limits">
        <p>These limits apply to every page of this documentation.</p>
        <ul>
          <li>
            Nothing is signed by the local tools, proved, sent, deployed or settled. <code>mori intent</code>{' '}
            prepares bytes for an external signer, and <code>mori verify-intent</code> checks a signature someone
            else made. A verified signature shows that this key signed this statement. It does not show that the
            key controls the account, that the key is unrevoked, or that any ledger saw the transaction.
          </li>
          <li>
            No beta command compiles, proves, sends or deploys anything. The package is built from a checkout and
            is not published to a registry.
          </li>
          <li>
            A scenario is a stipulation, not a fact about any ledger. A prepared result is a candidate computed from
            it. It is not authenticated, proved, signed or settled.
          </li>
          <li>
            Identities, chain and network strings, source hashes and policy digests are claims written in the
            source. Nothing authenticates them.
          </li>
          <li>
            The eight DeFi examples are specified only. <code>check</code> accepts them; <code>expand</code> and{' '}
            <code>simulate</code> answer <code>Unsupported</code> with <code>BETA_PROFILE_UNSUPPORTED</code> and
            publish no effects or post-state. On that refusal, <code>qualification: local-stipulation-only</code>{' '}
            says the refusal came from the local tool. It says nothing about the financial action.
          </li>
          <li>
            Proposed horizon syntax appears only in <a href="#horizon">its own section</a>. <code>check</code>{' '}
            rejects a file that declares the horizon profile, with <code>BETA_PROFILE</code>.
          </li>
          <li>
            Each example carries a <a href={`${R}#support-labels`}>support label</a> that says what the tools do
            with that one file today. A local pass is called a local pass. The limits of a single example, such as
            a string the checker stores but does not enforce, are stated with that example.
          </li>
        </ul>
        <SeeAlso
          items={[
            { mode: 'reference', href: `${R}#support-labels`, label: 'Reference: support labels' },
            { mode: 'reference', href: `${R}#support-matrix`, label: 'Reference: support by operation family' },
            { mode: 'reference', href: `${R}#release`, label: 'Reference: release and sources' },
          ]}
        />
      </Section>

      {/* ------------------------------------------------------------------ The eight examples */}
      <Families />

      {/* ------------------------------------------------------------------ Proposed directions */}
      <Horizon />
    </DocShell>
  );
}

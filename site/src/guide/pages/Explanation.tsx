import { DocShell } from '../DocShell';
import { Code, Section, SeeAlso, ThemedFigure } from '../components';
import { GETTING_STARTED, PRODUCT_CONTRACT, PROFILE } from '../data';
import { Families } from '../explanation/Families';
import { Horizon } from '../explanation/Horizon';
import { CONSOLIDATED_DESIGN, H, R, ROADMAP, T } from '../explanation/links';

const PIPELINE_CAPTION =
  'The five steps, in order. Author and Propose are covered locally by the beta tooling; Authorize is a separate optional external check; Prove and Settle are the target and are not delivered. A rejected proposal returns to Propose to be revised.';

const FEE_CAP_EXCERPT = `gross_cap: price + fee,   // 1010 atoms: the fee counts toward the cap
fee_cap: fee,             // 10 atoms
net_floor: price,         // the recipient must receive at least 1000`;

export default function Explanation() {
  return (
    <DocShell
      page="explanation"
      eyebrow="Explanation"
      title="Explanation"
      lead={
        <p>
          Why the language is shaped the way it is: the ideas behind it, how a run is put together, where it stops,
          what the eight examples say, and where it may go next. For what nothing on these pages does, read{' '}
          <a href="#stance">what these pages do not claim</a>.
        </p>
      }
      groups={['Ideas', 'How it works', 'Limits', 'The eight examples', 'Proposed directions']}
      path={[]}
    >
      {/* ------------------------------------------------------------------ Ideas */}
      <Section id="purpose" title="What Moriarty is for" nav="What it is for" group="Ideas">
        <ThemedFigure
          name="hero"
          alt=""
          caption="Illustration: a written agreement, its parts kept in separate containers, and a gate that stays shut until the last step. Decorative: it depicts no transaction, signature, proof or settlement."
          priority
          className="doc-hero"
        />
        <p>
          Moriarty states a financial agreement so that every part of the money is explicit: which asset, in which
          domain, from which account to which account, how many units, under what cap and until which round. The
          part of a program that holds these terms is the <em>intent</em>. It records every term an owner would
          sign, so nothing is left to a default the owner never saw.
        </p>
        <p>
          Most contract languages start from code and let the financial meaning emerge from what the code does. Moriarty
          starts from the agreement. The source says what may happen, and the tools check that the text is
          well-formed and, for two operations, work out locally what the agreement would do to a stated starting
          position. The value of this order is that a reader can check the terms without simulating a program in
          their head. The cost is verbosity: a ten-dollar transfer takes a page of declarations.
        </p>
        <p>
          The language is the product. Any developer may write, check and prepare programs in it without anyone's
          permission and without a registry or hosted service. The Federated DeFi Kernel (the <em>kernel</em> for
          short) is a separate, optional federation of services that may later coordinate evidence and routing
          around such programs. No tutorial here depends on it, and the{' '}
          <a href="#federation">section on federation</a> explains where it fits.
        </p>
        <p>
          These pages cover the beta profile <code>{PROFILE}</code>. The repository also documents a richer
          successor profile in the <a href="docs/language.html">formal language specification</a>. That is a
          different language with different syntax; its samples are not accepted by the beta tools. The beta is the
          deliberately small subset that can be checked and run today, and the successor shows the direction the
          design is meant to grow in.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'tutorial', href: `${T}#overview`, label: 'Tutorial: write, check and test your first agreement' },
            { mode: 'reference', href: `${R}#release`, label: 'Reference: release and sources' },
            { mode: 'explanation', href: '#architecture', label: 'How an intent becomes a candidate' },
          ]}
        />
      </Section>

      <Section id="names-ids-symbols" title="Names, identities, symbols and atoms" nav="Names and atoms" group="Ideas">
        <p>
          Three different things carry a name in a Moriarty program, and most mistakes in reading one come from
          treating them as the same. A <em>source name</em> such as <code>Buyer</code> or <code>USD</code> is a
          label for navigating one file. The <code>id</code> field is the <em>economic identity</em> that
          scenarios, results and the evaluator use. A <code>symbol</code> such as <code>"USD"</code> is display
          text and nothing more.
        </p>
        <Code kind="excerpt" title="From the starter transfer">
          {'account Buyer = { domain: Preview, id: "Owner" };'}
        </Code>
        <p>
          In this line the account is <code>Owner</code> in the domain whose id is <code>Midnight</code>. Renaming{' '}
          <code>Buyer</code> leaves the identity unchanged, and two declarations with the same economic id are
          rejected. In some examples the source name and the id are the same string, such as <code>Alice</code>,
          but they still play different roles: renaming the declaration would not by itself retarget the id.
        </p>
        <h3>Accounts, keys, state and custody</h3>
        <p>
          An account is a domain plus an economic id. Nothing more is attached to it. A <code>key</code> on an
          intent, such as <code>key: "key1"</code>, is an opaque reference: it is not a public key and it is not
          proof that anyone owns the account. A domain's <code>chain: "midnight"</code> and{' '}
          <code>network: "preview"</code> are claims written in the source; the string <code>preview</code> is not a
          connection to Midnight's Preview network. The <code>inspect</code> command reports every one of these
          identities as <code>authenticated: false</code>, because the language records nominal identities and does
          not authenticate that an account belongs to anyone or that an asset is backed by anything.
        </p>
        <p>
          Custody is expressed by the <code>owner</code> argument of an operation. The owner and the quantity it
          moves must share one domain. Where an operation reaches another domain, as a bridge does, only the named{' '}
          <code>source</code> or <code>destination</code> argument may name it. State, in the sense of a
          predecessor or a signed head, appears only on intents that declare it, such as the transfer and
          repayment.
        </p>
        <h3>Why quantities are counted in atoms</h3>
        <p>
          An asset is its domain, economic id, scale and representation. The scale fixes how a decimal literal
          becomes an integer count of the asset's smallest unit, called an <em>atom</em>. At scale 2,{' '}
          <code>10.00 USD</code> is 1000 atoms; at scale 3, <code>0.900 GOLD</code> is 900 atoms. Every amount in a
          scenario or a result is a canonical decimal string of atoms.
        </p>
        <p>
          The language has no floating point, no division, no rounding and no implicit conversion. That rules out
          a whole class of disagreement: two parties computing the same transfer always get the same integers, and
          a rounding choice can only appear where an agreement names one. The price of this choice is that ratios,
          interest and prices cannot be written casually. Where a future operation needs them, the proposed
          language makes the rounding direction and its beneficiary explicit terms rather than a property of the
          arithmetic.
        </p>
        <p>
          Equal atom counts do not make equal assets. In the bridge example, 10000 atoms of{' '}
          <code>USDCanonical</code> on <code>Midnight</code> and 10000 atoms of <code>WrappedUSD</code> on{' '}
          <code>Foreign</code> are different assets, and nothing converts one into the other. A shared symbol
          would not change that either.
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
          A Moriarty intent has no defaults. The starter transfer writes out its signer, key reference, nonce, prior
          head, round window, caps, floor, failure policy and the operation, and it also writes{' '}
          <code>observations: []</code>, <code>disclosures: []</code>, <code>retained_effects: []</code> and{' '}
          <code>retained_duties: []</code>. Those empty lists look redundant. They are there because emptiness is
          itself a signed term: an owner who signs an intent with no disclosures has agreed to disclose nothing, and
          a later reader can see that rather than infer it.
        </p>
        <p>
          Two of those terms need a gloss. The <em>nonce</em> is a one-time label. Together with the domain and the
          signer it forms the <em>replay key</em>, which a run marks as consumed so the same intent cannot be used
          twice. The <em>head</em> (<code>pre_head</code>) names the state the operation extends; in the starter it
          is <code>h0</code>, and a successful run advances it to <code>h1</code>.
        </p>
        <p>
          The same idea shapes the rest of the language. Operations take named arguments only, from a closed
          registry, so an unknown or misspelt argument is rejected instead of being ignored. Each declaration may
          refer only to declarations written before it, so a program is a finite chain of immutable bindings and a
          reader can follow every name upward to where it was defined. Source files have fixed size and depth
          limits, so a check always terminates on input of a known size.
        </p>
        <p>
          Hard bounds are part of the signed terms. The gross cap limits everything that leaves the owner's
          account, the fee cap limits the fee, and the net floor sets the least the recipient may receive. The alternative, common in transaction formats, is to sign a computed result and trust the
          code that computed it. Signing bounds instead means the owner's consent covers every result inside the
          bounds and none outside them, whoever computes the result.
        </p>
        <p>
          The trade-off is real. Explicit terms make programs long and make small changes noisy, and a language
          with defaults is easier to write. Moriarty chooses the reader and the signer over the writer, on the view
          that a financial agreement is read and relied on far more often than it is written.
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
          Three different questions are easy to blur, and each has its own answer. A label that answers one of
          them never answers another.
        </p>
        <p>
          <strong>Owner authorization</strong> asks whether the owner agreed to these terms. The beta answers part
          of it: a native signature check on a prepared intent can succeed. Whether the key has authority over the
          account, and whether it has been revoked, stays open. <strong>Objective acceptance</strong> asks whether
          the transaction really happened: a proof of the transition and the ledger's own judgment. The beta
          produces neither. <strong>Optional coordination</strong> asks who helps carry evidence, routing and
          recovery between parties. That is the federation's role. It never accepts a transaction on anyone's
          behalf, and no program needs it.
        </p>
        <h3>What the owner signs and what stays unsigned</h3>
        <p>
          The local authoring tools never produce a signature. In the separate signed-intent flow,{' '}
          <code>mori intent</code> prepares a canonical intent frame, an external signer signs it, and{' '}
          <code>mori verify-intent</code> checks that signature with a native verifier before running the same
          local preparation. The signed statement binds the SHA-256 of the source bytes together with the agreement,
          the action and the owner's terms; the full list is in{' '}
          <a href={`${R}#results`}>Reference: results</a>. The signature covers that statement, which commits to
          the hash of the source bytes; it is not a signature over the source file itself. Editing a
          comment or a space changes the source SHA-256 and so needs a new signature. Formatting never rewrites
          the commitments written inside an intent, such as its <code>source_hash</code> claim.
        </p>
        <p>
          The scenario is not signed. Its balances, allowance (how much of the owner's balance this operation may
          spend), work counters (a local step budget the operation consumes), proposed effects and post-state are
          stated by whoever runs the check. They form a <em>candidate</em> that Core judges. Core is the reference
          evaluator: it decides caps, funds, expiry and replay for a scenario. A changed scenario can keep the
          signature valid while producing a different candidate or a rejection. Local round-window and replay
          checks do not establish the current chain time or the global replay state.
        </p>
        <p>
          A successful signature check gives the result <code>SignedPreparedUnqualified</code>. Its result fields
          are listed in <a href={`${R}#results`}>Reference: results</a>; none of them says that the key may act for
          the account, that a proof exists or that a ledger saw the transaction. Evidence that a candidate was
          accepted, meaning authenticated state, a native proof and ledger acceptance, is a separate layer that
          these pages do not claim exists. Signed preparation also starts by expanding the action, so it applies
          only to the transfer and the repayment; the eight family examples expand as unsupported, and the flow
          prepares no signature for them.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'howto', href: `${H}#sign-intent`, label: 'How to verify a signed intent locally' },
            { mode: 'reference', href: `${R}#results`, label: 'Reference: results, rejections and judgment order' },
            { mode: 'reference', href: `${R}#support-labels`, label: 'Reference: support labels' },
          ]}
        />
      </Section>

      {/* ------------------------------------------------------------------ How it works */}
      <Section id="architecture" title="How an intent becomes a candidate" nav="Architecture" group="How it works">
        <p>
          The path from source to result has a fixed order, and each step answers one question. Keeping the steps
          apart is the point: a result that has passed one step says nothing about the next, and a reader can see
          exactly where any given result stops.
        </p>
        <ThemedFigure name="pipeline" alt="" caption={PIPELINE_CAPTION} />
        <ol>
          <li>
            <strong>Author.</strong> A <code>.mori</code> program declares domains, accounts, assets, intents and
            actions. <code>mori check</code> reads its names, units and shapes. This step runs locally.
          </li>
          <li>
            <strong>Authorize.</strong> An external native verifier checks the owner's signature over a canonical
            intent frame that binds the SHA-256 of the exact source bytes. The local authoring tools do not do this
            and do not sign. A valid signature does not establish that the key has authority over the account.
          </li>
          <li>
            <strong>Propose.</strong> <code>mori simulate</code> applies a stated scenario and returns ordered
            effects and a candidate post-state, or a rejection to revise. <code>mori expand</code> shows the
            generated intermediate program, called Source/6, that the evaluator runs. This step runs locally, for
            the transfer and the repayment only.
          </li>
          <li>
            <strong>Prove.</strong> The target. No beta tool produces a native proof on Midnight.
          </li>
          <li>
            <strong>Settle.</strong> The target. Ledger acceptance is not established by any example here.
          </li>
        </ol>
        <p>
          Today's tooling covers Author and, locally, Propose. Authorize is a separate, optional check whose key
          authority is still open. Prove and Settle are shown so a reader can see where a result stops. A check can
          pass while a particular scenario is later rejected, because <code>check</code> looks at the program and
          Core looks at the program against a state. A rejected candidate is revised and proposed again; it is
          never carried forward as a result.
        </p>
        <p>
          The design could have merged Author and Propose into one command that both checks and runs. Keeping them
          apart lets the eight family examples be checked today, before any of them can be run, and makes it
          impossible to mistake a well-formed program for a successful payment.
        </p>
        <h3 id="architecture-kernel">Where the kernel fits</h3>
        <p>
          The optional federation coordinates evidence and routing around this path. It does not provide the ledger
          acceptance that Settle requires, and the owner's authorization and the ledger's own judgment stay separate
          from it. Authoring, checking and preparation never call it, and no program needs its approval. The{' '}
          <a href="kernel.html">kernel explanation</a> describes how it is meant to work.
        </p>
        <SeeAlso
          title="Put it into practice"
          items={[
            { mode: 'tutorial', href: `${T}#transfer`, label: 'Tutorial: run the transfer through check and simulate' },
            { mode: 'reference', href: `${R}#cli`, label: 'Reference: command-line reference' },
            { mode: 'explanation', href: '#federation', label: 'The optional federation and where it fits' },
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
          A test is only as good as the source of its expected values. The beta test format lets a case state the
          full ordered effect vector and the full post-state, and the tutorial asks you to work those out by hand
          before running anything. There is an obvious shortcut: run <code>simulate</code>, copy what it printed
          into the test, and the test passes. That shortcut makes the test agree with the evaluator by
          construction, so it can never catch an evaluator mistake.
        </p>
        <p>
          The useful test is the one that could disagree. When your own arithmetic says the owner ends with 8990
          atoms and the evaluator says the same, two independent computations agree. When they differ, the test
          names the field, and you learn whether your model of the agreement or the evaluator is wrong. The shipped
          examples follow the same rule: their expectations are derived from the economics of the example and the
          explicit fixture state, then compared with the output, never copied from it.
        </p>
        <p>
          The same reasoning explains why a test that asserts only a status, or a status and a code, is weaker. It
          shows that the evaluator reached that status. It says nothing about which amounts moved. Both kinds have a
          place: a rejection test needs only the status and code, while a test of a successful payment should pin
          every effect and the whole post-state.
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

      <Section id="patterns" title="Design patterns and the reasoning behind them" nav="Design patterns" group="How it works">
        <p>
          A handful of patterns recur across the examples. Each protects against a specific way an agreement can
          be misread or abused. The recipes for applying them are how-to guides; this section is about why they
          exist.
        </p>
        <h3>Why the fee sits inside the cap</h3>
        <p>
          The owner's gross cap must cover the value plus the fee, while the recipient's floor is checked against
          the value alone. In the starter transfer, a price of 1000 atoms and a fee of 10 give a gross debit of
          1010: the owner goes from 10000 to 8990, the recipient receives 1000, the fee account receives 10, and the
          allowance falls by 1010.
        </p>
        <Code kind="excerpt" title="Fee inside the cap, from the starter transfer">
          {FEE_CAP_EXCERPT}
        </Code>
        <p>
          If the fee sat outside the cap, an owner who signed a cap of 1000 could lose 1010, or any larger amount a
          fee field allowed. Putting the fee inside means the cap is the most the owner can lose, which is the
          number an owner actually cares about.
        </p>
        <h3>Why interest is paid before principal</h3>
        <p>
          A funded repayment is applied to accrued interest before principal, an order called accrual-first. With
          principal 100000, accrued 1000 and a payment of 3000 atoms, accrued becomes 0 and principal becomes
          98000, and the loan stays outstanding. Applying the same payment to principal first would leave interest
          sitting unpaid while the balance that earns interest shrinks, which is a different loan. Fixing the order
          in the operation means the borrower and the creditor cannot disagree about it later. Interest-only,
          partial and full repayment are the same operation with a different explicit amount.
        </p>
        <h3>Why a rejection is tested as behaviour</h3>
        <p>
          A payment above the outstanding balance still produces a well-formed candidate, with the debt cells
          unchanged, and Core rejects it. The rejection publishes no new effects or post-state, and the candidate
          is never presented as a prepared result. Writing that rejection as its own test makes failure a checked
          behaviour rather than an accident. It also keeps two different situations apart: a test whose money
          expectation is wrong disagrees with a successful preparation, while a rejection test agrees with a
          refusal.
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
          Cross-domain movement is split into separate stages that share one claim identifier, and recovery is an
          action of its own rather than a side effect of waiting. A timeout alone is not proof that the other side
          never received. The beta bridge states that principle as text; the proposed language would make it a
          type, so that an unknown or conflicting outcome cannot release funds. The{' '}
          <a href="#bridges">bridge example</a> works through the race in detail.
        </p>
        <h3>Why a solver chooses only inside signed bounds</h3>
        <p>
          Some agreements leave part of the result to be filled in later, for example the exact fee and output of a
          swap. Filling those values in is called a <em>completion</em>, and whoever does it is a solver. The owner
          fixes recipients, assets, the net floor and the budgets. A solver may choose only inside them. Because the
          bounds are signed terms, choice never widens authority: whatever the solver picks, the fee cannot exceed
          the signed fee cap and the output cannot fall below the signed floor. In the beta AMM example the fee cap
          and floor are fixed in the operation and nothing is chosen by a solver; the proposed language adds
          declared choice points with finite ranges.
        </p>
        <p>
          The proposed forms for duties, recovery and solver choice are written in the{' '}
          <em>horizon</em> profile, a proposed future syntax that no beta tool accepts. They are collected in{' '}
          <a href="#horizon">the proposed horizon language</a>.
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
          An agreement whose effects stay on Midnight is meant to be judged by Midnight's own proof and state
          transition. The Federated DeFi Kernel is an optional coordinator around that. It can discover solvers,
          obtain evidence, hold threshold custody, route a cross-chain effect, observe finality and run recovery. A
          supported program does not need the federation, a hosted solver or a project's approval in order to be
          authored for Midnight. Asset-owner authorization still applies where the agreement uses it.
        </p>
        <p>
          Making the federation optional is a deliberate constraint. A language that needed a coordinator to be
          useful would give that coordinator a veto over every program. Keeping it outside means the language can
          be judged on its own, and the federation has to earn its place by being useful rather than required.
        </p>
        <p>
          The project <a href={ROADMAP}>roadmap</a> places conditional settlement, including a late success racing a
          refund, in milestone U3, and the federation itself in U5. U3 does not depend on U5. The roadmap records
          scoped local evaluation, finite comparisons against a formal model, and scoped Preview loan and swap
          results as foundations. It records general source-to-ledger correspondence, native recursive history and
          private composition as open. None of these milestones is closed by the example files on these pages.
        </p>
        <p>
          The direct path and the optional kernel are both described on the <a href="kernel.html#agreement">kernel
          page</a>, with its <a href="kernel.html#status">implementation status</a>. The design behind both is in
          the <a href={CONSOLIDATED_DESIGN}>consolidated design</a> and the <a href={PRODUCT_CONTRACT}>product
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
          A local run starts from a <em>scenario</em>: a JSON file that states the balances, allowance, work
          counters, replay state and head before the operation. The scenario is a stipulation. It says "suppose the
          world looks like this", and nothing checks that the world does. Every test report therefore carries the
          qualification <code>local-stipulation-only</code>.
        </p>
        <p>
          Within that stipulation, a pass is precise. A case with complete <code>effects</code> and{' '}
          <code>post</code> expectations that passes shows that Core, given your stated state, produced exactly
          those records. A case that asserts only a status and code shows that status and code. Neither
          authenticates the scenario, checks a signature, proves anything or touches a ledger.
        </p>
        <h3>Premises and bindings a local result leaves open</h3>
        <p>
          A local preparation of the transfer or repayment rests on four external premises and four unverified
          bindings; <a href={`${R}#premises-bindings`}>Reference: local premises and unverified bindings</a> lists
          them. What they have in common is that each names something a real system would still have to
          establish, and none is closed by a matching hash or a passing test.
        </p>
        <p>
          The same holds for the claims a source makes
          about itself: <code>source_hash</code> and <code>policy_digest</code> are signed claims, while the
          computed <code>sourceHash</code> is a digest of the actual input text, and nothing compares the two for
          you.
        </p>
        <p>
          The premise and binding lists belong to the small set of operations the local evaluator implements,
          called S0 (the transfer and the repayment), and are defined in the package's{' '}
          <a href={GETTING_STARTED}>GETTING-STARTED.md</a>. On an action that is{' '}
          <a href={`${R}#support-labels`}>specified only</a>, the lists are empty.
          Empty means that this particular catalog does not apply, not that the action has no premises or is
          qualified. Authentication and the financial relation stay open there.
        </p>
        <SeeAlso
          items={[
            { mode: 'reference', href: `${R}#premises-bindings`, label: 'Reference: local premises and unverified bindings' },
            { mode: 'reference', href: `${R}#scenario-format`, label: 'Reference: local scenario format' },
            { mode: 'explanation', href: '#stance', label: 'What these pages do not claim' },
          ]}
        />
      </Section>

      <Section id="stance" title="What these pages do not claim" nav="What is not claimed" group="Limits">
        <p>
          This is the one place where the documentation states its limits in full. Other pages link here instead
          of repeating them.
        </p>
        <p>
          Nothing on these pages is signed, proved, sent, deployed or settled. The local authoring tools never
          produce a signature; the separate signed-intent flow checks a signature someone else made, and even a
          valid one leaves key authority unverified. No beta command proves, sends or deploys anything, and the
          documentation shows no such command because none exists for the beta package. The package is built from
          a checkout; no registry release is claimed.
        </p>
        <p>
          A scenario is a stipulation, not a fact about any ledger. A prepared result is a candidate computed from
          that stipulation. It is not authenticated, proved, signed or settled. Identities, chain and network
          strings, source hashes and policy digests are claims written in the source, and nothing on these pages
          authenticates them.
        </p>
        <p>
          The eight family examples are <a href={`${R}#support-labels`}>specified only</a>: their source is checked for structure and names, and
          execution is refused as unsupported, with no effects or post-state published. The{' '}
          <code>qualification: local-stipulation-only</code> that appears on such a refusal labels the refusal
          itself. It does not qualify the financial action. Proposed horizon syntax is shown only in its own
          section and is accepted by no beta tool. The illustrations are decoration; they depict no transaction,
          balance, check result or acceptance.
        </p>
        <h3>Why the limits are stated this way</h3>
        <p>
          Each example carries a label that says what the tooling does with it today. Labels qualify the example
          next to them; they are not a score of the project. A label is a statement of evidence, and it is kept as
          narrow as the evidence: a local pass is called a local pass, not a payment.
        </p>
        <p>
          Stating the limits once, here, is a choice too. Repeating a disclaimer beside every example trains
          readers to skip it, and lets small variations creep into the wording until two pages seem to disagree.
          One statement, linked from everywhere, keeps every caveat true and readable. The specific limits of a
          single example, such as a string the checker stores but does not enforce, stay with that example.
        </p>
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

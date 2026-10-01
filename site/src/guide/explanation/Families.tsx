import { FamilyStrip, Section, SeeAlso, type SeeAlsoItem } from '../components';
import { FAMILIES, type FamilyId } from '../data';
import { FRONTEND, H, R, T } from './links';

const GROUP = 'The eight examples';

/** The standard footer of a family essay: the family's how-to and reference entry, plus extras. */
function FamilySeeAlso({ id, extra = [] }: { id: FamilyId; extra?: SeeAlsoItem[] }) {
  const fam = FAMILIES.find((f) => f.id === id);
  const name = fam?.name ?? id;
  return (
    <SeeAlso
      title="Put it into practice"
      items={[
        { mode: 'howto', href: `${H}#write-${id}`, label: `How to write ${fam?.agreement ?? 'one'}` },
        { mode: 'howto', href: `${H}#check-family`, label: 'How to check an example from the eight DeFi families' },
        { mode: 'reference', href: `${R}#family-${id}`, label: `Reference: the ${name === name.toUpperCase() ? name : name.toLowerCase()} example file, operations and atoms` },
        ...extra,
      ]}
    />
  );
}

/** Link to a family's reference entry, where the amounts are listed in atoms. */
function AtomsRef({ id }: { id: FamilyId }) {
  const name = FAMILIES.find((f) => f.id === id)?.name ?? id;
  return <a href={`${R}#family-${id}`}>amounts in atoms: Reference: {name}</a>;
}

const DIAGNOSTICS = `${R}#diagnostics`;

export function Families() {
  return (
    <>
      <Section id="amm" title="The swap example: a fixed input and a minimum output" nav="AMM" group={GROUP}>
        <FamilyStrip id="amm" here="explanation" />
        <p>
          Each of the eight family examples is <a href={`${R}#support-labels`}>specified only</a>:{' '}
          <code>check</code> accepts its structure and
          names, and running it is refused, so everything below is about what the source says rather than what a
          run produces. Each essay ends with a <em>hostile case</em>, a reading or a result an adversary might try
          to pass off as valid, and what does or does not stop it.
        </p>
        <h3>What the AMM file states</h3>
        <p>
          Alice spends exactly 100.00 USD and requires at least 0.900 GOLD. <code>USD</code> and{' '}
          <code>GOLD</code> are names in this file; their economic ids are <code>USDCanonical</code> and{' '}
          <code>GoldCanonical</code>. All of it sits on domain <code>Preview</code> (economic id{' '}
          <code>Midnight</code>). Pool <code>Spot</code> lists both assets. The agreement also mints and redeems
          liquidity shares for Alice, and declares Bob, who is not used. The fee cap is 0.30 USD (<AtomsRef id="amm" />).
        </p>
        <h3>What the swap intent fixes and what a kernel must decide</h3>
        <p>
          The operation fixes the pool, the owner, the input quantity, the output asset, a minimum net output and
          a fee cap. The checker requires the floor to be a quantity of the output asset and the fee cap a quantity
          of the input asset, so a floor written in USD is rejected at authoring with{' '}
          <code>BETA_ASSET_MISMATCH</code> (see <a href={DIAGNOSTICS}>Reference: diagnostic codes</a>). The intent
          has no gross cap, and its <code>rounding</code> and <code>failure</code> terms are stored strings.
        </p>
        <p>
          What the file leaves open is the financial relation. The amount Alice receives, the fee actually charged
          and any price remain for a later completion, inside the signed bounds. Something, whether a kernel or a
          Midnight program that uses none, would have to decide how the fee relates to the pool's reserves, how the
          output is bounded by the reserve, and how the gross debit is counted. Minting and redeeming shares are
          separate operations; their share amounts are plain scalars, which asset scale does not change. The
          proposed answers are in <a href="#horizon-amm">the proposed swap relations</a>.
        </p>
        <h3>Hostile case: a completion outside the swap's bounds</h3>
        <p>
          A completion that delivers one gold atom less than the floor misses it. A fee one dollar atom above the
          cap misses the cap. A fee quietly folded into the reserves breaks the proposed exact-input relation, which pays
          the fee outside the reserves, even when the floor and the cap both hold. Reading the price as
          dollars-per-gold when it is gold-per-dollar inverts the direction of the trade and leaves the authored
          floor behind.
        </p>
        <p>
          The <code>rounding</code> and <code>failure</code> strings do not enforce anything. A rounding law and an
          empty published effect vector on failure are judgments a kernel would make. There is a Midnight-specific
          subtlety too: a fallible phase can keep effects and fees from the guaranteed phase that ran before it, so
          the phase policy has to be stated with the intent rather than assumed.
        </p>
        <FamilySeeAlso id="amm" />
      </Section>

      <Section id="lending" title="The lending example: borrow, roll and liquidate" nav="Lending" group={GROUP}>
        <FamilyStrip id="lending" here="explanation" />
        <h3>What the lending file states</h3>
        <p>
          A borrower takes a loan from a named creditor, the loan can be rolled to a later round, and it can be
          liquidated. Alice is the debtor and Bob the creditor, both on domain <code>Preview</code>. Obligation{' '}
          <code>Loan</code> is denominated in <code>USD</code> (economic id <code>USDCanonical</code>), and the
          principal is 500.00 USD. The collateral of 1.000 GOLD (<code>GoldCanonical</code>) is a separate quantity
          on the same domain (<AtomsRef id="lending" />). The roll names round 200
          as a plain scalar. The liquidation names the obligation and stores the continuation{' '}
          <q>residual debt persists</q>.
        </p>
        <p>
          Paying the loan back is a different operation in a different file. The accrual-first repayment is one of
          the two operations that run locally, and the tutorial walks through it. The three operations in this file
          are <a href={`${R}#support-labels`}>specified only</a>.
        </p>
        <h3>What the borrow, roll and liquidate intents leave to a kernel</h3>
        <p>
          The checker binds the principal to the obligation's asset. None of the three intents carries a signer, a
          nonce, a gross cap or a scenario. The relation string records the author's expectations, aggregate locks,
          funded principal and open financial validation, but the lock total and the funding check stay open. A
          kernel would have to decide how origination ties the debt to the collateral lock, how often a roll may
          accrue interest, and what a liquidation leaves owed after the sale. It would also have to keep the debt
          as a liability, separate from any token supply. The proposed answers, with a worked atom path, are in{' '}
          <a href="#horizon-lending">the proposed loan lifecycle</a>.
        </p>
        <h3>Hostile case: a repayment or liquidation that erases debt</h3>
        <p>
          Taking a whole repayment from principal while interest stays accrued is a different loan; the shipped
          repayment pays accrued interest first. A payment larger than the
          outstanding balance is an invalid candidate: principal, accrued, outstanding and status stay as they
          were, and Core rejects it, at the Intent judgment when that applies and otherwise at the effect range. A
          rejected candidate is never published as a prepared post-state. A debt reduction without the matching
          credit to the creditor fails the repayment relation.
        </p>
        <p>
          On the liquidation side, the hostile reading is a sale that deletes both the obligation and its
          continuation, so a shortfall vanishes. The continuation string names the persistence of the residual
          debt, but it is only a string; the financial check that would enforce it is open.
        </p>
        <FamilySeeAlso
          id="lending"
          extra={[{ mode: 'tutorial', href: `${T}#repayment`, label: 'Tutorial: the repayment you can run' }]}
        />
      </Section>

      <Section id="stablecoins" title="The stablecoin example: backing, redemption and the duty that remains" nav="Stablecoins" group={GROUP}>
        <FamilyStrip id="stablecoins" here="explanation" />
        <h3>What the stablecoin file states</h3>
        <p>
          Alice deposits backing and receives an issued balance she can burn back for at least a named quantity of
          that backing. If the instrument is settled in an emergency, a redemption that is still owed remains a
          duty. Instrument <code>Stable</code> issues <code>USD</code> (economic id <code>USDCanonical</code>)
          against backing <code>GOLD</code> (economic id <code>GoldCanonical</code>), on domain{' '}
          <code>Preview</code>. Mint issues 100.00 USD against 0.200 GOLD. Redeem burns the same 100.00 USD and asks
          for at least 0.190 GOLD. Emergency names a claim of the same 100.00 USD and stores the continuation{' '}
          <q>existing redemption duties preserved</q> (<AtomsRef id="stablecoins" />). Bob is declared and unused.
        </p>
        <h3>What the mint, redeem and emergency intents leave to a kernel</h3>
        <p>
          The instrument record separates the issued asset from the backing asset, and the checker holds every
          quantity to its side: supply, burn and claim must be quantities of the issued asset, backing and{' '}
          <code>minimum_backing</code> quantities of the backing asset. Backing written in USD is rejected at
          authoring with <code>BETA_ASSET_MISMATCH</code> (see <a href={DIAGNOSTICS}>Reference: diagnostic
          codes</a>). The gap between the mint backing and the redeem minimum is simply a distance the author wrote; the redeem output, any
          fee and any price stay unwritten. The observation string <q>authenticated peg required; open</q> records
          that the peg is an open requirement.
        </p>
        <p>
          A kernel would have to tie issued supply to the matching debt, release backing at or above the signed
          minimum, and, in an emergency, pay the frozen claim while keeping what is still owed as a duty. The
          product contract keeps that debt off the token-supply balance: the issued quantity is supply, the
          redemption is a residual obligation. The proposed answers are in{' '}
          <a href="#horizon-stablecoins">the proposed stablecoin relations</a>.
        </p>
        <h3>Hostile case: backing in the wrong asset, or a dropped redemption duty</h3>
        <p>
          A mint that posts <code>USDCanonical</code> as backing fails the instrument split before any reserve is
          considered. A redeem that releases one gold atom less than the signed minimum falls short of it, while a
          completion that pays more than the minimum is allowed. The subtler attack is an emergency result that
          burns the claim, pays nothing and deletes the redemption duty, treating a debt as if it were supply. The
          continuation string states that the duty is preserved, and authoring stores that string. The duty, the
          peg and any ledger balance stay open.
        </p>
        <FamilySeeAlso id="stablecoins" />
      </Section>

      <Section id="options" title="The options example: fixing, exercise and settlement of a call" nav="Options" group={GROUP}>
        <FamilyStrip id="options" here="explanation" />
        <h3>What the options file states</h3>
        <p>
          The file sketches a cash-settled call held by Alice. The underlying is GOLD (economic id{' '}
          <code>GoldCanonical</code>) and settlement is in USD (economic id <code>USDCanonical</code>), both on
          domain <code>Preview</code>. The strike is the string <code>100.00</code> with units{' '}
          <code>USD per GOLD</code>. Collateral is 500.00 USD. The exercise round is the scalar 150. Settlement
          names Alice and a payoff of 100.00 USD (<AtomsRef id="options" />).
        </p>
        <p>
          Three intents carry the lifecycle. Fixing names the observation that would record a USD-per-GOLD value.
          Exercise names Alice and stores the continuation <q>missing fixing retains reserve and exercise duty</q>.
          Settlement names the holder, the payoff and the rounding string{' '}
          <q>payoff floor benefits reserve</q>. The check accepts this record. It does not multiply a fixing by a
          notional and it does not move the collateral. Bob is declared and is not an argument of any of the three.
        </p>
        <h3>What the option intents fix and what stays open</h3>
        <p>
          The file has no fee quantity and no fee account, and the optional intent fields <code>gross_cap</code>,{' '}
          <code>fee_cap</code> and <code>net_floor</code> are absent. Settlement's schema is instrument, holder and
          payoff; it has no credit effect. Contrast the transfer that runs locally, which requires three distinct
          accounts on one domain, a value and a fee of the same asset, and a signer equal to the paying account.
          If an author added a fee cap and an intent asset here, the check would require the fee cap to be a
          quantity of that asset, and the action would still be{' '}
          <a href={`${R}#support-labels`}>specified only</a>. These files declare no signer,
          key or signature at all. Everything that would make the call pay out correctly, the fixing, the payoff
          arithmetic and the reserve, belongs to a kernel; see{' '}
          <a href="#horizon-options">the proposed option lifecycle</a>.
        </p>
        <h3>Hostile case: a payoff in the wrong asset, or a strike that binds nothing</h3>
        <p>
          The checker catches the structural attacks. A payoff in GOLD fails with <code>BETA_ASSET_MISMATCH</code>.
          An instrument missing its underlying or settlement asset fails with <code>BETA_MISSING_FIELD</code>.
          Instrument, observation and holder on different domains fail with <code>BETA_DOMAIN_MISMATCH</code>. The
          messages are listed in <a href={DIAGNOSTICS}>Reference: diagnostic codes</a>. These rules live in the checker source,{' '}
          <a href={FRONTEND}>frontend.ts</a>; the package does not ship a rejected option fixture.
        </p>
        <p>
          The economic attacks pass. The payoff is not tied to the strike string, to the fixing round or
          to the collateral. The continuation string creates no reserve liability or exercise duty, and the
          rounding string selects no beneficiary. The exercise round and the observation's round happen to be the
          same number, 150, but nothing requires them to match, and no round window limits exercise to that round.
          A signature over a statement that binds this file's SHA-256 would still leave key authority unverified;
          it would not make Alice the ledger holder or make the source string <code>feed</code> an issuer.
        </p>
        <FamilySeeAlso id="options" />
      </Section>

      <Section id="oracles" title="The oracle example: a named observation that publishes no price" nav="Oracles" group={GROUP}>
        <FamilyStrip id="oracles" here="explanation" />
        <h3>What the oracle file states</h3>
        <p>
          One action, <code>read_fixing</code>, selects an observation named <code>Fixing</code> on domain{' '}
          <code>Preview</code>. Its unit string is <code>USD per GOLD</code>, its source string{' '}
          <code>approved-feed</code>, its observed round the scalar 140 and its maximum age the scalar 5. Finality
          is the string <code>finalized</code> and provenance the string <code>provider binding open</code>. The
          intent's failure string is <code>ObservationStale or ObservationDisputed</code>, and its continuation is{' '}
          <code>Pending retains dependent duty</code>.
        </p>
        <h3>Why the observation is a record, not a number</h3>
        <p>
          A later payment may need a GOLD price in USD. The pattern keeps that input as a declared observation
          rather than a number inside the payment, with the issuer, domain, freshness bound and trust assumption
          written beside it. Dependent work can then name the same observation, and a duty that depends on it can
          stay pending while the observation is missing, stale or disputed. The options example declares its own
          observation of the same shape with different claims: source <code>feed</code>, round 150, no maximum
          age.
        </p>
        <p>
          In this file, though, every one of those safeguards is authored data. The source is a string, not an
          account. Freshness is two scalars stored side by side. Trust is two strings. The check stores the fields;
          it does not compute an age and does not accept or reject the feed. A hash written into provenance would
          still be an authored string, and matching hashes do not establish authority. The product contract treats
          an external observation as a named assumption, never as a substitute for the proof of the program and the
          transition. How an observation would be qualified is described in{' '}
          <a href="#horizon-oracles">the proposed observation qualification</a>.
        </p>
        <h3>Hostile case: a stale or unqualified observation</h3>
        <p>
          <code>finalized</code> and <code>provider binding open</code> are strings, and the check accepts any other
          strings in their place. The observed round and the maximum age are independent scalars, never compared
          with a ledger round. The failure string names <code>ObservationStale</code> and{' '}
          <code>ObservationDisputed</code>, but running the action is refused before any scenario is applied, so
          those names are not rejection codes this action can produce. An observation has no issuer field; a grant
          record can name an issuer account, and this file declares no grant. A document hash in provenance would
          identify bytes the author claimed, without making the unit string a real quote or discharging the
          dependent duty. There is no fee to redirect, because no fee is declared.
        </p>
        <FamilySeeAlso id="oracles" />
      </Section>

      <Section id="governance" title="The governance example: a delayed policy change with a veto" nav="Governance" group={GROUP}>
        <FamilyStrip id="governance" here="explanation" />
        <h3>What the governance file states</h3>
        <p>
          Three actions share one policy: queue, execute and veto. Policy <code>Terms</code> is at epoch 3, with the
          authored <code>source_hash</code> string <code>terms-v3</code> and <code>duty_preservation</code> set to{' '}
          <code>required</code>. Queue asks for epoch 4, with an earliest round of 180 and a veto deadline of 179,
          and the relation string <q>existing signed duties retain terms</q>. Execute and veto name the same policy
          and add no round fields. The intended change would become eligible at round 180; the authored integers
          and strings do not enforce that.
        </p>
        <h3>Why queue, execute and veto are separate intents</h3>
        <p>
          Splitting the change into three intents means a signature on one is not a signature on the others, and
          that a veto is its own action rather than the absence of an execute. The delay lives on the queue intent,
          beside the call, which is why the round fields can be present on queue and absent on execute and veto.
          Existing duties are protected in words: the relation string and the policy value <code>required</code>.
          Those are stored authoring hints and do not establish that duties keep their terms.
        </p>
        <p>
          The check requires each epoch and round field to be an unsigned scalar when present. It does not order
          them and does not consult a clock. The policy record has no domain field and the three operations take no
          domain argument, so there is no second domain to compare. No signer or intent domain is named. A kernel
          would have to enforce the timelock and the veto window; see <a href="#horizon-governance">the proposed
          timelock</a>.
        </p>
        <h3>Hostile case: executing a change without its timelock</h3>
        <p>
          Execute's schema does not include the earliest round, and veto's does not include the deadline. Writing
          all three actions in one agreement does not attach round 180 or 179 to execute or veto, so a reader can
          queue epoch 4 and, in a separate intent, execute the policy with no timelock at all, and the check accepts
          it. The string <code>terms-v3</code> is not the SHA-256 of this file and is not compared with it;
          replacing it with another label still type-checks.
        </p>
        <p>
          No grant, signer list or quorum is declared, and Alice and Bob receive no authority to amend, veto or
          execute. A later signature would show that a key signed a statement; it would not show that the key is the
          policy authority. Nor would it give anyone a role in the project. Checking or running a program needs no
          quorum or permission; where such roles appear, they are roles inside an agreement.
        </p>
        <FamilySeeAlso id="governance" />
      </Section>

      <Section id="bridges" title="The bridge example: escrow, claim and recovery across two domains" nav="Bridges" group={GROUP}>
        <FamilyStrip id="bridges" here="explanation" />
        <h3>What the bridge file states</h3>
        <p>
          A bridge names one claim on two domains: a local escrow, a foreign claim, and a recovery of the escrow.
          Escrow locks 100.00 USD (<code>USDCanonical</code>) of Alice's on <code>Midnight</code>, destination{' '}
          <code>Foreign</code>, claim id <code>claim-1</code>. Claim pays ForeignAlice 100.00 of{' '}
          <code>WrappedUSD</code> on <code>Foreign</code>, with representation <code>bridge-claim</code>, naming
          Preview as the source. Recover returns the same 100.00 USD to Alice under the same{' '}
          <code>claim-1</code> (<AtomsRef id="bridges" />). Bob and GOLD are declared and used
          by none of the three actions. Observation of the other domain, finality and settlement are outside the
          file.
        </p>
        <h3>What the bridge intents fix and what evidence they lack</h3>
        <p>
          Each intent is the operation the author wrote, with its owner, amount, domain argument and claim id.
          Recover also stores the failure string <q>timeout alone is not proof of foreign nonreceipt</q>. That string
          is authoring data: the checker stores it and does not treat it as a proved duty, a policy or a finality
          observation.
        </p>
        <p>
          Running a bridge would require admitting an authentic observation of the foreign result and binding how
          final that observation is. The beta checker does none of that work, and its report lists authentication,
          native proof, financial correspondence and atomic ledger acceptance as open gates. A kernel, or a
          Midnight program that uses none, would have to supply that evidence. This example supplies none of it.
        </p>
        <h3>One pending claim, one terminal consumption</h3>
        <p>
          The design pattern is one pending claim, <code>claim-1</code>, with exactly one way to finish it. The
          specified rule admits the claim when an authentic observation says the source lock finalized, and admits
          the recovery when a separate observation says the foreign side did not receive. Those two terminals are
          alternatives for the same claim id.
        </p>
        <p>
          The beta operations do not carry that observation. The claim takes owner, amount, source and claim id;
          the recovery takes owner, amount and claim id, and its failure string is the only record of the
          nonreceipt rule. The agreement contains all three actions and the checker accepts it, but nothing
          consumes <code>claim-1</code>.
        </p>
        <h3>Late result and refund</h3>
        <p>
          A timeout records that a round bound has passed. It leaves the foreign execution unknown: the claim may
          have finalized after the last observation, it may still be queued, or it may never have been submitted.
          Recover becomes eligible only with a separate nonreceipt fact. The failure string on the recover intent is
          that rule, written as text. Running recover because the clock expired, while the foreign claim may already
          have succeeded, spends the escrow and the claim against the same <code>claim-1</code>.
        </p>
        <p>
          A result that arrives after a refund attempt is a different transition from the refund. Whichever
          transition is authenticated consumes <code>claim-1</code> once. The other cannot pay again and cannot
          erase the effects already committed. A committed debit stays committed. A compensating payment is a new
          authorized action with its own fees and residual duties. Midnight phase rules are not a global rollback,
          and this authoring file does not encode one. The same distinction is drawn for conditional settlement on
          the <a href="kernel.html#unknown">kernel page</a>.
        </p>
        <p>
          The checker does reject one concrete mis-statement of custody. An escrow whose owner account is on one
          domain and whose amount asset is on the other is <code>AuthoringRejected</code> with{' '}
          <code>BETA_DOMAIN_MISMATCH</code> (see <a href={DIAGNOSTICS}>Reference: diagnostic codes</a>). Owner and amount stay on the
          local domain. Only the named <code>source</code> or <code>destination</code> argument may name the foreign
          domain. That rejection is a nominal domain check. It is not a finality proof and it does not decide the
          refund race.
        </p>
        <FamilySeeAlso
          id="bridges"
          extra={[{ mode: 'explanation', href: '#patterns', label: 'Why recovery needs evidence, not a timeout' }]}
        />
      </Section>

      <Section id="staking" title="The staking example: deposit, reward, slash and a two-step exit" nav="Staking" group={GROUP}>
        <FamilyStrip id="staking" here="explanation" />
        <h3>What the staking file states</h3>
        <p>
          Staking here is a share class backed by one asset, plus named changes to that backing and a two-step
          exit. The file deposits 100.00 USD into share class <code>VaultShare</code>, adds a reward of 5.00 USD,
          removes a slash of 3.00 USD, unbonds 100 share atoms, and withdraws those same 100 share atoms with a
          minimum of 90.00 USD. The backing asset is <code>USDCanonical</code>; the 100 share atoms are a scalar,
          not a USD quantity (<AtomsRef id="staking" />). Alice owns the
          deposit, unbond and withdraw. Reward and slash name the share class and an amount, and no owner. GOLD and
          Bob are declared and unused.
        </p>
        <h3>What the staking intents fix and what a kernel must compute</h3>
        <p>
          Each intent fixes one operation and its arguments, and the checker requires the backing argument to be a
          quantity of the share class's declared backing asset. Deposit's rounding string is{' '}
          <q>share mint floor benefits vault</q>; slash's loss string is{' '}
          <q>signed slash priority; pending withdrawal remains explicit</q>. Both are prose hints, and the checker
          derives no share balance, rounding remainder, slash order or surviving duty from them.
        </p>
        <p>
          The remaining work is the financial relation: how many shares the deposit mints, who is credited the
          reward, whose claim the slash reduces, and whether the 100 share atoms may later leave with at least the
          withdrawal minimum. The file computes no share price, reward index or redemption.
        </p>
        <h3>Why the unbond request and the withdrawal are separate actions</h3>
        <p>
          The pattern separates changes to the backing from the exit. Deposit, reward and slash speak in atoms of the
          backing asset. Unbond speaks in share atoms and names no payout. Withdraw repeats the share count and adds
          an explicit minimum in the backing asset. An unbond is a request; the withdrawal is the later action that
          would deliver backing, and only if its minimum is met. Keeping them apart stops the request from being
          read as delivery of 90.00 USD.
        </p>
        <h3>Hostile case: a slash that swallows the pending exit</h3>
        <p>
          The loss hint says a slash keeps a pending withdrawal explicit. Because the five actions are separate, a
          slash does not delete the unbond of 100 share atoms, and the unbond does not pay the
          withdrawal minimum. The checker neither orders those actions nor preserves a duty across them; the hint
          is the whole of the specified scope.
        </p>
        <p>
          A deposit whose owner is on a different domain from the share class is rejected at authoring with{' '}
          <code>BETA_DOMAIN_MISMATCH</code> (see <a href={DIAGNOSTICS}>Reference: diagnostic codes</a>). Owner, share class and
          backing quantity share the declared domain. That check fixes the identity of the position. It does not
          decide slash priority, and it does not pay the exit.
        </p>
        <FamilySeeAlso id="staking" />
      </Section>
    </>
  );
}

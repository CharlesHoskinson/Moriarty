import { Code, Section, SeeAlso } from '../components';
import { PROFILE } from '../data';
import { HORIZON_DOC, R } from './links';

/*
 * The only place on the documentation pages that shows proposed horizon syntax. Every block here is
 * Code kind="horizon": captioned as proposed, with no copy control.
 */

const HORIZON_DUTY = `// moriarty-horizon/0.1: proposed syntax, from the shared horizon prelude
struct Duty<A> { claim: ClaimId<domain(A)>, owed: Qty<A>, terms: Digest,
                 continuation: Set<ActionId,16>, recovery: Set<ActionId,8> }
enum DebtStatus { Outstanding, Defaulted, Settled }`;

const HORIZON_RECOVERY = `// moriarty-horizon/0.1: proposed syntax, from the bridge library
enum ReceiptKnowledge { FinalReceipt, QualifiedNonreceipt, Unknown, Conflict }
relation Recover<A>(source: EscrowState<A>, nonreceipt: NonreceiptProof) -> EscrowState<A>;
// Recover: count=0 AND final nonreceipt/exclusion proof -> release 100 USD once.`;

const HORIZON_HOLES = `// moriarty-horizon/0.1: proposed syntax, excerpt of the Exchange agreement
intent Trade {
  fixed { owner: Alice; recipient: Alice; fee_to: Treasury; pool: Spot;
          input: 100.00 USD; output_asset: GOLD; net_floor: 0.900 GOLD; }
  holes { fee: QuantityHole<USD,31>(id: Fee, allowed: interval(0.00 USD,0.30 USD,0.01 USD));
          output: QuantityHole<GOLD,64>(id: Output, allowed: interval(0.900 GOLD,0.963 GOLD,0.001 GOLD)); }
  bounds { USD: budget(gross: 100.30 USD, fees: 0.30 USD);
           GOLD: budget(gross: 0.000 GOLD, fees: 0.000 GOLD); work: 1; }
}`;

export function Horizon() {
  return (
    <Section id="horizon" title="The proposed horizon language" nav="Horizon language" group="Proposed directions">
      <p>
        The horizon is a proposed future profile, <code>moriarty-horizon/0.1</code>, set out in the{' '}
        <a href={HORIZON_DOC}>full-language horizon</a> document. It is collected here, and only here, so that no
        page that teaches or describes <code>{PROFILE}</code> mixes it with runnable syntax. Everything in this
        section is proposed. No beta parser, checker or evaluator accepts it, and none of its figures is a result
        from any shipped example file.
      </p>
      <p>
        The beta stops where it does because each step past it needs evidence the beta cannot produce: typed duties
        need a lifecycle that outlives a single run, recovery needs authenticated observations of another domain,
        and solver choice needs refinement proofs that a filled-in value stays inside the signed bounds. The
        horizon writes down what those pieces would look like so that the beta's shape does not close them off.
      </p>

      <h3 id="horizon-duties">Proposed: duties with a typed shape</h3>
      <p>
        A duty gets a claim, an amount owed, its terms, and the bounded sets of actions that may continue or
        recover it. A debt carries an explicit status. The beta lending example can only record this intent as a
        continuation string.
      </p>
      <Code kind="horizon" title="Duty and debt status" href={HORIZON_DOC}>
        {HORIZON_DUTY}
      </Code>

      <h3 id="horizon-recovery">Proposed: recovery that requires evidence</h3>
      <p>
        What is known about a foreign receipt becomes a type with four cases, and recovery takes a nonreceipt
        proof as an argument. A release can then require a qualified nonreceipt, and an unknown or conflicting
        outcome cannot release funds. Foreign verification, finality and nullifier proofs are not implemented.
      </p>
      <Code kind="horizon" title="Recovery needs evidence" href={HORIZON_DOC}>
        {HORIZON_RECOVERY}
      </Code>

      <h3 id="horizon-holes">Proposed: solver holes inside fixed bounds</h3>
      <p>
        The owner fixes recipients, assets, the floor and the budgets. A solver may fill only the declared holes,
        each a finite interval with a step. Whatever it picks, the fee hole cannot exceed the signed fee cap and the
        output cannot fall below the signed floor. The pool invariant and the refinement proofs this relies on are
        also open.
      </p>
      <Code kind="horizon" title="Finite holes with fixed bounds" href={HORIZON_DOC}>
        {HORIZON_HOLES}
      </Code>

      <h3 id="horizon-price">Proposed price orientation</h3>
      <p>
        Canonical price orientation is base-per-quote: <code>Price&lt;Base, Quote, Scale&gt;</code> counts units of
        the base asset for one unit of the quote asset. The horizon's worked reading is{' '}
        <code>Price&lt;USD, GOLD, 2&gt;</code> with mantissa 10000, meaning 100 USD for one GOLD at scale 2.
        Swapping the type arguments reverses the direction, and a reciprocal needs an explicit conversion and a
        rounding beneficiary. The beta has no price type: the swap states its bound as an input quantity and a
        minimum output quantity.
      </p>

      <h3 id="horizon-amm">Proposed swap and liquidity relations</h3>
      <p>
        The horizon treats the swap as an exact input <code>x</code>, a fee <code>f</code> paid outside the
        reserves, and an output <code>y</code>. The proposed equations require <code>y</code> to be at least the
        floor and within the output reserve, <code>f</code> to be within the fee cap, and the gross debit to be{' '}
        <code>x + f</code>. The illustrated budget bounds the input asset at 10030 atoms gross (100.30 at scale 2),
        of which at most 30 are fee, against a receipt floor of 900 gold atoms. The proposed mint floors the share
        amount and leaves the excess in the pool reserve; the proposed redeem floors both returned assets and leaves
        the dust in the reserve.
      </p>

      <h3 id="horizon-lending">Proposed loan lifecycle</h3>
      <p>
        Origination creates the debt and the collateral lock together. A roll accrues interest once across the
        named window. Liquidation pays the bound creditor, moves the pledged collateral once, and keeps{' '}
        <code>max(outstanding - sale, 0)</code> as a residual duty, which survives a timeout and a zero assignment.
        Debt remains a liability, separate from token supply.
      </p>
      <p>
        The illustration funds 500.00 of the debt asset from creditor to debtor, opens principal 50000 atoms with
        accrued 0, and locks the 1000 gold atoms so that every lock together stays inside custody. One roll adds
        interest of <code>ceil(principal × 1/100)</code>, 500 atoms, into accrued, so outstanding becomes 50500
        atoms (505.00). A payment of 30.00 (3000 atoms) leaves 47500. A sale of 400.00 (40000 atoms) leaves 7500
        atoms (75.00), with status Defaulted and the creditor duty still held. No beta command settles any of this.
      </p>

      <h3 id="horizon-stablecoins">Proposed stablecoin relations</h3>
      <p>
        The proposed mint increases issued supply and the matching debt together and takes the backing deposit in
        the same stage. The proposed redeem burns issued units and releases backing at or above the signed minimum,
        leaving disclosed reserve dust. The proposed emergency burns once, pays the frozen claim, and keeps a duty
        for what is owed minus what was paid. A pro-rata backing result below 0.190 GOLD stays pending in that
        illustration, because it has no separate signed haircut. Freeze, which would fix the rate, exists only in
        the horizon and has no entry in the beta operation registry.
      </p>
      <p>
        The proposed peg direction is <code>Price&lt;GOLD, issued, 6&gt;</code>: units of backing for one unit of
        the issued asset, with backing as the base. Reading 10000 issued atoms against 200 gold atoms as
        issued-per-gold reverses it. The horizon illustration names the issued asset <code>MUSD</code>; the beta
        file names it <code>USD</code> with economic id <code>USDCanonical</code>.
      </p>

      <h3 id="horizon-options">Proposed option lifecycle</h3>
      <p>
        The horizon specifies a typed call with phases Fund, Fix, Exercise and Settle. Its illustration uses strike{' '}
        <code>Price&lt;USD,GOLD,2&gt;(10000)</code>, read as 100 USD per GOLD, notional 1.000 GOLD, premium 5.00
        USD and collateral 500.00 USD. A fixing of 140 USD per GOLD gives a payoff of 40 USD to Alice (140 minus 100,
        times notional 1), a residual reserve of 460 USD to Bob (500 minus 40), and a result for Alice of 35 USD
        before other costs (40 minus the 5 USD premium). A missing or disputed fixing retains the 500 USD reserve and
        the exercise and settlement duty. The beta file differs: its payoff is 100.00 USD, Bob is unused, and there
        is no premium.
      </p>

      <h3 id="horizon-oracles">Proposed observation qualification</h3>
      <p>
        The horizon specifies a typed select with Final, Missing, Stale and Disputed results, a requested round, and
        a maximum age compared with a current round. Its candidate uses <code>price&lt;USD,GOLD,2&gt;(14000)</code>,
        which on the same scale reads as 140.00 USD per GOLD, with observed round 150 and received round 151. Age 1
        at round 151 meets maximum age 5; at round 156 the observation is Stale (156 minus 150 is 6). Value and time
        remain claims until qualified, a policy record does not compute a median, and no timestamp authenticates a
        feed. A selector digest binds the query identity without making the price true. The beta observation is a
        different record, with observed round 140, no current round and no mantissa, so this age arithmetic does
        not apply to it.
      </p>

      <h3 id="horizon-governance">Proposed timelock</h3>
      <p>
        The horizon specifies queue, veto and execute as separate stages. With next epoch 4, earliest round 180 and
        a veto through round 179, a veto is allowed only while the current round is at most 179, and execute only
        when the round is at least 180, with a final no-veto fact. Enacting a policy changes which policy authorizes
        new commitments; it does not rewrite a stored debt, redemption or withdrawal duty. The horizon's fixed terms
        name <code>digest("terms-v4")</code> as the next commitment, while the beta policy string is{' '}
        <code>terms-v3</code>. They are different claims, and neither is the digest of the beta file.
      </p>

      <SeeAlso
        items={[
          { mode: 'explanation', href: '#patterns', label: 'Design patterns and the reasoning behind them' },
          { mode: 'reference', href: `${R}#reading-code-blocks`, label: 'Reference: how the code blocks are labelled' },
          { mode: 'reference', href: `${R}#support-matrix`, label: 'Reference: support by operation family' },
        ]}
      />
    </Section>
  );
}

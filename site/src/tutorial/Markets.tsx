import ammSource from '../../../packages/moriarty-beta/examples/amm.mori?raw';
import lendingSource from '../../../packages/moriarty-beta/examples/lending.mori?raw';
import stablecoinSource from '../../../packages/moriarty-beta/examples/stablecoin.mori?raw';
import repaySource from '../../../packages/moriarty-beta/examples/local/repay/repayment.mori?raw';
import repayScenario from '../../../packages/moriarty-beta/examples/local/repay/scenario.json?raw';
import repayTests from '../../../packages/moriarty-beta/examples/local/repay/mori.tests.json?raw';

function excerpt(source: string, start: string, end: string): string {
  const from = source.indexOf(start);
  const to = from < 0 ? -1 : source.indexOf(end, from);
  if (from < 0 || to < 0) throw new Error(`Markets excerpt not found: ${start}`);
  return source.slice(from, to + end.length);
}

const swapCall = excerpt(ammSource, 'intent Swap', 'action swap uses Swap;');
const mintLpCall = excerpt(ammSource, 'intent MintLP', 'action mint_lp uses MintLP;');
const redeemLpCall = excerpt(ammSource, 'intent RedeemLP', 'action redeem_lp uses RedeemLP;');
const borrowCall = excerpt(lendingSource, 'intent Borrow', 'action borrow uses Borrow;');
const rollCall = excerpt(lendingSource, 'intent Roll', 'action roll uses Roll;');
const liquidateCall = excerpt(lendingSource, 'intent Liquidate', 'action liquidate uses Liquidate;');
const repayCall = excerpt(repaySource, 'intent Repayment', 'action repay_loan uses Repayment;');
const mintCall = excerpt(stablecoinSource, 'intent Mint', 'action mint uses Mint;');
const redeemCall = excerpt(stablecoinSource, 'intent Redeem', 'action redeem uses Redeem;');
const emergencyCall = excerpt(stablecoinSource, 'intent Emergency', 'action emergency uses Emergency;');

function Status({ value }: { value: string }) {
  return <strong className="guide-status">{value}</strong>;
}

function Code({ children }: { children: string }) {
  return (
    <pre className="guide-code">
      <code>{children}</code>
    </pre>
  );
}

function FullSource({ summary, source }: { summary: string; source: string }) {
  return (
    <details>
      <summary>{summary}</summary>
      <Code>{source}</Code>
    </details>
  );
}

export function Markets() {
  return (
    <>
      <section id="amm" className="guide-section" aria-labelledby="amm-title">
        <h2 id="amm-title">Swap a fixed input for at least a named output</h2>
        <p>
          Alice spends a fixed quantity of one asset and requires a minimum quantity of another.
          In <code>AmmDemo</code> the input is <code>100.00 USD</code> and the minimum output is{' '}
          <code>0.900 GOLD</code>. <code>USD</code> and <code>GOLD</code> are declaration names.
          Their economic ids are <code>USDCanonical</code> and <code>GoldCanonical</code>, on domain{' '}
          <code>Preview</code>, whose economic id is <code>Midnight</code>. The chain string{' '}
          <code>midnight</code> and the network string <code>preview</code> are claims in the source.
          Symbol text is display metadata. Pool <code>Spot</code> lists both assets. The same
          agreement mints and redeems liquidity shares for Alice. Those three operations name Alice.
          The agreement also declares Bob.
        </p>
        <p>
          Support <Status value="SpecifiedOnly" />. Names and quantities are checked. Financial
          relations stay <Status value="Open" />. Local preparation is <Status value="Unsupported" />.
          Evidence is <code>AuthoringOnly</code>. The open gates on a successful check are
          authentication, native proof, financial correspondence and atomic ledger acceptance.
          Empty S0 premise and binding arrays mean the transfer/repay catalog is a different
          operation. The swap&apos;s financial relation stays open.
        </p>
        <aside className="guide-callout">
          <p>
            From <code>packages/moriarty-beta</code>, after that package&apos;s documented build,{' '}
            <code>node dist/cli.js check examples/amm.mori</code> is the structural check.{' '}
            <code>expand</code> and <code>simulate</code> refuse a <Status value="SpecifiedOnly" />{' '}
            action with <code>BETA_PROFILE_UNSUPPORTED</code>. The published effect vector is null,
            The scenario stays unchecked. The refusal is the support result: no quote, no reserve
            update and no settled trade is published.
          </p>
        </aside>

        <h3>The authored swap</h3>
        <Code>{swapCall}</Code>
        <p>
          Scale 2 makes <code>100.00 USD</code> 10000 atoms of <code>USDCanonical</code>, and{' '}
          <code>0.30 USD</code> 30 atoms. Scale 3 makes <code>0.900 GOLD</code> 900 atoms of{' '}
          <code>GoldCanonical</code>. The checker requires <code>net_floor</code> to use the output
          asset and <code>fee_cap</code> to use the input asset. A floor written as <code>USD</code>{' '}
          fails as <code>Expected Qty&lt;GOLD&gt;, received Qty&lt;USD&gt;</code>. A fee written as{' '}
          <code>GOLD</code> fails as <code>Expected Qty&lt;USD&gt;, received Qty&lt;GOLD&gt;</code>.
          The names in that diagnostic are declaration names.
        </p>
        <div className="guide-grid">
          <article>
            <h3>Intent terms in the beta file</h3>
            <p>
              The operation fixes the pool, the owner, the input quantity, the output asset, a
              minimum net output and a fee cap. <code>rounding</code> and <code>failure</code> store{' '}
              <q>output floor benefits pool</q> and <q>first failure; no effects</q>. Fee cap and net
              floor sit on the operation. The intent record has no <code>gross_cap</code> field. The
              received amount, and any price, remain for a later completion.
            </p>
          </article>
          <article>
            <h3>Proposed kernel relation</h3>
            <p>
              The horizon profile <code>moriarty-horizon/0.1</code> treats this trade as an exact
              input <code>x</code>, a fee <code>f</code> outside the reserves, and an output{' '}
              <code>y</code>. The proposed equations require <code>y</code> at least the floor,{' '}
              <code>y</code> within the output reserve, <code>f</code> within the fee cap, and gross{' '}
              <code>x + f</code>. The illustrated budget bounds the input asset at 10030 atoms gross
              (100.30 at scale 2), of which at most 30 atoms are fee, against a receipt floor of 900
              gold atoms. The beta
              checker accepts profile <code>moriarty-beta/1</code>. The horizon text remains a
              proposed lifecycle.
            </p>
          </article>
        </div>
        <p>
          Canonical price orientation is base-per-quote. <code>Price&lt;Base, Quote, Scale&gt;</code>{' '}
          counts units of the base asset for one unit of the quote asset. The horizon&apos;s worked
          reading is <code>Price&lt;USD, GOLD, 2&gt;</code> with mantissa 10000: 100 units of USD for
          one unit of GOLD, at scale 2. Swapping the type arguments reverses the direction. A
          reciprocal needs an explicit conversion and a rounding beneficiary. The beta swap states
          the bound as an input quantity and a minimum output quantity. Inverting those two
          quantities reverses base-per-quote and leaves the authored floor behind.
        </p>

        <h3>Exact-input bound</h3>
        <p>
          The owner names the spent asset and its amount, names the asset to receive, and sets the
          floor in that received asset and the fee cap in the spent asset. A later completion can
          choose a fee and an output only inside those bounds. Gross debit is the input plus the
          fee, so a fee paid beside the input still counts against the owner. Share mint and share
          redeem are separate operations. Their share amounts are scalars, and asset scale leaves
          those scalars unchanged.
        </p>
        <Code>{mintLpCall}</Code>
        <Code>{redeemLpCall}</Code>
        <p>
          Mint deposits <code>100.00 USD</code> (10000 atoms) and <code>1.000 GOLD</code> (1000
          atoms) and requires at least 10 share atoms. Redeem burns 10 share atoms. The proposed
          mint floors the share amount and leaves excess in the pool reserve. The proposed redeem
          floors both returned assets and leaves the dust in the reserve. The beta file records the
          calls and the minimum share scalar. The floor formulas belong to those proposed relations.
        </p>
        <table>
          <caption>Atoms for the quantities written in amm.mori</caption>
          <thead>
            <tr>
              <th scope="col">Authored quantity</th>
              <th scope="col">Declaration</th>
              <th scope="col">Economic id</th>
              <th scope="col">Scale</th>
              <th scope="col">Atoms</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>100.00 USD</td>
              <td>USD</td>
              <td>USDCanonical</td>
              <td>2</td>
              <td>10000</td>
            </tr>
            <tr>
              <td>0.30 USD</td>
              <td>USD</td>
              <td>USDCanonical</td>
              <td>2</td>
              <td>30</td>
            </tr>
            <tr>
              <td>0.900 GOLD</td>
              <td>GOLD</td>
              <td>GoldCanonical</td>
              <td>3</td>
              <td>900</td>
            </tr>
            <tr>
              <td>1.000 GOLD</td>
              <td>GOLD</td>
              <td>GoldCanonical</td>
              <td>3</td>
              <td>1000</td>
            </tr>
            <tr>
              <td>10 share atoms</td>
              <td>scalar</td>
              <td>share atoms</td>
              <td>none</td>
              <td>10</td>
            </tr>
          </tbody>
        </table>

        <h3>Hostile case</h3>
        <p>
          A completion that delivers 899 gold atoms misses the 900-atom floor. A fee of 31 dollar
          atoms misses the 30-atom cap. A fee folded into the reserve product breaks the proposed
          exact-input relation, which pays the fee outside the reserves, even when the floor and
          the cap hold. Quoting 900 gold atoms against 10000 dollar atoms as quote-per-base, then
          taking a reciprocal, reverses base-per-quote. The <code>rounding</code> and{' '}
          <code>failure</code> fields store their strings. A rounding law, and an empty published
          effect vector, are kernel judgments. The proposed stage outcome rejects at the first
          failing judgment. A Midnight fallible phase can retain guaranteed-phase effects and fees,
          so that phase policy has to be stated with the intent.
        </p>
        <FullSource summary="Complete amm.mori" source={ammSource} />
      </section>

      <section id="lending" className="guide-section" aria-labelledby="lending-title">
        <h2 id="lending-title">Repay accrued interest, then principal</h2>
        <p>
          A borrower pays a named creditor, and any unpaid balance remains an obligation. The
          operation you can prepare locally is that payment. Opening the loan, rolling it to a later
          round and liquidating it are written beside it, and those three stay specified. The local
          lesson is <a href="#lending-repay">the repayment you can run</a>, in{' '}
          <code>packages/moriarty-beta/examples/local/repay/</code>.
        </p>

        <h3>Specified origination, roll and liquidation</h3>
        <p>
          Support for <code>borrow</code>, <code>roll</code> and <code>liquidate</code> is{' '}
          <Status value="SpecifiedOnly" />. Financial relations are <Status value="Open" />. Local
          preparation is <Status value="Unsupported" />. The same structural check as the pool
          applies: <code>node dist/cli.js check examples/lending.mori</code> from{' '}
          <code>packages/moriarty-beta</code> after the package build. Simulation returns the support
          refusal and a null effect vector.
        </p>
        <Code>{borrowCall}</Code>
        <p>
          Alice is the debtor and Bob is the creditor, both on domain <code>Preview</code> (economic
          id <code>Midnight</code>). Obligation <code>Loan</code> is denominated in declaration{' '}
          <code>USD</code>, economic id <code>USDCanonical</code>, scale 2. Principal{' '}
          <code>500.00 USD</code> is 50000 atoms. Collateral <code>1.000 GOLD</code> is 1000 atoms
          of <code>GoldCanonical</code>, scale 3. The checker binds principal to the obligation
          asset. Collateral is a separate quantity on the same domain. The relation string records
          aggregate locks, funded principal, and open financial validation. The lock total and the
          funding check remain open.
        </p>
        <Code>{rollCall}</Code>
        <Code>{liquidateCall}</Code>
        <p>
          <code>to_round: 200</code> is a scalar round index. Liquidation names the obligation and
          stores the continuation <q>residual debt persists</q>. The proposed kernel illustration
          funds 500.00 of the debt asset from the creditor to the debtor, opens principal 50000
          atoms with accrued 0, and locks the 1000 gold atoms so that every lock together stays
          inside custody. One later roll adds interest of <code>ceil(principal × 1/100)</code> once,
          500 atoms, and leaves that interest in accrued. Outstanding becomes 50500 atoms, which is
          505.00 at scale 2. A payment of 30.00 (3000 atoms) then leaves 47500 atoms. A sale of
          400.00 (40000 atoms) leaves 7500 atoms, 75.00 at scale 2, with status Defaulted and the
          creditor duty still held. <code>lending.mori</code> records the three calls. This atom path
          is the proposed illustration, and the beta package has no command that settles it.
        </p>
        <div className="guide-grid">
          <article>
            <h3>Intent terms in lending.mori</h3>
            <p>
              Originate names debtor, creditor, principal and collateral. Roll-forward names the
              destination round. Liquidate names the obligation and a continuation string. None of
              the three carries a signer, a nonce, a gross cap or a scenario.
            </p>
          </article>
          <article>
            <h3>Proposed kernel relation</h3>
            <p>
              Origination creates the debt and the lock together. Roll-forward accrues once across
              the named window. Liquidation pays the bound creditor, moves the pledged collateral
              once, and keeps <code>max(outstanding − sale, 0)</code> as a residual duty. The duty
              survives a timeout and survives a zero assignment. Debt remains a liability, separate
              from token supply.
            </p>
          </article>
        </div>
        <FullSource summary="Complete lending.mori" source={lendingSource} />

        <h3 id="lending-repay">The repayment you can run</h3>
        <p>
          Support for <code>repay_loan</code> is <Status value="LocalS0" />. Check coverage delegates
          the financial relation to Core during local preparation, and local preparation is
          available and unqualified. A passing local run has status{' '}
          <Status value="PreparedUnqualified" />. The test report&apos;s qualification is{' '}
          <code>local-stipulation-only</code>. The scenario is stipulated data. Canonical intent
          signature, snapshot-to-head, head extension and atomic ledger compare-and-consume stay
          external premises. Agreement id, selected program, asset scale and authenticated
          predecessor stay unverified bindings.
        </p>
        <Code>{repayCall}</Code>
        <p>
          Declaration <code>USD</code> has economic id <code>A</code> and scale 2, and this file sets
          no symbol. <code>30.00 USD</code> is 3000 atoms of <code>A</code>. Payer and creditor are
          accounts on domain <code>Preview</code>. The obligation id is <code>Loan</code>. Gross cap
          is the payment. Fee cap and net floor are <code>0.00 USD</code>, which is 0 atoms. The
          window is inclusive rounds 0 through 10 on that domain. <code>source_hash</code> and{' '}
          <code>policy_digest</code> are claims in the source. The computed hash of the file bytes
          is a different value. <code>init --template repay</code> writes a source that differs from
          this shipped file by a terminal newline, so those two hashes differ. Digest the file you
          actually run.
        </p>
        <p>
          From <code>packages/moriarty-beta</code>, after <code>npm ci</code> and{' '}
          <code>npm run build</code> in that package:
        </p>
        <Code>{`node dist/cli.js init /tmp/loan --template repay
node dist/cli.js test /tmp/loan
node dist/cli.js simulate /tmp/loan/repayment.mori --action repay_loan --scenario /tmp/loan/scenario.json`}</Code>
        <p>
          <code>init</code> refuses an existing directory. The shipped fixture, with its independent
          effect and post-state expectations, is the directory linked above. Expansion names the
          selected Source/6 program <code>RepayAccrualFirst</code>. The action name remains{' '}
          <code>repay_loan</code>. Source shows the bare nonce <code>n1</code>. The
          prepared replay key is the tuple <code>[&quot;Midnight&quot;,&quot;Payer&quot;,&quot;n1&quot;]</code>.
          The signed ECDSA raw example of the same repayment shape is{' '}
          <code>packages/moriarty-beta/examples/signed-intent/repay-ecdsa-raw/</code>, described in{' '}
          <code>packages/moriarty-beta/SIGNED-INTENT.md</code>. A verified signature there yields{' '}
          <code>SignedPreparedUnqualified</code> and leaves the same four premises open.
        </p>

        <h3>Accrual-first allocation</h3>
        <p>
          Let the payment be <code>n</code>, accrued <code>a</code> and principal <code>p</code>.
          The discharged interest is <code>da = min(n, a)</code> and the discharged principal is{' '}
          <code>dp = n − da</code>. The creditor and the asset come from the obligation. The payer,
          the debtor and the signer are the same account. Fee cap and net floor are zero, and the
          conversion is identity: mantissa 1, scale 0, rounding none. Status becomes Settled when
          the new outstanding is zero. Interest-only, partial and full repayment are this same
          operation with a different explicit amount.
        </p>
        <p>
          The shipped scenario sets principal 100000, accrued 1000 and outstanding 101000 atoms.
          The sum of principal and accrued is the outstanding. Payment 3000 discharges all 1000 of
          accrued and 2000 of principal. Principal and outstanding become 98000, accrued becomes 0,
          and status stays Outstanding. Payer balance 200000 becomes 197000. Creditor balance 0
          becomes 3000. Allowance remaining 200000 becomes 197000 and spent becomes 3000. Work
          remaining 10 becomes 9 and work spent becomes 1. Head advances from <code>h0</code> to{' '}
          <code>h1</code>. The ordered effects are debit, credit, set-obligation, use-allowance,
          use-replay and advance-head. The predecessor string is echoed into Source/6 and stays
          unauthenticated.
        </p>
        <table>
          <caption>Shipped repayment expectation, in atoms of asset A</caption>
          <thead>
            <tr>
              <th scope="col">Cell</th>
              <th scope="col">Before</th>
              <th scope="col">After</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>Payer balance</td>
              <td>200000</td>
              <td>197000</td>
            </tr>
            <tr>
              <td>Creditor balance</td>
              <td>0</td>
              <td>3000</td>
            </tr>
            <tr>
              <td>Principal</td>
              <td>100000</td>
              <td>98000</td>
            </tr>
            <tr>
              <td>Accrued</td>
              <td>1000</td>
              <td>0</td>
            </tr>
            <tr>
              <td>Outstanding</td>
              <td>101000</td>
              <td>98000</td>
            </tr>
            <tr>
              <td>Allowance remaining / spent</td>
              <td>200000 / 0</td>
              <td>197000 / 3000</td>
            </tr>
            <tr>
              <td>Work remaining / spent</td>
              <td>10 / 0</td>
              <td>9 / 1</td>
            </tr>
          </tbody>
        </table>

        <h3>Hostile case</h3>
        <p>
          Taking all 3000 atoms from principal and leaving accrued at 1000 is a different loan. The
          shipped post-state takes the 1000 accrued atoms first. A payment larger than outstanding
          is a documented invalid proposal: principal, accrued, outstanding and status stay as they
          were, and Core rejects at Intent when that applies and otherwise at Effect range. The
          placeholder stays unpublished as a prepared post-state. A debt reduction missing the
          matching creditor credit fails the repay relation. On the specified liquidation side, a
          sale that deletes the obligation and the continuation is the hostile reading. The proposed
          illustration keeps the 7500-atom creditor claim. The beta continuation string names that
          persistence, and the financial check stays open.
        </p>
        <FullSource summary="Complete repayment.mori" source={repaySource} />
        <FullSource summary="Complete repay scenario.json" source={repayScenario} />
        <FullSource summary="Complete repay mori.tests.json" source={repayTests} />
      </section>

      <section id="stablecoins" className="guide-section" aria-labelledby="stablecoins-title">
        <h2 id="stablecoins-title">Mint against backing, and keep the redemption duty</h2>
        <p>
          Alice deposits backing and receives an issued balance she can burn back for at least a
          named quantity of that backing. If the instrument is settled in an emergency, a redemption
          that is still owed remains a duty. Instrument <code>Stable</code> has economic id{' '}
          <code>Stable</code> on domain <code>Preview</code> (economic id <code>Midnight</code>).
          Its issued asset is declaration <code>USD</code>, economic id <code>USDCanonical</code>,
          scale 2. Its backing asset is declaration <code>GOLD</code>, economic id{' '}
          <code>GoldCanonical</code>, scale 3. Mint, redeem and emergency name Alice. The agreement
          also declares Bob.
        </p>
        <p>
          Support for <code>mint</code>, <code>redeem</code> and <code>emergency</code> is{' '}
          <Status value="SpecifiedOnly" />. Financial relations are <Status value="Open" />. Local
          preparation is <Status value="Unsupported" />. From <code>packages/moriarty-beta</code>,
          after the package build, the structural command is{' '}
          <code>node dist/cli.js check examples/stablecoin.mori</code>. The check confirms the three
          operation schemas. Peg, reserve, redemption and settlement stay{' '}
          <Status value="Open" />. Expansion and simulation return{' '}
          <code>BETA_PROFILE_UNSUPPORTED</code> with a null effect vector.
        </p>
        <h3>Mint, redeem and emergency claim</h3>
        <Code>{mintCall}</Code>
        <Code>{redeemCall}</Code>
        <Code>{emergencyCall}</Code>
        <p>
          Mint issues <code>100.00 USD</code>, 10000 atoms of <code>USDCanonical</code>, against
          backing <code>0.200 GOLD</code>, 200 atoms of <code>GoldCanonical</code>. Redeem burns
          the same 10000 issued atoms and sets a minimum backing release of <code>0.190 GOLD</code>,
          190 atoms. The 10-atom gap between 200 and 190 is the authored distance between mint
          backing and the redeem minimum. The redeem output, any fee and any price stay unwritten.
          Emergency names a claim of the same 10000 issued atoms and stores the continuation{' '}
          <q>existing redemption duties preserved</q>. The observation string{' '}
          <q>authenticated peg required; open</q> records the open peg requirement. Price, time and
          verifier result remain separate evidence.
        </p>
        <div className="guide-grid">
          <article>
            <h3>Intent terms in the beta file</h3>
            <p>
              The instrument record separates <code>asset</code> from <code>backing</code>. Mint
              states supply and backing. Redeem states burn and <code>minimum_backing</code>.
              Emergency states <code>claim</code>. The checker requires supply, burn and claim to
              use the instrument asset, and backing and <code>minimum_backing</code> to use the
              backing asset. A backing quantity written as <code>USD</code> fails as{' '}
              <code>Expected Qty&lt;GOLD&gt;, received Qty&lt;USD&gt;</code>.
            </p>
          </article>
          <article>
            <h3>Proposed kernel relation</h3>
            <p>
              The proposed mint increases issued supply and the matching debt together and takes
              the backing deposit in the same stage. The proposed redeem burns issued units and
              releases backing at or above the signed minimum, leaving disclosed reserve dust. The
              proposed emergency burns once, pays the frozen claim, and keeps a duty for owed minus
              paid. A pro-rata backing result below 0.190 GOLD stays pending in that illustration,
              because the illustration has no separate signed haircut. Freeze, which would fix the
              rate, is horizon-only and has no entry in the beta operation registry. The issued
              quantity is supply. The redemption is a residual obligation. The product contract
              keeps debt off the token-supply balance.
            </p>
          </article>
        </div>
        <p>
          The proposed peg direction is <code>Price&lt;GOLD, issued, 6&gt;</code>: units of the
          backing asset for one unit of the issued asset. That is base-per-quote, with backing as
          the base and the issued token as the quote. The beta file leaves <code>Price</code>{' '}
          unwritten. Reading 10000 issued atoms against 200 gold atoms as issued-per-gold reverses
          that orientation. The horizon illustration names the issued asset <code>MUSD</code>. This
          beta file names it <code>USD</code> with economic id <code>USDCanonical</code>.
        </p>

        <h3>Issued asset, backing asset, residual duty</h3>
        <p>
          Every quantity that creates, burns or claims the token uses the instrument&apos;s issued
          asset. Every quantity that posts or releases collateral uses the backing asset. Mint pairs
          the two. Redeem pairs a burn with a minimum release. Emergency pairs a claim with a
          continuation that an existing redemption duty survives the call. A completion may pay more
          backing than the minimum. A release of 189 gold atoms misses the 190-atom floor. Dropping
          an unpaid duty misses the emergency continuation.
        </p>
        <table>
          <caption>Atoms for the quantities written in stablecoin.mori</caption>
          <thead>
            <tr>
              <th scope="col">Authored quantity</th>
              <th scope="col">Role</th>
              <th scope="col">Economic id</th>
              <th scope="col">Scale</th>
              <th scope="col">Atoms</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>100.00 USD supply, burn and claim</td>
              <td>issued asset</td>
              <td>USDCanonical</td>
              <td>2</td>
              <td>10000</td>
            </tr>
            <tr>
              <td>0.200 GOLD backing</td>
              <td>backing posted at mint</td>
              <td>GoldCanonical</td>
              <td>3</td>
              <td>200</td>
            </tr>
            <tr>
              <td>0.190 GOLD minimum backing</td>
              <td>minimum release at redeem</td>
              <td>GoldCanonical</td>
              <td>3</td>
              <td>190</td>
            </tr>
          </tbody>
        </table>

        <h3>Hostile case</h3>
        <p>
          A mint that posts 200 atoms of <code>USDCanonical</code> as backing fails the instrument
          split before any reserve is considered. A redeem that releases 189 gold atoms is below the
          190-atom minimum. An emergency result that burns the claim, pays nothing, and deletes the
          redemption duty treats the duty as supply. The proposed relation pays the frozen claim and
          retains owed minus paid. The continuation string states that preservation. Authoring
          acceptance stores the string. The duty, the peg and a ledger balance stay{' '}
          <Status value="Open" />.
        </p>
        <FullSource summary="Complete stablecoin.mori" source={stablecoinSource} />
      </section>
    </>
  );
}

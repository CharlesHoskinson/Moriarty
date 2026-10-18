import React from 'react';
import depth from '../../depth-2026-10-17/COPY.json';

export function invoiceModel(a) {
  return {
    id:'invoice', title:`${a.openingCash} CASH becomes ${a.amount} paid, ${a.fee} in fees and ${a.cash} retained`,
    inputs:[
      {id:'invoice-escrow-before',label:'Invoice escrow',kind:'state',lines:[`${a.openingCash} CASH held`,`Owes seller 100 CASH`,`Owes buyer 10 CASH reserve`],checks:['invoice-payment','invoice-fee','invoice-duty']},
      {id:'invoice-claim',label:'Issued execution claim',kind:'evidence',lines:['One whole-execution proof checked at Issue','One claim output consumed at settlement','Claim token burned at settlement'],checks:['invoice-payment','invoice-fee','invoice-duty']},
    ],
    outputs:[
      {id:'invoice-seller',label:'Seller',kind:'asset',lines:[`${a.amount} CASH received`,`Invoice discharged by ${a.amount} CASH`],checks:['invoice-payment','invoice-duty']},
      {id:'invoice-fees',label:'Program fee recipient',kind:'asset',lines:[`${a.fee} CASH received`,'No invoice debt discharged'],checks:['invoice-fee']},
      {id:'invoice-escrow-after',label:'Continuing invoice escrow',kind:'state',lines:[`${a.cash} CASH held`,`Owes seller ${a.invoiceDuty} CASH`,`Owes buyer ${a.reserveDuty} CASH reserve`],checks:['invoice-payment','invoice-fee','invoice-duty']},
    ],
    checks:[
      {id:'invoice-payment',label:'Payment and custody',explanation:`The seller receives ${a.amount} CASH. Escrow loses the payment and fee together: ${a.amount} + ${a.fee} = ${a.debit} CASH. Its continuing custody is ${a.cash} CASH.`},
      {id:'invoice-fee',label:'Separate fee',explanation:`The fee recipient receives ${a.fee} CASH. That payment counts towards escrow’s gross debit; it does not reduce the seller’s invoice.`},
      {id:'invoice-duty',label:'Continuing obligations',explanation:`The invoice decreases from 100 to ${a.invoiceDuty} CASH. The reserve remains ${a.reserveDuty} CASH. The next state therefore records ${a.duties} CASH still owed.`},
    ],
    note:'A conceptual transaction map for the one-Strand teaching design. CASH values exclude ADA fees, collateral, minimum ADA, funding/change outputs and any thread reserve. The supplied sources do not specify exact datum, redeemer or transaction encodings.',
  };
}

export function applicationModel(id,invoice) {
  return id==='invoice' ? invoiceModel(invoice) : depth.models.find(model=>model.id===id);
}

export function InvoiceExplanation({invoice:a}) {
  return <div className="depth-prose invoice-account">
    <h3>Cash and promises are separate accounts</h3>
    <p>A duty records an amount still owed by one party to another. Here, escrow owes the seller 100 CASH for the invoice and owes the buyer a separate 10 CASH reserve. A payment changes custody and may discharge a duty; the two changes need to be recorded separately.</p>
    <p>For this payment, the seller receives {a.amount} CASH and the fee recipient receives {a.fee}. Escrow loses {a.debit} CASH in total, leaving {a.cash}. Only the seller’s payment reduces the invoice: 100 − {a.amount} = {a.invoiceDuty}. The reserve stays at {a.reserveDuty}, so the agreement continues with {a.duties} CASH owed.</p>
    <div className="account-equation" aria-label="Cash conservation"><span>{a.openingCash} held</span><b aria-hidden="true">=</b><span>{a.amount} seller</span><b aria-hidden="true">+</b><span>{a.fee} fee</span><b aria-hidden="true">+</b><span>{a.cash} escrow</span></div>
    <p>Counting the fee as repayment would erase debt without paying the seller. Counting the remaining duties as extra payments would count promises as coins. The next escrow output holds the remaining cash and records what it still owes.</p>
    <details className="evidence-drawer"><summary>What happens after another payment?</summary><p>Each partial payment charges another 2 CASH. The extra 2 in the 112-CASH illustration covers one partial only. A later payment needs its own funding rule. The source’s <code>finish()</code> pays and discharges the remaining invoice; it leaves the reserve obligation in place. That source has no reserve-return action, so paying the invoice does not complete the entire agreement.</p></details>
  </div>;
}

export function CaseNarrative({id}) {
  if(id==='invoice')return <div className="depth-prose case-narrative"><p>The three checks below describe one <code>partial</code> action. The whole-execution proof covers their arithmetic together; the settlement consumer connects its statement to the escrow it spends and the payments it creates. The slider above changes the same amounts throughout this example.</p></div>;
  const c=depth.cases[id];
  return <div className="depth-prose case-narrative">{c.intro.map((p,i)=><p key={i}>{p}</p>)}<details className="evidence-drawer"><summary>Follow the accounting and continuing state</summary>{c.details.map((p,i)=><p key={i}>{p}</p>)}<p className="case-source-note">{c.note}</p></details></div>;
}

export function ProofPackaging() {
  return <div className="proof-packaging">
    <h3>One payment can have several checks and one proof</h3>
    <p>For the invoice walkthrough, one whole-execution Strand covers the partial-payment action. Its payment, fee and remaining-duty checks are parts of that action. The Web contains that Strand; it is not three proofs made by renaming three checks.</p>
    <ol className="packaging-flow" aria-label="Invoice evidence packaging">
      <li><strong>Three financial checks</strong><span>Payment · fee · continuing duties</span></li>
      <li><strong>One Strand</strong><span>One execution proof, statement and binding context</span></li>
      <li><strong>One Web</strong><span>The evidence this consumer requires for the action</span></li>
    </ol>
    <p>On the reviewed Issue route, one Issue transaction checks the proof and creates one claim output, and one settlement transaction later consumes it. A proof claim never mints the application's own token. These are the counts for this teaching design. A different decomposition requires real separate relations and an explicitly compatible consumer.</p>
  </div>;
}

export function DeveloperWalkthrough() {
  const source=depth.invoiceSource;
  const partial=source.source.text.match(/  action partial\(amount: UInt\) \{[\s\S]*?\n  \}/)?.[0]||source.source.text;
  return <div className="developer-walkthrough">
    <div className="depth-prose"><h3>The agreement supplies the rules</h3><p>This is the actual partial-payment action in the supplied invoice source. It requires an open invoice and a positive payment smaller than the outstanding amount. The fee comes before the seller transfer; discharge reduces the invoice, and the postcondition checks the resulting duty.</p></div>
    <figure className="code-example"><figcaption>installment-invoice.mori · partial action</figcaption><pre tabIndex="0"><code>{partial}</code></pre></figure>
    <div className="depth-prose"><p>The source also preserves <code>marker</code>, keeps <code>opened</code> true and increases <code>paid</code> by the payment. A correct transition accounts for preserved state as well as new payments. The proposed 112-CASH opening is an additional funding policy; this excerpt does not implement it.</p><h3>The compiler selects an action and a profile</h3><p>The checked-in selector identifies <code>moriarty/1</code> source, <code>moriarty-core/1</code> semantics and the <code>partial</code> action. Its adapter and accounting choices describe the relation being compiled. They are not inferred from the three diagram labels.</p><p><code>NotBound</code> means this relation has no cap slot, so the circuit does not enforce the owners’ signed caps; in the reviewed design that check belongs to the consuming validator, which has not been built. <code>TouchedOnly</code> means the relation does not authenticate a complete ledger frame.</p></div>
    <details className="evidence-drawer"><summary>Read the actual relation selector</summary><pre className="source-json" tabIndex="0"><code>{JSON.stringify(source.selector,null,2)}</code></pre><p className="source-id">{source.selectorPath}</p></details>
    <div className="artifact-flow"><table><caption>What moves through the experimental compiler path</caption><thead><tr><th scope="col">Artifact</th><th scope="col">What it supplies</th><th scope="col">What to inspect</th></tr></thead><tbody>
      <tr><th scope="row">Moriarty program</th><td data-label="What it supplies">State, assets, actions and financial obligations.</td><td data-label="What to inspect">Preconditions, ordered effects, preserved state and postconditions.</td></tr>
      <tr><th scope="row">Relation selector</th><td data-label="What it supplies">The action, semantics and admitted accounting profile.</td><td data-label="What to inspect">Whether the selected profile covers the intended action.</td></tr>
      <tr><th scope="row">Transition request</th><td data-label="What it supplies">Before-state, balances, duties, inputs and context.</td><td data-label="What to inspect">Exact scenario and independently calculated after-state.</td></tr>
      <tr><th scope="row">Relation and inspection output</th><td data-label="What it supplies">Descriptor, statement, public inputs and decoded financial slots.</td><td data-label="What to inspect">All violated rows, not command completion alone.</td></tr>
      <tr><th scope="row">Proof and transaction integration</th><td data-label="What it supplies">Evidence for the statement and its consuming ledger route.</td><td data-label="What to inspect">Authorization, statement reconstruction and complete effect readback.</td></tr>
    </tbody></table></div>
    <details className="evidence-drawer"><summary>Source entry points and the three different invoice scenarios</summary>
      <p>The experimental README documents compilation and inspection with these forms. They are source references, not commands run by this page.</p>
      <pre className="source-json" tabIndex="0"><code>{'mori-k2 compile --source FILE --selector FILE\nmori-k2 inspect --source FILE --request FILE --vk-digest HEX'}</code></pre>
      <p>The checked-in request uses 109 CASH in escrow, a 90-CASH invoice and a 14-CASH reserve. It supplies synthetic context. It is a different scenario from the source-opening calculation and the website’s funding illustration.</p>
      <div className="artifact-flow"><table><caption>Each row applies payment 35 and fee 2; none is a new execution receipt</caption><thead><tr><th scope="col">Scenario</th><th scope="col">Cash before → after</th><th scope="col">Invoice before → after</th><th scope="col">Reserve</th></tr></thead><tbody>{depth.scenarioComparison.map(r=><tr key={r.label}><th scope="row">{r.label}</th><td data-label="Cash before → after">{r.cashBefore} → {r.cashAfter}</td><td data-label="Invoice before → after">{r.invoiceBefore} → {r.invoiceAfter}</td><td data-label="Reserve">{r.reserveBefore}</td></tr>)}</tbody></table></div>
      <p>The documented proof export does not emit Plutus Data, CBOR or a transaction. An application still needs the exact consumer interface and a transaction builder.</p>
      <p className="source-id">{source.source.path}<br/>{source.requestPath}</p>
    </details>
  </div>;
}

import React, {useRef, useState} from 'react';
import brandBanner from '../../../assets/moriarty-banner.png';
import WebDiagram from './WebDiagram.jsx';
import SevenLayers from './SevenLayers.jsx';
import Groth16 from './Groth16.jsx';
import {invoiceAmounts} from './invoice.mjs';
import data from './data.generated.json';

const main=data.v2.main;
const caseCopy=Object.fromEntries(main.caseCopy.map(c=>[c.id,{...c,roles:Object.fromEntries(c.roles.map(role=>[role.id,role]))}]));


const Arrow=()=> <span aria-hidden="true">↗</span>;
const renderTemplate=(value, values)=>String(value).replace(/\{([A-Za-z][A-Za-z0-9]*)\}/g,(match,key)=>Object.hasOwn(values,key)?String(values[key]):match);
const templateCopy=(value,values)=>typeof value==='string'?renderTemplate(value,values):Array.isArray(value)?value.map(item=>templateCopy(item,values)):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,templateCopy(item,values)])):value;
const refs=(items=[])=>items.map(x=>typeof x==='string'?x:[x.label,x.document,x.pointer].filter(Boolean).join(' · ')).join(' / ');
function Sources({items=[]}) {return items.length>0&&<details className="evidence-drawer source-details"><summary>Source notes</summary><ul className="source-list">{items.map((x,i)=><li key={i}><span className="source-id">{typeof x==='string'?x:[x.label,x.document].filter(Boolean).join(' · ')}</span>{x.pointer&&<><br/>{x.pointer}</>}</li>)}</ul></details>}
function Heading({number,kicker,title,id,children}) {return <div className="section-heading"><span className="eyebrow"><span className="section-number">{number}</span>{kicker}</span><h2 id={id}>{title}</h2><p>{children}</p></div>}
function DisclosureLink({id,children='Implementation details'}) {return <a className="text-link" href={`#${id}`} onClick={()=>{const target=document.getElementById(id);if(target?.tagName==='DETAILS')target.open=true}}>{children} <span aria-hidden="true">↗</span></a>}
function TextBlocks({value}) {if(value==null)return null;if(typeof value==='string'||typeof value==='number')return <p>{value}</p>;if(Array.isArray(value))return <ul>{value.map((item,i)=><li key={i}><TextBlocks value={item}/></li>)}</ul>;return <dl className="detail-list">{Object.entries(value).filter(([key])=>!['sourceRefs','id','status','integrationRule'].includes(key)).map(([key,item])=><React.Fragment key={key}><dt>{key.replace(/([a-z])([A-Z])/g,'$1 $2')}</dt><dd><TextBlocks value={item}/></dd></React.Fragment>)}</dl>}
function Explorer({invoice}){
 const [caseId,setCaseId]=useState('invoice');
 const source=data.scenarios.find(x=>x.id===caseId);
 const scenario={...source,...caseCopy[caseId],strands:source.strands.map(s=>({...s,...caseCopy[caseId].roles[s.id]}))};
 const [selected,setSelected]=useState('invoice-payment');
 const [omitted,setOmitted]=useState([]);
 const inspector=useRef(null);
 const strand=scenario.strands.find(x=>x.id===selected)||scenario.strands[0];
 const choose=(x)=>{setCaseId(x.id);setSelected(x.strands[0].id);setOmitted([])};
 const select=(id)=>{setSelected(id);if(window.matchMedia('(max-width: 760px)').matches)requestAnimationFrame(()=>{const el=inspector.current;const box=el.getBoundingClientRect();if(box.bottom>window.innerHeight||box.top<0)el.scrollIntoView({block:'start',behavior:window.matchMedia('(prefers-reduced-motion: reduce)').matches?'auto':'smooth'})})};
 const toggle=(id)=>setOmitted(xs=>xs.includes(id)?xs.filter(x=>x!==id):[...xs,id]);
 const included=scenario.strands.length-omitted.length;
 const values={...invoice,included,required:scenario.strands.length};
 const text=(v)=>renderTemplate(v,values);
 return <section className="section explorer" id="explorer" aria-labelledby="explorer-title">
 <Heading number="02" kicker="Applications" title="What each application needs to prove" id="explorer-title">{main.explorerIntro}</Heading>
 <div className="case-tabs" role="group" aria-label="Application case">{data.scenarios.map((x,i)=><button key={x.id} aria-pressed={caseId===x.id} data-case={x.id} onClick={()=>choose(x)}><span className="case-index">0{i+1}</span>{x.title}</button>)}</div>
 <div className="scenario-intro"><div><h3>{scenario.title}</h3></div><p>{text(scenario.summary)}</p></div>
 {caseId==='invoice'&&<div className="example-strip"><span><strong>Shared invoice example</strong> · {invoice.amount} CASH payment · {invoice.openingCash} CASH funding</span><a href="#worksheet-title">Change the amount <span aria-hidden="true">↓</span></a></div>}
 <div className="explorer-layout" data-complete={omitted.length===0}>
  <div className="diagram-panel web-stage"><div className="diagram-header"><span className="tag">Contract requirements</span><small>Select a check below</small></div>
   <WebDiagram scenario={scenario} selectedStrand={strand.id} onSelect={select} omittedStrands={omitted}/>
   <p className="diagram-legend">Each button is a condition the contract requires. A proof can cover more than one condition.</p>
   <div className={`assembly-status ${omitted.length>0?'is-incomplete':''}`} role="status" data-testid="assembly-status"><strong>{text(omitted.length>0?main.assemblyIncomplete:main.assemblyComplete)}</strong><p>{included} of {scenario.strands.length} conditions covered.</p></div>
  </div>
  <aside className="inspector" id="role-inspector" ref={inspector} aria-label="Selected contract condition"><span className="eyebrow">Selected check</span><h3>{strand.label}</h3><p>{text(strand.claim)}</p><dl><dt>What the claim says</dt><dd data-testid="role-statement">{text(strand.statement)}</dd>{strand.authentication&&<><dt>Where the inputs come from</dt><dd>{text(strand.authentication)}</dd></>}</dl><label className="toggle-row"><input type="checkbox" checked={omitted.includes(strand.id)} onChange={()=>toggle(strand.id)} data-testid="omit-strand"/><span>Remove proof coverage for this check<small className="role-subtitle">The contract still requires it.</small></span></label></aside>
 </div>
 <div className="case-outcomes"><div><span className="eyebrow">Financial result</span><ul>{(caseId==='invoice'?[`Seller receives ${invoice.amount} CASH; the program fee recipient receives ${invoice.fee} CASH.`,`Gross escrow debit is ${invoice.debit} CASH; ${invoice.cash} CASH remains in escrow.`,`The unpaid invoice is ${invoice.invoiceDuty} CASH; with the ${invoice.reserveDuty} CASH reserve obligation, ${invoice.duties} CASH remains owed.`]:scenario.strands.map(role=>role.statement)).map((x,i)=><li key={i}>{text(x)}</li>)}</ul></div><div><span className="eyebrow">What the validator receives</span><p>A Web covering these conditions, bound to the transaction that spends the contract state.</p></div></div>
 <div className="binding-explanation"><h3>How the proof connects to the transaction</h3><p>{main.proofRoleExplanation}</p></div>
 </section>
}
function Worksheet({invoice:a,amount,setAmount,openingCash,setOpeningCash}){
 const w=main.worksheet,text=(value)=>renderTemplate(value,a);
 return <section className="section" id="worksheet" aria-labelledby="worksheet-title"><Heading number="01" kicker="A payment example" title="Paying part of an invoice" id="worksheet-title">A payment changes both the escrow balance and the amount owed. Move the slider to see why the fee must be accounted for separately.</Heading>
 <div className="worksheet"><div className="worksheet-layout"><div><span className="eyebrow">Invoice escrow</span><h3>{w.title}</h3><p className="worksheet-intro">{text(w.intro)}</p><div className="worksheet-controls"><div className="range-heading"><label htmlFor="payment">{w.amountLabel}</label><output htmlFor="payment" data-testid="payment-value">{amount} <small>CASH</small></output></div><input id="payment" type="range" min="1" max="99" step="1" value={amount} onChange={e=>setAmount(Number(e.target.value))}/><div className="range-limits"><span>1 CASH</span><span>99 CASH</span></div><p className="range-note">{w.amountRangeNote}</p><div className="funding-options" role="group" aria-label="Invoice example funding"><button aria-pressed={openingCash===112} onClick={()=>setOpeningCash(112)} data-funding="proposed112">{w.modeLabel}</button><button aria-pressed={openingCash===110} onClick={()=>setOpeningCash(110)} data-funding="source110">110 CASH · without the fee allowance</button></div><p className="funding-description" data-testid="example-mode">Selected: {openingCash===112?w.modeLabel:'110 CASH · without the fee allowance'}.</p>{openingCash===112&&<p className="funding-description">{w.modeBody}</p>}</div></div>
 <div><div className="ledger"><div className="ledger-row"><span>Opening escrow funding</span><strong>{openingCash} CASH</strong></div><div className="ledger-row"><span>Seller receives</span><strong>{amount} CASH</strong></div><div className="ledger-row"><span>Program fee recipient receives</span><strong>{a.fee} CASH</strong></div><div className="ledger-row gross-debit"><span>Gross escrow debit</span><output data-testid="gross-debit">{a.debit} CASH</output></div><div className="ledger-row total"><span>Cash remaining</span><output data-testid="closing-cash">{a.cash} CASH</output></div></div><div className="metric-grid"><div className="metric"><span className="metric-label">Invoice still owed</span><output className="metric-value" data-testid="invoice-duty">{a.invoice}</output></div><div className="metric"><span className="metric-label">Reserve obligation</span><output className="metric-value" data-testid="reserve-duty">{a.reserve}</output></div><div className="metric"><span className="metric-label">Total still owed</span><output className="metric-value" data-testid="total-duties">{a.duties}</output></div></div><div className="backing-chart" aria-hidden="true"><div><span>Cash</span><i style={{width:`${a.cash/112*100}%`}}/></div><div><span>Owed</span><i style={{width:`${a.duties/112*100}%`}}/></div></div><p className="backing-result" role="status" data-testid="backing-gap">{a.gap>0?`The cash is ${a.gap} CASH short of the remaining obligations.`:text(w.backingLine)}</p><p className="worksheet-note">{text(w.feeRule)} {text(w.reserveNote)}</p></div></div>
 <p className="worksheet-note">{text(w.oneStepNote)} {w.networkFeeNote}</p><p className="worksheet-note">{w.assetNote} {w.proofSeparation}</p></div><DisclosureLink id="invoice-source-notes">Invoice source and funding details</DisclosureLink></section>
}
function Lifecycle(){return <section className="section" id="lifecycle"><Heading number="03" kicker="On Cardano" title={main.lifecycleTitle}>{main.lifecycleIntro}</Heading><div className="lifecycle">{main.lifecycleSteps.map((step,i)=><article className="lifecycle-step" key={i}><span className="step-index">Step {i+1}</span><h3>{step.title}</h3><p>{step.body}</p></article>)}</div><DisclosureLink id="security-card-issuance-route">Issuance assumptions and funding</DisclosureLink></section>}
const handoff = [
  ['Write the agreement', 'Describe the financial state and the actions that may change it.', 'Include payment rules, fees and any obligations that survive the action.'],
  ['Specify the claims', 'Identify the conditions this transaction must satisfy.', 'Match each claim to a compiled relation and its verification key.'],
  ['Build and submit the transaction', 'Collect the required Strands into a Web.', 'The consuming validator checks the claims against the transaction and enforces the authorized effects.'],
];
function Developers() {
  return <section className="section" id="developers">
    <Heading number="04" kicker="Building an application" title="From a financial program to a transaction">
      Write the agreement in Moriarty, then connect its claims to the Cardano validator that spends the contract state. The application must keep those claims consistent with the transaction it builds.
    </Heading>
    <div className="artifact-flow" role="region" aria-label="Developer integration" tabIndex="0">
      <table><thead><tr><th scope="col">Work</th><th scope="col">Input</th><th scope="col">What to include</th></tr></thead>
      <tbody>{handoff.map(([name,input,result])=><tr key={name}><th scope="row">{name}</th><td>{input}</td><td>{result}</td></tr>)}</tbody></table>
    </div>
    <div className="developer-layout developer-notes">
      <div><h3>Check the owner’s actual outcome</h3><p className="section-intro">For a swap, an owner can limit the input amount and require a minimum net receipt of the output asset. The validator checks the recipient’s actual output against those signed limits. Pool arithmetic alone does not establish that the owner received the agreed amount.</p></div>
      <div><h3>Carry unpaid obligations into the next state</h3><p className="section-intro">A loan payment leaves a remaining debt. A partial invoice payment leaves an unpaid balance. The next contract state records those amounts so later transactions can continue the agreement.</p><a className="text-link" href="https://github.com/CharlesHoskinson/Moriarty/tree/utxo" target="_blank" rel="noreferrer">Browse the Moriarty source <Arrow/></a><DisclosureLink id="available-tooling">Compiler interfaces and implementation notes</DisclosureLink></div>
    </div>
  </section>;
}
function FAQ(){return <section className="section" id="questions"><Heading number="07" kicker="Questions" title="Using Webs in an application">Answers about authorization, unused proof claims and transaction costs.</Heading>{main.faqs.map((item,i)=><details className="faq-item" key={i}><summary>{item.question}</summary><p>{item.answer}</p>{item.technicalCardId&&<DisclosureLink id={`security-card-${item.technicalCardId}`}>Assumptions and implementation details</DisclosureLink>}{item.sourceRefs&&<p className="source-id">{refs(item.sourceRefs)}</p>}</details>)}</section>}
function Evidence(){const s=data.security,b=data.learning.broaderScope,t=data.v2.technical;return <section className="section technical-section" id="evidence" aria-labelledby="evidence-title"><Heading number="08" kicker="Technical notes" title="Implementation and evidence" id="evidence-title">The guide explains the complete design. The records below describe what has been implemented and tested, with the remaining assumptions and integration work.</Heading><div className="evidence" id="implementation-status"><div className="evidence-header"><h3>{data.v2.technical.title||'Current implementation'}</h3><span className="tag">Research & implementation record</span></div><TextBlocks value={data.v2.technical.summary}/><div className="status-links">{['setup-keys','binding-chain','recorded-evidence'].map(id=>{const card=t.trustNotes.find(c=>c.id===id);return <div key={id}><DisclosureLink id={`security-card-${id}`}>{card.title}</DisclosureLink><p>{card.evidenceStage}</p></div>})}</div><p>{t.cardsIntro}</p><div className="security-cards">{t.trustNotes.map(card=><details className="evidence-drawer" key={card.id} id={`security-card-${card.id}`}><summary>{card.title}</summary><p>{card.body}</p>{card.assumptions&&<div><strong>Assumptions &amp; limits</strong><ul>{(Array.isArray(card.assumptions)?card.assumptions:[card.assumptions]).map((x,i)=><li key={i}>{x}</li>)}</ul></div>}{card.evidenceStage&&<p><strong>Evidence stage:</strong> {card.evidenceStage}</p>}<p className="source-id">{refs(card.sourceRefs)}</p></details>)}</div></div>
 <details className="evidence-drawer" id="security-card-signer-boundary"><summary>Transaction signers and authorization boundaries</summary><p>{t.signerBoundary}</p></details><details className="evidence-drawer" id="security-card-readback-boundary"><summary>Acceptance, finality and effect readback</summary><p>{t.readbackBoundary}</p></details>
 <details className="evidence-drawer" id="invoice-source-notes"><summary>Invoice source, asset and funding comparison</summary><p>{t.sourceVersusDesign.replace('The executed source differs.','The S12 source differs from this design.')}</p><p>{t.networkFeeNote}</p><p>CASH fixture identity: network <code>{data.invoice.asset.network}</code>, issuer <code>{data.invoice.asset.issuer}</code>, reference <code>{data.invoice.asset.reference}</code>, scale {data.invoice.asset.scale}.</p><ul>{data.invoice.limitations.map((x,i)=><li key={i}>{x}</li>)}</ul><Sources items={data.invoice.sourceRefs}/></details>
 <details className="evidence-drawer" id="available-tooling"><summary>Current source interfaces and developer entry point</summary><p>The source README documents <code>mori-k2 compile --source FILE --selector FILE</code> and <code>mori-k2 inspect --source FILE --request FILE --vk-digest HEX</code>. It separately documents <code>setup</code>, <code>prove</code> and <code>verify</code> with their required files. These commands were not run for this page. An inspect exit of zero can include violated rows.</p><p>The page is a self-contained React explanation with local arithmetic. It makes no wallet connection, signature, proof, network request or transaction. The explorer shows proof roles; it does not contain concrete proof instances.</p><a className="text-link" href="https://github.com/CharlesHoskinson/Moriarty/blob/utxo/README.md" target="_blank" rel="noreferrer">Read the repository documentation <Arrow/></a><p className="source-id">S04-RUST-README.md · Build and ordinary reproduction / Proof API boundary; DEVELOPER-EXPERIENCE.md · First-use walkthrough</p></details>
 <details className="evidence-drawer"><summary>Correspondence and the complete retained scope</summary><p>{data.learning.evidence.find(x=>x.id==='conditional-t1').body}</p><ul>{data.learning.evidence.find(x=>x.id==='conditional-t1').limitations.map((x,i)=><li key={i}>{x}</li>)}</ul><p>{b.body}</p><p>{b.familiesStatus}</p><ul>{b.retainedFamilies.map(x=><li key={x}>{x}</li>)}</ul><ul>{b.furtherTracks.map(x=><li key={x}>{x}</li>)}</ul><p><strong>ACTUS:</strong> {b.actus}</p><p><strong>DeFi:</strong> {b.defi}</p><p>{b.target}</p><p>{b.qualification}</p><p>{b.catalogue}</p><Sources items={b.sourceRefs}/></details>
 <details className="evidence-drawer"><summary>Source-reading scope and evidence-stage glossary</summary><p>{s.statusScope.sourceReadingCaveat}</p><dl className="detail-list">{Object.entries(s.statusScope.stageLabels).map(([term,meaning])=><React.Fragment key={term}><dt>{term}</dt><dd>{meaning}</dd></React.Fragment>)}</dl><p>{s.footer}</p></details>
 <details className="evidence-drawer"><summary>Case research and integration dependencies</summary>{data.scenarios.map(scenario=><article className="technical-case" key={scenario.id}><h3>{scenario.title}</h3><ul>{scenario.dependencies.map((x,i)=><li key={i}>{x}</li>)}</ul><details className="evidence-drawer"><summary>Role premises and source records</summary>{scenario.strands.map(strand=><div key={strand.id}><h4>{strand.label}</h4><p>{strand.authentication}</p><p>{strand.limits}</p></div>)}<Sources items={scenario.sourceRefs}/></details></article>)}</details>
 <details className="evidence-drawer" id="book-source-checks"><summary>Book passages and source checks</summary><p>These checks cover the passages used on this page; they do not certify the whole book.</p>{data.bookFindings.map(finding=><article className="technical-case" key={finding.id}><h4>{finding.summary}</h4><p>{finding.evidence}</p></article>)}</details>
 <Sources items={t.sourceRefs}/>
 </section>}
function BrandMark() {
  return <svg className="brand-mark" viewBox="0 0 48 48" aria-hidden="true">
    <path d="M29 5a19 19 0 1 0 0 38C10 36 10 12 29 5Z" fill="currentColor"/>
    <path d="M32 10c0 10-1 13-7 14 6 1 7 4 7 14 0-10 1-13 7-14-6-1-7-4-7-14Z" fill="currentColor"/>
    <circle cx="24" cy="24" r="22" fill="none" stroke="currentColor" strokeWidth=".5"/>
  </svg>;
}
function Definitions() {
  return <section className="definitions" aria-labelledby="definitions-title">
    <div className="definition-heading"><h2 id="definitions-title">What are Webs and Strands?</h2><p>The payment above has several conditions to check. These are the names for the evidence that travels with it.</p></div>
    <div className="definition-strip">{main.definitions.slice(0,2).map(item=><article key={item.term}><h3>{item.term}</h3><p>{item.body}</p></article>)}</div>
    <details className="evidence-drawer"><summary>How can an application reuse a kind of claim?</summary><p>{main.definitions[2].body}</p></details>
  </section>;
}
export default function App() {
  const [amount,setAmount] = useState(35);
  const [openingCash,setOpeningCash] = useState(112);
  const invoice = invoiceAmounts(amount,openingCash);
  return <>
    <a className="skip-link" href="#main">Skip to content</a>
    <div className="shell">
      <header className="site-header">
        <a className="brand" href="#top" aria-label="Moriarty Webs, start"><BrandMark/><span className="brand-name">MORIARTY<small>Webs for Cardano</small></span></a>
        <nav className="site-nav" aria-label="Page navigation"><a href="#worksheet">Payment example</a><a href="#explorer">Applications</a><a href="#lifecycle">Settlement</a><a href="#groth16">Groth16</a><a href="#evidence">Technical notes</a></nav>
      </header>
      <main id="main">
        <section className="hero" id="top" aria-labelledby="page-title">
          <div className="hero-copy"><p className="eyebrow">Moriarty on Cardano</p><h1 id="page-title">Prove what a Cardano transaction does.</h1><p className="hero-lede">{main.heroBody}</p><p className="hero-example">{main.heroExample}</p><div className="hero-actions"><a className="primary-button" href="#worksheet">Follow a payment <span aria-hidden="true">↓</span></a><a className="text-link" href="#groth16">How the proofs work <Arrow/></a></div></div>
          <figure className="hero-art"><img className="moriarty-portrait" src={brandBanner} alt=""/><figcaption>Moriarty · Webs and Strands</figcaption></figure>
        </section>
        <Worksheet invoice={invoice} amount={amount} setAmount={setAmount} openingCash={openingCash} setOpeningCash={setOpeningCash}/>
        <Definitions/>
        <Explorer invoice={invoice}/>
        <Lifecycle/>
        <Developers/>
        <Groth16 content={templateCopy(data.groth16,invoice)}/>
        <SevenLayers content={templateCopy(data.sevenLayers,invoice)}/>
        <FAQ/>
        <Evidence/>
      </main>
      <footer className="site-footer"><a className="brand footer-brand" href="#top"><BrandMark/><span>MORIARTY</span></a><p>Webs and Strands for Cardano<br/>2026</p><div className="footer-links"><a href="https://github.com/CharlesHoskinson/Moriarty/tree/utxo" target="_blank" rel="noreferrer">GitHub <Arrow/></a><a href="#evidence">Implementation notes</a><details><summary>Sources and credits</summary><p>Artwork adapted from the Moriarty repository banner. The architecture section uses the seven-layer framework from <em>Proving Nothing</em>; its source passages are linked in that section.</p><p>React, React DOM, Scheduler and esbuild: MIT. Full notices are embedded in this file.</p></details></div></footer>
    </div>
  </>;
}

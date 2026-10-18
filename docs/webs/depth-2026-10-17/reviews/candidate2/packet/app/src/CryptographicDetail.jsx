import React from 'react';
import depth from '../../depth-2026-10-17/COPY.json';

const copy=depth.opus;
const Paragraphs=({items})=>items.map((p,i)=><p key={i}>{p}</p>);

export function ProofExplanation() {
  const g=copy.groth16;
  return <div className="proof-explanation">
    <h3>What the verifier actually checks</h3>
    <Paragraphs items={g.proofExplanation.slice(0,4)}/>
    <h3>A private witness and a public transaction</h3>
    <Paragraphs items={[g.publicAndPrivateInputs[0],g.publicAndPrivateInputs[2]]}/>
    <h3>Execution, universal properties and ledger facts</h3>
    <p>{g.proofExplanation[4]}</p>
    <Paragraphs items={[g.ledgerClaimsExplanation[0],g.ledgerClaimsExplanation[2]]}/>
  </div>;
}

export function CircuitIdentityExplanation() {
  return <div className="proof-explanation">
    <h3>Why changing a payment differs from changing a circuit</h3>
    <Paragraphs items={copy.groth16.circuitIdentityExample}/>
    <details className="evidence-drawer"><summary>How parameterized reuse fits the current compiler</summary>
      <Paragraphs items={copy.groth16.countExplanation}/>
      <p>{copy.groth16.publicAndPrivateInputs[1]}</p>
    </details>
  </div>;
}

export function BindingReference() {
  return <details className="evidence-drawer" id="binding-reference"><summary>Which identities connect the proof to this transaction?</summary>
    <p>The following source-level map separates identities the consumer compares from correspondences that still need evidence.</p>
    <div className="binding-cards">{copy.binding.map(card=><article key={card.title}><h4>{card.title}</h4><p>{card.body}</p><p className="source-id">{card.source}</p></article>)}</div>
  </details>;
}

export function SetupEvidence() {
  return <details className="evidence-drawer"><summary>Who performed the recorded setup?</summary>
    <Paragraphs items={copy.groth16.sourceSetupStatus.slice(0,3)}/>
  </details>;
}

export function ImplementationSummary() {
  return <div className="depth-prose"><Paragraphs items={copy.groth16.sourceSetupStatus.slice(0,3)}/></div>;
}

export function SettlementLifecycle() {
  const c=copy.lifecycle;
  const stages=c.stages;
  const card=id=>{
    const step=stages.find(s=>s.id===id);
    return <article className="lifecycle-card" key={id} data-stage={id}><h4>{step.title}</h4><p>{step.action}</p><dl><dt>Consumes or uses</dt><dd>{step.inputs}</dd><dt>Produces</dt><dd>{step.outputs}</dd></dl></article>;
  };
  return <>
    <div className="depth-prose"><p>{c.intro}</p></div>
    <div className="lifecycle-lanes" aria-label="Offchain preparation and onchain transactions">
      <section className="lifecycle-lane"><h3>Offchain · prepare the authorized calculation</h3><div className="lifecycle-cards">{['authorize','prove'].map(card)}</div></section>
      <div className="lifecycle-handoff">↓ Proof, statement and binding context go to Issue</div>
      <section className="lifecycle-lane"><h3>Onchain · issue evidence, then spend it</h3><div className="lifecycle-cards">{['issue','settle'].map(card)}</div></section>
    </div>
    <div className="depth-prose"><h3>The invoice route has two onchain transactions</h3><p>For the one-Strand invoice design: one Issue checks the proof and creates one claim output. One settlement consumes that claim output and the escrow state, burns the claim token, pays the seller and fee recipient, and creates the continuing escrow. Opening the agreement and any later recovery are separate transactions.</p><p>{c.cardinality[0]}</p></div>
    <details className="evidence-drawer"><summary>Signatures, funding and transaction counts</summary>
      <div className="artifact-flow"><table><caption>Source-described responsibilities; no ADA amounts are measured here</caption><thead><tr><th scope="col">Event</th><th scope="col">Frequency</th><th scope="col">Signing and funding</th></tr></thead><tbody>{stages.map(s=><tr key={s.id}><th scope="row">{s.title}</th><td>{s.lane}</td><td>{s.signersAndFunding}</td></tr>)}</tbody></table></div>
      <Paragraphs items={c.cardinality.slice(1)}/>
      <Paragraphs items={c.fundingAndSigners}/>
      <p>Open composition question from the source review: {copy.openQuestions.find(q=>q.startsWith('How does the rule'))}</p>
    </details>
    <details className="evidence-drawer"><summary>Unused claims, stale state and readback</summary>
      <p>{copy.binding.find(c=>c.title==='Spent state and sequence').body}</p>
      <p>{stages.find(s=>s.id==='reclaim').action} {stages.find(s=>s.id==='reclaim').signersAndFunding}</p>
      <p>{c.failureBranch[0]}</p><p>{c.failureBranch[2]}</p>
    </details>
    <BindingReference/>
  </>;
}

import React, {useState} from 'react';

const fill=(text,values)=>String(text).replace(/\{([A-Za-z][A-Za-z0-9]*)\}/g,(match,key)=>Object.hasOwn(values,key)?values[key]:match);
const fillText=(text,values)=>Array.isArray(text)?text.map(item=>fill(item,values)):typeof text==='string'?fill(text,values):text;
function Paragraphs({text}) {return (Array.isArray(text)?text:[text]).filter(Boolean).map((paragraph,i)=><p key={i}>{paragraph}</p>)}

/** The counter varies an assumed number of compatible distinct circuits. */
export default function Groth16({content:c}) {
  const [count,setCount]=useState(c.counter.initial);
  const values={K:count,k:count,count};
  return <section className="section groth16-section" id="groth16" aria-labelledby="groth16-title">
    <div className="section-heading"><span className="eyebrow"><span className="section-number">06</span>{c.kicker}</span><h2 id="groth16-title">{c.title}</h2><div className="section-intro"><Paragraphs text={c.intro}/></div></div>
    <div className="groth16-concepts">{c.concepts.map((concept,i)=><article key={i}><span className="eyebrow">{concept.label}</span><h3>{concept.title}</h3><Paragraphs text={concept.body}/></article>)}</div>
    <div className="setup-process"><div className="setup-process-intro"><h3>{c.setupTitle}</h3><Paragraphs text={c.setupIntro}/></div><div className="setup-stages">{c.setupStages.map((stage,i)=><article key={i}><span className="step-index">{stage.label}</span><h3>{stage.title}</h3><Paragraphs text={stage.body}/></article>)}</div></div>
    <div className="setup-counter"><div><span className="eyebrow">{c.counter.kicker}</span><h3>{c.counter.title}</h3><Paragraphs text={fillText(c.counter.body,values)}/><label htmlFor="circuit-count">{c.counter.label}</label><div className="counter-input"><button aria-label={c.counter.decreaseLabel} onClick={()=>setCount(n=>Math.max(c.counter.min,n-1))} disabled={count===c.counter.min}>−</button><input id="circuit-count" type="number" inputMode="numeric" min={c.counter.min} max={c.counter.max} step="1" value={count} onChange={e=>{const next=Number(e.target.value);if(Number.isInteger(next)&&next>=c.counter.min&&next<=c.counter.max)setCount(next)}}/><button aria-label={c.counter.increaseLabel} onClick={()=>setCount(n=>Math.min(c.counter.max,n+1))} disabled={count===c.counter.max}>+</button></div></div><div className="setup-count-result"><div role="status"><p className="count-formula">{fill(c.counter.formula,values)}</p><p>{fill(c.counter.result,values)}</p></div><Paragraphs text={fillText(c.counter.conditions,values)}/></div></div>
    <div className="reuse-examples"><h3>{c.reuseTitle}</h3>{c.reuseExamples.map((example,i)=><details className="evidence-drawer" key={i}><summary>{example.title}</summary><Paragraphs text={example.body}/></details>)}</div>
    <div className="setup-trust"><h3>{c.trustTitle}</h3><Paragraphs text={c.trust}/></div>
    <div className="cardano-count"><h3>{c.cardanoTitle}</h3><Paragraphs text={c.cardanoCount}/></div>
    <a className="text-link" href="#security-card-setup-keys" onClick={()=>{document.getElementById('security-card-setup-keys').open=true}}>{c.technicalLinkLabel} <span aria-hidden="true">↗</span></a>
    {c.sourceRefs?.length>0&&<details className="evidence-drawer section-sources"><summary>Sources</summary><ul className="source-list">{c.sourceRefs.map((source,i)=><li key={i}>{source.url?<a href={source.url} target={source.url.startsWith('#')?undefined:'_blank'} rel={source.url.startsWith('#')?undefined:'noreferrer'}>{source.label}</a>:<span>{source.label}</span>}{source.pointer&&<p className="source-id">{source.pointer}</p>}</li>)}</ul></details>}
  </section>
}

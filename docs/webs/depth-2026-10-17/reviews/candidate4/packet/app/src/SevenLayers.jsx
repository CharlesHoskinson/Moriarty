import React, {useRef, useState} from 'react';
import ConcernGraph, {relationPairs} from './ConcernGraph.jsx';

const guideLinks={1:['#groth16','Groth16 setup and circuit counts'],2:['#developers','Financial language and the developer handoff'],3:['#worksheet-title','Invoice inputs and remaining duties'],4:['#worksheet-title','Invoice arithmetic and duty updates'],5:['#groth16','Groth16 proofs and setup'],6:['#groth16','Groth16 keys and setup trust'],7:['#lifecycle','The proof-claim lifecycle']};
function Paragraphs({text}) {return (Array.isArray(text)?text:[text]).filter(Boolean).map((paragraph,i)=><p key={i}>{paragraph}</p>)}

/** The seven concerns keep their canonical numbering; navigation follows dependencies. */
export default function SevenLayers({content:c}) {
  const [selected,setSelected]=useState(1);
  const layer=c.layers.find(layer=>layer.number===selected)||c.layers[0];
  const relatedNumbers=relationPairs(c.layers).flatMap(([a,b])=>a===layer.number?[b]:b===layer.number?[a]:[]).sort((a,b)=>a-b);
  const panel=useRef(null);
  const select=(number)=>{setSelected(number);if(window.matchMedia('(max-width:760px)').matches)requestAnimationFrame(()=>{const el=panel.current,box=el.getBoundingClientRect();if(box.bottom>window.innerHeight||box.top<0)el.scrollIntoView({block:'start',behavior:window.matchMedia('(prefers-reduced-motion: reduce)').matches?'auto':'smooth'})})};
  return <section className="section seven-layers-section" id="seven-layers" aria-labelledby="seven-layers-title">
    <div className="section-heading"><span className="eyebrow"><span className="section-number">06</span>{c.kicker}</span><h2 id="seven-layers-title">{c.title}</h2><div className="section-intro"><Paragraphs text={c.intro}/></div></div>
    <div className="layer-explorer">
      <ConcernGraph layers={c.layers} selected={selected} onSelect={select}/>
      <article className="layer-panel" id="selected-layer" ref={panel} aria-label={c.panelLabel}>
        <span className="eyebrow">{String(layer.number).padStart(2,'0')} / {layer.name}</span>
        <h3>{layer.headline||layer.name}</h3>{layer.plainLanguage?<><Paragraphs text={layer.plainLanguage}/><details className="evidence-drawer"><summary>Technical encoding</summary><Paragraphs text={layer.body}/></details></>:<Paragraphs text={layer.body}/>}
        {layer.artifact&&<div className="layer-artifact"><h4>{c.artifactLabel}</h4><Paragraphs text={layer.artifact}/></div>}
        <div className="layer-guide-links"><h4>Related example</h4><a className="text-link" href={guideLinks[layer.number][0]}>{guideLinks[layer.number][1]} <span aria-hidden="true">↗</span></a></div>
        {relatedNumbers.length>0&&<div className="layer-dependencies"><h4>{c.dependenciesLabel}</h4><div className="dependency-buttons">{relatedNumbers.map(number=>{const target=c.layers.find(x=>x.number===number);return <button key={number} onClick={()=>select(number)} data-dependency={number}>{number} · {target.name} <span aria-hidden="true">↗</span></button>})}</div></div>}
      </article>
    </div>
    {c.relationships?.length>0&&<details className="evidence-drawer layer-relationships"><summary>{c.relationshipsTitle}</summary><ul>{c.relationships.map((relation,i)=><li key={i}>{relation}</li>)}</ul></details>}
    <details className="evidence-drawer layer-mapping"><summary>See how all seven concerns apply to a Web</summary><div className="artifact-flow"><table><caption>{c.mappingTitle}</caption><thead><tr><th scope="col">{c.mappingHeaders[0]}</th><th scope="col">{c.mappingHeaders[1]}</th></tr></thead><tbody>{c.layers.map(item=><tr key={item.number}><th scope="row">{item.number} · {item.name}</th><td>{item.mapping}</td></tr>)}</tbody></table></div></details>
    {c.sourceRefs?.length>0&&<details className="evidence-drawer section-sources"><summary>Sources and the seven-layer framework</summary><p>{c.credit}</p><p><a href={c.sourceUrl} target="_blank" rel="noreferrer">{c.sourceLabel}</a></p><ul className="source-list">{c.sourceRefs.map((source,i)=><li key={i}>{source.url?<a href={source.url} target={source.url.startsWith('#')?undefined:'_blank'} rel={source.url.startsWith('#')?undefined:'noreferrer'}>{source.label}</a>:<span>{source.label}</span>}{source.pointer&&<p className="source-id">{source.pointer}</p>}</li>)}</ul></details>}
  </section>
}

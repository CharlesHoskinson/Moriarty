import React, {useRef, useState} from 'react';

const guideLinks={1:['#groth16','Groth16 setup and circuit counts'],2:['#developers','Financial language and the developer handoff'],3:['#worksheet-title','Invoice inputs and remaining duties'],4:['#worksheet-title','Invoice arithmetic and duty updates'],5:['#groth16','Groth16 proofs and setup'],6:['#groth16','Groth16 keys and setup trust'],7:['#lifecycle','The proof-claim lifecycle']};
function Paragraphs({text}) {return (Array.isArray(text)?text:[text]).filter(Boolean).map((paragraph,i)=><p key={i}>{paragraph}</p>)}

/** Layer numbers are book references. Navigation follows authored dependencies. */
export default function SevenLayers({content:c}) {
  const [selected,setSelected]=useState(1);
  const layer=c.layers.find(layer=>layer.number===selected)||c.layers[0];
  const panel=useRef(null);
  const select=(number)=>{setSelected(number);if(window.matchMedia('(max-width:760px)').matches)requestAnimationFrame(()=>{const el=panel.current,box=el.getBoundingClientRect();if(box.bottom>window.innerHeight||box.top<0)el.scrollIntoView({block:'start',behavior:window.matchMedia('(prefers-reduced-motion: reduce)').matches?'auto':'smooth'})})};
  return <section className="section seven-layers-section" id="seven-layers" aria-labelledby="seven-layers-title">
    <div className="section-heading"><span className="eyebrow"><span className="section-number">05</span>{c.kicker}</span><h2 id="seven-layers-title">{c.title}</h2><div className="section-intro"><Paragraphs text={c.intro}/></div></div>
    <div className="layer-explorer">
      <div className="layer-selector" role="group" aria-label={c.selectorLabel}>
        {c.layers.map(item=><button key={item.number} className="layer-select" aria-pressed={selected===item.number} aria-controls="selected-layer" data-layer={item.number} onClick={()=>select(item.number)}><span className="layer-number">{item.number}</span><span>{item.name}{item.subtitle&&<small>{item.subtitle}</small>}</span><span className="layer-select-arrow" aria-hidden="true">↗</span></button>)}
      </div>
      <article className="layer-panel" id="selected-layer" ref={panel} aria-label={c.panelLabel}>
        <span className="eyebrow">{String(layer.number).padStart(2,'0')} / {layer.name}</span>
        <h3>{layer.headline||layer.name}</h3><Paragraphs text={layer.body}/>
        {layer.artifact&&<div className="layer-artifact"><h4>{c.artifactLabel}</h4><Paragraphs text={layer.artifact}/></div>}
        <div className="layer-guide-links"><h4>Explore this in the guide</h4><a className="text-link" href={guideLinks[layer.number][0]}>{guideLinks[layer.number][1]} <span aria-hidden="true">↗</span></a></div>
        {layer.dependencies?.length>0&&<div className="layer-dependencies"><h4>{c.dependenciesLabel}</h4><div className="dependency-buttons">{layer.dependencies.map(number=>{const target=c.layers.find(x=>x.number===number);return <button key={number} onClick={()=>select(number)} data-dependency={number}>{number} · {target.name} <span aria-hidden="true">↗</span></button>})}</div></div>}
      </article>
    </div>
    {c.relationships?.length>0&&<details className="evidence-drawer layer-relationships"><summary>{c.relationshipsTitle}</summary><ul>{c.relationships.map((relation,i)=><li key={i}>{relation}</li>)}</ul></details>}
    <div className="artifact-flow layer-mapping"><table><caption>{c.mappingTitle}</caption><thead><tr><th scope="col">{c.mappingHeaders[0]}</th><th scope="col">{c.mappingHeaders[1]}</th></tr></thead><tbody>{c.layers.map(item=><tr key={item.number}><th scope="row">{item.number} · {item.name}</th><td>{item.mapping}</td></tr>)}</tbody></table></div>
    <p className="book-credit"><span>{c.credit}</span> <a href={c.sourceUrl} target="_blank" rel="noreferrer">{c.sourceLabel} <span aria-hidden="true">↗</span></a></p>
    {c.sourceRefs?.length>0&&<details className="evidence-drawer section-sources"><summary>Sources</summary><ul className="source-list">{c.sourceRefs.map((source,i)=><li key={i}>{source.url?<a href={source.url} target={source.url.startsWith('#')?undefined:'_blank'} rel={source.url.startsWith('#')?undefined:'noreferrer'}>{source.label}</a>:<span>{source.label}</span>}{source.pointer&&<p className="source-id">{source.pointer}</p>}</li>)}</ul></details>}
  </section>
}

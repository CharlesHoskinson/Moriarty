import React from 'react';

/** The diagram's nodes are native, visible buttons over original SVG lines. */
export default function WebDiagram({scenario, selectedStrand, onSelect, omittedStrands=[]}) {
  return <div className="strand-diagram" aria-label={`${scenario.title}: required contract checks`}>
    <svg className="strand-diagram-lines" viewBox="0 0 600 360" preserveAspectRatio="none" aria-hidden="true" focusable="false">
      {scenario.strands.map((strand,i)=><path key={strand.id} d={`M 270 ${60+i*120} C 420 ${60+i*120}, 420 180, 485 180`} className={omittedStrands.includes(strand.id)?'is-omitted':''}/>) }
      <circle cx="485" cy="180" r="48"/>
      <circle cx="485" cy="180" r="58" className="diagram-orbit"/>
    </svg>
    <div className="diagram-nodes" role="group" aria-label="Select a contract check">
      {scenario.strands.map((strand,i)=><button key={strand.id} className={`strand-button diagram-node ${omittedStrands.includes(strand.id)?'is-omitted':''}`} aria-pressed={selectedStrand===strand.id} aria-controls="role-inspector" onClick={()=>onSelect(strand.id)} data-strand={strand.id}>
        <span className="strand-number">{i+1}</span><span className="strand-label">{strand.label}<small className="role-subtitle">{omittedStrands.includes(strand.id)?'Coverage removed':'Required check'}</small></span><span className="select-arrow" aria-hidden="true">↗</span>
      </button>)}
    </div>
    <div className="diagram-hub" aria-hidden="true"><strong>Web</strong><small>proof collection</small></div>
  </div>;
}

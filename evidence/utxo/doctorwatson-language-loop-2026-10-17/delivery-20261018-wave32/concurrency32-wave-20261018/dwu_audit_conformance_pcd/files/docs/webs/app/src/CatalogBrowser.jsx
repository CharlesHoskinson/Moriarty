import React, {useMemo, useState} from 'react';
import {loadForDisplay} from './catalogue.ts';

const SourceJSON=({value})=><pre style={{whiteSpace:'pre-wrap',overflowWrap:'anywhere'}}><code>{JSON.stringify(value,null,2)}</code></pre>;

/** Read-only source inspector. Selection never changes evidence or acceptance. */
export default function CatalogBrowser({catalogueText,source}) {
  const catalogue=useMemo(()=>loadForDisplay(catalogueText,source),[catalogueText,source]);
  const [selectedId,setSelectedId]=useState(()=>(
    catalogue.entries.find(entry=>entry.identity.sourceHead&&entry.publicStatement.schema)?.identity.versionedId
    ||catalogue.entries[0]?.identity.versionedId||''
  ));
  const entry=catalogue.entries.find(item=>item.identity.versionedId===selectedId);
  const schema=entry?.publicStatement.schema;
  return <details className="evidence-drawer" id="retained-predicate-catalogue">
    <summary>Retained predicate catalogue</summary>
    <p><code>{catalogue.catalogueId}</code> · <code>{catalogue.catalogueVersion}</code></p>
    <p>These are retained source declarations. Selecting a version changes only this inspector.</p>
    <p>{catalogue.coverage.versionedRows} versioned rows · {catalogue.coverage.sourceHeads} source heads. Every retained version is available below. Source heads follow explicit supersession references.</p>
    <label htmlFor="catalogue-row">Predicate and version</label>{' '}
    <select id="catalogue-row" value={selectedId} onChange={event=>setSelectedId(event.target.value)}>
      {catalogue.entries.map(item=><option key={item.identity.versionedId} value={item.identity.versionedId} data-catalogue-row={item.identity.versionedId}>
        {item.identity.versionedId} · {item.title}
      </option>)}
    </select>
    {entry&&<article className="technical-case" data-selected-catalogue-row={entry.identity.versionedId}>
      <h3>{entry.title}</h3>
      <dl className="detail-list">
        <dt>Predicate ID</dt><dd><code>{entry.identity.id}</code></dd>
        <dt>Version</dt><dd><code>{entry.identity.version}</code></dd>
        <dt>Versioned identity</dt><dd><code>{entry.identity.versionedId}</code></dd>
        <dt>Family</dt><dd>{entry.identity.family}</dd>
        <dt>Supersedes</dt><dd>{entry.identity.supersedes||'No predecessor declared in this row.'}</dd>
        <dt>Source status</dt><dd>{entry.evidence.sourceStatus}</dd>
        <dt>Source category</dt><dd>{entry.evidence.sourceCategory}</dd>
        <dt>Claim kind</dt><dd>{entry.claimKind}</dd>
        <dt>Property</dt><dd>{entry.property}</dd>
        <dt>Public input description</dt><dd>{entry.inputs.publicDescription}</dd>
        <dt>Witness description</dt><dd>{entry.inputs.privateWitnessDescription}</dd>
        <dt>Result schema</dt><dd>{entry.outputs.description??'No separate result schema declared in this row.'}</dd>
        <dt>Source semantics</dt><dd>{entry.profile.sourceSemanticsId??'Not declared in this row.'}</dd>
        <dt>Source anchor type</dt><dd>{entry.target.sourceAnchorType??'Not declared in this row.'}</dd>
        <dt>Temporal scope</dt><dd>{entry.target.sourceTemporalScope}</dd>
        <dt>Freshness and validity</dt><dd>{entry.target.sourceFreshnessValidity}</dd>
        <dt>Source assumptions</dt><dd>{entry.assumptions.sourceDescription}</dd>
        <dt>Implementation evidence</dt><dd>{entry.evidence.implementationDescription}</dd>
        <dt>Recorded audit reference</dt><dd>{entry.evidence.sourceAuditRef??'Not declared in this row.'}</dd>
        <dt>First blocker</dt><dd>{entry.limits.firstBlocker}</dd>
        <dt>Not established</dt><dd>{entry.limits.notEstablished}</dd>
      </dl>
      {schema?<>
        <h4>Source-declared statement fields</h4><p>{schema.encoding}</p>
        <div className="artifact-flow" role="region" aria-label="Source-declared statement fields" tabIndex="0">
          <table><thead><tr><th scope="col">Field</th><th scope="col">Complete source definition</th></tr></thead>
            <tbody>{schema.fields.map((field,index)=><tr key={index} data-catalogue-field={field.name}>
              <th scope="row">{field.name}</th><td data-label="Complete source definition"><SourceJSON value={field}/></td>
            </tr>)}</tbody>
          </table>
        </div>
        <p>{schema.note}</p>
        <details className="evidence-drawer"><summary>Source-declared binding constraints</summary><SourceJSON value={schema.binding_constraints}/></details>
      </>:<p>This retained row provides a public-input description without a typed statement field list.</p>}
      <details className="evidence-drawer"><summary>Display metadata and source location</summary>
        <dl className="detail-list">
          <dt>Profile selection</dt><dd>{entry.profile.status}</dd>
          <dt>Target selection</dt><dd>{entry.target.status}</dd>
          <dt>Setup identity</dt><dd>{entry.setupIdentity.status}</dd>
          <dt>Source document</dt><dd><code>{entry.source.path}</code></dd>
          <dt>Source location</dt><dd><code>{entry.source.jsonPointer}</code></dd>
          <dt>Source file SHA-256</dt><dd><code>{entry.source.sha256}</code></dd>
        </dl>
      </details>
      <details className="evidence-drawer"><summary>Complete original source row</summary><SourceJSON value={entry.sourceRow}/></details>
    </article>}
  </details>;
}

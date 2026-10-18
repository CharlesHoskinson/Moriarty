import React, {useId} from 'react';

/**
 * A controlled, display-only transaction map. `checks` associates an entire
 * node's displayed values with check IDs; it does not describe proof topology.
 * Use kind: 'evidence' to keep evidence inputs distinct from asset/state inputs.
 * See README.md for the model contract and an illustrative invoice model.
 */
export default function TransactionDiagram({model, selectedCheck, onSelect}) {
  const instanceId = useId();
  const titleId = `${instanceId}-title`;
  const detailsId = `${instanceId}-check-details`;
  const inputs = model.inputs ?? [];
  const outputs = model.outputs ?? [];
  const financial = model.mode === 'financial';
  const checks = model.checks ?? [];
  const stateInputs = inputs.filter(node => node.kind !== 'evidence');
  const evidenceInputs = inputs.filter(node => node.kind === 'evidence');
  const activeCheck = checks.find(check => check.id === selectedCheck);
  const covers = node => Boolean(activeCheck && node.checks?.includes(activeCheck.id));
  const coveredInputs = inputs.filter(covers);
  const coveredOutputs = outputs.filter(covers);

  return (
    <section className="transaction-diagram" aria-labelledby={titleId} data-model={model.id}>
      <header className="td-heading">
        <p className="td-kicker">{financial ? 'Financial model, before and after' : 'One settlement, before and after'}</p>
        <h3 id={titleId}>{model.title}</h3>
        <p className="td-intro">{financial ? 'Trace the payments and continuing accounts. These financial elements do not specify a serialized transaction or datum layout.' : 'Follow the escrow input into the transaction, then read the recipient payments and continuing state.'}</p>
      </header>

      <div className="td-checks" role="group" aria-label="Choose a check to trace its displayed values">
        {checks.map((check, index) => (
          <button
            key={check.id}
            data-check={check.id}
            type="button"
            className="td-check"
            aria-pressed={activeCheck?.id === check.id}
            aria-controls={detailsId}
            onClick={() => onSelect?.(check.id)}
          >
            <span className="td-check-number" aria-hidden="true">{index + 1}</span>
            <span>{check.label}</span>
            <span className="td-check-action" aria-hidden="true">Trace values</span>
          </button>
        ))}
      </div>

      <div className="td-selection" id={detailsId} aria-live="polite" aria-atomic="true">
        {activeCheck ? (
          <>
            <p className="td-selected-title"><strong>{activeCheck.label}</strong></p>
            <p>{activeCheck.explanation}</p>
            <p className="td-selection-summary">
              {coveredInputs.length + coveredOutputs.length > 0
                ? `Highlighted display values: ${[...coveredInputs, ...coveredOutputs].map(node => node.label).join('; ')}.`
                : 'This check has no values mapped in the supplied diagram.'}
            </p>
          </>
        ) : <p>Select a check to highlight its related input and output values.</p>}
      </div>

      <figure className="td-figure">
        <div className="td-legend" aria-label="Diagram key">
          <span><i className="td-key td-key-state" aria-hidden="true"/>Payments and state changes</span>
          <span><i className="td-key td-key-evidence" aria-hidden="true"/>Evidence input</span>
          <span><i className="td-key td-key-selected" aria-hidden="true"/>Related to the selected check</span>
        </div>

        <div className="td-flow">
          <section className="td-lane td-inputs" aria-label="Transaction inputs">
            <div className="td-lane-heading">
              <span className="td-step" aria-hidden="true">01</span>
              <h4>{financial ? 'Inputs and opening accounts' : 'Spent UTxOs / state'}</h4>
              <p>Values before this transaction</p>
            </div>
            <NodeList nodes={stateInputs} side="input" covers={covers} financial={financial}/>
            {evidenceInputs.length > 0 && (
              <div className="td-evidence-group">
                <h5>Evidence inputs</h5>
                <p className="td-group-description">Shown separately from asset and state values.</p>
                <NodeList nodes={evidenceInputs} side="input" covers={covers} financial={financial}/>
              </div>
            )}
          </section>

          <FlowLink className="td-link-in" label="Inputs to settlement" shortLabel="Consume"/>

          <section className="td-settlement" aria-label="Settlement transaction">
            <span className="td-step" aria-hidden="true">02</span>
            <h4>{financial ? 'Application action' : 'Settlement transaction'}</h4>
            <p className="td-boundary">{financial ? 'Reconcile the same amounts on both sides' : 'The transaction boundary'}</p>
            <p>{financial ? 'Payments change custody. Duties record what remains owed. Limits describe what the action may do.' : 'Consumes the escrow and claim input; produces the recipient payments and continuing escrow.'}</p>
            {!financial && <dl className="td-transaction-counts">
              <div><dt>Shown inputs</dt><dd>{inputs.length}</dd></div>
              <div><dt>Shown outputs</dt><dd>{outputs.length}</dd></div>
            </dl>}
          </section>

          <FlowLink className="td-link-out" label="Settlement produces outputs" shortLabel="Produce"/>

          <section className="td-lane td-outputs" aria-label="Transaction outputs">
            <div className="td-lane-heading">
              <span className="td-step" aria-hidden="true">03</span>
              <h4>{financial ? 'Payments and continuing accounts' : 'Produced outputs / next state'}</h4>
              <p>Values after this transaction</p>
            </div>
            <NodeList nodes={outputs} side="output" covers={covers} financial={financial}/>
          </section>
        </div>

        <figcaption className="td-caption">
          The arrows show transaction direction. Highlighting shows the check-to-value mapping supplied by the author, not a verification result.
        </figcaption>
      </figure>

      <div className="td-notes">
        <p className="td-proof-note">Check buttons name conditions, not individual proofs. One Strand can cover several checks; the number of proofs or Issue transactions is not inferred from these buttons.</p>
        {model.note && <p className="td-model-note"><strong>Example and source notes</strong><span>{model.note}</span></p>}
      </div>
    </section>
  );
}

function NodeList({nodes, side, covers, financial}) {
  if (nodes.length === 0) return <p className="td-empty">No {side === 'input' ? 'asset or state inputs' : 'outputs'} supplied in this display.</p>;

  return (
    <ul className="td-node-list">
      {nodes.map(node => {
        const isEvidence = node.kind === 'evidence';
        const isCovered = covers(node);
        const kindLabel = isEvidence ? 'Claim output' : node.kind === 'state' ? 'State UTxO' : node.kind === 'asset' ? 'Asset UTxO' : (node.kind || 'Account').replaceAll('-', ' ');
        return (
          <li key={node.id} className={`td-node${isEvidence ? ' td-node-evidence' : ''}${isCovered ? ' td-node-covered' : ''}`} data-node={node.id}>
            <div className="td-node-meta"><span>{kindLabel}</span>{isCovered && <span className="td-coverage-label">Selected check</span>}</div>
            <h5>{node.label}</h5>
            <ul className="td-values">
              {(node.lines ?? []).map((line, index) => <li key={index}>{line}</li>)}
            </ul>
            <p className="td-node-direction">
              <span className="td-node-line" aria-hidden="true"/>
              {financial ? (side === 'output' ? 'After the action' : 'Before the action') : side === 'output' ? 'Produced by settlement' : isEvidence ? 'Evidence consumed by settlement' : 'Spent by settlement'}
            </p>
          </li>
        );
      })}
    </ul>
  );
}

function FlowLink({className, label, shortLabel}) {
  return <div className={`td-link ${className}`}><span className="td-link-label" aria-hidden="true">{shortLabel}</span><span className="td-flow-arrow" aria-hidden="true"/><span className="td-visually-hidden">{label}</span></div>;
}

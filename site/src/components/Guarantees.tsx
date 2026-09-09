import { useState } from 'react';
import {
  ASSURANCE_LAYERS,
  AUDIT_BOUNDARIES,
  MANDATORY_CLAIMS,
  PROPERTIES,
  THREATS,
} from '../data/assurance';
import { useMode } from '../mode';

/**
 * Section six. The guarantees.
 *
 * The gaps carry the labels, not the links. The eye should read the refusals
 * between the layers rather than a list of achievements, because that is the
 * correct reading of the thesis.
 *
 * The property table keeps three columns and the threat table keeps residual
 * risk as its loudest. Collapsing either is the standard overclaim.
 */
export function Guarantees() {
  const { mode } = useMode();
  const [open, setOpen] = useState<number | null>(0);

  return (
    <div className="wrap">
      <p className="eyebrow">The guarantees</p>
      <h2>Establishing one does not establish the next.</h2>
      <p className="lede">
        A type error caught in source is not a proof. A proof is not a ledger
        acceptance. A ledger acceptance is not the transaction you signed. Each
        of these is a separate obligation, and collapsing them into one word is
        the standard overclaim in this field.
      </p>

      <div className="chain">
        {ASSURANCE_LAYERS.map((l, i) => (
          <div key={l.name}>
            <button
              type="button"
              className={'link' + (open === i ? ' is-open' : '')}
              onClick={() => setOpen(open === i ? null : i)}
              aria-expanded={open === i}
            >
              <span className="link-name">{l.name}</span>
              <span className="link-holds">{l.holds}</span>
            </button>
            {i < ASSURANCE_LAYERS.length - 1 ? (
              <p className="gap" aria-hidden="true">
                does not establish
              </p>
            ) : null}
          </div>
        ))}
        <p className="chain-end">
          What no layer establishes: the residual risk in every row below.
        </p>
      </div>

      <hr className="rule" />

      <h3>Four claims on every accepted transaction</h3>
      <p>Fixed names, fixed order, none optional.</p>
      <div className="claims">
        {MANDATORY_CLAIMS.map((c, i) => (
          <article key={c.name} className={i === MANDATORY_CLAIMS.length - 1 ? 'claim is-last' : 'claim'}>
            <h4 className="mono">{c.name}</h4>
            <p>{c.means}</p>
          </article>
        ))}
      </div>
      <p className="claims-note">
        The last one is what separates this from contract-level verification. A
        valid proof of a valid transition, against a history that is itself
        invalid, must not be accepted.
      </p>

      <hr className="rule" />

      <h3>Properties, and the assumptions that qualify them</h3>
      <p>
        {mode === 'verify'
          ? 'Evidence comes from the Marlowe and Isabelle lineage at a pinned commit. The assumption is what that evidence rests on, and the obligation is what falls to Moriarty. The three never collapse into one.'
          : 'Each property has evidence behind it, an assumption that limits what the evidence covers, and work that still falls to Moriarty.'}
      </p>
      <div className="scroll-x">
        <table className="props">
          <thead>
            <tr>
              <th scope="col">Property</th>
              <th scope="col">Assumption that qualifies it</th>
              <th scope="col">Moriarty obligation</th>
            </tr>
          </thead>
          <tbody>
            {PROPERTIES.map((p) => (
              <tr key={p.name}>
                <th scope="row">{p.name}</th>
                <td className="col-assume">{p.assumption}</td>
                <td>{p.obligation}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <hr className="rule" />

      <h3>Trust boundaries, and what survives them</h3>
      <p>
        The residual column is the point. It is the part nobody publishes.
      </p>
      <div className="scroll-x">
        <table className="threats">
          <thead>
            <tr>
              <th scope="col">Threat</th>
              <th scope="col">Required control</th>
              <th scope="col" className="col-residual-head">Residual risk</th>
            </tr>
          </thead>
          <tbody>
            {THREATS.map((t) => (
              <tr key={t.threat}>
                <th scope="row">
                  {t.threat}
                  <span className="threat-path">{t.path}</span>
                </th>
                <td className="col-control">{t.control}</td>
                <td className="col-residual">{t.residual}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="audit card">
        <h4>A single audit is not an adequate claim</h4>
        <p>These boundaries are audited separately, against different evidence.</p>
        <ul>
          {AUDIT_BOUNDARIES.map((b) => (
            <li key={b}>{b}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

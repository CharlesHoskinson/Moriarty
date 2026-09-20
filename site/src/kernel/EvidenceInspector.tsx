import { useState } from 'react';
import { BINDINGS, DESTINATIONS, MECHANISMS, type Mechanism } from './evidence';

/**
 * Selects one mechanism, one binding field and one destination profile and
 * shows claim, assumptions, required bindings and possible bypass. Presentation
 * only: nothing here verifies anything.
 */
export function EvidenceInspector() {
  const [mech, setMech] = useState<Mechanism['id']>('zk');
  const [binding, setBinding] = useState(BINDINGS[0]!.id);
  const [dest, setDest] = useState(DESTINATIONS[0]!.id);

  const m = MECHANISMS.find((x) => x.id === mech)!;
  const b = BINDINGS.find((x) => x.id === binding)!;
  const d = DESTINATIONS.find((x) => x.id === dest)!;

  return (
    <div className="k-inspector">
      <div className="k-insp-col">
        <div className="k-ctl-group" role="group" aria-labelledby="insp-mech-h">
          <h3 id="insp-mech-h" className="k-ctl-head">Mechanism</h3>
          <div className="k-seg">
            {MECHANISMS.map((x) => (
              <button key={x.id} type="button" aria-pressed={x.id === mech} onClick={() => setMech(x.id)} data-mech={x.id}>
                {x.name}
              </button>
            ))}
          </div>
        </div>
        <div className="k-insp-panel" aria-live="polite">
          <p className="k-insp-label">Establishes</p>
          <p className="k-insp-claim">{m.claim}</p>
          <p className="k-insp-label">Under these named assumptions</p>
          <ul className="k-insp-list">
            {m.assumptions.map((a) => <li key={a}>{a}</li>)}
          </ul>
          <p className="k-insp-label">Does not establish</p>
          <p className="k-insp-not">{m.doesNot}</p>
        </div>
      </div>

      <div className="k-insp-col">
        <div className="k-ctl-group" role="group" aria-labelledby="insp-bind-h">
          <h3 id="insp-bind-h" className="k-ctl-head">Common binding</h3>
          <div className="k-seg k-seg-wrap">
            {BINDINGS.map((x) => (
              <button key={x.id} type="button" aria-pressed={x.id === binding} onClick={() => setBinding(x.id)} data-binding={x.id}>
                {x.field}
              </button>
            ))}
          </div>
        </div>
        <div className="k-insp-panel" aria-live="polite">
          <p className="k-insp-label">Why every mechanism must bind it</p>
          <p>{b.why}</p>
          <p className="k-insp-label">If it is missing</p>
          <p className="k-insp-not">{b.ifMissing}</p>
          <p className="k-insp-fine">
            Where one operator controls the keys, hardware or code behind several
            mechanisms, those checks share a failure and count as correlated
            dependencies, not multiplied protection. The distinct hardware and
            cryptographic assumptions still stand on their own.
          </p>
        </div>
      </div>

      <div className="k-insp-col">
        <div className="k-ctl-group" role="group" aria-labelledby="insp-dest-h">
          <h3 id="insp-dest-h" className="k-ctl-head">External enforcement</h3>
          <div className="k-seg">
            {DESTINATIONS.map((x) => (
              <button key={x.id} type="button" aria-pressed={x.id === dest} onClick={() => setDest(x.id)} data-dest={x.id}>
                {x.name}
              </button>
            ))}
          </div>
        </div>
        <div className="k-insp-panel" aria-live="polite">
          {d.hypothetical ? <p className="k-insp-flag">Hypothetical profile for comparison. It names its checks and nothing else.</p> : null}
          <p className="k-insp-label">The destination enforces</p>
          <ul className="k-insp-list">
            {d.enforces.map((e) => <li key={e}>{e}</li>)}
          </ul>
          <p className="k-insp-label">Possible bypass</p>
          <p className="k-insp-not" data-bypass>{d.bypass}</p>
          <p className="k-insp-label">What Midnight can and cannot do about it</p>
          <p>{d.localLimit}</p>
        </div>
      </div>
    </div>
  );
}

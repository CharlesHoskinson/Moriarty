import { MILESTONES, SPRINTS, TRACKS } from '../data/roadmap';

/**
 * Section nine. Delivery.
 *
 * The completion column is a test rather than a status label, because a
 * sprint is done when something can be checked and not when someone says so.
 */
export function Roadmap() {
  return (
    <div className="wrap">
      <p className="eyebrow">Delivery</p>
      <h2>What would have to be true.</h2>
      <p className="lede">
        The <a href="docs/requirements.html#delivery">consolidated U0–U7 roadmap</a> is the current delivery sequence.
        The twelve sprint contracts below retain their acceptance obligations
        within that sequence; they are not a separate execution queue.
      </p>

      <h3>Recorded foundations</h3>
      <div className="milestones">
        {MILESTONES.map((m) => (
          <article key={m.title}>
            <h4>{m.title}</h4>
            <p>{m.detail}</p>
            {m.figures.length ? (
              <dl className="figs">
                {m.figures.map((f) => (
                  <div key={f.label}>
                    <dt className="mono">{f.value}</dt>
                    <dd>{f.label}</dd>
                  </div>
                ))}
              </dl>
            ) : null}
          </article>
        ))}
      </div>

      <h3 className="tracks-head">Earlier workstream decomposition</h3>
      <div className="tracks">
        {TRACKS.map((t) => (
          <div key={t.name} className="track">
            <span className="track-name">{t.name}</span>
            <ol>
              {t.path.map((s) => (
                <li key={s} className="mono">{s}</li>
              ))}
            </ol>
          </div>
        ))}
      </div>
      <p className="tracks-note">
        These historical tracks identify separate evidence questions. Current
        sequencing and dependencies are defined by U0–U7. One asks whether the successor specification can be
        completed and executed in K. The native track asks whether the
        recursive proof backend can verify a history: a leaf proves S
        consecutive steps, an IVC node at k=20 folds each leaf proof, and a
        k=20 run decider seals the run over the 5/6-lane digest instance
        with arity-4 Poseidon (B group 9/9 on the consolidation line). The
        leaf design proposed on 2026-11-10 is a small processor: a core
        with a 32-word register file, and chips that it reaches only through
        a lookup bus. On a spike, a 16-step leaf is 6,511 rows at k=13, and
        the node and decider over its key are 823,466 and 929,166 rows,
        inside their k=20 gates; these are cost-model rows, and no proof
        over the new leaf has been made. The third asks
        whether the accepted atomic subset can settle real loan and swap
        effects on the public network.
      </p>

      <div className="scroll-x">
        <table className="sprints">
          <thead>
            <tr>
              <th scope="col">Sprint</th>
              <th scope="col">Deliverable</th>
              <th scope="col">What would decide it</th>
            </tr>
          </thead>
          <tbody>
            {SPRINTS.map((s) => (
              <tr key={s.id}>
                <th scope="row" className="mono">{s.id}</th>
                <td>{s.deliverable}</td>
                <td className="sprint-ev">{s.evidence}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <p className="closing">
        Proof-carrying financial settlement is under development. This is not a
        production kit and not an audited deployment.
      </p>
    </div>
  );
}

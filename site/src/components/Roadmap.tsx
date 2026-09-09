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
        Each sprint carries the evidence that would decide it. None of these is
        a status label, and none is satisfied by agreement.
      </p>

      <h3>Reached</h3>
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

      <h3 className="tracks-head">Running in parallel</h3>
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
        The tracks are parallel because their blocking questions are
        independent. One asks whether the successor specification can be
        completed and executed in K. The native track asks whether the
        recursive proof backend can verify a history at all, which is open
        after the first experiment ran out of rows at k17. The third asks
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

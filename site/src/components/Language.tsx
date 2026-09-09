import { useState } from 'react';
import { BOUNDS, DECLARATIONS, PROFILES, SAMPLES, STATEMENTS, WORKFLOW, WORKFLOW_RULES } from '../data/language';
import { useMode } from '../mode';

/**
 * Section seven. The language.
 *
 * Real source carries this section. Every sample is verbatim repository
 * source, and the point beside it says what the sample forbids.
 */
export function Language() {
  const { mode } = useMode();
  const [sample, setSample] = useState(0);
  const current = SAMPLES[sample]!;

  return (
    <div className="wrap">
      <p className="eyebrow">The language</p>
      <h2>The guarantee is written in the source.</h2>
      <p className="lede">
        {mode === 'build'
          ? 'A developer reading twelve lines should know what the agreement forbids, because the agreement says so.'
          : 'Source files carry the extension .mori. Two syntax profiles exist and they are not interchangeable.'}
      </p>

      <div className="profiles">
        {PROFILES.map((p) => (
          <article key={p.id} className="profile">
            <h4 className="mono">{p.id}</h4>
            <p className="profile-name">{p.name}</p>
            <p>{p.has}</p>
            <p className="profile-note">{p.note}</p>
          </article>
        ))}
      </div>

      <div className="src">
        <nav className="src-files" aria-label="Source samples">
          {SAMPLES.map((s, i) => (
            <button
              key={s.id}
              type="button"
              className={i === sample ? 'is-at' : undefined}
              onClick={() => setSample(i)}
              aria-pressed={i === sample}
            >
              <span className="src-title">{s.title}</span>
              <span className="src-profile mono">{s.profile}</span>
            </button>
          ))}
        </nav>
        <div className="src-body">
          <pre>
            <code>{current.source}</code>
          </pre>
          <p className="src-point">{current.point}</p>
        </div>
      </div>

      <hr className="rule" />

      <div className="two-col">
        <div>
          <h3>What the language declares</h3>
          <ul className="kw">
            {DECLARATIONS.map((d) => (
              <li key={d} className="mono">{d}</li>
            ))}
          </ul>
          <h4 className="kw-head">Inside an action</h4>
          <ul className="kw">
            {STATEMENTS.map((d) => (
              <li key={d} className="mono">{d}</li>
            ))}
          </ul>
          <h4 className="kw-head">Bounds on the agreement itself</h4>
          <ul className="kw">
            {BOUNDS.map((d) => (
              <li key={d} className="mono">{d}</li>
            ))}
          </ul>
          <p className="kw-note">
            <code>lifetime</code> caps how many actions an instance can ever
            execute. <code>horizon</code> caps the time past which none is
            admitted. A bounded agreement cannot become an unbounded
            subscription.
          </p>
        </div>
        <div>
          <h3>What a developer does</h3>
          <ol className="flow">
            {WORKFLOW.map((w) => (
              <li key={w}>{w}</li>
            ))}
          </ol>
          <div className="flow-rules">
            {WORKFLOW_RULES.map((r) => (
              <p key={r}>{r}</p>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

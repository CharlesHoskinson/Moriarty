import { useState } from 'react';
import { PIPELINE, SPEC_LAYERS, VISIBILITY, ZKIR_DEFERRAL } from '../data/language';
import { ASSURANCE_LAYERS } from '../data/assurance';
import { useMode } from '../mode';

/**
 * Section five. How the language is specified, and what the pipeline does to a
 * transaction.
 *
 * The pipeline is walked rather than diagrammed. At each stage the value has
 * taken a different representation, and the assurance ladder beside it shows
 * which obligations are settled there and which are still owed.
 */
export function Formalization() {
  const { mode } = useMode();
  const [stage, setStage] = useState(0);
  const current = PIPELINE[stage]!;

  return (
    <div className="wrap">
      <p className="eyebrow">The formalization model</p>
      <h2>What a program looks like, and what it means.</h2>
      <p className="lede">
        {mode === 'build'
          ? 'Each layer is written in the notation that suits it, and each fixes a different question about the source.'
          : 'Lexical rules, an ISO/IEC 14977 grammar, typing judgments, an executable operational semantics in K, and correctness claims stated over those semantics.'}
      </p>

      <div className="scroll-x">
        <table className="layers">
          <thead>
            <tr>
              <th scope="col">Layer</th>
              <th scope="col">Specified by</th>
              <th scope="col">What it fixes</th>
            </tr>
          </thead>
          <tbody>
            {SPEC_LAYERS.map((l) => (
              <tr key={l.layer}>
                <th scope="row">{l.layer}</th>
                <td className="layer-method">{l.method}</td>
                <td>{l.fixes}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {mode === 'verify' ? (
        <p className="aside">
          EBNF describes the grammar and deliberately does not decide surface
          style. K describes execution through configurations and rewrite rules,
          so typing judgments define admissible programs while contract
          properties state what must be proved. A denotational model can support
          a particular financial analysis; it does not replace the execution
          definition. The control layer is presented with Felleisen&ndash;Hieb
          reduction semantics.
        </p>
      ) : null}

      <hr className="rule" />

      <h3>One transaction, seven representations</h3>
      <p>
        A long-lived agreement is a sequence of bounded one-transition proofs,
        not one circuit that executes an entire lifetime. Walk a swap through
        the pipeline and watch what the value becomes.
      </p>

      <ol className="rail">
        {PIPELINE.map((p, i) => (
          <li key={p.stage}>
            <button
              type="button"
              className={i === stage ? 'rail-stop is-at' : i < stage ? 'rail-stop is-done' : 'rail-stop'}
              onClick={() => setStage(i)}
              aria-current={i === stage ? 'step' : undefined}
            >
              <span className="rail-dot" aria-hidden="true" />
              <span className="rail-name">{p.stage}</span>
            </button>
          </li>
        ))}
      </ol>

      <div className="stage card">
        <h4>{current.stage}</h4>
        <p className="stage-detail">{current.detail}</p>

        <h5>Guarantees at this point</h5>
        <ul className="ladder">
          {ASSURANCE_LAYERS.map((l, i) => {
            const state = i < stage ? 'settled' : i === stage ? 'here' : 'owed';
            return (
              <li key={l.name} className={`rung is-${state}`}>
                <span className="rung-name">{l.name}</span>
                <span className="rung-state mono">{state === 'settled' ? 'fixed earlier' : state === 'here' ? 'fixed here' : 'still owed'}</span>
              </li>
            );
          })}
        </ul>
        <p className="stage-residual">
          Nothing on this page is ticked, green or marked verified. A stage that
          settles one obligation leaves the rest exactly where they were.
        </p>
      </div>

      <hr className="rule" />

      <div className="two-col">
        <div>
          <h3>Why Compact, and not the circuit IR directly</h3>
          <p>{ZKIR_DEFERRAL}</p>
        </div>
        <div>
          <h3>Every datum is classified</h3>
          <ul className="vis">
            {VISIBILITY.map((v) => (
              <li key={v} className="mono">{v}</li>
            ))}
          </ul>
          <p>
            Beyond the classification, every oracle and external effect carries
            a named capability and an assurance boundary. Compact&rsquo;s
            disclosure authorizes a flow past the compiler&rsquo;s privacy
            analysis, and it does not make that flow semantically safe, so
            Moriarty generates one only from an explicit source visibility
            transition.
          </p>
        </div>
      </div>
    </div>
  );
}

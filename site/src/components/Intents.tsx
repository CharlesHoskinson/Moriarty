import { useState } from 'react';
import {
  ADVERSARIAL_RULES,
  AUTHORITY_RULES,
  BRIDGE_TRUST,
  INTENT_ARTIFACTS,
  MULTICHAIN_CHALLENGE,
  MULTICHAIN_CONTRADICTIONS,
  MULTICHAIN_RULES,
  SAFETY_MECHANISMS,
  SIGNING_PROFILES,
} from '../data/intents';
import { useMode } from '../mode';

/**
 * Section eight. Intents, multichain settlement, and the safety that follows.
 *
 * The authority rule is shown by running the naive system rather than by
 * asserting the correct one. A reader who watches net spending stay flat while
 * gross doubles does not need to be told that refunds must not restore
 * allowance.
 */
export function Intents() {
  const { mode } = useMode();

  return (
    <div className="wrap">
      <p className="eyebrow">Intents, settlement and safety</p>
      <h2>What you authorized, and what a refund does to it.</h2>
      <p className="lede">
        {mode === 'build'
          ? 'Five artifacts that most systems collapse into one. Keeping them apart is what lets a solver choose a route without choosing what you agreed to.'
          : 'An intent fixes an outcome and an authority before a route exists. A plan is one chosen route. A receipt records what was checked and what remains owed.'}
      </p>

      <ol className="artifacts">
        {INTENT_ARTIFACTS.map((a) => (
          <li key={a.name}>
            <h4>{a.name}</h4>
            <p className="artifact-is">{a.is}</p>
            <p className="artifact-fixes">{a.fixes}</p>
          </li>
        ))}
      </ol>

      <div className="signing">
        {SIGNING_PROFILES.map((s) => (
          <article key={s.name}>
            <h4>{s.name}</h4>
            <p>{s.means}</p>
            <p className="signing-cost">{s.tradeoff}</p>
          </article>
        ))}
      </div>

      <hr className="rule" />

      <h3>Why authority is counted gross</h3>
      <RefundTrace />

      <h4 className="rules-head">The rules that follow</h4>
      <dl className="auth-rules">
        {AUTHORITY_RULES.map((r) => (
          <div key={r.rule}>
            <dt>{r.rule}</dt>
            <dd>{r.detail}</dd>
          </div>
        ))}
      </dl>

      <hr className="rule" />

      <h3>Multichain, as documented rather than as advertised</h3>
      <div className="scroll-x">
        <table className="contra">
          <thead>
            <tr>
              <th scope="col">The claim</th>
              <th scope="col">What the documentation says</th>
            </tr>
          </thead>
          <tbody>
            {MULTICHAIN_CONTRADICTIONS.map((c) => (
              <tr key={c.claim}>
                <th scope="row">{c.claim}</th>
                <td>{c.documented}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <ol className="mc-rules">
        {MULTICHAIN_RULES.map((r) => (
          <li key={r}>{r}</li>
        ))}
      </ol>

      <p className="challenge">{MULTICHAIN_CHALLENGE}</p>

      <p className="bridge-split">
        Bridges split by trust model:{' '}
        {BRIDGE_TRUST.map((b, i) => (
          <span key={b}>
            {i > 0 ? ' and ' : ''}
            <strong className="mono">{b}</strong>
          </span>
        ))}
        . One fails when a message is forged or a genuine one is rejected. The
        other fails when the party holding the assets does. Recording only
        &ldquo;bridge&rdquo; on the facet tells a reader nothing about which
        failure to plan for.
      </p>

      <hr className="rule" />

      <h3>Four mechanisms, four classes of failure</h3>
      <div className="mechs">
        {SAFETY_MECHANISMS.map((m) => (
          <article key={m.mechanism}>
            <h4>{m.mechanism}</h4>
            <p>{m.detail}</p>
            <p className="mech-kills">
              <span className="mech-label">Closes</span> {m.kills}
            </p>
          </article>
        ))}
      </div>

      <ul className="adv-rules">
        {ADVERSARIAL_RULES.map((r) => (
          <li key={r}>{r}</li>
        ))}
      </ul>
    </div>
  );
}

/**
 * The naive authorization system, run.
 *
 * Net spending stays at 100 across the whole trace while gross reaches 200,
 * and the recipient becomes an unbounded router of the principal's funds.
 */
function RefundTrace() {
  const [step, setStep] = useState(3);
  const STEPS = [
    { act: 'authorize', delta: 0, refund: 0, note: 'cap of 100 on transfers to R' },
    { act: 'send 100 to R', delta: 100, refund: 0, note: 'net 100, gross 100' },
    { act: 'R refunds 100', delta: -100, refund: 100, note: 'net returns to 0' },
    { act: 'send 100 to R', delta: 100, refund: 0, note: 'net 100 again' },
  ];

  let net = 0;
  let gross = 0;
  const rows = STEPS.slice(0, step + 1).map((s) => {
    net += s.delta;
    if (s.delta > 0) gross += s.delta;
    return { ...s, net, gross };
  });
  const last = rows[rows.length - 1]!;

  return (
    <div className="trace card">
      <p className="trace-lede">
        A competent engineer tracks net spending against the cap. Run a refund
        through it.
      </p>
      <div className="scroll-x">
        <table className="trace-table">
          <thead>
            <tr>
              <th scope="col">Step</th>
              <th scope="col">Net spent</th>
              <th scope="col">Gross out</th>
              <th scope="col">Net-tracking system says</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((r, i) => (
              <tr key={i} className={i === rows.length - 1 ? 'is-now' : undefined}>
                <td>{r.act}</td>
                <td className="mono">{r.net}</td>
                <td className="mono">{r.gross}</td>
                <td className={r.net <= 100 ? 'verdict-ok' : 'verdict-no'}>
                  {r.net <= 100 ? 'within cap, permitted' : 'refused'}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="trace-ctl">
        <button type="button" onClick={() => setStep(Math.max(0, step - 1))} disabled={step === 0}>
          Back
        </button>
        <button
          type="button"
          onClick={() => setStep(Math.min(STEPS.length - 1, step + 1))}
          disabled={step === STEPS.length - 1}
        >
          Next
        </button>
        <span className="trace-note">{last.note}</span>
      </div>
      {step === STEPS.length - 1 ? (
        <p className="trace-verdict">
          Net spending never exceeded the cap, and 200 left the account. Repeat
          the refund and the recipient becomes an unbounded router of the
          principal&rsquo;s funds. Counting gross refuses the second send,
          because a refund never restores allowance.
        </p>
      ) : null}
    </div>
  );
}

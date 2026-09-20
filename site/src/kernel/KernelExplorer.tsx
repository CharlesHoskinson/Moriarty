import { useEffect, useReducer, useRef } from 'react';
import {
  CANDIDATES,
  CONDITIONS,
  EVIDENCE,
  OBSERVATIONS,
  SOLVER_KINDS,
  apply,
  availableEvents,
  derive,
  evaluateEvidence,
  fmt,
  initialState,
  type EventRecord,
  type ScenarioEvent,
  type ScenarioState,
} from '../data/kernel-scenarios.mjs';

const RESPONSIBILITY: Record<EventRecord['responsibility'], string> = {
  owner: 'Owner',
  solver: 'Solver',
  moriarty: 'Moriarty',
  kernel: 'Kernel',
  midnight: 'Midnight',
  external: 'External',
};

const PHASE_LABEL: Record<ScenarioState['phase'], string> = {
  uncommitted: 'Uncommitted. A proposal can still be compared or discarded.',
  committed: 'Committed and idle. A permitted stage may be attempted.',
  'in-flight': 'An attempt is in flight against reserved authority.',
  observed: 'Observed. The external result is accounted and its identifier consumed; the fill awaits acceptance against every predicate.',
  complete: 'Complete. Every fill is accepted and the delivery duty is discharged.',
};

/**
 * The scenario explorer. One reducer, one account, one event record. Every
 * button dispatches a real event; nothing here is decorative.
 */
export function KernelExplorer() {
  const [state, dispatch] = useReducer(apply, undefined, () => initialState());
  const d = derive(state);
  const available = availableEvents(state);
  const can = (t: ScenarioEvent['type']) => available.includes(t);
  const evidence = EVIDENCE.find((e) => e.id === state.evidenceId)!;
  const evidenceResult = evaluateEvidence(evidence, state.policy);

  // The concise result is announced once per event, never the whole page.
  const announcement = state.lastRejected
    ? `Refused: ${state.lastRejected.reason}`
    : state.lastNote ?? '';

  const logRef = useRef<HTMLOListElement>(null);
  useEffect(() => {
    const el = logRef.current;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  }, [state.events.length]);

  return (
    <div className="k-explorer" data-phase={state.phase}>
      {/* --- controls ------------------------------------------------- */}
      <div className="k-controls">
        <fieldset className="k-fs">
          <legend>1 · Candidate route</legend>
          <div className="k-seg k-seg-wrap">
            {CANDIDATES.map((c) => (
              <button
                key={c.id}
                type="button"
                aria-pressed={state.candidateId === c.id}
                disabled={state.phase !== 'uncommitted'}
                onClick={() => dispatch({ type: 'select-candidate', candidateId: c.id })}
                data-candidate={c.id}
              >
                {c.label}
              </button>
            ))}
          </div>
          {state.candidateVerdict ? (
            <div className="k-verdict" data-accepted={state.candidateVerdict.accepted}>
              <p className="k-verdict-sum">{CANDIDATES.find((c) => c.id === state.candidateId)?.summary}</p>
              <ul className="k-checks">
                {state.candidateVerdict.checks.map((c) => (
                  <li key={c.id} data-met={c.met}>
                    <span className="k-check-mark" aria-hidden="true">{c.met ? '·' : '×'}</span>
                    <span className="k-check-name">{c.name}</span>
                    <span className="k-check-state">{c.met ? 'holds' : 'fails'}</span>
                    <span className="k-check-detail">{c.detail}</span>
                  </li>
                ))}
              </ul>
            </div>
          ) : (
            <p className="k-hint">Pick a route. Two are compliant. Two fail a named check and never reach the account.</p>
          )}
          <p className="k-hint k-hint-judgments">
            These checks are illustrative JavaScript rules. They stand in for the four
            acceptance judgments the design keeps distinct, and produce none of their
            native proof obligations: <span>contract properties</span> (the recipient,
            cap and goal checks), <span>intent refinement</span> (the committed route
            stays within what was signed), <span>transition validity</span> (each event
            moves the account only by its stated effect) and <span>history
            compliance</span> (each stage extends the consumed-identifier record from
            the fixture&rsquo;s origin).
          </p>
          <div className="k-row">
            <span className="k-row-label" id="solver-kind-h">Proposed by</span>
            <div className="k-seg k-seg-small" role="group" aria-labelledby="solver-kind-h">
              {SOLVER_KINDS.map((k) => (
                <button key={k.id} type="button" aria-pressed={state.solverKind === k.id} onClick={() => dispatch({ type: 'set-solver-kind', kind: k.id })} data-solver={k.id}>
                  {k.label}
                </button>
              ))}
            </div>
            <span className="k-row-note">Label only. Authority and acceptance rules do not read it.</span>
          </div>
        </fieldset>

        <fieldset className="k-fs">
          <legend>2 · Consent and commitment</legend>
          <div className="k-btns">
            <button type="button" className="k-act" disabled={!can('provider-accepts-duties')} onClick={() => dispatch({ type: 'provider-accepts-duties' })} data-event="provider-accepts-duties">
              Provider accepts duties
            </button>
            <button type="button" className="k-act k-act-primary" disabled={!can('commit-candidate')} onClick={() => dispatch({ type: 'commit-candidate' })} data-event="commit-candidate">
              Commit candidate
            </button>
          </div>
          <p className="k-hint">Consent comes first. Commitment binds the route and spends nothing.</p>
        </fieldset>

        <fieldset className="k-fs">
          <legend>3 · Release conditions</legend>
          <div className="k-conds">
            {CONDITIONS.map((c) => (
              <div key={c.id} className="k-row k-row-tight">
                <span className="k-row-label" id={`cond-${c.id}`}>{c.label}</span>
                <div className="k-seg k-seg-small" role="group" aria-labelledby={`cond-${c.id}`}>
                  <button type="button" aria-pressed={state.conditions[c.id]} onClick={() => dispatch({ type: 'set-condition', condition: c.id, present: true })} data-condition={c.id} data-present="true">
                    Present
                  </button>
                  <button type="button" aria-pressed={!state.conditions[c.id]} onClick={() => dispatch({ type: 'set-condition', condition: c.id, present: false })} data-condition={c.id} data-present="false">
                    Withheld
                  </button>
                </div>
              </div>
            ))}
          </div>
          <span className="k-row-label k-row-label-block" id="doc-evidence-h">Document attestation</span>
          <div className="k-seg k-seg-wrap" role="group" aria-labelledby="doc-evidence-h">
            {EVIDENCE.map((e) => (
              <button key={e.id} type="button" aria-pressed={state.evidenceId === e.id} onClick={() => dispatch({ type: 'present-evidence', evidenceId: e.id })} data-evidence={e.id}>
                {e.label}
              </button>
            ))}
          </div>
          <p className="k-evidence-result" data-status={evidenceResult.status}>
            <span className="k-status-word">{evidenceResult.status}</span> {evidenceResult.reason}
          </p>
          <p className="k-hint">All three together release a fill. Missing evidence is unknown, not false; unsupported evidence establishes nothing either way.</p>
        </fieldset>

        <fieldset className="k-fs">
          <legend>4 · Next event</legend>
          <div className="k-btns">
            <button type="button" className="k-act" disabled={!can('finalize-fill')} onClick={() => dispatch({ type: 'finalize-fill' })} data-event="finalize-fill">
              Finalize a fill
            </button>
            <button type="button" className="k-act" disabled={!can('reserve-fill')} onClick={() => dispatch({ type: 'reserve-fill' })} data-event="reserve-fill">
              Reserve and submit a fill
            </button>
            <button type="button" className="k-act" disabled={!can('timeout')} onClick={() => dispatch({ type: 'timeout' })} data-event="timeout">
              Deadline passes
            </button>
          </div>
          <div className="k-row">
            <span className="k-row-label" id="observe-h">Then observe</span>
            <div className="k-btns" role="group" aria-labelledby="observe-h">
              {OBSERVATIONS.map((o) => (
                <button key={o.id} type="button" className="k-act" disabled={!can('observe')} onClick={() => dispatch({ type: 'observe', outcome: o.id })} data-observe={o.id}>
                  {o.label}
                </button>
              ))}
            </div>
          </div>
          <div className="k-btns">
            <button type="button" className="k-act k-act-primary" disabled={!can('accept-observed-fill')} onClick={() => dispatch({ type: 'accept-observed-fill' })} data-event="accept-observed-fill">
              Accept the observed fill
            </button>
          </div>
          <p className="k-hint">The first fill is finalized locally; the second is reserved and submitted externally. An observed external result is accounted when it arrives. Acceptance against every predicate is a separate step, taken once.</p>
        </fieldset>

        <fieldset className="k-fs k-fs-adverse">
          <legend>5 · Try a conflicting event</legend>
          <div className="k-btns">
            <button type="button" className="k-act" disabled={!can('second-solver-reserve')} onClick={() => dispatch({ type: 'second-solver-reserve' })} data-event="second-solver-reserve">
              Second solver reserves
            </button>
            <button type="button" className="k-act" disabled={!can('replay-fill')} onClick={() => dispatch({ type: 'replay-fill' })} data-event="replay-fill">
              Replay a consumed fill
            </button>
            <button type="button" className="k-act" disabled={!can('duplicate-observation')} onClick={() => dispatch({ type: 'duplicate-observation' })} data-event="duplicate-observation">
              Duplicate terminal result
            </button>
            <button type="button" className="k-act" disabled={!can('premature-refund')} onClick={() => dispatch({ type: 'premature-refund' })} data-event="premature-refund">
              Refund on timeout
            </button>
          </div>
          <p className="k-hint">Each is refused for a stated reason and leaves the account untouched. Foreign signer compromise is a separate risk, explored in the evidence inspector.</p>
        </fieldset>

        <div className="k-reset-row">
          <button type="button" className="k-act k-act-reset" onClick={() => dispatch({ type: 'reset' })} data-event="reset">
            Reset to a new illustration
          </button>
          <span className="k-row-note">Starts a fresh uncommitted fixture. It is not a rollback of anything modeled.</span>
        </div>
      </div>

      {/* --- account and record ------------------------------------------- */}
      <div className="k-ledger">
        <p className="k-announce" role="status" aria-live="polite" aria-atomic="true" data-kind={state.lastRejected ? 'refused' : 'note'}>
          {announcement}
        </p>

        <div className="k-phase" data-testid="phase">{PHASE_LABEL[state.phase]}</div>

        <table className="k-account">
          <caption>Financial account, in A and B</caption>
          <thead className="sr-only"><tr><th scope="col">Quantity</th><th scope="col">Value</th></tr></thead>
          <tbody>
            <Row k="Actual gross debit" v={`${fmt(state.account.grossDebit)} A`} id="grossDebit" strong />
            <Row k="of which fees" v={`${fmt(state.account.fees)} A`} id="fees" />
            <Row k="Active reservation" v={`${fmt(state.account.reserved)} A`} id="reserved" />
            <Row k="Spent plus reserved" v={`${fmt(d.exposure)} A of ${fmt(state.policy.grossCap)} A`} id="exposure" />
            <Row k="Fee capacity remaining" v={d.feeEncumbered ? `${fmt(d.feeFree)} A free · ${fmt(d.feeEncumbered)} A encumbered by the pending attempt` : `${fmt(d.feeRemaining)} A`} id="feeRemaining" />
            <Row k="Confirmed receipt" v={`${fmt(state.account.confirmedReceipt)} B of ${fmt(state.policy.minReceive)} B`} id="confirmedReceipt" strong />
            <Row k="Last confirmed escrow balance" v={`${fmt(state.account.custody)} A`} id="custody" />
          </tbody>
        </table>
        <p className="k-hint k-hint-custody" data-testid="custody-note">
          The escrow row is the last confirmed, accounted balance, not a live
          reading. A reservation is not custody.
          {state.phase === 'in-flight'
            ? ` While the submitted attempt is unresolved, the current location of the ${fmt(state.account.reserved)} A it covers is unknown: the destination may already have executed.`
            : ''}
        </p>

        <h3 className="k-sub">Accepted duties</h3>
        {state.duties.length === 0 ? (
          <p className="k-hint">None. No one owes anything until the provider consents.</p>
        ) : (
          <ul className="k-duties">
            {state.duties.map((x) => (
              <li key={x.id} data-duty={x.id} data-status={x.status}>
                <span className="k-duty-bearer">{x.bearer}</span>
                <span className="k-duty-text">{x.duty}</span>
                <span className="k-duty-discharge">Discharged by: {x.discharge}</span>
                <span className="k-duty-status">{x.status}</span>
              </li>
            ))}
          </ul>
        )}

        <h3 className="k-sub">Event record</h3>
        {state.events.length === 0 ? (
          <p className="k-hint">Empty. A fresh illustration has no history.</p>
        ) : (
          <ol className="k-log" ref={logRef}>
            {state.events.map((e) => (
              <li key={e.n} data-event-type={e.type} data-resp={e.responsibility}>
                <div className="k-log-head">
                  <span className="k-log-n">{e.n}</span>
                  <span className="k-log-resp">{RESPONSIBILITY[e.responsibility]}</span>
                  <span className="k-log-title">{e.title}</span>
                </div>
                <p className="k-log-meaning">{e.meaning}</p>
                <p className="k-log-acct">
                  <Delta d={e.delta} />
                  <span className="k-log-after">
                    after: {fmt(e.account.grossDebit)} A gross · {fmt(e.account.fees)} A fees · {fmt(e.account.reserved)} A reserved · {fmt(e.account.confirmedReceipt)} B confirmed
                  </span>
                </p>
              </li>
            ))}
          </ol>
        )}
        {state.consumed.length ? (
          <p className="k-consumed">Consumed identifiers: {state.consumed.map((c) => <code key={c}>{c}</code>)}</p>
        ) : null}
      </div>
    </div>
  );
}

function Row({ k, v, id, strong }: { k: string; v: string; id: string; strong?: boolean }) {
  return (
    <tr data-account-row={id} className={strong ? 'is-strong' : undefined}>
      <th scope="row">{k}</th>
      <td className="num">{v}</td>
    </tr>
  );
}

function Delta({ d }: { d: EventRecord['delta'] }) {
  if (!d) return <span className="k-log-delta k-log-delta-none">no account change</span>;
  const parts: string[] = [];
  const sign = (n: number) => (n > 0 ? `+${fmt(n)}` : fmt(n));
  if (d.grossDebit) parts.push(`gross ${sign(d.grossDebit)} A`);
  if (d.fees) parts.push(`fees ${sign(d.fees)} A`);
  if (d.reserved) parts.push(`reserved ${sign(d.reserved)} A`);
  if (d.custody) parts.push(`custody ${sign(d.custody)} A`);
  if (d.confirmedReceipt) parts.push(`confirmed ${sign(d.confirmedReceipt)} B`);
  return <span className="k-log-delta">{parts.join(' · ')}</span>;
}

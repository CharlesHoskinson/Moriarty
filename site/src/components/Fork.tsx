import { useState } from 'react';
import type { ActionTarget } from '../data/actionTargets';

/**
 * The Fork.
 *
 * One pre-state, one action, two post-states. The left is what a plausible
 * balance-based implementation writes. The right is what the distinguishing
 * test requires. The line where they part is the only coloured line.
 *
 * What the plausible model cannot hold is set in grey rather than red. This is
 * a precision argument, not a scare page: the wrong answer is less defined,
 * not more dangerous.
 *
 * Two targets compute rather than quote, because the repository supplies the
 * arithmetic for them. The rest state their divergence structurally.
 */
export function Fork({ target }: { target: ActionTarget }) {
  const live = LIVE[target.id];
  return (
    <div className="fork">
      <div className="fork-head">
        <span className="fork-id mono">{target.id}</span>
        <h4>{target.action}</h4>
      </div>

      <dl className="fork-terms">
        <dt>Requirement</dt>
        <dd>{target.requirement}</dd>
        <dt>Distinguishing test</dt>
        <dd className="fork-test">{target.distinguishingTest}</dd>
      </dl>

      {live ? <LiveFork target={target} /> : <StaticFork target={target} />}
    </div>
  );
}

/* -- the two computed cases ------------------------------------------------ */

const LIVE: Record<string, true> = { DA01: true, DA06: true };

function LiveFork({ target }: { target: ActionTarget }) {
  return target.id === 'DA01' ? <SwapFork /> : <RepayFork />;
}

/** Constant-product swap. Reserves, fee and formula are the repository's own. */
function SwapFork() {
  const RESERVE_A = 1_000_000n;
  const RESERVE_B = 2_000_000n;
  const AMOUNT_IN = 10_000n;
  const FEE_N = 997n;
  const FEE_D = 1_000n;

  const [minOut, setMinOut] = useState('19743');

  const effective = AMOUNT_IN * FEE_N;                       // 9,970,000
  const numerator = effective * RESERVE_B;                   // 19,940,000,000,000
  const denominator = RESERVE_A * FEE_D + effective;         // 1,009,970,000
  const output = numerator / denominator;                    // 19,743
  const remainder = numerator % denominator;                 // 162,290,000

  const requested = (() => {
    try { return BigInt(minOut || '0'); } catch { return null; }
  })();
  const rejects = requested !== null && requested > output;

  const f = (n: bigint) => n.toLocaleString('en-US');

  return (
    <>
      <div className="fork-input">
        <label htmlFor="min-out">
          <code>min_out</code>
        </label>
        <input
          id="min-out"
          className="mono"
          value={minOut}
          inputMode="numeric"
          onChange={(e) => setMinOut(e.target.value.replace(/[^\d]/g, ''))}
          aria-describedby="min-out-help"
        />
        <span id="min-out-help" className="fork-hint">
          Ask for {f(output + 1n)} and the guard fails by name.
        </span>
      </div>

      <div className="fork-cols">
        <Column kind="pre" title="Pre-state">
          <Row k="reserve_a" v={`${f(RESERVE_A)} A`} />
          <Row k="reserve_b" v={`${f(RESERVE_B)} B`} />
          <Row k="amount_in" v={`${f(AMOUNT_IN)} A`} />
          <Row k="fee" v={`${f(FEE_N)}/${f(FEE_D)}`} />
        </Column>

        <Column kind="plausible" title="Plausible implementation">
          <Row k="output" v={`${f(output)} or ${f(output + 1n)}`} undef />
          <Row k="rounding direction" v="unstated" undef />
          <Row k="remainder" v="unaccounted" undef />
          <Row k="reserve safety" v="unchecked" undef />
        </Column>

        <Column kind="required" title="Required behaviour">
          <Row k="effective_input" v={f(effective)} />
          <Row k="numerator" v={f(numerator)} />
          <Row k="denominator" v={f(denominator)} />
          <Row k="floor_div" v={f(output)} strong />
          <Row k="remainder" v={`${f(remainder)} stays in reserve_b`} />
          {rejects ? (
            <p className="fork-reject">
              guard <code>arg.min_out &lt;= output_calculated</code>
              <br />
              {f(requested)} &lt;= {f(output)} fails
              <br />
              <strong>&ldquo;minimum output not met&rdquo;</strong>
            </p>
          ) : (
            <>
              <Row k="reserve_a after" v={f(RESERVE_A + AMOUNT_IN)} />
              <Row k="reserve_b after" v={f(RESERVE_B - output)} />
            </>
          )}
        </Column>
      </div>
      <p className="fork-note">
        The only division sits on the last line, so that is the only place a
        rounding decision exists. Rounding up would need a numerator the pool
        does not have, and it would pay a trader who chooses their own input a
        fraction on every swap.
      </p>
    </>
  );
}

/** Partial repayment. One payment against one obligation, under each rule. */
function RepayFork() {
  const PRINCIPAL = 100n;
  const INTEREST = 10n;
  const [payment, setPayment] = useState('7');

  const p = (() => {
    try { return BigInt(payment || '0'); } catch { return 0n; }
  })();
  const total = PRINCIPAL + INTEREST;

  const accrualFirst = { i: p > INTEREST ? 0n : INTEREST - p, pr: p > INTEREST ? PRINCIPAL - (p - INTEREST) : PRINCIPAL };
  const principalFirst = { i: p > PRINCIPAL ? INTEREST - (p - PRINCIPAL) : INTEREST, pr: p > PRINCIPAL ? 0n : PRINCIPAL - p };
  const proRataPr = (p * PRINCIPAL) / total;
  const proRata = { i: INTEREST - (p - proRataPr), pr: PRINCIPAL - proRataPr };

  const f = (n: bigint) => n.toLocaleString('en-US');

  return (
    <>
      <div className="fork-input">
        <label htmlFor="payment">payment</label>
        <input
          id="payment"
          className="mono"
          value={payment}
          inputMode="numeric"
          onChange={(e) => setPayment(e.target.value.replace(/[^\d]/g, ''))}
        />
        <span className="fork-hint">
          against principal {f(PRINCIPAL)} and interest {f(INTEREST)}
        </span>
      </div>

      <div className="fork-cols">
        <Column kind="pre" title="Pre-state">
          <Row k="principal" v={`Debt ${f(PRINCIPAL)}`} />
          <Row k="interest" v={`Debt ${f(INTEREST)}`} />
          <Row k="payment" v={f(p)} />
        </Column>

        <Column kind="plausible" title="Plausible implementation">
          <Row k="balance" v={`${f(total)} − ${f(p)} = ${f(total - p)}`} />
          <Row k="allocation" v="undefined" undef />
          <Row k="principal after" v="not distinguished" undef />
          <Row k="obligation discharged" v="not identified" undef />
        </Column>

        <Column kind="required" title="Required behaviour">
          <Row k="AccrualFirst" v={`interest ${f(accrualFirst.i)} · principal ${f(accrualFirst.pr)}`} />
          <Row k="PrincipalFirst" v={`interest ${f(principalFirst.i)} · principal ${f(principalFirst.pr)}`} />
          <Row k="ProRata" v={`interest ${f(proRata.i)} · principal ${f(proRata.pr)}`} />
          <p className="fork-note-inline">
            Three correct systems, three different splits. A balance
            subtraction throws the split away and cannot say which obligation
            was discharged.
          </p>
        </Column>
      </div>
      <p className="fork-note">
        <code>ensures post.principal == pre.principal</code> is the successor
        profile stating the invariant in the source. Paying interest may not
        touch principal, and the allocation rule is declared rather than
        inferred from the order of subtractions.
      </p>
    </>
  );
}

/* -- the structural case --------------------------------------------------- */

function StaticFork({ target }: { target: ActionTarget }) {
  return (
    <div className="fork-cols fork-cols-2">
      <Column kind="plausible" title="What a balance-based model writes">
        <p className="fork-undef">{plausibleFor(target)}</p>
      </Column>
      <Column kind="required" title="What the test requires">
        <p>{target.distinguishingTest}</p>
      </Column>
    </div>
  );
}

/**
 * The wrong reading, derived from the target's own test sentence. Nothing is
 * invented: each names the state a balance-based model has no way to hold.
 */
function plausibleFor(t: ActionTarget): string {
  const known: Record<string, string> = {
    DA02: 'share entitlement: proportional by assumption · donation and zero-supply boundaries: not represented',
    DA03: 'position identity: unbounded · fee allocation across ranges: not traversed',
    DA04: 'failed withdrawal: erases the claim rather than rejecting',
    DA05: 'collateral release: permitted whenever a balance allows it',
    DA07: 'partial liquidation: not represented · residual debt after seizure: dropped',
    DA08: 'repayment within the transaction: assumed rather than required',
    DA09: 'old debt at refinance: superseded without discharge',
    DA10: 'released collateral: unlinked from the debt rule',
    DA11: 'funding sign: unmodelled · settlement convention: ambiguous',
    DA12: 'expiry: sweeps every claim, including one already exercised',
    DA13: 'merge: adds balances, so a winning claim can be duplicated',
    DA14: 'entitlement: read off the nominal balance, which does not move',
    DA15: 'exit request: recorded as delivery · pending state: not represented',
    DA16: 'burn on this chain: treated as settlement off it',
    DA17: 'rounding: one direction everywhere, so one method leaks',
    DA18: 'residual request amount: not retained, so a claim can be replayed',
    DA19: 'unwind: reports the position closed while its debt persists',
    DA20: 'observation: accepted without provenance, freshness or domain',
    DA21: 'authority: tracked net, so a refund restores spending capacity',
    DA22: 'parameter change: applied without regard to existing positions',
    DA23: 'delivery: assumed once · refund duty: not represented',
    DA24: 'join: sums the branches, so a partitioned resource can be duplicated',
  };
  return known[t.id] ?? 'state the test names: not represented';
}

/* -- primitives ------------------------------------------------------------ */

function Column({
  kind,
  title,
  children,
}: {
  kind: 'pre' | 'plausible' | 'required';
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className={`fork-col fork-col-${kind}`}>
      <h5>{title}</h5>
      {children}
    </div>
  );
}

function Row({ k, v, undef, strong }: { k: string; v: string; undef?: boolean; strong?: boolean }) {
  return (
    <div className={'fork-row' + (undef ? ' is-undef' : '') + (strong ? ' is-strong' : '')}>
      <span className="fork-k">{k}</span>
      <span className="fork-v mono">{v}</span>
    </div>
  );
}

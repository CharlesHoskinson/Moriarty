import { useMode } from '../mode';

/**
 * Section one. The claim, then the problem it answers, then where the
 * financial behavior comes from.
 */
export function Opening() {
  const { mode } = useMode();

  return (
    <div className="wrap opening">
      <p className="eyebrow">A language for bounded financial contracts</p>
      <h1>
        Financial meaning that survives
        <br />
        compilation, proof and settlement.
      </h1>

      {mode === 'build' ? (
        <p className="lede">
          A repayment reduces a balance. Which debt did it discharge, to which
          creditor, and what is still owed? Most contract languages cannot
          answer from the source, because the source never said. Moriarty makes
          the answer part of the type system and the operational semantics
          rather than part of the audit.
        </p>
      ) : (
        <p className="lede">
          Moriarty separates what a program looks like from what it means:
          token rules, an EBNF grammar, typing judgments, an executable
          operational semantics in K, and correctness claims stated over those
          semantics. Each layer is specified in the notation that suits it, and
          the correspondences between them are obligations rather than
          assumptions.
        </p>
      )}

      <div className="opening-grid">
        <div>
          <h3>What it is</h3>
          <p>
            Developers describe financial state, permitted actions, payment
            obligations and authorization rules. Those descriptions compile to
            Compact, and every transaction carries a proof that its execution
            and the contract history it extends satisfy the agreement.
          </p>
          <p>
            Moriarty is a new bounded financial-agreement language. It is not a
            renamed Marlowe and not a general-purpose Compact dialect.
          </p>
        </div>
        <div>
          <h3>What Compact leaves open</h3>
          <p>
            Compact supports Midnight contracts and zero-knowledge circuits. It
            gives you circuits and state. It does not decide which asset an
            amount denotes, how interest rounds, when a payment becomes due,
            what a participant authorized, or which obligations survive a
            transaction.
          </p>
          <p>
            Those questions decide whether a repayment discharged the right debt
            to the right creditor without erasing the remaining principal.
          </p>
        </div>
      </div>

      <hr className="rule" />

      <h3 className="ancestry-head">Where the behavior comes from</h3>
      <div className="ancestry">
        {ANCESTRY.map((a) => (
          <article key={a.name}>
            <h4>{a.name}</h4>
            <p>{a.what}</p>
            <p className="ancestry-take">{a.take}</p>
          </article>
        ))}
      </div>
    </div>
  );
}

const ANCESTRY = [
  {
    name: 'ACTUS',
    what: 'The Algorithmic Contract Types Unified Standards describe financial contracts through rules for events, state transitions and cash flows.',
    take: 'Moriarty must reproduce the financial behavior, dates and rounding included. Naming a contract type is not enough.',
  },
  {
    name: 'The DeFi corpus',
    what: 'A catalogue of decentralized-finance behaviors drawn from protocol sources: swaps, liquidity, lending and composition.',
    take: 'It supplies implementation and conformance requirements. Applications do not connect to it as a runtime service.',
  },
  {
    name: 'Marlowe',
    what: 'A financial-contract language built so that contract behavior can be analyzed before it executes.',
    take: 'Moriarty adopts that goal and builds its own authoring, proof and settlement architecture for Midnight.',
  },
] as const;

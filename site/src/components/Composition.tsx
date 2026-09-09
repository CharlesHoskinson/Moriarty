import { COMPOSITION_OPERATORS, COMPOSITION_RESULT as R } from '../data/actionTargets';
import { useMode } from '../mode';

/**
 * Section four. The measured result that motivates the work.
 *
 * The field is drawn as pairs rather than as a bar chart, because the claim is
 * about where failures land and not about a height. Marks are placed by count.
 * They are not the identity of any particular pair.
 */
export function Composition() {
  const { mode } = useMode();
  const ratio = (R.crossCategoryRate / R.withinCategoryRate).toFixed(2);

  return (
    <div className="wrap">
      <p className="eyebrow">Composition</p>
      <h2>Failures cluster where categories meet.</h2>
      <p className="lede">
        {mode === 'build'
          ? 'Protocols compose. When a pair fails to compose cleanly, the failure lands somewhere, and it is worth knowing where.'
          : 'A construction pair composes cleanly when both members combine without violating either one’s stated obligations. The audit sorted every eligible pair by whether both members sat in the same legacy category.'}
      </p>

      <div className="comp">
        <Field
          label="Within a category"
          pairs={R.withinCategoryPairs}
          failures={R.withinCategoryFailures}
          rate={R.withinCategoryRate}
        />
        <Field
          label="Across categories"
          pairs={R.crossCategoryPairs}
          failures={R.crossCategoryFailures}
          rate={R.crossCategoryRate}
          lit
        />
      </div>

      <div className="comp-read">
        <p>
          Of <Fig n={R.eligiblePairs} /> eligible pairs, <Fig n={R.clean} /> compose
          cleanly. The rest fail, and <Fig n={R.crossCategoryFailures} /> of those{' '}
          <Fig n={R.failures} /> failures cross a category boundary. Crossing one
          is <strong>{ratio} times</strong> as likely to fail.
        </p>
        <p className="comp-why">
          A pair inside one category shares its facet assumptions. Custody means
          the same thing on both sides, settlement finality means the same
          thing, and an authority granted on one side is read the same way on
          the other. Cross a boundary and each side carries assumptions the
          other never checks. That is what a type system and an operational
          semantics can police, and it is why the language is organized by
          category rather than by product.
        </p>
        <p className="comp-caveat">
          An earlier informal claim of sixty times was checked against the
          corpus and is wrong. Marks are placed by count, not by pair identity.
        </p>
      </div>

      <hr className="rule" />

      <h3>The composition operators</h3>
      <p>
        DA24 names them. Each carries its own authority rules, its own duty
        propagation, its own conflict semantics and its own behavior when work
        fans back in.
      </p>
      <ol className="ops">
        {COMPOSITION_OPERATORS.map((op) => (
          <li key={op}>
            <span className="mono">{op}</span>
          </li>
        ))}
      </ol>
      <p className="ops-rule">
        Split partitions work and claims. Join cannot duplicate resource.
      </p>
    </div>
  );
}

/** One field of pairs. Failures are inked; clean pairs are the ground. */
function Field({
  label,
  pairs,
  failures,
  rate,
  lit,
}: {
  label: string;
  pairs: number;
  failures: number;
  rate: number;
  lit?: boolean;
}) {
  const dots = Array.from({ length: pairs }, (_, i) => i < failures);
  return (
    <figure className={'field' + (lit ? ' is-lit' : '')}>
      <figcaption>
        <span className="field-label">{label}</span>
        <span className="field-rate mono">{(rate * 100).toFixed(2)}%</span>
      </figcaption>
      <div className="field-dots" role="img" aria-label={`${failures} of ${pairs} pairs failed`}>
        {dots.map((failed, i) => (
          <span key={i} className={failed ? 'dot is-fail' : 'dot'} />
        ))}
      </div>
      <p className="field-legend">
        <Fig n={failures} /> failed of <Fig n={pairs} />
      </p>
    </figure>
  );
}

const Fig = ({ n }: { n: number }) => <span className="mono">{n.toLocaleString('en-US')}</span>;

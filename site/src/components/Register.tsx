import { useMemo } from 'react';
import { FACETS, FAMILIES, type FamilyId } from '../data/categories';
import { ACTION_TARGETS, targetsForFamily } from '../data/actionTargets';

/**
 * The Register.
 *
 * Family, facet and action target on one surface. Columns are the categories
 * and carry the tabs; rows are the facets. Column width is proportional to how
 * many targets a family owns, so the surface shows its own extent without
 * reporting it.
 *
 * Reading marks are neutral ink. They record what the reader has opened, never
 * what has been established, and nothing here is ever counted back at them.
 */
export function Register({
  selected,
  onSelect,
  lens,
  onLens,
  read,
}: {
  selected: FamilyId;
  onSelect: (id: FamilyId) => void;
  lens: string | null;
  onLens: (facet: string | null) => void;
  read: ReadonlySet<string>;
}) {
  const widths = useMemo(() => {
    const counts = FAMILIES.map((f) => Math.max(targetsForFamily(f.id).length, 1));
    const total = counts.reduce((a, b) => a + b, 0);
    return counts.map((c) => `${(c / total) * 100}%`);
  }, []);

  return (
    <div className="register scroll-x">
      <table>
        <caption className="sr-only">
          DeFi categories against the mandatory facets, with the action targets each category owns
        </caption>
        <thead>
          <tr>
            <th scope="col" className="reg-corner">
              <span className="reg-corner-x">Targets</span>
              <span className="reg-corner-y">Facets</span>
            </th>
            {FAMILIES.map((f, i) => {
              const targets = targetsForFamily(f.id);
              const active = f.id === selected;
              return (
                <th
                  key={f.id}
                  scope="col"
                  style={{ width: widths[i] }}
                  className={active ? 'reg-col reg-col-active' : 'reg-col'}
                >
                  <button type="button" onClick={() => onSelect(f.id)} aria-pressed={active}>
                    <span className="reg-id">{f.id === 'X' ? '⊥' : f.id}</span>
                    <span className="reg-name">{f.tab}</span>
                  </button>
                  <ul className="reg-chips">
                    {targets.map((t) => (
                      <li
                        key={t.id}
                        className={
                          'reg-chip' +
                          (read.has(t.id) ? ' is-read' : '') +
                          (t.families.length > 1 ? ' is-shared' : '')
                        }
                        title={t.families.length > 1 ? `${t.id} — shared with another family` : t.id}
                      >
                        {t.id}
                      </li>
                    ))}
                  </ul>
                </th>
              );
            })}
          </tr>
        </thead>
        <tbody>
          {FACETS.map((facet) => {
            const lit = lens === facet;
            return (
              <tr key={facet} className={lit ? 'reg-row reg-row-lit' : 'reg-row'}>
                <th scope="row">
                  <button
                    type="button"
                    onClick={() => onLens(lit ? null : facet)}
                    aria-pressed={lit}
                    title={`Compare ${facet} across every category`}
                  >
                    {facet}
                  </button>
                </th>
                {FAMILIES.map((f) => (
                  <td
                    key={f.id}
                    className={f.id === selected ? 'is-col-active' : undefined}
                    onClick={() => onSelect(f.id)}
                  >
                    {f.facets[facet]}
                  </td>
                ))}
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

/** DA12 sits in two families, so the surface must not read it as two targets. */
export const SHARED_TARGETS = ACTION_TARGETS.filter((t) => t.families.length > 1).map((t) => t.id);

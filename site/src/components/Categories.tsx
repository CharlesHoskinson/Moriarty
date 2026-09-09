import { useCallback, useState } from 'react';
import { FACETS, FAMILIES, type FamilyId } from '../data/categories';
import { targetsForFamily } from '../data/actionTargets';
import { useMode } from '../mode';
import { Register } from './Register';
import { Fork } from './Fork';

/**
 * Section three. The category tabs.
 *
 * The Register carries the tabs as its own columns, so choosing a category and
 * seeing the whole surface are the same gesture. Beneath it every tab renders
 * the same blocks in the same order, which is what makes the section read as
 * one instrument rather than a set of assembled pages.
 */
export function Categories() {
  const { mode } = useMode();
  const [selected, setSelected] = useState<FamilyId>('F1');
  const [lens, setLens] = useState<string | null>(null);
  const [read, setRead] = useState<ReadonlySet<string>>(new Set());
  const [open, setOpen] = useState<string | null>(null);

  const family = FAMILIES.find((f) => f.id === selected) ?? FAMILIES[0]!;
  const targets = targetsForFamily(selected);

  const openTarget = useCallback((id: string) => {
    setOpen((cur) => (cur === id ? null : id));
    setRead((cur) => (cur.has(id) ? cur : new Set(cur).add(id)));
  }, []);

  return (
    <div className="wrap">
      <p className="eyebrow">The category tabs</p>
      <h2>Every category, to the same depth.</h2>
      <p className="lede">
        {mode === 'build'
          ? 'One tab per category. Each says what the category is, where it sits on the facets, which actions the language must execute and the bug that tells a correct implementation from a plausible one.'
          : 'The families are the human-facing map. The action targets are the conformance surface, and each carries the case that separates a correct implementation from a plausible-looking wrong one.'}
      </p>

      <Register
        selected={selected}
        onSelect={setSelected}
        lens={lens}
        onLens={setLens}
        read={read}
      />

      {lens ? <FacetLens facet={lens} onClose={() => setLens(null)} /> : null}

      <article className="tab" aria-live="polite">
        <header className="tab-head">
          <span className="tab-id mono">{family.id === 'X' ? '⊥' : family.id}</span>
          <h3>{family.name}</h3>
        </header>

        <Block label="What this category is">
          <p>{family.fn}</p>
          <p className="tab-boundary">
            <strong>Boundary.</strong> {family.boundary}
          </p>
        </Block>

        <Block label="Facet profile">
          <dl className="facets">
            {FACETS.map((f) => (
              <div key={f} className={lens === f ? 'facet is-lit' : 'facet'}>
                <dt>{f}</dt>
                <dd>{family.facets[f]}</dd>
              </div>
            ))}
          </dl>
        </Block>

        <Block label="Action targets">
          <ul className="targets">
            {targets.map((t) => {
              const isOpen = open === t.id;
              return (
                <li key={t.id}>
                  <button
                    type="button"
                    className={'target-row' + (isOpen ? ' is-open' : '')}
                    onClick={() => openTarget(t.id)}
                    aria-expanded={isOpen}
                  >
                    <span className="target-id mono">{t.id}</span>
                    <span className="target-action">{t.action}</span>
                    {t.families.length > 1 ? <span className="target-shared">shared</span> : null}
                    <span className="target-test">{t.distinguishingTest}</span>
                  </button>
                  {isOpen ? <Fork target={t} /> : null}
                </li>
              );
            })}
          </ul>
        </Block>

        <Block label="What goes wrong">
          <p>{family.goesWrong}</p>
        </Block>

        <Block label="What Moriarty does">
          <p>{family.response}</p>
        </Block>

        <Block label="Reference sources">
          <ul className="sources">
            {family.sources.map((s) => (
              <li key={s}>{s}</li>
            ))}
          </ul>
        </Block>
      </article>
    </div>
  );
}

/** The second axis. One facet lifted across every family at once. */
function FacetLens({ facet, onClose }: { facet: string; onClose: () => void }) {
  return (
    <aside className="lens">
      <div className="lens-head">
        <h3>
          <span className="lens-label">Facet</span> {facet}
        </h3>
        <button type="button" onClick={onClose}>
          Close
        </button>
      </div>
      <p className="lens-note">
        The same facet across every category. Reading one row is how the model
        shows itself to be a classification rather than a list of products.
      </p>
      <dl className="lens-grid">
        {FAMILIES.map((f) => (
          <div key={f.id}>
            <dt>
              <span className="mono">{f.id === 'X' ? '⊥' : f.id}</span> {f.tab}
            </dt>
            <dd>{f.facets[facet as keyof typeof f.facets]}</dd>
          </div>
        ))}
      </dl>
    </aside>
  );
}

function Block({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <section className="tab-block">
      <h4>{label}</h4>
      <div>{children}</div>
    </section>
  );
}

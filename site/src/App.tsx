import { useCallback, useEffect, useMemo, useState } from 'react';
import { ModeContext, MODES, readStoredMode, storeMode, type Mode } from './mode';
import { SECTIONS } from './sections';

/**
 * The single page. One section spine, two reader modes, no routing beyond
 * in-page anchors.
 */
// The shared stylesheet hides the section jump below this width, so the
// Tutorials entry is rendered on its own there.
const JUMP_HIDDEN = '(max-width: 1180px)';

export function App() {
  const [compact, setCompact] = useState(() => window.matchMedia(JUMP_HIDDEN).matches);
  useEffect(() => {
    const mq = window.matchMedia(JUMP_HIDDEN);
    const on = () => setCompact(mq.matches);
    mq.addEventListener('change', on);
    return () => mq.removeEventListener('change', on);
  }, []);
  const [mode, setModeState] = useState<Mode>('build');

  // Read the stored preference after mount so the first server-free render and
  // the hydrated one agree.
  useEffect(() => setModeState(readStoredMode()), []);

  const setMode = useCallback((next: Mode) => {
    setModeState(next);
    storeMode(next);
  }, []);

  const value = useMemo(() => ({ mode, setMode }), [mode, setMode]);

  return (
    <ModeContext value={value}>
      <a className="skip" href="#main">
        Skip to content
      </a>
      <header className="masthead">
        <span className="wordmark">Moriarty</span>
        <span className="tagline">bounded financial contracts</span>
        <nav className="jump" aria-label="Sections">
          {SECTIONS.map((s) => (
            <a key={s.id} href={`#${s.id}`} title={s.title}>
              {s.nav}
            </a>
          ))}
          <a href="tutorial.html" title="Tutorials: write, check and locally prepare a Moriarty agreement">Tutorials</a>
          <a href="kernel.html" title="The Federated DeFi Kernel: an interactive illustration">Kernel</a>
          <a href="docs/requirements.html">Docs</a>
        </nav>
        {compact ? (
          <a
            href="tutorial.html"
            style={{ marginLeft: 'auto', color: 'var(--ink)', fontSize: '0.9rem', borderBottom: '1px solid var(--accent)', textDecoration: 'none', whiteSpace: 'nowrap' }}
          >
            Tutorials
          </a>
        ) : null}
        <ModeSwitch mode={mode} setMode={setMode} />
      </header>
      <main id="main">
        {SECTIONS.map(({ id, title, Component }) => (
          <section key={id} id={id} aria-label={title}>
            <Component />
          </section>
        ))}
      </main>
    </ModeContext>
  );
}

function ModeSwitch({ mode, setMode }: { mode: Mode; setMode: (m: Mode) => void }) {
  return (
    <div className="mode-switch" role="group" aria-label="Reader mode">
      {(Object.keys(MODES) as Mode[]).map((m) => (
        <button
          key={m}
          type="button"
          aria-pressed={mode === m}
          onClick={() => setMode(m)}
          title={`Written for ${MODES[m].audience}: leads with ${MODES[m].leads}`}
        >
          {MODES[m].label}
        </button>
      ))}
    </div>
  );
}

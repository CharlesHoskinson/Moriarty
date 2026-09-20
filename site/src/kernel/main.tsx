import { Component, StrictMode, useEffect, type ErrorInfo, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { KernelExplorer } from './KernelExplorer';
import { EvidenceInspector } from './EvidenceInspector';
import '../styles.css';
import './styles.css';

/**
 * Progressive enhancement for kernel.html.
 *
 * The static article and transcripts are complete on their own. This entry
 * mounts the explorer and the inspector beside them. The static block is hidden
 * only after React has actually committed the first render, which an effect
 * observes; `createRoot().render()` returning proves nothing about that. If a
 * component throws during the first render or any later update, an error
 * boundary catches it, unmounts the interactive root and restores the static
 * block. Print rules show the static block regardless.
 */

/** Reports the first successful commit of its subtree. */
function Committed({ onCommit, children }: { onCommit: () => void; children: ReactNode }) {
  useEffect(() => { onCommit(); }, [onCommit]);
  return <>{children}</>;
}

class Boundary extends Component<{ onFail: (err: unknown) => void; children: ReactNode }, { failed: boolean }> {
  state = { failed: false };

  static getDerivedStateFromError() {
    return { failed: true };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    this.props.onFail(error);
    // Keep the failure visible to anyone watching the console; the reader has
    // already been handed the static article.
    console.error('kernel explorer failed; static article restored', error, info.componentStack);
  }

  render() {
    return this.state.failed ? null : this.props.children;
  }
}

function mount(rootId: string, staticId: string, node: ReactNode) {
  const rootEl = document.getElementById(rootId);
  const fallback = document.getElementById(staticId);
  if (!rootEl || !fallback) return;

  let root: Root | null = null;
  const enhance = () => {
    rootEl.setAttribute('data-enhanced', 'true');
    fallback.setAttribute('hidden', '');
    fallback.setAttribute('data-superseded', 'true');
  };
  const restore = () => {
    rootEl.removeAttribute('data-enhanced');
    fallback.removeAttribute('hidden');
    fallback.removeAttribute('data-superseded');
    fallback.setAttribute('data-restored', 'true');
    // Unmount after the current React work finishes so the failed tree
    // leaves nothing behind in the root.
    const r = root;
    root = null;
    if (r) queueMicrotask(() => r.unmount());
  };

  try {
    root = createRoot(rootEl, { onUncaughtError: (err) => { restore(); console.error(err); } });
    root.render(
      <StrictMode>
        <Boundary onFail={restore}>
          <Committed onCommit={enhance}>{node}</Committed>
        </Boundary>
      </StrictMode>,
    );
  } catch (err) {
    restore();
    throw err;
  }
}

mount('kernel-root', 'kernel-static', <KernelExplorer />);
mount('kernel-evidence-root', 'kernel-evidence-static', <EvidenceInspector />);

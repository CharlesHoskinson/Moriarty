import { StrictMode, type ComponentType } from 'react';
import { createRoot } from 'react-dom/client';
import type { PageKey } from './data';
import '../styles.css';
import './styles.css';

/** Each HTML entry names its page on <body data-page>; only that page's module is loaded. */
const LOADERS: Record<PageKey, () => Promise<{ default: ComponentType }>> = {
  documentation: () => import('./pages/Documentation'),
  tutorial: () => import('./pages/Tutorial'),
  howto: () => import('./pages/HowTo'),
  reference: () => import('./pages/Reference'),
  explanation: () => import('./pages/Explanation'),
};

const root = document.getElementById('root');
if (!root) throw new Error('missing #root');
const key = document.body.dataset.page as PageKey;
const load = LOADERS[key];
if (!load) throw new Error(`unknown page ${key}`);
void load().then(({ default: Page }) => {
  createRoot(root).render(
    <StrictMode>
      <Page />
    </StrictMode>,
  );
});

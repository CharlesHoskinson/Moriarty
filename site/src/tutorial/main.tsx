import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { GuideApp } from './GuideApp';
import '../styles.css';
import './styles.css';

const root = document.getElementById('root');
if (!root) throw new Error('missing #root');
createRoot(root).render(
  <StrictMode>
    <GuideApp />
  </StrictMode>,
);

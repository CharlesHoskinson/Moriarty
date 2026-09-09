import type { ComponentType } from 'react';
import { Opening } from './components/Opening';
import { Categories } from './components/Categories';
import { Composition } from './components/Composition';
import { Formalization } from './components/Formalization';
import { Guarantees } from './components/Guarantees';
import { Language } from './components/Language';
import { Intents } from './components/Intents';
import { Roadmap } from './components/Roadmap';

/**
 * The section spine, in reading order. A section registers here once its
 * component exists; the page renders exactly what is registered.
 */
export interface SectionEntry {
  id: string;
  title: string;
  Component: ComponentType;
}

export const SECTIONS: readonly SectionEntry[] = [
  { id: 'opening', title: 'What Moriarty is', Component: Opening },
  { id: 'categories', title: 'The category tabs', Component: Categories },
  { id: 'composition', title: 'Composition', Component: Composition },
  { id: 'formalization', title: 'The formalization model', Component: Formalization },
  { id: 'guarantees', title: 'The guarantees', Component: Guarantees },
  { id: 'language', title: 'The language', Component: Language },
  { id: 'intents', title: 'Intents, settlement and safety', Component: Intents },
  { id: 'roadmap', title: 'Delivery', Component: Roadmap },
];

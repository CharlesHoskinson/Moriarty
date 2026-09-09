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
  /** Full name, used for the landmark label. */
  title: string;
  /** Short label for the jump strip, which has a masthead to fit inside. */
  nav: string;
  Component: ComponentType;
}

export const SECTIONS: readonly SectionEntry[] = [
  { id: 'opening', title: 'What Moriarty is', nav: 'Overview', Component: Opening },
  { id: 'categories', title: 'The category tabs', nav: 'Categories', Component: Categories },
  { id: 'composition', title: 'Composition', nav: 'Composition', Component: Composition },
  { id: 'formalization', title: 'The formalization model', nav: 'Semantics', Component: Formalization },
  { id: 'guarantees', title: 'The guarantees', nav: 'Guarantees', Component: Guarantees },
  { id: 'language', title: 'The language', nav: 'Language', Component: Language },
  { id: 'intents', title: 'Intents, settlement and safety', nav: 'Intents', Component: Intents },
  { id: 'roadmap', title: 'Delivery', nav: 'Delivery', Component: Roadmap },
];

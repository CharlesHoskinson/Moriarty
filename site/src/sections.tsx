import type { ComponentType } from 'react';
import { Opening } from './components/Opening';
import { Categories } from './components/Categories';

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
];

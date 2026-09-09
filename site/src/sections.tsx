import type { ComponentType } from 'react';

/**
 * The nine-section spine. Sections register here in reading order; the selected
 * storyboard decides what each one renders.
 */
export interface SectionEntry {
  id: string;
  title: string;
  Component: ComponentType;
}

export const SECTIONS: readonly SectionEntry[] = [];

/**
 * Reader mode.
 *
 * The site renders one section spine for two audiences. Mode changes copy,
 * emphasis and which depth layer of a shared visualization is expanded. It does
 * not fork the component tree into two applications, and it never changes the
 * facts.
 */

import { createContext, useContext } from 'react';

export type Mode = 'build' | 'verify';

export const MODES: Record<Mode, { label: string; audience: string; leads: string }> = {
  build: {
    label: 'Build',
    audience: 'DeFi protocol engineers',
    leads: 'the language and the failures it catches',
  },
  verify: {
    label: 'Verify',
    audience: 'programming-language and formal-methods readers',
    leads: 'the specification stack and what each assurance layer establishes',
  },
};

const STORAGE_KEY = 'moriarty.mode';

/** Build is the default. A stored preference wins; unreadable storage is not an error. */
export function readStoredMode(): Mode {
  try {
    const v = window.localStorage.getItem(STORAGE_KEY);
    return v === 'verify' || v === 'build' ? v : 'build';
  } catch {
    return 'build';
  }
}

export function storeMode(mode: Mode): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, mode);
  } catch {
    // A private window or blocked site data is a normal condition, not a failure.
  }
}

export interface ModeState {
  mode: Mode;
  setMode: (mode: Mode) => void;
}

export const ModeContext = createContext<ModeState>({
  mode: 'build',
  setMode: () => {},
});

export const useMode = (): ModeState => useContext(ModeContext);

/** Pick between two mode-specific values without branching at every call site. */
export const byMode = <T,>(mode: Mode, build: T, verify: T): T =>
  mode === 'build' ? build : verify;

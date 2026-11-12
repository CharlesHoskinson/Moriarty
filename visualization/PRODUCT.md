# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Engineers, cryptographers and technical decision makers who need to understand how Sherlock turns a Moriarty program into a claim that the Midnight network accepts. They read and watch closely, step by step, and want the mechanism, not a summary. The owner of the project uses the site to explain the system.

## Product Purpose

The Moriarty visualization site explains the Sherlock execution and proving system. The pipeline page shows the eight steps from a program to an accepted claim on Midnight: the program, the run, leaf proofs, node proofs, the decider, the transaction, the contract and the network. Success: a viewer who watches all eight scenes can explain how a program becomes an accepted claim, and why the keys, proof size and verification cost stay fixed while proving work grows linearly with the steps.

## Positioning

One universal key set proves any program of the Sherlock machine language. The program and its input are digests (`prog_d`, `input_d`). The decider publishes exactly `[prog_d, input_d, outcome_d, n_steps]`, and one contract accepts every program.

## Operating Context

A static site in `visualization/` (no build step), served locally by `server.js` and on GitHub Pages. Pages: the workbench (`index.html`), motion and slideshow pages, and the pipeline animation (`pipeline.html`). Viewers use desktop browsers mostly, with phone support required.

## Capabilities and Constraints

- Three.js r175, vendored, on the WebGPU renderer with a WebGL2 fallback (`?webgl`). No npm install at build time.
- The page plays, pauses, steps and scrubs; every moment is addressable by URL (`?scene=N&beat=B&t=T`) and renders deterministically.
- Narration is full DOM text, beside the 3D stage.
- Terminology: Sherlock, Moriarty, leaf, node, decider, `prog_d`, `input_d`, `outcome_d`, `n_steps`, `claim_d`, `acceptRun`, Midnight. The facts come from `research/sherlock-pipeline-animation-20261111/BRIEF.md`.

## Brand Commitments

- Google Material Design 3 is the design system (owner instruction, 2026-11-11).
- Dark mode only (owner decision, 2026-11-11). No light scheme.
- Voice: present-tense mechanism, as built; no status labels, no hedging, no account of how the work was made (the owner's `my-voice` profile).

## Evidence on Hand

The design documents and measured figures in `research/sherlock-final-design-20261111/` and `openspec/changes/`. No testimonials, customers, benchmarks or deployment claims beyond those documents. Never invent numbers.

## Product Principles

- Mechanism first: every beat shows what moves, why, and what it proves.
- Exhaustive and exact: detail wins over brevity; every number comes from the sources.
- One system: chrome and 3D stage read as one Material 3 dark design.
- Addressable: every moment can be linked, captured and reviewed.

## Accessibility & Inclusion

WCAG 2.2 AA contrast on the dark scheme, keyboard control of the player, visible focus, 48px touch targets, `prefers-reduced-motion` stepped mode, narration as real text.

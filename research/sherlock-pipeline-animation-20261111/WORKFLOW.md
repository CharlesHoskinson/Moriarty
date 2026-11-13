# Sherlock pipeline animation: workflow (run `sherlock-pipeline-animation-20261111`)

Owner, 2026-11-11: "We need a three.js animation showcasing how all this works step by step. Have fable 5.2 be a creative director in charge of 3 gemini and 3 astra agents with 10 terra agents and 10 sonnet. Figure out a workflow. All 8 steps need to be explained in exhaustive detail, do not hedge or tell me what's not designed. Assume its built. have a council of agents examine the flow and design and use it as a revision step to see if anything in our design needs improvement".

## Deliverable

A new page `visualization/pipeline.html` with modules under `visualization/src/pipeline/`, on the existing site stack (vendored three r175 WebGPU build, `visualization/vendor/three.webgpu.js`, `three/tsl`; import map as in `visualization/index.html`). It plays the eight steps in order, each as a scene with exhaustive narration, with play, pause, step and scrub controls. It presents the system as built: present tense, no status labels, no "designed" or "planned" wording.

The eight steps (from the lead walkthrough of 2026-11-11):
1. The program: Moriarty source compiled to the Sherlock machine language; `prog_d`, `input_d`; one key set for every program.
2. The run: the CEK machine executes rule by rule (120 rules); the witness stream; the three outcomes.
3. Leaf proofs: 16-step slices; the chip leaf (CORE, DEC, ALU, RANGE, HASH, BUS, MEM window, dense memory backend, PAGE/ROM cache lines, STACKS, VALUE DIGEST, BOUNDARY); dense tail-buffered memory with the depth-63 Poseidon tree; partition independence.
4. Node proofs: the self-recursive node, the accumulator, the 64-bit height counter (2^64 is the counter range, not a circuit size); linear work; GP-4 trees in parallel.
5. The decider: seal once; the public statement `[prog_d, input_d, outcome_d, n_steps]`; `outcome_d` and `sherlock-value/1`.
6. The transaction: the proof and statement in a Midnight transaction calling `SherlockRun.acceptRun`.
7. The contract: verify, refuse replay by `claim_d`, record; claims consumed by other proofs (GP-2), foreign proofs through the registry (GP-3).
8. On the network: ledger 9 verification, the accepted claim, composition across runs.

Facts come from `research/sherlock-final-design-20261111/ARCHITECTURE.md`, `openspec/changes/sherlock-modular-leaf-20261111/`, `openspec/changes/sherlock-vm-consensus-20261107/` (`design.md`, `requirements.md`, `general-purpose.md`), `EXPERIMENTS*.md` and `DECISIONS.md`. Numbers shown on screen are the measured or design values from those files.

## Roles

| Role | Who | Count | Job |
|---|---|---|---|
| Creative director | Fable 5.2 (Claude, model `fable`) | 1 | Owns the vision: storyboard, visual system, scene contract, assignment, review and sign-off at every phase |
| Technical writers | GPT-6 Astra (`codex exec -m gpt-6-astra`, read-only) | 3 | Exhaustive, exact narration and on-screen labels: Astra 1 steps 1-3, Astra 2 steps 4-5, Astra 3 steps 6-8 |
| Visual concept artists | Gemini 4 Carbon High (agy pane `w1:pC`, subagents) | 3 | Independent visual concepts and motion language for the whole piece, each a full storyboard proposal |
| Builders | Claude Sonnet (model `sonnet`, isolated worktrees) | 10 | Sonnet 1 shell (timeline, controls, narration panel, camera rig, shared materials); Sonnets 2-9 scenes 1-8; Sonnet 10 integration, performance and the fallback path |
| Visual QA | GPT-6.1 Terra (`codex exec -m gpt-6.1-terra`, `--image` screenshots) | 10 | Terra 1-8 one scene each, Terra 9 the whole sequence and transitions, Terra 10 typography, legibility, accessibility, mobile and Material 3 dark conformance (color roles, type scale, components, state layers, touch targets) and the impeccable slop checks (`impeccable detect`, craft-floor bans). Terra 3, 4, 5 and 9 are Quint lanes (below) |
| Quint lanes | Terra 3, 4, 5, 9 with the quint-llm-kit skills (`quint-modeling`, `quint-lang`, `quint-execute-spec` in `~/.agents/skills`) | 4 of the 10 | Each writes a Quint model of the mechanism its scene shows and checks that the animated sequence is a valid trace of it: Terra 3 the leaf memory (tail, flush at 8, depth-63 tree, cold reads, partition independence), Terra 4 the node (MERGE linking, heights below 2^64, accumulator carry), Terra 5 the decider seal (genesis `h_lo = 0`, terminal-only seal, `claim_d` without `n_steps`, no state digest public), Terra 9 the end-to-end flow including `acceptRun` replay refusal by `claim_d` across partitions and GP-2 consumption at the seal. Lane folder `~/.cache/pipeline-anim/terra-quint/<n>/`, `-s workspace-write` there, repository read-only by path. They run `quint typecheck`, `quint test` and `quint run --invariant` themselves; the lead re-runs every model. A violated invariant is a finding for the animation (wrong depiction) or for the design (goes to the council revision) |
| Council | Fable 5.2, GPT-6 Astra, Gemini 4 Carbon, Claude Opus (blinded, cross-family) | 4 | Examine the eight-step flow as animated and the design behind it; produce design improvement findings |

## Phases

1. **Brief (lead).** This file plus `BRIEF.md` (facts, sources, constraints).
2. **Content and concepts, in parallel.** Astra 1-3 write `content/astra-<n>.md`. Gemini 1-3 write `concepts/gemini-N.md` (independent; they do not read each other).
3. **Direction (Fable).** Fable reads all six and writes `STORYBOARD.md`: one scene spec per step (beats with durations, camera, objects, motion, the narration text final per beat, on-screen labels and numbers), the visual system (palette, materials, type, motion rules), and `SCENE-CONTRACT.md` (the module interface every scene implements, the shell API, file ownership).
4. **Build.** Sonnet 1 builds the shell first. Sonnets 2-9 build scenes against the contract in parallel. Sonnet 10 integrates, measures frame time and adds the reduced-motion and no-WebGPU fallbacks. Each builder works in its own worktree, rebased on main, and does not push.
5. **Visual QA.** The lead renders every beat to screenshots with headless Chromium (WebGPU on lavapipe) and a short frame sequence per scene. Terra 1-10 review against `STORYBOARD.md` and return findings. Fable triages. Sonnets fix. One fix round, then Fable signs off.
6. **Council.** Four cross-family members review, blinded, the eight-step flow (`STORYBOARD.md` and the narration) against `ARCHITECTURE.md` and the specs: Is any step unsound, slow, fragile or more complex than it needs to be? Does the flow reveal a gap? Each admissible finding becomes a design revision item in `COUNCIL.md`, the lead adds them to `research/sherlock-final-design-20261111/DECISIONS.md` as proposals, and the architecture or the spec is revised where the fix is clear. Council output is advisory.
7. **Land.** The lead merges, links the page from the site index, updates the wiki and the README, commits and pushes.

## Rules

- Heavy work (builds of Rust crates, proving) is not part of this run. Browsers and screenshot runs are light; still run them one at a time.
- Long CLI lanes run as `systemd-run --user` units with logs under `~/.cache/pipeline-anim/`.
- No AI attribution in commits. Never Ollama. No API key variables.
- Builders rebase on main first and never push. The lead pushes.

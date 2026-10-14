# S1 report: fresh direct-URL anchor repair

Status: source repaired in `site/src/tutorial/GuideApp.tsx` only (plus these reports). **Not typechecked, not built, not browser tested.** The worktree has no `node_modules` (`tsc` and `react` are missing), so `tsc` could only report missing modules. I made no browser, network, native or git action. Root must check the actual behaviour after copying.

Startup: read AGENTS.md and ran `status --json` (SP01.6 loan-swap-subset, evidence gaps; unrelated, not acted on). `PUBLICATION-ASTRA-REVIEW.md` was not present at the given path, so I worked from the symptom report.

## Diagnosis (from source, not reproduced)
- On a fresh load the mount `useLayoutEffect` called `target.scrollIntoView()` once, at first paint. Modules render afterwards, and fonts and lazy or async images settle. Lessons above the target then grow, so the target ends up far below the boundary (h2 at 796 and scrollY 60644 of max 62018 in the report).
- The current-lesson pin lasts 1500 ms and then falls back to the geometry rule. With the target displaced, that rule reports the lesson above it (Support and trust).
- A click works because layout is already settled when it scrolls. `hashchange` had the same single-shot weakness.
- The `--guide-offset` and `scroll-margin` logic and the measure `ResizeObserver` are not at fault.

## Repair
- New `hold(target)`, armed after the initial hash scroll and on `hashchange`. It runs a `requestAnimationFrame` loop with no timers. Each frame it re-aligns the target to the measured sticky offset with an instant `scrollTo` if it has drifted more than 1px, and it keeps the active-lesson pin alive for that target.
- The hold ends when the document is `complete`, fonts are loaded, all content images are `complete`, and `scrollHeight` has been steady for 30 consecutive frames. A 900-frame cap bounds it regardless.
- Reader input cancels it immediately (capture-phase `wheel`, `touchstart`, `pointerdown`, `mousedown`, `keydown`). Keyboard scrolling, scrollbar drags and Tab are covered, so it will not re-scroll a user who has started reading. It is also released on unmount and replaced when a new hash is armed.
- It applies to any id, so all 17 lessons and h3 anchors such as `#architecture-kernel` take the same path.
- Unchanged: `follow()` for clicks (no hold), active section and heading logic, focus, search, the mobile drawer, and the existing wheel/touch pin drop.

## Limits and what root should check
- Check fresh loads of all 17 `#lesson` URLs and `#architecture-kernel`, sampled after networkidle and again after a few more seconds. Expect the target top at about the offset, with `aria-current` on the right lesson or heading.
- Wheel, touch or key press during load must stop the holding. Check mobile width too.
- If the `scrollTo({behavior:'instant'})` type is rejected by the installed TypeScript lib, use `'auto'`. The page has no CSS `scroll-behavior`.
- A target near the end of the page that cannot reach the boundary stays at max scroll. This is the previous behaviour.
- The root server on 8896 serves the old source until root copies this file.

## Follow-up: reader takeover clears the pin (same S1)

Not typechecked, built or browser tested; no test, build, native, git or network action. Root reruns the checks.

Diagnosis (from root's evidence): a real click on a disclosure summary cancelled the hold, but `pinRef` stayed live until its 1500 ms `until`. A programmatic `scrollIntoView` in that window moved the section to 75.56px, yet the pin kept `currentTransfer`, and no later scroll or update arrived to expire it. Only `wheel` and `touchstart` dropped the pin, and the hold's release never did, so keyboard PageDown could leave a stale pin with no further update.

Change (`site/src/tutorial/GuideApp.tsx` only):
- `release` now only stops the frame loop. Hold replacement (`hold` calls it first) and unmount still use it, so a pin freshly set by `arrive` is not cleared by a replaced hold.
- New `takeover` = `release` + `pinRef.current = null`.
- One persistent capture, passive listener on `window` for every `INPUTS` type (`wheel`, `touchstart`, `pointerdown`, `mousedown`, `keydown`) calls `takeover`, installed in a mount effect and removed on unmount. It is no longer tied to a hold, so it clears the pin after the hold has expired too.
- Removed the per-hold listeners and the old `drop` wheel/touchstart listeners in the current-place effect, which this supersedes.
- Unchanged: geometry update, focus, nav, `follow()`, the 1500 ms pin set by `arrive` and refreshed by `hold` frames, and no new timers or event framework.

Root should check: fresh `#transfer` then a real summary click, then programmatic `scrollIntoView` shows the geometry lesson (Repayment) without a wheel; real PageDown moves current; clicking a nav link still pins the destination (pointerdown clears the old pin, `follow` sets the new one on click); hold still survives load without input.

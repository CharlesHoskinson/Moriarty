# Tests

Four suites, and they check different kinds of claim.

## `npm test`

Runs `src/data/completeness.test.mjs` against the data modules. The fixed sets
are part of the brief, so a design that quietly shows the main entries and
elides the rest fails here rather than passing unnoticed. It also checks the
composition arithmetic against its own stated ratio, and refuses vendor
attribution in the data layer.

It also runs `src/data/kernel-scenarios.test.mjs` against the kernel page's
educational model. Each test states an expected account in integer hundredths
from the approved design's accounting table and checks the reducer against it:
the accepted prefix, reservation as exposure, timeout, late success, retained
failure fee, still unknown, refused refund, second-solver contention, replay
and duplicate terminal results, withheld conditions, immutable policy and
reset. It also covers the orderings the fixture must refuse and the ones it
must reconcile: the first fill is finalized locally and the second is submitted
externally, in that order only; an authenticated external success is accounted
and its attempt identifier consumed even when an agreement predicate is
missing, no later conflicting result can release or rewrite it, and restoring
the predicate accepts the same observed result exactly once; no consumed
identifier is ever reused, checked under a more generous policy as well so the
consumption rule, not the fee cap, is what refuses the reuse; every duty status
is asserted on every terminal branch; and event narratives are checked against
the arithmetic of the state they describe. One test reads the static
transcripts in `kernel.html` and checks their numbers against the same reducer,
so the no-script fallback cannot drift from the model. Passing establishes
internal consistency of the illustration and nothing about Moriarty, the kernel
or any chain.

## `npm run test:site`

Drives the built home page in a browser and asserts what it renders. It covers
the states a screenshot cannot reach: both reader modes, both themes, phone
width, an opened action target, the facet lens, the swap that rejects 19,744 by
name, and the repayment that splits three ways. Console errors and page errors
are collected and treated as failures.

## `python3 test/docs.spec.py`

Checks the two static reference pages: one h1, a two-item documentation
navigation, no scripts, no horizontal overflow at phone and desktop widths,
every local link resolving, and the requirement and rule anchors present.

## `npm run test:kernel`

Drives the built kernel page. It walks the explorer through the default path
to completion and reads the rendered account at each step, then the
authenticated-failure and still-unknown branches, rejected candidates, every
evidence outcome, withheld conditions, the conflicting events, the evidence
inspector, keyboard operation with visible focus, widths from 320px up, dark
theme, reduced motion, print, and every local route link including the
existing `docs/kernel.html` redirect. It checks that the controls offer only
the approved ordering (first fill local, second fill external), that an
observed success with a missing predicate is accounted and held open with no
conflicting observation offered, and that restoring the predicate accepts it
once. It reads the enhanced and static evidence prose for the same narrowed
threshold and TEE claims, and checks the four acceptance judgments are on the
page. React drops synthetic clicks on a control whose `disabled` prop is set,
so the reducer's own refusal of out-of-order events is exercised by the Node
suite rather than by forcing clicks here.

It then opens the page with JavaScript disabled, with the enhancement bundle
blocked, and with the bundle replaced by a script that throws, and checks that
the static transcripts remain visible in all three cases. Two further contexts
inject a real component-render fault into the production bundle through the
browser, with no debug switch in the page: one makes React's first render of
the explorer throw, the other lets the explorer mount and then makes a later
update throw. In both, the static transcripts must come back, the explorer root
must be un-enhanced and empty, the separately rooted inspector must keep
working, and print must show the full fallback. Console errors and failed
requests are treated as failures in the main context.

Serve the build first:

```
npm run build && (cd dist && python3 -m http.server 8891)
```

The CI job also serves the build under `/Moriarty/` and points `URL` at that
subpath, so a direct refresh of `/Moriarty/kernel.html` and its relative assets
are checked rather than assumed from the origin root. Locally:

```
mkdir -p /tmp/pages && ln -sfn "$PWD/dist" /tmp/pages/Moriarty
python3 -m http.server 8892 --directory /tmp/pages
URL=http://localhost:8892/Moriarty/ npm run test:kernel
```

Set `URL` to point somewhere else, `CHROME` to pick a browser binary, and
`SHOTS` to choose where the captures land.

### Why it drives rather than looks

A screenshot proves a page rendered. It does not prove the guard fires, the
mark lands, or the control fits inside the bar it sits in. The masthead
overflow that clipped the mode control at desktop widths passed every visual
review it was given, and the assertion at seven widths is what caught it.

### What no automation here establishes

No screen reader was driven by these suites. The kernel suite checks the
status region's role and politeness, text labels on every control, focus
visibility and tab order, which are the preconditions for a screen-reader pass
rather than the pass itself.

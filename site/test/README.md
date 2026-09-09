# Tests

Two suites, and they check different kinds of claim.

## `npm test`

Runs `src/data/completeness.test.mjs` against the data modules. The fixed sets
are part of the brief, so a design that quietly shows the main entries and
elides the rest fails here rather than passing unnoticed. It also checks the
composition arithmetic against its own stated ratio, and refuses vendor
attribution in the data layer.

## `npm run test:site`

Drives the built page in a browser and asserts what it renders. It covers the
states a screenshot cannot reach: both reader modes, both themes, phone width,
an opened action target, the facet lens, the swap that rejects 19,744 by name,
and the repayment that splits three ways. Console errors and page errors are
collected and treated as failures.

Serve the build first:

```
npm run build && (cd dist && python3 -m http.server 8891)
```

Set `URL` to point somewhere else, `CHROME` to pick a browser binary, and
`SHOTS` to choose where the captures land.

### Why it drives rather than looks

A screenshot proves a page rendered. It does not prove the guard fires, the
mark lands, or the control fits inside the bar it sits in. The masthead
overflow that clipped the mode control at desktop widths passed every visual
review it was given, and the assertion at seven widths is what caught it.

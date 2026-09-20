# Existing site integration findings

- Source pin: 3c93e775a8e432aa995317e2428c169696257430.
- `site/scripts/build-docs.mjs` generates two script-free reference pages, then generates `docs/kernel.html` as a redirect to requirements.html#architecture. A real kernel page must replace or supersede that alias deliberately; do not leave duplicate outputs fighting over the same path.
- `site/test/docs.spec.py` deliberately checks two reference nav entries and zero scripts. Keep its static-reference contract; add distinct kernel browser coverage rather than globally allowing scripts in the references.
- `site/src/App.tsx` is a React educational landing page with section anchors, two reader modes and a Docs link. The build already supports interactive assets; no live backend is needed for an educational fixture explorer.
- `site/CONTENT-SPEC.md` predates the consolidated architecture. Its blanket Compact pipeline and every-transaction history claims must not override the new ZKIRv3 target and open native recursion obligations. Scope a new kernel content specification and explicitly establish current-source precedence in the old document when implementing.
- DeFiFormal import graph identifies semantic dependencies; it is not a deployed federation topology. Keep any diagram of runtime components explicitly target-design and source its behavior from the consolidated design.
- User asks for a plan. Do not build or deploy the new page during planning. Existing authorization permits publishing the planning document with the repository when complete.

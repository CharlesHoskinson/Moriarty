# STARK code patches for the Sherlock E path

The E path code lives in two upstream clones, not in this repository. This folder carries their commits as `git format-patch` series, so the branch `stark` holds the full workstream.

| Folder | Upstream | Base commit | Branch in the clone |
|---|---|---|---|
| `stark-backend/` | https://github.com/openvm-org/stark-backend | see `stark-backend/BASE` | `e2-lfr` (E1 Fr base field, M7b Poseidon config, E2 tags and height bound) |
| `openvm/` | https://github.com/openvm-org/openvm | see `openvm/BASE` | `e2-lfr` (M7 bench crate, `sherlock-lfr` crate: the L_Fr tape through I10, the E2 gate) |
| `openvm-e5/` | https://github.com/openvm-org/openvm | `c4bd312` (the top of `openvm/`) | `e5-wrap` (the `sherlock-wrap-export` crate for the E5 wrap fixtures) |

Apply with `git checkout $(cat BASE) && git am *.patch` in a clone of each upstream. The OpenVM workspace uses the backend through its path or git dependency; see the M7 and E2 RESULTS.md files for the build setup.

# External developer install launcher correction

Base: `74a909d69639df8823a527a4d608a7516ccd69a2` (main snapshot requested for L22).
Status: **unapplied ordinary documentation patch**. Only this new packet directory was written.

The beta archive instructions omit the launcher needed to follow the generated starter README after a local install. `GETTING-STARTED.md` recommends installing the archive into the developer's project and running `npx mori` there. The actual `init` consumer writes a starter README whose examples invoke bare `mori`. A local package install does not by itself put that binary into an interactive shell's global command path. In a clean shell with no global Moriarty executable or existing shell function, following those generated commands therefore has an unresolved launcher. This is a source-derived defect, not a reproduced shell failure in this task. An existing globally installed Rust `mori` could also select a different language profile; the fix selects the beta artifact explicitly, without evaluating that other binary.

The proposed change adds the already documented website pattern: define a shell function using an absolute path to the installed beta's `dist/cli.js` before entering the generated project. It then shows the concrete transfer filename/action/scenario arguments consumed by `init`, `check`, `test` and `simulate`. The helper lasts only for the current bash/zsh session, needs Node 24 on PATH, and can be redefined from the install project in a new shell. The archive is still built from the checkout and installed locally; there is no claim of npm registry publication. No global package, PATH or client configuration change is proposed.

The actual source path is root README → beta getting-started document → archive `files`/`bin` declaration → build's bundled CLI → `main`/`init` → generated README. The site's `tutorial.html` loads `src/guide/main.tsx`, whose loaders select the tutorial/how-to components. The archive how-to already defines the same absolute launcher in its `ia-define` example. The site's shared link helper pins an older `df8b155143e90450168af04fbfd20f7620043620` documentation version; this patch targets the requested main snapshot's package guide and does not change that site pin or claim a live website publication.

The product contract's external-developer criterion is substantially broader: a clean installation without project metadata/reviewer records must author a new supported program, inspect claims, produce real proofs and submit through its own Midnight environment. This launcher correction addresses an ordinary local example-consumption boundary only. The examples stay **PreparedUnqualified**. It neither satisfies the full criterion nor changes objective proof, owner authority or ledger requirements.

Files:

- `INSTALL-LAUNCHER.patch`: one-file unified diff against the exact base.
- `candidate/packages/moriarty-beta/GETTING-STARTED.md`: complete proposed document.
- `source/`: thirteen exact public source blobs used for the trace.
- `SOURCE-MANIFEST.json`: source and candidate SHA-256 pins.
- `VALIDATION.json`: nine direct source comparisons, all observed; no runtime results.

Source locations in the pinned snapshot:

- `README.md:50` — `The standalone [Moriarty beta package]`.
- `packages/moriarty-beta/GETTING-STARTED.md:29` — `For an installation outside`.
- `packages/moriarty-beta/package.json:10` — `"mori": "dist/cli.js"`.
- `packages/moriarty-beta/build.mjs:5` — `['src/cli.ts','dist/cli.js']`.
- `packages/moriarty-beta/src/cli.ts:99` — `const path=resolve(positional[0]);`.
- `packages/moriarty-beta/src/cli.ts:106` — `writeFileSync(join(path,'README.md')`.
- `site/src/guide/pages/HowTo.tsx:700` — `id="install-archive"`.
- `site/src/guide/howto/runs.ts:285` — `"ia-define"`.
- `site/src/guide/data.ts:11` — `export const PINNED =`.
- `docs/MORIARTY-PRODUCT-CONTRACT.md:74` — `A clean external developer installation`.

Validation read the package bin/files mapping, build output path, actual argument parser/init filenames and generated README, and the pre-existing site launcher. The patch contains only the documentation hunk and preserves the unqualified-result boundary. No new regression test is added for this reversible prose correction.

No installer, npm command, Cargo/build, Node/CLI execution, SDK, proof, target runtime, wallet, ledger, provider dispatch, network request, global-tool change or shared-worktree/Git mutation ran. The development skill was loaded. A fresh guarded CLI startup was not invoked because this lane permits writes only under this new packet and status can persist administrative intervals; no campaign action was selected. Existing checkout graph data was not treated as evidence for the exact pinned commit. Clean-install success and examples' actual results remain unexecuted, as do all proof/target/product exits. No security judgment is supplied.

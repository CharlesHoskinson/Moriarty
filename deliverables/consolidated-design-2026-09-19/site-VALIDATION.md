# Documentation candidate validation

Result: passed local documentation validation; no deployment or implementation acceptance claim.

- npm run verify: TypeScript check passed, 11/11 existing data tests passed, Vite production build passed.
- Six static reference pages generated; 110 mathematical expressions rendered at build time to native MathML using pinned KaTeX 0.16.22. No browser script or external font service is required by the reference.
- Existing site/test/site.spec.py: 49/49 browser checks passed.
- New site/test/docs.spec.py: all six pages passed at 375px and 1280px. Checked HTTP navigation, one h1, six navigation links and current-page marker, no page overflow, no scripts, no unresolved math placeholders, >80 semantics MathML nodes, and all 16 ZR + 8 MNR anchors on the requirements page.
- Repository-source link existence check: no missing paths in README or six HTML fragments. External hosted pages were not asserted deployed or checked live.
- Financial-agreement-source/5 EBNF compared against its canonical source: exact equality ignoring trailing whitespace.
- Independent kernel-agent content review approved README plus six page fragments. Findings concerning optional federation vs local evaluator and canonical paths were corrected before approval.
- Desktop syntax/requirements and mobile overview screenshots captured; root visually checked desktop syntax and mobile overview. Captures are in validation-shots/.

Initial local browser invocation failed because installed Playwright expected Chromium revision 1223. Re-ran successfully with the existing Chromium headless-shell 1234 binary via CHROME. CI already installs Playwright's matching Chromium. Initial new test incorrectly expected MNR anchors on recursion rather than the linked requirements page; corrected assertion without duplicating content.

CHANGED-FILES.txt is the exact integration allowlist (17 files). SOURCE-MANIFEST.json hashes it. Copy only those files; node_modules, generated public/docs, dist, screenshots and validation reports are not repository source changes. Existing Pages workflow is preserved with one added static-docs browser check. Existing deployment-on-main conditions are unchanged.

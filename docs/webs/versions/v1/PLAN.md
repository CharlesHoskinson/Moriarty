# Moriarty Webs React draft implementation plan

> For agentic workers: execute the disjoint contribution tasks below. Root is sole Git integrator; no nested agents.

Goal: one beautiful, self-contained local HTML/React explanation of Webs and Strands, plus editable source and reviewed evidence.
Architecture: one React owner integrates independent content/data/SVG/CSS contributions; browser-local interactions have no blockchain, wallet, network or cryptographic behavior. Bundle React and all assets into index.html with an ordinary pinned build tool. Keep source and third-party attribution.
Stack: React, editable SVG, plain CSS, ordinary JS bundling, existing Python Playwright for bounded UI verification.

Global constraints: BRIEF.md controls scope. Webs and Strands only as public vocabulary. Every proof/application illustration remains proposed/illustrative; actual observations and Opus caveats are attributed exactly. No native/SDK/prover/ledger/private access. All source paths are named explicitly. No write to the concept packet, user files or other contribution directories. At most3modelworkers beyond root. Low-impact presentation changes use meaningful browser/interaction checks, without redundant implementation-mirroring unit tests.

- [ ] 03 writes contributions/03-visual/design.css, HeroWeb.jsx, VISUAL-NOTES.md: coherent visual identity, responsive rules, decorative hero SVG. export default HeroWeb; no React dependency installation or shared entry changes.
- [ ] 04 writes contributions/04-scenarios/scenarios.json, invoice-fixtures.json, NOTES.md: five proposed scenarios and independent worksheet fixtures, original source refs and premise labels.
- [ ] 01 writes contributions/01-language/glossary.json and COPY.md: selected Web/Strand meanings and clear terminology, no new protocol terms.
- [ ] 02 writes contributions/02-positioning/positioning.json and COPY.md: audience utility, honest USPs, concise page copy.
- [ ] 08 writes contributions/08-learning/learning.json and NOTES.md: staged DevEx, FAQ, current evidence/gaps and links, preserving source scopes.
- [ ] 09 Luna writes contributions/09-map/WebDiagram.jsx, map.json and NOTES.md: a usable, editable source-attributed interactive illustration; default export WebDiagram({scenario,selectedStrand,onSelect,omittedStrands,onToggleOmit}); separate proof requirements from actual proof evidence. No source graph mutation.
- [ ] 06 Opus supplies contributions/06-security/SECURITY-COPY.json and SECURITY-NOTES.md from complete exact reviewed wording, with adjacent assumptions and maturity; no implementation/native authority.
- [ ] 07 sole React owner writes app/ source, package.json/lock, build script and final index.html, integrates all named contributions. Ordinary frontend dependency installation/build within owned page root permitted; no arbitrary dependency changes elsewhere. Inputs and output pins in REPORT.json.
- [ ] 05 independently inspects actual implementation and browser observations for motion/performance/accessibility defects; writes contributions/05-review/REVIEW.md and FINDINGS.json. Report unobserved limits; no invented benchmarks or conformance verdict.
- [ ] Root runs actual ordinary build and bounded browser checks: five selectors, all Strand controls, incomplete illustration state, invoice arithmetic (fund110,pay35→cash73,duties75; fund112→cash75,duties75), keyboard, reduced motion, mobile/desktop overflow and console. Inspect actual screenshots. Correct observed defects only.
- [ ] 10 fresh Muse reviews complete final page source/artifact and root's actual UI results; Opus reviews exact final security wording separately. Changed bytes require fresh applicable review.
- [ ] Root stages only exact reviewed owned source/artifact/evidence files for GitHub utxo, preserving unrelated untracked publication tools. No external Site hosting deployment. Report local HTML path, GitHub evidence and actual remaining verification limits.

Interfaces are content/data contracts for this draft, not Moriarty SDK APIs. The React owner may adapt contributor file shapes within its owned app source; contributors freeze delivered outputs and record changes separately. CLI/API code examples are explicitly proposed unless literally source documented. 'Assembled' in the illustration never means verified or settled.

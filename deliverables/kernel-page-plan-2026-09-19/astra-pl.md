# Proposal: follow an intention through one unfinished trade

Persona: programming-language and formal-semantics expert. Independent planning proposal; no implementation, network execution, or assurance review. Sources are the frozen brief and its named repository documents. Required development skill and CLI status were inspected in `/home/charl/Moriarty-pages-20260919`; no pending transaction notifications were reported. CLI dispatch restrictions do not block this educational planning task.

## Recommended format and rationale

Three formats are plausible:

1. **A guided agreement story with a small experiment at each step.** Explain the owner's benefit first, then let readers choose a candidate, expose a failed condition, and follow partial settlement. Recommended: the same concrete objects can connect financial purpose to formal meaning without teaching a notation first.
2. **An interactive architecture map.** Good for readers seeking component ownership, but arrows too easily imply a working integration or a trusted central controller. Keep a small boundary diagram as a supporting view.
3. **A source-and-proof playground.** Useful eventually for developers, but the current richer-source-to-native gap makes an apparent compile/prove/run experience misleading. A clearly identified local evaluator demo could be linked separately; do not make it the kernel story.

Use the title **“The Federated DeFi Kernel: coordinating execution within your rules.”** The opening example says: “You authorize a trade with a spending cap and delivery conditions. Solvers may find different routes. The rules still apply when a route changes or execution stops halfway.” Immediately below: “Interactive illustration of the target design; kernel integration remains under development.” This establishes usefulness before qualification detail without implying a deployed service. [S1–S3]

## Evidence categories visible throughout

**Repository facts:** supported local source programs can be authored and simulated; restricted older-profile lowering and scoped demonstrations do not establish the full general native financial path. Kernel, OWS, and x402 integrations are planned. The frozen README supplies the public current-capability account; an operational CLI summary is not a replacement for scoped deliverable evidence. [S1, S4]

**Target design:** mandatory acceptance connects financial meaning, signed authority, complete effects, and history to Midnight execution. The optional kernel coordinates solvers, evidence, signing, external submission, observation, and recovery. Federation membership is unnecessary for supported direct Midnight programs. No maintainer license or reviewer receipt authorizes public programs. [S2, S3]

**UI recommendations:** all scenario numbers, controls, animations, and state traces below are pedagogical fixtures. Label them “Illustrative policy” and “Model prediction.” Never label a browser calculation “native proof verified,” “transaction confirmed,” or “secure.” Link each conceptual section to its normative source and existing references. [S4]

## Page sequence

**1. Specify what may happen.** Show an inspectable intention card: exact assets/domains, recipient, gross spending ceiling, fee ceiling, successful-completion minimum, partial-fill rules, evidence policy, recovery authority, and disclosure policy. A short sentence distinguishes an exact plan from an outcome intention. The illustration uses the latter. Avoid invented `.mori` syntax: the current profiles do not implement the entire target lifecycle. [S1, S4]

**2. Let a solver choose within those rules.** Offer two compliant route cards and one deliberately invalid candidate. Switching between human-authored and AI-proposed labels changes no validity rule. A preference such as lower fees may rank admissible routes; it never changes admissibility. Display “candidate only: no funds moved” before model execution. An invalid candidate should identify the violated condition, not merely turn red.

**3. Follow a bounded stage.** A step button advances a textual event trace and financial accounting table. Each row distinguishes proposed effects, accepted model transition, observed external status, and remaining duties. A stage can finish its bounded computation while the agreement continues. A continuation is the authenticated record needed for the next stage in the target design, not unlimited source recursion. [S2]

**4. Stop halfway.** Introduce unavailable evidence, unknown finality, or failure after a committed prefix. The page's principal insight should be that unfinished execution has explicit meaning: fees, reserved authority, and consented duties remain visible.

**5. Reveal responsibility and trust.** Open a compact diagram and evidence inspector tied to the currently selected trace event. End with “What exists today / What this design still requires” and links to Requirements and Syntax & semantics. Do not replace either reference or the README's top image. [S1, S5]

## Worked scenario with explicit arithmetic

Use the roadmap discriminator: spend at most **11 A**, including at most **1 A fees**, for at least **20 B on successful completion**. Assets have toy but exact domain-qualified identities; the recipient is fixed. Counterparties explicitly consent to the obligation creation. The illustrative signed policy permits two partial fills, each exchanging 5 A principal for 10 B, with a 0.5 A fee. A selected issuer's fresh attestation and required counterparty acceptance gate the modeled action. A document hash alone cannot establish the predicate. [S2, S5]

The first fill commits and delivers 10 B. Accounting shows gross debit 5.5 A, fees 0.5 A, received 10 B, and the unfinished 10 B obligation. Do not call this completed success simply because this stage succeeded. The remaining fill reserves 5.5 A, including 0.5 A of fee capacity. Display spent plus reserved exposure as 11 A. A competing solver cannot reserve another 5.5 A against the same authority.

Submitting the second fill may produce **unknown finality**. Keep its reservation and request identity. “Timeout” records elapsed time and enables only remedies authorized by the policy; it does not conclude nonexecution. “Reconcile” learns an authenticated result and does not itself authorize another transfer.

Offer three pre-authored continuations:

- **Late success:** the second fill is established; cumulative debit 11 A, fees 1 A, and delivery 20 B satisfy this illustrative completion policy. The corresponding terminal rights are consumed, so refund cannot also complete.
- **Known failed second attempt with its fee retained:** second principal remains controlled, the 0.5 A attempt fee is charged under the displayed signed failure policy. Totals are 6 A gross debit, 1 A fees, 10 B delivered. Release only reservations established as unused. The existing fees remain spent; a third charged attempt would exceed the fee cap. A funded reserve refund, if authorized, does not recreate already spent authority. Remaining duties require explicit discharge, amendment, or other consented remedy.
- **Unresolved:** evidence remains unavailable. Keep “unknown” and the pending exposure; show which inclusion, witness, or observation assumption prevents progress. Do not manufacture a green terminal state.

Separately, rejecting a candidate before any accepted phase changes no agreement balances. An accepted failed phase may retain fees and effects. This distinction is more educational than a generic success/failure toggle. All numbers and failure policies are fixtures, not financial product quotations. [S2–S5]

## Interactions and state contract

Keep four interactions, each with a clear output:

1. **Choose candidate:** input is a fixed route fixture; output is eligibility plus explanations for asset, recipient, gross, fee, partiality, and evidence checks. Reject a better-priced route to the wrong recipient. No state transition occurs.
2. **Supply evidence:** input selects fresh authorized evidence, wrong issuer, stale evidence, or unavailable evidence. Output names the predicate established or unmet. Unavailable is not false and is not proven. An unsupported check gets its own explicit result.
3. **Advance or reconcile:** input selects a permitted next event; output is a new trace entry and the complete accounting delta. Buttons unavailable under the selected policy explain why. Every replay uses one logical request ID.
4. **Inspect enforcement:** input selects a stage field or mechanism; output identifies who must enforce it and what assumption remains. Include an optional “changed epoch” example: new signers cannot reset consumed authority or erase duties.

Implement a small pure fixture transition model for this page. Keep outcome status, evidence status, financial accounting, and authority consumption as separate fields; one overloaded “verified” Boolean would teach the wrong architecture. Reset starts a visibly new illustration, never a modeled refund.

## Exact boundaries and honest proof language

The responsibility diagram has four labeled areas. **Moriarty** supplies meaning and acceptance obligations. **The kernel** supplies optional coordination and configured services, without power to amend signed constraints. **Midnight** verifies the pinned native relation and enforces its ledger's state and consumption rules. **External domains** execute their own transactions and supply observations under named finality and trust assumptions. Draw a direct Moriarty-to-Midnight route beside the optional federated route. A local evaluator is an authoring aid, outside the settlement arrow. [S1–S3]

The advanced inspector separates four questions: does the program preserve its contract properties; does this candidate refine the signed intent; is this state/effect transition valid; is its history compliant? Explain that the target must enforce all applicable judgments, rather than displaying four completed proof badges.

Show source, Core, ZKIRv3, verifier key, and actual ledger effects as distinct artifacts linked by required correspondence. Their hashes need not be identical. ZKIRv3 is the current target. Proposed ZKIRv4 requirements and native recursive history belong in a future-capability note. If the approximately March 2027 horizon appears, explicitly identify the September 19 user planning assumption, not an upstream release commitment. Lean is not required. [S2–S5]

Give ZK, MPC/threshold signatures, and TEEs separate descriptions and assumptions. Bind their intended statements to the same program, intention, stage, domain, epoch, and effects. ZK proves an encoded relation; threshold signing distributes control; attestation concerns an execution environment. A signature-only external account can be bypassed after threshold compromise even if honest signers checked proofs. Shared operators may create correlated failures. No mechanism establishes oracle truth merely by authenticating a report. No global rollback follows from a local atomic step. [S1, S2]

## Delivery, access, and acceptance

Add a standalone `docs/kernel-explainer.html` educational route using the existing site build, with small progressive-enhancement JavaScript. Preserve `docs/kernel.html`'s existing requirements redirect and both reference URLs. Extend the build's explicit page/link handling deliberately; its present allowlist assumes only two references. Reuse current styling and avoid a new framework or proof service. [S6]

Deliver readable static HTML containing the normal and unknown-outcome traces before JavaScript runs. On mobile, place event detail below its event and transform accounting columns into labeled rows. Use semantic buttons, native disclosure controls, keyboard navigation, visible focus, a concise polite status announcement, and explicit text labels alongside color. Support reduced motion, 200% zoom, narrow screens, print, and direct anchor links. Avoid autoplay, drag-only interaction, fake live metrics, and wallet connection.

Acceptance should cover: exact fixture arithmetic; invariant preservation across every reachable modeled event; fees surviving failures; no duplicate reservation or terminal consumption; timeout staying unknown; signed fallback policy preservation; identical checks for both solver labels; meaningful no-JavaScript content; keyboard and mobile traversal; unchanged reference destinations and README image; and existing site checks/build/browser tests. Content review must distinguish fixture results from repository evidence. These checks validate an educational page, not the kernel or a deployed protocol.

My principal dissent is against a comprehensive animated stack tour as the main experience: it rewards breadth while concealing the acceptance boundary. One thoroughly explained incomplete trade makes the kernel's purpose and limits easier to understand.

## Source references

- **S1:** `README.md`, “How the Federated DeFi Kernel works with Moriarty,” current capabilities, and proof limitations.
- **S2:** `docs/MORIARTY-CONSOLIDATED-DESIGN.md`, responsibility boundary, canonical stage, conditional settlement, and native history.
- **S3:** `docs/MORIARTY-PRODUCT-CONTRACT.md`, permissionlessness, execution target, and provable intention.
- **S4:** `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`, target semantic interfaces and current-profile limits.
- **S5:** `ROADMAP.md`, U3/U5 and immediate delivery discriminator; `deliverables/defiformal-study-2026-09-19/RESULT.md`, nontransfer of model theorems.
- **S6:** `site/package.json` and `site/scripts/build-docs.mjs`, existing delivery commands, page allowlist, and aliases.

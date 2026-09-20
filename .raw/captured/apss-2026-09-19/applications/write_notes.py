from pathlib import Path
import json
R=Path(__file__).parent
M={x['id']:x for x in json.loads((R/'manifest.json').read_text())}
notes={
'APP01':('CAKE application boundary','Article sections introducing the four layers and chain abstraction',[],'''**Source claims.** CAKE separates applications, permissions, solving and settlement. It argues for an application experience that need not expose every network interaction and discusses different mechanisms below that experience.

**Moriarty inference.** Applications may simplify interaction, while the language must retain exact signed asset/domain semantics and allow independently implemented wallets and solvers. CAKE's permissions layer concerns authority to act for users; it does not imply a project license to deploy source programs.

**Limitation and counter-position.** This is an architecture essay, not a formal semantics or a proof of completion. Its abstraction goal can conflict with developer needs for explicit domains and assumptions. Do not make one managed router the only supported way to use Moriarty. Cross-chain routing is optional application scope, not evidence that Midnight can atomically execute all foreign steps.'''),
'APP02':('Combinators versus a fixed product catalog','PDF pp. 1–2; introduction and sections 2.1–2.2',[1,2],'''**Source claims.** Peyton Jones, Eber and Seward propose defining financial contracts compositionally rather than expanding a fixed catalog. The paper describes a Haskell combinator library and compositional valuation semantics. The examples combine payments and reverse parties' rights/obligations with `give`.

**Moriarty inference.** ACTUS/DeFi coverage should qualify primitives and libraries without becoming a deployment allowlist. A new composition of supported constructors should be expressible and analyzable without maintainer registration.

**Limitation.** Valuation semantics is not execution correctness, custody enforcement or a ledger correspondence proof. The historical examples use representations such as floating-point dates that should not be copied into an exact bounded financial kernel. The application is the contract description; enforcing and funding that description remains a separate obligation. Text extraction damages some glyphs; the reviewed page images establish the actual title and argument.'''),
'APP03':('Marlowe semantic guarantees and refunds','PDF pp. 2, 10–11; sections 2, 5.1–5.3',[2,10,11],'''**Source claims.** Marlowe builds financial contracts from a small number of constructs, accounts and continuations. The authors describe Isabelle proofs for semantic properties, including money preservation and a timeout after which an empty transaction closes a contract and returns account funds. The proof translation differs from Haskell in identifier and map representation.

**Moriarty inference.** Separate language-wide conservation/boundedness properties from a particular developer's desired payment. Expose termination and recoverability with their hypotheses. A refund transition being enabled is not a guarantee that a network participant will submit it.

**Limitation.** This 2020 paper is evidence of its described Marlowe design and proof scope, not a current Cardano deployment audit or a Moriarty theorem. Returning escrow cannot erase separate outstanding debt. Mechanized semantics and a compiled ledger implementation need an explicit correspondence argument; close-to-source translation alone is insufficient.'''),
'APP04':('Findel: composition can preserve the wrong bargain','PDF pp. 7–8; example 2.3, limitations and motivating option example',[7,8],'''**Source claims.** Arusoaie's paper gives a formal Coq treatment of Findel and exposes a contract in which a participant can obtain an incentive while controlling whether the other branch proceeds. The inspected pages also describe the examined Findel model's two-party limitation, lack of loops and absence of balance constraints preventing debt.

**Moriarty inference.** A well-typed compositional contract can fail the developer's intended economic bargain. Application specifications must name who controls choices, who owes each residual obligation, and which progress or funding assumptions make the bargain feasible. Add a negative test in which an actor takes a reward but avoids its intended reciprocal duty.

**Limitation.** These findings concern the examined Findel version, not every financial DSL. The selected pages do not establish the complete soundness of the certification system or every paper case study. A bounded repetition limit is an engineering design choice, not itself a safety defect; the danger is failing to model the intended lifecycle.'''),
'APP05':('Resource specifications and modular collaboration','PDF pp. 2 and 22; contributions and implementation/evaluation',[2,22],'''**Source claims.** Bräm and colleagues propose resource-oriented specifications and modular reasoning about collaborating contracts, including unverified external code and re-entrancy. Their 2Vyper implementation translates Vyper/specifications through Viper to SMT verification. The paper states that selected liveness aspects are expressed as safety properties.

**Moriarty inference.** Prove application-owned resource transfers and obligations at composition boundaries, not merely arithmetic outputs. An interface should describe effects and assumptions so a consumer need not assume the collaborator is benign. Hidden external effects cannot disappear from the proof footprint.

**Limitation.** Ethereum/Vyper re-entrancy semantics do not directly describe bounded Moriarty/Compact execution. A successful verification result establishes the supplied specification, not unstated user intent. The selected evaluation page identifies implementation and method; no benchmark reproduction or current tool qualification was performed here.'''),
'APP06':('Private applications with separate resource predicates','PDF pp. 29–31; sections 6.1–6.2',[29,30,31],'''**Source claims.** Zexe describes custom assets using record predicates for minting/conservation and private decentralized exchanges using access predicates. It distinguishes trade confidentiality from trade anonymity, discusses intent-based versus order-based exchanges, and notes a trade-off with market price discovery. Its privacy discussion assumes anonymous communication channels for user interaction.

**Moriarty inference.** Private DeFi applications need explicit disclosure goals and witness distribution rules in addition to validity proofs. The language can support applications with different ownership or issuance policies without adopting a universal identity operator.

**Limitation.** The captured manuscript is dated February 21, 2019. Its construction/performance is not a measured Midnight backend capability. A private proof does not automatically hide mempool timing, network identity, user-to-solver communication or application choice. Application policy authority is distinct from permission to deploy an application.'''),
'APP07':('Three levels of resource logic','Resource Logic; Proving; Instance, Witness and Constraints sections',[],'''**Source claims.** Anoma resource logic constrains resource creation/consumption; the corresponding proof is required for action validity. The specification distinguishes architecture-level, instantiation-level and application-level inputs/constraints, with commitment and nullifier integrity checks.

**Moriarty inference.** Separate universal proof/ledger invariants, Midnight-specific encodings, and developer-defined predicates. This offers a useful design vocabulary for permissionless applications: shared validity does not imply a centrally curated catalog of application meanings.

**Limitation.** This is a particular Anoma specification snapshot, not a proof that Moriarty implements its machine. Its generic claim that a predicate is computable cannot replace Moriarty's documented finite fragment and cost bounds. Resource logic proofs do not by themselves establish observed external facts or solve transaction ordering/conflicts.'''),
'APP08':('Intent is a constraint rather than a complete plan','Intents, the abstraction; Two ways to express constraints; Comparison',[],'''**Source claims.** Khalniyazova distinguishes intent from an unbalanced transaction. Desired outcomes can be expressed by creating specified desired resources or by predicates describing properties of acceptable resources. A carrier resource can require satisfaction of a predicate when consumed.

**Moriarty inference.** Provide both exact-plan and outcome-constraint authoring. A solver may choose unspecified details but must not weaken signed constraints. A concrete transaction skeleton is one encoding of an intent, not its universal definition.

**Limitation.** This is explanatory design writing and shares an independence key with APP07. It does not independently corroborate the resource-machine proof system. Predicates describe formalized preferences; neither a solver nor a proof system infers an unexpressed economic requirement. The paper's unconstrained composition intuition requires explicit bounds when translated to Moriarty.'''),
'APP09':('Programmable orders, optional watchers and funding hazards','README Methodology; TWAP; Just-in-time funding; Handler compatibility',[],'''**Source claims.** ComposableCoW separates conditional-order parameters, handlers, generated discrete orders and cancellation. Event dispatch to a watchtower is optional. Its funding-poller documentation allows independently built handlers without a handler allowlist but warns that handler outputs control token/amount and that new digests can trigger repeated funding.

**Moriarty inference.** A programmable order service is an application built from language capabilities. Permissionless extension must be paired with signed cumulative debit and lifecycle constraints, not trust that a handler's new digest represents a genuinely new economic authorization.

**Limitation.** This is captured repository documentation, not a source-code audit or deployment test. No claim is made that all CoW solver participation is permissionless. Optional watchtower disclosure does not prove end-to-end privacy. Handler authority and token approvals must be analyzed separately from the ability to publish handlers.'''),
'APP10':('AMM validity versus order-generation advice','Overview; Limitations; Settling a custom order; Risk profile',[],'''**Source claims.** CoW AMM permits custom orders respecting a nondecreasing reserve-product invariant, restricts each AMM to one order per batch, and requires a pre-interaction commitment. Its oracle assists order generation but does not determine order validity. The document states that this implementation does not pool liquidity across users.

**Moriarty inference.** Keep optimization/advice outside mandatory validity predicates. A solver can propose another valid trade without inheriting the oracle's authority. Composition constraints such as one operation per instance per batch belong in the formal effect relation, not only the UI.

**Limitation.** Reserve-product preservation alone does not prove best execution or the user's minimum net receipt. The documentation's economic-performance assertions are not reproduced here. Both protocol-specific assumptions and complete user intent remain necessary; do not adopt this AMM as Moriarty's sole swap primitive.'''),
'APP11':('Asynchronous vault applications need explicit claim states','Specification: Definitions, Request Flows, Request Lifecycle, Request Ids; cancellation rationale',[],'''**Source claims.** ERC-7540 separates Pending, Claimable and Claimed requests, with controller/operator roles. Request and claim are distinct calls; supported asynchronous preview functions revert. Pending amounts need not retain yield or a fixed asset/share exchange rate. The standard does not prescribe a general cancellation flow.

**Moriarty inference.** Model a pending entitlement as an outstanding obligation, not a completed payment. Bind request identity/controller, consumed input, claimed amount and residual amount across transitions. Application-specific recovery and cancellation require explicit semantics instead of assumed ERC behavior.

**Limitation.** This is an EVM interface standard, not an implementation proof or a Midnight adapter. A interface-compatible contract can still contain bugs. No fixed redemption price or eventual availability follows from the existence of a claim. The current captured page identifies the standard as Final; its bytes and retrieval date are retained.'''),
'APP12':('Resolver interoperability is not settlement correctness','Abstract; Motivation; Resolvers; Rationale; Security Considerations',[],'''**Source claims.** The current ERC-7683 draft standardizes solver-facing resolution of protocol-specific orders, rather than a common escrow or settlement contract. Resolvers disclose assumptions. Its security section explicitly separates the interface from the security of the settlement protocol and considers the full capital/authorization exposure window.

**Moriarty inference.** Export useful order descriptions without making a single router mandatory. Resolver vetting is a solver's risk policy; it must not become a language-wide developer allowlist. Prove the actual accepted effects satisfy the signed program/intent regardless of a resolver's reputation.

**Limitation.** This current draft differs materially from older order-interface versions; historical captures must retain their version scope. Offchain `eth_call` resolution does not itself verify financial settlement. No EVM interoperability implementation is claimed for Moriarty.''')}
visual=[]
for sid,(heading,locator,pages,body) in notes.items():
 m=M[sid]
 text=f'''---
id: apss.applications.{sid.lower()}
title: "{heading}"
status: draft
source_id: {sid}
reviewed_at: 2026-09-19
---

# {heading}

Source: [{m['title']}]({m.get('pinned_url',m['final_url'])}). Publication/version: {m.get('version') or m['publication_date'] or 'not determined'}. Retrieved {m['retrieved_at']}.

SHA-256: `{m['sha256']}`. Capture: [`{m['path']}`](../{m['path']}). Independence key: `{m['independence_key']}`.

Evidence locator: {locator}. PDF pages visually read: {', '.join(map(str,pages)) if pages else 'not applicable (HTML/Markdown source)'}.

{body}
'''
 (R/'notes'/f'{sid}.md').write_text(text)
 if pages:visual.append(dict(source_id=sid,pdf_sha256=m['sha256'],pages_visual_read_one_based=pages,pixelshot_dpi=100,tiles=[f'pixelrag/{sid}.png.tiles/tile_{page-1:04d}.jpg' for page in pages],scope='Selected pages inspected as images; full-document rendering is not full-document reading.'))
(R/'visual-reading.json').write_text(json.dumps(visual,indent=2)+'\n')

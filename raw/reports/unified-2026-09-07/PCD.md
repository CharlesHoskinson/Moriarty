# Proof-Carrying Transactions: Proof-Carrying Data as a Complement to Cryptocurrency Transactions

**Evidence cutoff: September 6, 2026**

## Executive decision

The central finding is that **proof-carrying transactions are technically viable today, but “proof-carrying transaction” should be treated as a layered design pattern rather than as synonymous with proof-carrying data, zero-knowledge proofs, or recursive SNARKs**. Foundational proof-carrying data has a stronger meaning: data moving through a distributed computation carries evidence that both the present datum and the computation history leading to it satisfy a prescribed compliance predicate. That history-compliance property is what distinguishes genuine PCD from an ordinary transaction with a proof attached. citeturn22search2turn22search1

The strongest near-term opportunity is therefore **not** to make every cryptocurrency transaction carry a recursively composed SNARK. It is to define a **typed, authenticated claim layer** through which a transaction can carry one or more independently machine-checkable claims: “these are the effects the signer authorized,” “this execution preserves invariant \(I\),” “this private credential satisfies policy \(P\),” “these are all state objects this transaction can touch,” or “this committed computation produced this result.” Some claims can be checked using signatures or ordinary contract assertions; some are best represented by reusable formal certificates; some require ZK proofs; and only claims depending recursively on prior proof-bearing data require genuine PCD.

This separation matters because current systems already demonstrate the component technologies. Zcash Orchard attaches a Halo 2 proof covering the actions in an Orchard bundle, but its specification explicitly says it does **not** use Halo 2 recursion. Mina permits account updates to be proof-authorized and recursively proves chain state. Midnight's current contract model combines public state, private computation, a public transcript and a ZK proof verified against a circuit verification key. General-purpose zkVMs such as RISC Zero and SP1 can prove program execution and recursively verify prior proofs. These are strong precedents for proof-bearing transactions, but they establish different statements and should not all be labeled PCD. citeturn23search6turn5search1turn5search0turn23search1turn18search0

Theorem-Carrying Transactions provides a particularly relevant alternative to cryptographic proof generation. Its prototype uses a transaction's concrete execution path plus symbolic reasoning to establish smart-contract interface properties, then reuses the theorem for future transactions satisfying the same guard and path. Its authors report roughly 0.20% runtime overhead for their token examples and roughly 0.57% for their Uniswap-V2 case on Geth 1.13, with theorem construction taking orders of magnitude longer but being amortizable through reuse. Importantly, the paper leaves consensus around theorem generation and several deployment questions unresolved. It is therefore compelling evidence for **reusable semantic certificates**, not evidence that a production theorem-carrying consensus layer exists. citeturn22search3turn22search4turn14view2

At the other end of the spectrum, Ethereum's 2025 EIP-8025 proposes **optional execution proofs** that would let nodes choose constant-time, stateless proof verification instead of re-executing an execution payload, without changing validity rules. As of the cutoff it remains an EIP rather than evidence that proof-based L1 execution validation is universally deployed. Ethereum's current state research has also moved from earlier Verkle-centered statelessness plans toward a unified binary-tree proposal. These developments are useful because they illustrate a migration principle this report recommends: **make expensive cryptographic acceleration optional before making it consensus-critical.** citeturn21search0turn21search1turn21search2

### Recommended build

The recommended first system is a **Proof-Carrying Transaction Claim Envelope**, abbreviated here as PCTE, containing authenticated, typed descriptions of claims plus either embedded evidence or content-addressed evidence sidecars.

It should initially support three categories:

| Priority | Claim class | What is established | Principal benefit | Recommendation |
|---|---|---|---|---|
| **Highest** | **Signed intent and effect constraints** | The resulting transaction effects satisfy an explicitly signed policy such as recipient, asset, max spend, min received, fee, approval and state-change bounds | Prevents transactions that are ledger-valid but contrary to what the user authorized | Build first |
| **Highest** | **Contract/post-state invariants** | A specified property holds for this transition, or a reusable theorem establishes it under explicit preconditions | Converts application-level promises into machine-checkable acceptance conditions | Build first |
| **High** | **Private authorization and eligibility** | A credential/capability satisfies a policy without exposing unnecessary underlying information | Privacy-preserving authorization and controlled disclosure | Build first where issuer trust is acceptable |
| **High but architecture-dependent** | **Authenticated state/dependency footprint** | Reads, writes and dependencies correspond to authenticated state; optionally, a claimed access set is complete | Stateless verification and safer parallel scheduling | Prototype |
| **High potential, higher systemic risk** | **Execution validity and recursively accumulated history** | Committed code executed correctly, perhaps with prior proof obligations recursively carried forward | Can reduce repeated execution and enable client-side or distributed history validation | Prototype as optional accelerator; genuine PCD as research track |

These rankings are an architectural judgment informed by the maturity of existing proof-authorized transaction systems, TCT's semantic-safety results, available zkVM composition mechanisms, current stateless-validation work and the continuing discovery of soundness bugs in sophisticated proof stacks. citeturn22search4turn18search0turn20view0turn20view1turn21search0

The key strategic principle is:

> **Use proofs to establish properties the ledger does not already know how to check, or to make expensive checks cheaper. Do not cryptographically re-prove simple facts that the ledger can enforce more safely and cheaply by ordinary deterministic validation.**

For example, value conservation, nonce checking or UTxO existence generally already have consensus rules. A proof of them may be useful for privacy or validation compression, but it does not automatically add safety. Conversely, a claim such as “the only token approval introduced by this transaction is a 100-unit allowance to contract X expiring at height H” can materially add a user-level guarantee because ordinary transaction validity normally does not encode the user's broader intent.

A second strategic principle is equally important:

> **Proof validity is not specification validity.**

A valid zkVM receipt establishes correct execution of the committed program; it does not establish that the program represents what the user intended. RISC Zero explicitly warns that its proof technology cannot prevent vulnerabilities in guest programs or contracts. Mina's documentation likewise warns about underconstrained proofs. SP1's own security history includes high-severity soundness issues—including an underconstrained completeness flag, a Plonky3 issue capable of making incorrect statements verifiable, and an April 2026 recursive-verifier row-count binding vulnerability—illustrating why the circuit, relation, verifier and binding logic must themselves be treated as security-critical. citeturn18search6turn5search3turn20view2turn20view1turn20view0

The design should consequently begin with **semantics and binding**, not with selection of a proving system.

## State of the art and terminology

The modern design space has four separate intellectual lineages: proof-carrying code and formal certificates; cryptographic PCD; recursive/accumulating proof systems; and blockchain-specific validity proofs. They increasingly overlap technically, but the guarantees remain different.

**Proof-carrying code** attaches evidence to code so a consumer can check a desired safety property using a relatively small trusted checker. The important idea for this study is not a particular proof system, but that the producer supplies evidence and the consumer need not rediscover the proof.

**Proof-carrying data** generalizes the carrying concept to distributed computations. In the foundational formulation, messages produced by mutually distrusting participants carry proofs that the current data **and the history that led to them** comply with a prescribed predicate. The PCD verifier's work need not grow with the whole history. citeturn22search2turn22search1

That leads to a useful formal distinction:

\[
\text{ordinary certificate}
\quad\neq\quad
\text{recursive proof}
\quad\neq\quad
\text{PCD}.
\]

A transaction carrying a proof that “program \(M\) produced output \(y\)” is a proof-bearing transaction. A recursively aggregated collection of ten such proofs is still not automatically PCD: aggregation can compress ten unrelated statements without establishing that their computation histories compose according to a common compliance predicate. Genuine PCD requires the recursive relationship itself to encode the relevant predecessor statements and compliance conditions. citeturn22search2turn0search0

**Incrementally verifiable computation**, or IVC, is the closely related sequential case in which repeated application of a step computation is recursively certified. Nova introduced folding schemes as an efficient way of constructing IVC: rather than fully verifying the preceding proof inside every new proof, it folds verification obligations into an accumulated instance. Nova's original construction has constant recursion overhead dominated by a small number of group operations, and can be compressed into a succinct terminal proof. It is an important PCD building block, but efficient single-prover IVC should not be mistaken for a complete solution to multi-party, branching PCD. citeturn1search3

Bünz, Chiesa, Mishra and Spooner's accumulation work established that PCD can be built using accumulation schemes rather than requiring the underlying argument's verifier to itself be fully succinct in the traditional recursive-SNARK sense. Later multi-folding work directly targets PCD over distributed DAG computation and reports improved prover behavior for its construction; its benchmark numbers remain author-reported and should not be treated as directly comparable with zkVM or production-network measurements. citeturn0search0turn0search1

The 2025–2026 research frontier is notable for tackling shortcomings that become especially relevant to transaction networks. Research on simulation-extractability observes that ordinary simulation-extractability definitions do not transfer automatically to PCD and develops PCD-specific notions, a warning against assuming standalone soundness is sufficient in adversarial recursive environments. Holography accumulation targets **stateless recursive proving**, motivated by the fact that folding-style approaches can require a prover to maintain or transfer large private accumulator state. Neo and SuperNeo pursue folding over small fields with plausible post-quantum security based on Module-SIS-style commitments and were revised as recently as September 4, 2026. All three are important research results; none should, on the evidence reviewed here, be treated as a production-ready universal transaction PCD layer. citeturn0search2turn0search3turn1search2

The stateless-prover issue is more than an implementation nuisance. In a cryptocurrency network, Alice may receive an asset from Bob and then send it to Carol. If extending the proof requires Bob to transfer a large secret accumulator state to Alice, the cryptographic construction has introduced a new custody, privacy and availability dependency. A construction that is excellent for a single rollup operator may therefore be poorly suited to a genuinely decentralized chain of mutually distrustful transaction producers. Holography accumulation was explicitly motivated by this gap. citeturn0search3

The broader landscape as of the cutoff is:

| System / line of work | What it actually proves or provides | Composition model | Status relevant to PCT |
|---|---|---|---|
| **Foundational PCD** | Current datum and its computation history satisfy a compliance predicate | Distributed/history-carrying | **Genuine PCD foundation** citeturn22search2 |
| **PCD from accumulation** | Construction of PCD using accumulation schemes | Recursive accumulated obligations | **Genuine PCD construction** citeturn0search0 |
| **Multi-folding PCD** | PCD targeting DAG computations with multi-folding | Fan-in/DAG-oriented | **Genuine PCD research** citeturn0search1 |
| **Nova** | IVC through folding of repeated computation | Primarily sequential IVC | **Important building block, not by itself general cross-party DAG PCD** citeturn1search3 |
| **Holography accumulation** | Stateless recursive proving using accumulated holographic checks | Recursive, designed to avoid persistent private prover state | **2026 PCD frontier** citeturn0search3 |
| **Neo/SuperNeo** | Small-field folding with plausible post-quantum security goals | Folding/IVC building block | **2026 frontier; not established deployment** citeturn1search2 |
| **TCT** | Symbolic theorem that a concrete-path family satisfies interface specifications under a guard | Reusable theorem + path hash, rather than cryptographic PCD | **Highly relevant semantic-certificate prototype** citeturn22search3turn22search4 |
| **RISC Zero** | Receipt of correct execution of committed RISC-V guest code; proof composition through conditional assumptions resolved recursively | Recursive receipt composition | **General certificate/PCD building block** citeturn18search0turn18search8 |
| **SP1** | Proofs of execution of RISC-V programs with recursive aggregation mechanisms | Recursive zkVM proof composition | **General certificate building block with material TCB history** citeturn4search1turn20view0 |
| **Zcash Orchard** | One Halo 2 proof covers all Orchard actions in a transaction bundle | Nonrecursive in current Orchard design | **Production-style proof-bearing transaction, explicitly not PCD** citeturn23search6 |
| **Mina zkApps / chain recursion** | Account updates can be proof-authorized; recursive proofs summarize chain computation | Proof-authorized updates plus recursive chain proof | **Strong adjacent architecture** citeturn5search1turn5search0 |
| **Midnight/Compact** | Private computation generates proof; transaction carries public transcript and is checked against a verifier key and current state | Application proofs; Compact itself forbids recursion | **Strong proof-bearing transaction architecture, not current general PCD** citeturn23search1turn17search2 |
| **Shielded CSV** | Recipient-side coin validity/history using the PCD abstraction; compact on-chain nullifier publication | Client-side history PCD | **Direct research example of cryptocurrency PCD** citeturn16search0 |
| **Ethereum EIP-8025** | Optional proof of an execution payload so consumers can avoid re-execution | Payload-level proof | **Proposed validation-accelerator architecture** citeturn21search0 |

Shielded CSV is particularly significant for this investigation because it uses the PCD abstraction for **client-side cryptocurrency validation** rather than simply applying recursion to a rollup. Its authors propose sending a coin plus its validity proof directly to the recipient while publishing only a 64-byte nullifier per transfer to the underlying chain, with the recipient's proof size and verification cost independent of transaction-history length. They discuss folding and recursive-STARK implementation strategies. These are research claims from a 2025 preprint, not evidence of a deployed 100-tps Bitcoin privacy layer, but they demonstrate that transaction-history PCD can enable a fundamentally different distribution of validation work. citeturn16search0

The most important terminology for transaction design is the following five-way distinction.

| Statement | What establishes it | What it **does not** establish |
|---|---|---|
| **A — Execution correctness:** “This program executed according to machine semantics.” | zkVM receipt or specialized execution circuit | That the program's purpose is correct, that its external inputs are true, or that its state is current |
| **B — Execution property:** “This particular execution also satisfies property \(P\).” | Relation/circuit/program explicitly checks \(P\), or a verified theorem covers the path | That all executions of the program satisfy \(P\) |
| **C — Program-wide property:** “All executions in the stated domain preserve \(P\).” | Static/formal proof, inductive invariant, exhaustive symbolic certificate or appropriately constructed PCD invariant | That environmental assumptions are true |
| **D — Current applicability:** “The transaction is valid against canonical state when included.” | Consensus/application freshness checks tied to the current state/version | Not obtainable merely from a proof against an old state root |
| **E — External truth:** “Its oracle/credential/data inputs accurately describe reality.” | Requires a trusted observation mechanism, issuer, oracle, consensus or external evidence | Cryptography can authenticate and privately process the assertion; it cannot manufacture its truth |

RISC Zero is a clean illustration of the A/B distinction. Its image ID binds a proof to particular guest code and its receipt certifies an execution of that code, while its own security model warns that guest-program vulnerabilities remain the application's responsibility. citeturn18search8turn18search6

TCT illustrates B versus C. A theorem in the paper takes the form of an entry function, guard and path hash; it establishes that the specified properties hold for executions matching those conditions. The reusable theorem can cover many future transactions that traverse the same proven path under the guard, but that is not the same statement as proving an arbitrary contract correct for every possible execution. citeturn22search4

Midnight illustrates A/B/D neatly: its current documentation describes a transaction whose proof is checked against a verifier key and whose public transcript is then applied only if the public state remains as expected. A valid historical proof is therefore not sufficient if the relevant ledger value has changed. citeturn23search1

Finally, no transaction-local proof should be described as proving availability, liveness, censorship resistance or real-world oracle truth unless the statement incorporates the necessary network or external evidence. Ethereum's documentation, for example, explicitly distinguishes state-transition validity from data availability: even a correct validity proof does not by itself ensure users can obtain the data needed to reconstruct or continue operating state. citeturn21search3

## Formal transaction and claim model

A useful PCT design begins with the ledger semantics, not with a SNARK.

Let

\[
S \in \mathcal S
\]

be ledger state,

\[
E \in \mathcal E
\]

the external consensus environment—block height, time, chain identifier, protocol version, oracle commitments and other explicitly admitted environmental facts—and

\[
t \in \mathcal T
\]

a transaction body.

Let

\[
L_\nu(S,E,t)\in\{0,1\}
\]

be the ordinary ledger-acceptance predicate under protocol version \(\nu\), and let

\[
\delta_\nu(S,E,t)=S'
\]

be the deterministic state transition when \(L_\nu=1\).

A proof-carrying transaction is then modeled as

\[
\mathrm{PCT}=
(t,\Gamma,\Pi,A)
\]

where \(\Gamma\) is a set of typed claim descriptors, \(\Pi\) is the associated evidence, and \(A\) is the authorization structure that binds the transaction and the claims.

A minimal claim descriptor should contain at least:

```text
ClaimDescriptor {
    envelope_version
    chain_id
    network_id
    protocol_semantics_version

    claim_type_id
    claim_type_version
    claim_mode              // mandatory | optional

    transaction_core_hash
    intent_commitment

    program_or_contract_hash
    specification_hash
    verifier_or_kernel_id
    verification_key_hash

    input_state_reference
    public_input_commitment
    output_or_effect_commitment

    dependency_claim_ids[]
    validity_start
    validity_end

    evidence_format
    evidence_hash
}
```

These fields are not decoration. They address distinct substitution attacks. `chain_id` prevents a valid certificate from silently changing meaning across chains. `protocol_semantics_version` prevents the same bytecode or instruction trace from being interpreted under different semantics. `program_or_contract_hash`, `specification_hash` and `verification_key_hash` prevent a prover from replacing the intended program, property or verifier. The state and effect commitments prevent a proof generated for one snapshot or result from being attached to another transaction.

The cryptographic relation for a representative claim \(\gamma\) can be written

\[
R_\gamma(x,w)=1,
\]

with public statement

\[
x=
(
c,\nu,H(t_{\rm core}),H(I),H(P),
H(M),H(vk),\rho_{\rm in},
\rho_{\rm effects},V,D
)
\]

and private witness \(w\) containing whatever private execution data, credentials, state openings or intermediate values are necessary.

A succinct proof then claims

\[
\pi_\gamma \leftarrow
\operatorname{Prove}(pk_\gamma,x,w)
\]

and acceptance requires

\[
\operatorname{Verify}(vk_\gamma,x,\pi_\gamma)=1.
\]

The crucial security statement is not merely “the proof verifies.” It is:

> Under the soundness assumptions of the evidence system, the verifier can conclude that there exists a witness \(w\) satisfying the **specific committed relation** \(R_\gamma(x,w)\).

That conclusion remains only as meaningful as \(R_\gamma\), its public-input bindings and the correctness of the implementation. Recent SP1 advisories demonstrate why this qualifier cannot be omitted: missing consistency constraints in recursive verification have been sufficient to create soundness vulnerabilities even in audited, production-oriented proof software. citeturn20view0turn20view2

For a formal rather than cryptographic certificate, the interface changes:

\[
K \vdash C :
\{Pre\}\ M\ \{Post\}
\]

where \(K\) is a trusted proof-checking kernel, \(C\) is a certificate, and `Pre`, \(M\), and `Post` are committed specification/program objects. A TCT-style certificate can specialize this further to a guard plus a concrete code-path family, permitting reuse without cryptographically reproving the execution each time. citeturn22search4

**Avoiding circular transaction commitments.** A proof often needs to refer to “the transaction,” while the final transaction identifier may itself hash the proof. A generic standard should therefore define a stable `transaction_core_hash` over proof-independent fields and a separate full-envelope identifier. One safe two-stage pattern is:

\[
I =
H(
\text{"PCTX-INTENT-v1"},
c,\nu,H(t_{\rm core}),H(\Gamma_{\rm semantic})
)
\]

followed by evidence whose public statement includes \(I\). After evidence generation,

\[
X =
H(
\text{"PCTX-ENVELOPE-v1"},
I,H(\Gamma),H(\Pi)
)
\]

can become the final envelope commitment. The user's semantic authorization must at minimum sign \(I\); where the same key also authorizes the final transaction encoding, it should additionally sign the final envelope according to the host ledger's transaction-signature semantics.

This separates **what the user intended** from **how a prover happens to encode evidence**. It also permits evidence sidecars without allowing a relay to strip a mandatory safety claim.

The acceptance rule should be approximately:

```text
verify_pct(state S, environment E, pct X):

    require canonical_decode(X)
    require ordinary_ledger_valid(S, E, X.tx)

    require verify_transaction_authorization(X)
    require signed_claim_root_matches(X)

    for claim in X.claims:
        require supported_or_ignorable_by_policy(claim)

        if claim.mode == MANDATORY:
            require transaction_binding_matches(claim, X.tx)
            require program_and_spec_binding_matches(claim)
            require state_reference_is_applicable(claim, S, E)
            require validity_window_contains(E)
            require dependencies_are_consistent(claim, X.claims)
            require evidence_hash_matches(claim)
            require verify_evidence(claim)

    return ACCEPT
```

Unknown **mandatory** claim types must cause rejection by an enforcing verifier. Unknown optional types can be ignored only if the signer has explicitly permitted that behavior. Otherwise extensibility becomes a proof-stripping vulnerability.

A semantic acceptance theorem can now be stated precisely.

Suppose a mandatory claim \(\gamma\) is intended to establish property \(P_\gamma(S,E,t,S')\). If:

\[
L_\nu(S,E,t)=1,
\]

all statement bindings are correct,

\[
Fresh_\gamma(S,E)=1,
\]

the evidence system is sound for committed relation \(R_\gamma\),

\[
R_\gamma(x,w)
\Rightarrow
P_\gamma(S,E,t,\delta_\nu(S,E,t)),
\]

the specification \(R_\gamma\) actually captures the intended safety requirement, and all external assumptions \(A_\gamma(E)\) hold, then acceptance of the PCT justifies relying on \(P_\gamma\).

That theorem makes visible the five distinct failure surfaces:

\[
\boxed{
\text{Assurance}
=
\text{proof soundness}
+
\text{correct binding}
+
\text{adequate specification}
+
\text{fresh applicability}
+
\text{valid external assumptions}
}
\]

—not proof soundness alone.

For a **network-wide invariant** \(I(S)\), stronger conditions are necessary:

\[
I(S_0)
\]

must hold at the base state, and every transition path permitted by consensus must satisfy

\[
I(S)\land Accept(S,E,t)
\Longrightarrow
I(\delta(S,E,t)).
\]

If an emergency transaction, unproved legacy operation, bridge mint, governance action or protocol upgrade can bypass the preservation condition, local transaction proofs do not establish the global invariant. This is exactly why double-spend freedom, total supply and global uniqueness cannot simply be delegated to unrelated local certificates.

Ledger architecture changes how difficult freshness and coverage are.

| Ledger style | Natural binding | PCT implication |
|---|---|---|
| **UTxO** | Exact output references and spending witnesses | Strong transaction-local dependency model; consensus still must establish that each input is unspent at inclusion |
| **Extended UTxO** | Inputs plus script/datum/redeemer and reference inputs | Particularly attractive for signed effect and state-dependency proofs because dependencies are explicit |
| **Account-based** | State root, account nonce, storage values and code identity | More vulnerable to proof staleness while waiting for inclusion; conditional-state proofs or rebase mechanisms become important |
| **Object/version based** | Object identity/version plus ownership/mutability mode | Can bind proofs to explicit object versions; shared-object ordering still requires ledger coordination |
| **Shielded** | Note commitments, membership paths, nullifiers and balance relations | Proof can hide ownership/value while proving conservation; global nullifier uniqueness remains a consensus responsibility |

Cardano's extended-UTxO design is especially relevant here. Plutus spending scripts authorize spending of UTxOs, while CIP-31 reference inputs permit a transaction to inspect an existing UTxO without consuming it. The reference output must still exist when the transaction is validated. That gives a PCT a natural way to commit to exact inputs and shared reference-state dependencies while leaving canonical “still unspent/still present” checks to the ledger. citeturn6search4turn6search0

Cardano also now has practical work toward proof verification: Input Output documented an open-source Halo2 Plutus-verifier prototype in August 2025. It should be treated as an enabling prototype, not as evidence that Cardano has a consensus-standard generic proof-carrying transaction layer. citeturn16search2

Midnight provides an almost complementary realization. Current Compact documentation describes public replicated ledger state, ZK circuits, private off-chain state/functions and generated proving circuits. Compact is bounded and disallows recursion, so the existing language is well suited to application-specific proof-bearing transactions but is not itself a general recursive PCD substrate. citeturn17search2

## Property catalog and assurance matrix

The five highest-value opportunities are those where proof-carrying evidence either adds a semantic guarantee absent from ordinary ledger validation or converts a large distributed validation burden into a much smaller checking burden.

### Highest-value properties

| Property | Exact claim | Mechanism | Retained assumptions | Enforcer | Practical maturity |
|---|---|---|---|---|---|
| **Authenticated user intent and effect bounds** | “For signed intent \(I\), this transaction's committed effects satisfy \(P_I\): recipient ∈ allowed set, input asset = A, out ≥ m, total debit ≤ x, fee ≤ f, no undeclared approval/delegation/state write.” | Deterministic effect checker, zkVM/specialized proof, or theorem over actual execution; signer commits to \(P_I\) | Wallet must faithfully construct/show \(P_I\); chain state must be fresh; specification must cover all effects | Wallet, smart account, contract or consensus envelope | **High for constrained policies; integration is the main gap.** Structured/clear signing already tackles readable intent but does not itself prove downstream effects. citeturn10search5turn11search0 |
| **Application invariant preservation** | “Given precondition \(Q\), this state transition preserves invariant \(I\).” | Contract assertion, formal theorem/TCT certificate, specialized ZK circuit, zkVM relation | Correct invariant/spec, correct code and semantics binding; external inputs trustworthy | Application verifier or consensus if invariant is protocol-critical | **Promising.** TCT demonstrates transaction-path semantic theorems on ERC20/Uniswap examples, with deployment questions unresolved. citeturn22search3turn22search4 |
| **Private authorization / credential predicate** | “The transaction actor holds an issuer-authenticated credential satisfying policy \(P\), without disclosing unnecessary attributes.” | ZK credential or proof of signature/commitment plus fresh revocation state | Credential issuer is trustworthy; revocation/root current; policy corresponds to desired authorization | Contract/application/ledger | **High.** ZK application platforms already expose this architecture; Midnight explicitly structures private computation around public proof verification. citeturn23search1turn17search2 |
| **Authenticated access/dependency footprint** | “Execution read exactly/at most set \(R\), wrote set \(W\), and authenticated values correspond to root \(\rho\); claimed scheduling relation is safe.” | State multiproofs + trace/execution proof; perhaps static theorem for complete access set | Root canonical; access set complete; dynamic effects accurately modeled | Scheduler, builder and/or validators | **Medium.** Stateless witnesses are an active protocol direction; proving *completeness* of arbitrary dynamic access remains harder than proving membership. Ethereum's current stateless work targets compact binary-tree witnesses. citeturn21search1turn21search2 |
| **Execution validity / recursive history** | “State transition/output resulted from specified computation; optionally every predecessor history satisfied compliance predicate \(C\).” | Specialized validity proof, zkVM receipt, recursive aggregation, IVC/PCD | Proof-system/VM/circuit semantics sound; data available; state freshness/finality separately enforced | Optional validator, contract, consensus or receiving client | **High for execution proofs; medium/research for general multi-party PCD.** RISC Zero implements recursive receipt composition; PCD research now directly addresses stateless cross-party recursion. citeturn18search0turn0search3 |

These properties should be evaluated across **four independent assurance axes**, rather than a synthetic “security score”:

| Claim class | Cryptographic/formal assurance attainable | Semantic scope | External/environmental trust | Engineering maturity |
|---|---|---|---|---|
| Signed intent/effects | Very high once effects and authorization are fully bound | Transaction-specific | Wallet/UI remains part of intent formation | High-medium |
| Invariant certificate | Very high relative to formal specification | Path, operation or program depending proof | Oracles and cross-contract assumptions may remain | Medium-high for targeted properties |
| Private eligibility | Very high for credential possession/predicate | Transaction/policy-specific | High dependence on issuer/revocation truth | High-medium |
| Access-set/parallelism | High for membership; harder for completeness | Transaction plus scheduler relation | Canonical state/order remains external | Medium |
| Recursive execution/history | High under proof-system assumptions | Potentially entire history/DAG | Consensus finality and data availability remain separate | Execution proof high-medium; general PCD medium-research |

### Broader property taxonomy

**Ledger integrity and authorization.** Proofs can establish hidden balance conservation, range constraints, knowledge of spending authority, correct note/nullifier construction, membership and mint/burn authorization. Zcash Orchard demonstrates the mature transaction-specific form: a single Halo 2 proof covers all actions in a bundle. But the protocol still uses global ledger mechanisms such as nullifier sets and pool balance checks. Orchard explicitly does not currently employ recursive Halo 2 proofs. citeturn23search6

A proof that “I know the secret key corresponding to an unspent note commitment” does not establish that the note has not already been spent somewhere else unless the statement references an authoritative current nullifier set and the ledger enforces uniqueness. This is a recurring principle:

\[
\text{local validity} \neq \text{global uniqueness}.
\]

**Contract semantics.** Useful claims include pre/postconditions, arithmetic safety, authorization invariants, controlled effects, absence of certain callback behaviors on the executed path, and refinement between an implementation and an interface specification. TCT is particularly relevant because it deliberately targets interface specifications rather than requiring every low-level bug pattern to be anticipated; it combines concrete path information with symbolic reasoning and can reuse the resulting theorem. citeturn22search3turn22search4

**User intent.** A transaction can prove an explicit machine-readable intent such as:

\[
\begin{aligned}
&\text{recipient}=R,\\
&\text{asset\_spent}=A,\\
&\text{amount\_spent}\leq X,\\
&\text{asset\_received}=B,\\
&\text{amount\_received}\geq Y,\\
&\text{fee}\leq F,\\
&\text{all writes}\subseteq W_{\rm allowed},\\
&\text{no new approval}>L.
\end{aligned}
\]

Ethereum's EIP-712 provides structured typed-data signing, and the Ethereum Foundation's 2026 Clear Signing work targets making transaction meaning human-readable. Those mechanisms improve what is shown and signed; a proof-carrying effect certificate would complement them by establishing that actual execution satisfies the signed constraints. citeturn10search5turn11search0

The remaining weakest link is **intent capture**. A compromised wallet can present one policy to the human and sign another. Efforts such as the Ethereum Foundation's 2026 WEBCAT work address parts of this supply-chain/UI problem by allowing wallets to verify an application's enrolled frontend against a manifest, but transaction proofs cannot eliminate the need for a trustworthy path from the user's understanding to the signed specification. citeturn11search4

**Resource guarantees.** A proof can establish “this abstract-machine execution took at most \(N\) steps,” “allocated at most \(M\) words,” “made at most \(K\) calls,” or “produced at most \(B\) bytes of state.” It cannot, without an additional hardware/time model, prove that a future verifier or node will complete within 10 milliseconds of physical time. Abstract computation bounds and wall-clock availability are different properties.

**Parallelism.** If a prover establishes a complete read set \(R(t)\) and write set \(W(t)\), then a scheduler can use conditions such as

\[
W(t_1)\cap
(R(t_2)\cup W(t_2))
=
\varnothing
\]

and the symmetric condition for \(t_2\) as a sufficient basis for safe independent execution in a simple state model. More sophisticated proofs could establish commutativity despite overlapping state.

The challenge is not proving that the declared items exist. It is proving **that no undeclared dependency exists**. Dynamic calls, code loaded through references, implicit global state, fees, timestamps, object creation, oracle reads and transaction ordering all enlarge the semantic access set.

This property is therefore particularly attractive in eUTxO systems, where consumed inputs are explicit by construction. Cardano reference inputs create a controlled form of read-only shared-state access, though the reference must remain valid when the transaction is actually checked. citeturn6search0

**Financial properties.** Proofs can establish accounting identities or collateral requirements *relative to explicit data*:

\[
\sum_i V_i(\rho_{\rm oracle})\geq
\alpha
\sum_j L_j(\rho_{\rm liabilities}).
\]

That does not prove the oracle is economically correct, that liabilities are complete, that collateral remains solvent tomorrow, or that off-chain obligations do not exist. Proof-of-reserves-like constructions are therefore best described as **proofs about committed accounting data**, not self-contained proofs of solvency.

**Privacy and eligibility.** A ZK proof can demonstrate credential possession, age/region/category predicates, spending limits or policy satisfaction while withholding the underlying attribute. Midnight's architecture directly demonstrates a contract model in which private state is supplied locally and only proof/public transcript data need reach the ledger. Its compiler also constrains potentially private values and requires explicit disclosure in relevant cases. citeturn17search2

However,

\[
\text{ZK}(\text{issuer says }x)
\]

proves neither \(x\)'s real-world truth nor legal compliance. It proves that an issuer-authenticated statement satisfying the encoded predicate exists.

**Provenance and interoperability.** A transaction can carry membership proofs, prior state commitments, proof of correct transformation and recursive history validity. Cross-chain use additionally requires a verifier to establish the source chain's consensus/finality rule—or to trust a bridge/light-client mechanism that does so. A proof of “event E appeared in block B” without proof that B belongs to the source chain's accepted/finalized history is incomplete.

**Off-chain solver results.** A solver can prove:

\[
Plan \in Feasible(S,I)
\]

and can potentially prove optimality over a **committed candidate space**:

\[
\forall p\in C,\quad
Objective(Plan)\geq Objective(p).
\]

It cannot prove it found “the best market price” unless the set of orders, venues, prices and timing assumptions defining the market are included in or authenticated to the statement. This is a useful distinction for intent-based exchanges.

**Availability, liveness and fairness.** Validity proofs can greatly reduce computation but do not prove that required data has been published. Ethereum's data-availability documentation explicitly makes this separation, including for ZK systems. Likewise, a local proof cannot normally establish future censorship resistance, global mempool fairness or network liveness because those are temporal/distributed properties involving observations outside a single transaction. citeturn21search3

## Architecture, lifecycle, and economics

Six integration architectures are viable, but they have very different failure modes.

| Architecture | Trust boundary and benefit | Protocol impact | Main limitation | Recommendation |
|---|---|---|---|---|
| **Wallet/recipient-side certificate** | Signer or recipient independently checks semantic/provenance claim | None | Ledger does not enforce it; recipient may reject after chain acceptance | Best deployment starting point for intent/audit claims |
| **Application-level verifier** | Contract/script refuses operations lacking valid claim | Application only | Verification cost and supported crypto depend on VM | Best near-term enforceable model |
| **Optional validation accelerator** | Node uses proof instead of expensive computation, retaining conventional validation as fallback | Usually networking/client changes, limited consensus change | Missing proof gives no acceleration; invalid-proof DoS must be controlled | Best path for execution-validity experimentation |
| **Consensus-enforced PCT envelope** | Every node enforces selected mandatory claim types | Protocol/consensus change | Largest TCB, upgrade and governance burden | Only after narrow claim formats mature |
| **Batch/block recursive certificate** | One proof amortizes many execution proofs | Block-builder/prover infrastructure plus possibly consensus | Latency, prover concentration and data-availability issues | Valuable for compute scaling, not first semantic-safety deployment |
| **Client-side PCD history** | Recipient validates asset/history while base chain provides minimal anchoring/uniqueness | Potentially little base-layer validation | Recipient witness/proof availability, bridging and recovery | High-potential research architecture |

Shielded CSV demonstrates the last category particularly directly, specifying its client-side coin protocol using PCD while using the base chain primarily for globally visible nullifier publication. citeturn16search0

Ethereum EIP-8025 illustrates the optional-accelerator category: proof-generating nodes would gossip execution proofs and proof-verifying nodes could check payloads without full re-execution, while the mechanism remains opt-in and does not change validity rules. The EIP explicitly presents this as a way to build operational experience before considering mandatory proofs. citeturn21search0

The transaction lifecycle should look like this:

```text
Human intent
    │
    ▼
Machine-readable policy P
    │
    ├── user authenticates H(P, tx_core, chain, expiry)
    │
    ▼
State / credential / code dependencies collected
    │
    ▼
Execution, formal proof, or cryptographic proof generated
    │
    ▼
Evidence bound to intent + tx core + spec + code + state reference
    │
    ▼
Final PCT envelope assembled
    │
    ▼
Cheap mempool prechecks
    │
    ├── canonical encoding
    ├── supported claim type
    ├── size / cost budget
    ├── signature and evidence hash
    └── obvious expiry / state-reference checks
    │
    ▼
Builder / validator
    │
    ├── ordinary ledger checks
    ├── proof / theorem verification
    ├── canonical-state freshness
    ├── dependency consistency
    └── transaction ordering / uniqueness
    │
    ▼
Included state transition
    │
    ▼
Finality
    │
    └── reusable certificates remain reusable only to the
        extent their conditions are state/version independent
```

**Freshness is the key lifecycle hazard.**

Suppose a proof is created at state commitment \(\rho_h\):

\[
\pi:
R(\rho_h,t,y)=1.
\]

At height \(h+k\), verification of \(\pi\) only establishes that \(t\) was correct **relative to \(\rho_h\)**. It does not establish:

\[
R(\rho_{h+k},t,y)=1.
\]

For a UTxO transaction this problem is partly transformed into a simple canonical check: have the exact inputs already been consumed? For an account transaction, nearly any relevant account/storage mutation can stale a proof. A robust PCT therefore needs one or more of four patterns:

1. **Exact-state proofs** that expire when the state commitment changes.
2. **Authenticated-fragment proofs** whose dependencies can be checked individually.
3. **Conditional transition proofs** of the form “for any current state satisfying \(Q\), this transition has effect \(P\).”
4. **Reusable semantic theorems** independent of a particular concrete state, plus lightweight current-state guard checking.

TCT's guard-and-path model is an example of the fourth strategy; its expensive concolic/symbolic work can be reused when a new transaction satisfies the stored theorem's guard and path. citeturn22search4

Midnight's documented transaction example illustrates the opposite end: the proof is accompanied by a transcript, and when verification occurs the ledger checks that the relevant public value is still what the transaction expected; if not, the transaction is invalid. citeturn23search1

Cardano's eUTxO structure likewise has an attractive freshness property. Consumed inputs identify exact state objects, while reference inputs can inspect shared state without consuming it. The validator still relies on ledger validation to ensure referenced outputs exist at the relevant time. citeturn6search0

### Efficiency model

Cryptographic proof verification changes **where** work happens. It does not automatically reduce total work.

Let:

- \(E\) = cost for one validator to directly execute/validate the expensive computation,
- \(V\) = cost to verify one proof,
- \(P\) = prover cost,
- \(N\) = number of validators that would otherwise duplicate \(E\),
- \(W\) = witness acquisition/state-opening cost,
- \(A\) = aggregation/finalization cost,
- \(D_p\) = proof communication/storage cost,
- \(D_t\) = ordinary transaction-data cost,
- \(R\) = expected retry/reproof cost due to state churn.

Ignoring common costs, conventional replicated execution is approximately

\[
C_{\rm direct}=N E.
\]

A proof-based path is approximately

\[
C_{\rm proof}
=
W+P+A+R+N V+D_p.
\]

A proof system lowers aggregate compute only when

\[
W+P+A+R+NV < NE.
\]

In the simplified case \(W=A=R=0\),

\[
N>
\frac{P}{E-V}
\]

is the replication threshold above which proof generation can break even, assuming \(V<E\).

This equation explains why proof systems can be valuable for highly replicated networks even when proving is far more expensive than one execution. It also explains why proving a cheap signature check individually is usually a poor idea.

There are at least five separate performance outcomes:

\[
\begin{array}{ll}
\text{Total resource cost} & C_{\rm total}\\
\text{Per-validator cost} & V\\
\text{Prover latency} & T_P\\
\text{Network/storage load} & D_p + D_t\\
\text{Capital/concentration pressure} & K_P
\end{array}
\]

and they can move in opposite directions. A design may dramatically lower the hardware required to **verify** the chain while creating a market in which only GPU-rich operators can efficiently produce timely proofs.

The same caution applies to recursive compression. A succinct proof does not imply the underlying transaction/state data can disappear. Ethereum's documentation explicitly notes that correctness proofs do not solve data availability: users may still need state data to know balances or create future state updates. citeturn21search3

Current primary evidence does not support a fair universal benchmark ranking among PCD, RISC Zero, SP1, TCT and bespoke circuits because the workloads, hardware, proof targets, security models and proof formats differ. The multi-folding PCD paper, for example, reports a 49-second step on one stated large-constraint benchmark and compresses a large intermediate proof using a SNARK; those figures demonstrate the construction's behavior under the authors' test, not a universal per-transaction cost. citeturn0search1

TCT has a similarly specific benchmark profile: the paper's Uniswap swap path had roughly 9,495 EVM instructions across five contracts and ten cross-contract calls; the authors report about 0.57% runtime checking overhead after a theorem repository hit, whereas theorem-generation components operate on second-scale times. That makes the method attractive precisely where reusable theorems amortize their creation cost. citeturn22search4

For a decision-grade PCT benchmark, all candidate systems should therefore be tested on identical transaction semantics and report at least:

\[
p50,\quad p95,\quad p99
\]

for proof generation, finalization, verification and end-to-end submission latency, along with peak memory, witness bytes, final proof bytes, network bytes, retry rates under state churn and hardware capital requirements.

A particularly important experiment is to vary contention. Suppose proving takes \(T_P\), while a relevant account state changes according to some workload-dependent rate. The cost model must include:

\[
R =
Pr[\text{proof stale before inclusion}]
\times
C_{\rm reproof}.
\]

A proof path that is cheap under static benchmarks can become uneconomic in a hot AMM, auction or lending market if proof latency produces repeated stale-state failures.

### Decentralization

Proof-carrying execution has a potentially desirable asymmetry:

\[
\text{expensive proving}
\longrightarrow
\text{few providers},
\qquad
\text{cheap verification}
\longrightarrow
\text{many validators}.
\]

Whether that improves decentralization depends on which role controls censorship-sensitive operations. If anyone can fall back to direct execution, prover concentration is primarily an efficiency issue. If consensus requires a proof before a transaction can exist, prover availability becomes a transaction-admission dependency.

That is why **semantic safety claims and acceleration claims should have different fallback rules**:

- An **optional acceleration proof** may safely fall back to conventional execution.
- A **mandatory safety proof** should not silently fall back, because the fallback eliminates the claimed safety property.

## Threat model, trusted computing base, and worked transactions

The fundamental adversary is not just “a malicious prover.” Relevant adversaries include a malicious transaction originator, wallet, application developer, contract author, solver, prover, aggregator, builder, validator, oracle, credential issuer, verifier-registry operator and protocol upgrader.

A PCT consequently has a larger trusted computing base than its cryptographic verification equation suggests.

| TCB layer | Failure mode | False acceptance possible? | Mitigation |
|---|---|---:|---|
| **Human-to-policy translation** | User thinks they authorized X; machine policy actually permits Y | Yes | Narrow intent language, clear signing, independent wallet rendering, deterministic policy templates |
| **Specification** | Property omits a critical effect or uses wrong invariant | Yes | Formal review, adversarial specifications, explicit effect completeness |
| **Source semantics** | Source program meaning differs from assumed semantics | Yes | Versioned formal semantics |
| **Compiler / circuit generator** | Source logic compiled to underconstrained relation | Yes | Translation validation, independent compiler, formal verification |
| **zkVM / arithmetic constraints** | VM permits invalid trace | Yes | Audits, differential testing, formalization, fuzzing, multiple implementations |
| **Witness generator** | Usually generates invalid/unprovable data; can exploit underconstraint if relation permits | Sometimes | Treat witness code separately from constraints; negative testing |
| **Proof system / recursion** | Unsound verifier, Fiat-Shamir error, missing recursive binding | Yes | Cryptographic review, version pinning, security margins |
| **Verifier key / predicate binding** | Proof checked against attacker's verifier/spec | Yes | Commit hashes in signed claim |
| **Serialization** | Different nodes hash/parse different statement | Yes | Canonical encoding and test vectors |
| **State binding** | Correct proof used against wrong/stale state | Yes at application level | Consensus freshness checks |
| **External data** | Authenticated oracle/issuer lies | Yes relative to real world | Explicit trust model, threshold/diverse sources |
| **Ledger integration** | Mandatory claim can be stripped or bypassed | Yes | Signature commits to claim root; fail-closed enforcement |
| **Upgrade/governance** | Unsafe verifier/spec replacement | Yes | Versioned activation, delayed upgrades, audit and rollback policy |

The proof system itself is demonstrably not an abstract ideal in deployed engineering. SP1's January 2025 advisory records an underconstrained `is_complete` flag that could make a partial recursive execution verify as complete; the same disclosure documents a Fiat-Shamir observation error and other verifier issues. A June 2025 advisory states that an upstream Plonky3 vulnerability allowed malicious provers to prove incorrect statements for affected SP1 versions. In April 2026, SP1 V6 disclosed another high-severity recursion-circuit binding gap in which commitment-side row counts and evaluation-side layout could diverge. citeturn20view2turn20view1turn20view0

These incidents are not a reason to reject zkVMs. They are strong empirical evidence for three architecture rules:

\[
\boxed{\text{proof verifier} \neq \text{tiny TCB in practice}}
\]

unless its complete implementation path is demonstrably small;

\[
\boxed{\text{audited} \neq \text{proved sound}}
\]

because some affected SP1 code had already undergone audits; and

\[
\boxed{\text{recursion introduces its own bindings}}
\]

that must be independently constrained. citeturn20view2

Zcash provides another current caution. Its ZIP repository states that the most recent settled upgrade at the cutoff was NU6.2, activated June 3, 2026, and describes it as containing an Orchard temporary vulnerability mitigation; the proposed NU6.3 transaction format is motivated by bolstering confidence in supply integrity after discovery and remediation of an Orchard soundness vulnerability. That is especially relevant to this report because Orchard is a mature, protocol-integrated proof-bearing transaction system. citeturn23search9turn23search10

A PCT standard should therefore make cryptographic agility and verifier versions first-class, but **agility cannot mean “accept any verifier advertised by the transaction.”** A transaction choosing its own proof predicate can trivially choose a predicate that approves everything. The relying party must independently define which claim type/version/verifier/specification is authoritative.

**Invalid-proof denial of service** needs particular attention. A node should not perform an expensive pairing, polynomial commitment or recursive proof verification before establishing cheap prerequisites. Mempool admission should proceed roughly:

\[
\text{parse}
\rightarrow
\text{size limits}
\rightarrow
\text{claim-type lookup}
\rightarrow
\text{expiry/state sanity}
\rightarrow
\text{cheap authorization}
\rightarrow
\text{fee/bond eligibility}
\rightarrow
\text{expensive verification}.
\]

Verification cost must also be bounded and reflected in fees or resource budgets. Otherwise an attacker can submit syntactically valid but cryptographically invalid proofs that consume validator CPU before the network can charge them.

### Worked transaction cases

| Case | Specification and evidence | Prover / verifier | Freshness and residual ledger checks | What remains unproved |
|---|---|---|---|---|
| **Transfer with signed intent** | User signs `asset=A`, `recipient=R`, `debit≤100`, `fee≤F`, `writes⊆W`, `no approval/delegation`; effect certificate proves resulting state delta satisfies these constraints | Wallet/local or outsourced prover; wallet, account contract or ledger checks | Nonce/UTxO unspent, balance and canonical inclusion still ledger checks | Whether UI faithfully represented the signed policy |
| **AMM swap** | `input≤X`, `output≥Y`, `recipient=R`, `deadline≤H`, plus pool-invariant preservation | zkVM/circuit or reusable theorem; AMM contract/PCT verifier | Current reserves/input objects and deadline must remain applicable | “Best available price” unless competing venues/orders are committed |
| **Lending action** | Post-state accounting plus collateral ratio \(\geq \alpha\) relative to signed oracle round \(r\) | Application prover; lending contract | Oracle round freshness and actual ledger state must be checked | Oracle truth, future prices, off-chain liabilities |
| **Parallel transaction** | Proof commits complete \(R,W\), authenticates reads under state root and proves all writes derive from those reads; scheduler proves conflict rule | Builder/prover; scheduler/validators | Root/version and transaction order must be canonical | Safe parallelism if access relation omitted an implicit dependency |
| **Private eligibility** | Proof of issuer-authenticated credential, predicate \(P(attribute)=1\), nonrevocation under root \(\rho_r\) | User wallet; application contract | Revocation root must be sufficiently fresh | Truth of issuer's claim and broad legal compliance |
| **Cross-chain/provenance** | Recursive proof that event/asset followed valid transitions from authenticated source commitment and satisfies compliance predicate at each hop | Successive holders or proof service; destination/client | Source consensus/finality root must be authenticated; uniqueness on destination enforced | Data availability, source-chain liveness, correctness of external bridge assumptions |

### The valid SNARK that violates user intent

This case is so important that it should be part of every prototype test suite.

Suppose the actual transaction code is:

```text
execute_swap():
    take 10,000 USDC from Alice
    swap through pool P
    receive 5 ETH
    send 5 ETH to Mallory
```

A zkVM proof can validly establish:

\[
\text{“program }M\text{ executed correctly and output this state root.”}
\]

If \(M\) really sends the output to Mallory, the proof is correct.

It does **not** establish Alice's actual objective:

```text
I_Alice:
    input_asset      = USDC
    max_input        = 10,000
    output_asset     = ETH
    min_output       = 4.9
    output_recipient = Alice
    max_fee          = 30 USDC
    approvals_added  = none
    expiry           = H
```

The useful relation is instead:

\[
R_{\rm intent}
=
R_{\rm execution}
\land
Effects(M,S,t)\models I_{\rm Alice}.
\]

The evidence statement must bind both

\[
H(M)
\quad\text{and}\quad
H(I_{\rm Alice}),
\]

and Alice's authorization must commit to \(H(I_{\rm Alice})\). This is the difference between **proving that software did what it did** and **proving that what it did satisfied what the user authorized**. RISC Zero's explicit warning that guest-program security remains an application responsibility reinforces precisely this distinction. citeturn18search6

### Cardano realization

For Cardano/eUTxO, a strong prototype claim can bind:

\[
\begin{aligned}
&\text{input UTxO IDs},\\
&\text{reference-input IDs},\\
&\text{script hashes},\\
&\text{datum/redeemer commitments},\\
&\text{mint/burn policy IDs},\\
&\text{complete output multiset},\\
&\text{fee bound},\\
&\text{signed semantic intent}.
\end{aligned}
\]

Cardano nodes already validate the relevant UTxO/script conditions, and CIP-31/33 provide reference-input/reference-script facilities. A first PCT implementation therefore does not need to replace Plutus execution. It can add a **semantic certificate** that a Plutus script or wallet checks, or experiment with a ZK-verification path using the documented Halo2 verifier prototype. citeturn6search4turn6search0turn6search3turn16search2

This is one of the better environments for testing state-bound proof carrying because the eUTxO model exposes transaction dependencies more explicitly than a general account VM.

### Midnight realization

Midnight is closer to the cryptographic end state. Current documentation describes public state plus private local computation and ZK circuits, while transactions carry proof-related public transcript information and are verified against a verifier key before the transcript is applied to current ledger state. citeturn17search2turn23search1

A PCT extension there would therefore focus less on “add a proof” and more on **standardizing higher-level claim semantics**:

\[
\text{proof of private execution}
+
\text{typed intent claim}
+
\text{typed credential claim}
+
\text{effect commitment}.
\]

Because Compact circuits are bounded and direct recursion is disallowed, a genuinely history-compositional PCD prototype would require an additional recursive subsystem rather than simply calling a Compact circuit recursively. citeturn17search2

### When proof composition is actually property composition

Suppose transaction \(T_3\) verifies proofs from \(T_1\) and \(T_2\).

It is **not** enough to establish

\[
Verify(\pi_1)=1
\land
Verify(\pi_2)=1.
\]

The outer relation must establish whatever semantic compatibility the application requires, for example:

\[
outputRoot(\pi_1)
=
inputRoot_1(\pi_3),
\]

\[
outputRoot(\pi_2)
=
inputRoot_2(\pi_3),
\]

and perhaps

\[
Policy_1
=
Policy_2
=
Policy_3,
\]

or an explicit rule explaining how their policies combine.

RISC Zero makes this distinction operationally visible through its concept of conditional receipts: verifying another receipt inside guest code introduces an assumption, and that assumption must subsequently be resolved for the outer receipt to become unconditional. citeturn18search0turn18search2

General PCD goes further by defining the compliance predicate over predecessor messages and their proof-carrying histories. That is why PCD is appropriate when the property is fundamentally:

> “This object is acceptable because every admissible transformation in its potentially branching provenance was acceptable.”

It is excessive when the property is merely:

> “This one swap gave me at least 4.9 ETH.”

## Prototype roadmap and final answers

The recommended program has three deliberately different prototypes. Their purpose is to discover whether the PCT idea has value at the semantic, computational and genuinely compositional levels rather than assuming one proving technology should serve all three.

**Semantic-safety prototype — signed intent plus effects.**

The prototype should implement a canonical claim envelope, a small intent language and an independent effect checker. Start with transfer and swap operations, then lending. Implement the same claim semantics in at least two ledger models; Cardano/eUTxO and an account-based environment are a useful pair because their state-dependency behavior differs materially.

The first experiment should compare:

\[
\text{ordinary signing}
\quad\text{vs}\quad
\text{clear/typed signing}
\quad\text{vs}\quad
\text{signed intent + machine-checked effect}.
\]

The important metric is not proving speed; it is whether adversarial transactions that remain perfectly ledger-valid are rejected because they violate signed semantic constraints.

**Efficiency prototype — optional execution proof.**

Implement an execution proof as a validator accelerator with a sound ordinary-execution fallback. Measure exactly the break-even inequality:

\[
W+P+A+R+NV
\overset{?}{<}
NE.
\]

Ethereum's EIP-8025 provides a current example of the optional-proof migration philosophy, though the experimental PCT should remain chain-agnostic. citeturn21search0

**Genuine PCD prototype — multi-party provenance.**

Construct a three-party branching computation:

```text
        asset / datum A
             │
        proof π0
          /     \
         /       \
   transform B   transform C
      π1             π2
         \           /
          \         /
        combine into D
             π3
```

Each participant should possess private witness material that is **not** shared with later parties. The experiment should compare at least one folding/accumulation approach with a stateless-recursion approach where implementable. The central question is not merely final proof size; it is whether cross-party proof extension can occur without transferring a large private prover state. That is the specific practical limitation Holography accumulation is designed to address. citeturn0search3

**Investigation phase through roughly ninety days.** Produce the formal PCTE schema, canonical test vectors, intent-language semantics, threat model and two chain adapters. Build transfer/swap semantic prototypes and a two-to-three-hop PCD demonstrator. Freeze benchmark hardware and software versions. Establish baseline direct-execution measurements before choosing final performance gates.

The preliminary go criteria should be relative rather than invented absolute numbers:

\[
T^{p95}_{verify,semantic}
\leq
0.10
T^{p95}_{baseline-validation}
\]

for a lightweight semantic checker that adds rather than replaces validation, and for an execution accelerator:

\[
C_{\rm proof}
\leq
0.75\,C_{\rm direct}
\]

on the chosen replicated workload, leaving a 25% margin against benchmark noise and operational overhead. These are proposed engineering gates, not claims about current systems.

The security gate is stricter: **zero false acceptances** are permitted in the deterministic adversarial suite covering mutated transaction body, chain ID, code hash, verifier key, specification, input root, output commitment, expiry, evidence and mandatory-claim root. Passing such a suite demonstrates test coverage, not mathematical absence of bugs.

**Engineering phase through roughly six months.** Add reusable semantic certificates, credential claims, state-access proofs and optional zkVM execution certificates. Run p50/p95/p99 testing under realistic state churn. Commission independent review of statement-binding logic separately from the underlying proof system. Differentially execute every proved transaction against a conventional reference implementation. Implement verifier-version revocation and safe migration.

No consensus change should occur during this phase.

**Deployment-decision phase through roughly twelve months.** Decide separately for each claim type whether it deserves wallet-only, application-enforced or consensus-enforced status. A mandatory consensus claim should advance only if at least two independently maintained verification implementations agree on a large differential corpus, specification/serialization have stable test vectors, upgrade and revocation behavior has been exercised, and its value cannot be obtained more simply by ordinary deterministic checks.

For an execution accelerator, deployment should require observed—not theoretical—break-even under representative validator multiplicity, hardware and state churn.

For genuine PCD, the decisive criterion should be a workload that **actually needs recursive provenance**. If the same business property can be expressed as a small reusable theorem, signature chain or ordinary state commitment, PCD has not earned its complexity.

### Evidence ledger and maturity assessment

| Evidence | Established by evidence | Not established |
|---|---|---|
| Foundational PCD | History-carrying compliance over distributed computation is a formal cryptographic abstraction citeturn22search2 | Production cryptocurrency economics |
| PCD from accumulation | Accumulation is sufficient to construct PCD in the presented framework citeturn0search0 | Universal optimality or cross-party operational suitability |
| Multi-folding PCD | DAG-oriented PCD construction and author benchmarks citeturn0search1 | Independent production benchmark |
| Nova | Efficient folding-based IVC with compression path citeturn1search3 | General stateless multi-party PCD |
| Simulation-extractable PCD | Additional adversarial-composition notions matter and can be constructed under stated conditions citeturn0search2 | That every deployed recursive system has these properties |
| Holography PCD | 2026 construction directly attacks private prover-state transfer/stateful recursion problem citeturn0search3 | Production readiness |
| Neo/SuperNeo | 2026 small-field/PQ-oriented folding research citeturn1search2 | End-to-end post-quantum cryptocurrency PCT |
| TCT | Path-specific reusable semantic theorems are experimentally practical on studied contracts, with very low repo-hit runtime overhead citeturn22search4 | Production consensus over theorem generation |
| RISC Zero | General execution receipts and recursive assumption resolution are implemented citeturn18search0turn18search8 | Correctness/security of arbitrary guest program |
| SP1 | General zkVM technology plus active security/audit process | Absence of soundness defects; multiple high-severity advisories show why this cannot be assumed citeturn20view2turn20view1turn20view0 |
| Zcash Orchard | Protocol-integrated transaction proof covering shielded action bundle; explicitly nonrecursive citeturn23search6 | General PCD; current Zcash history also illustrates proof-system soundness risk citeturn23search9turn23search10 |
| Mina | Proof-authorized account updates and recursive chain-proof architecture citeturn5search1turn5search0 | That every application property follows from proof authorization |
| Midnight | Application architecture based on private computation, public transcripts, verifier keys and transaction ZK proofs citeturn17search2turn23search1 | General recursive PCD; Compact currently forbids recursion |
| Shielded CSV | Direct PCD-based client-side cryptocurrency-validation design citeturn16search0 | Deployed Bitcoin-scale implementation of the paper's performance scenario |
| Ethereum stateless/proof work | Optional execution proofs and binary-tree stateless research are current protocol directions citeturn21search0turn21search1 | Mainnet mandatory proof-based execution |
| Cardano/eUTxO | Explicit input/script architecture plus reference inputs/scripts provide useful PCT bindings; Halo2 Plutus verification has been prototyped citeturn6search0turn6search3turn16search2 | Generic production PCT standard |

The largest unresolved research questions are practical rather than purely cryptographic: how to express user intent without moving the specification bug into the wallet; how to make proof generation survive high state churn; how to prove a dynamic access set is **complete** rather than merely authenticating declared accesses; how to compose heterogeneous properties without hidden inconsistent assumptions; how to move PCD across parties without private prover-state transfer; and how much proof-system complexity a base-layer validator can safely absorb.

### Direct answers to the decision questions

**What can a transaction practically prove today?**

It can practically prove knowledge and authorization, conservation/range relations over private values, correct execution of committed programs, satisfaction of explicitly encoded postconditions, possession of credentials, authenticated membership/nonmembership, correct processing of committed oracle data, bounded computation, and—using recursive systems—correct composition with prior certified computations. Zcash, Mina, Midnight and general-purpose zkVMs provide concrete examples of several of these patterns, while Shielded CSV provides a direct PCD-based client-side cryptocurrency design. citeturn23search6turn5search1turn23search1turn18search0turn16search0

What is **not** automatically practical is a universal transaction theorem asserting arbitrary high-level semantics, current applicability, external-world truth and network-wide safety all at once.

**What important guarantees are missing from ordinary validation?**

The most important missing guarantee is usually **semantic authorization**. Ordinary validation answers “is this transaction allowed by protocol and contract rules?” It often does not answer “does this transaction satisfy the user's independently stated objective?”

Related missing guarantees include application-specific invariant preservation, effect bounds, proof that an off-chain solver followed a required algorithm or constraint set, and privacy-preserving policy satisfaction. TCT's research explicitly targets the gap between code-level execution and higher-level interface specifications. citeturn22search3

**Which additional proofs materially improve safety or correctness?**

The strongest candidates are signed-intent/effect claims, application invariant certificates, narrowly defined private authorization proofs, provenance/history-compliance proofs where assets really move through untrusted parties, and execution proofs when validators would otherwise rely on outsourced computation.

They improve safety only when their claim becomes an **enforced acceptance condition**. A wallet generating a beautiful proof that no validator, contract or recipient is obligated to check does not change consensus safety.

**Which reduce total cost, and which only redistribute it?**

Reusable semantic theorems can have very low amortized checking cost when many transactions share a proven guard/path; TCT provides experimental evidence for this pattern. citeturn22search4

Execution proofs reduce repeated validator work when

\[
W+P+A+R+NV<NE.
\]

They may lower per-validator cost even when this inequality is false, in which case they **redistribute** rather than reduce total computation. Recursive compression similarly saves verifier/network proof overhead but does not make transaction or state data automatically unnecessary. citeturn21search3

Simple local properties should remain ordinary checks when the direct check is cheaper than producing and distributing proof evidence.

**When is recursive PCD necessary rather than a simpler certificate?**

Recursive PCD is justified when the claim itself has recursive provenance:

\[
\text{“this datum is compliant because its predecessors were compliant,
and this transformation preserved compliance.”}
\]

It is especially justified when the history may be arbitrarily long, distributed among mutually distrustful parties or branching into a DAG, and each recipient wants bounded verification cost. That is exactly the setting in the foundational PCD definition and the multi-folding work. citeturn22search2turn0search1

It is **not** necessary to prove a single swap's slippage bound, validate one credential, enforce one transaction's recipient restriction, or authenticate one Merkle state opening. A simple assertion, signature, SNARK or reusable theorem is preferable there.

**Which properties require network-wide enforcement?**

At minimum:

\[
\begin{array}{l}
\text{canonical state selection},\\
\text{double-spend / nullifier uniqueness},\\
\text{account nonce uniqueness where applicable},\\
\text{transaction ordering},\\
\text{global supply invariants if universally claimed},\\
\text{consensus finality},\\
\text{protocol upgrade/version rules}.
\end{array}
\]

Data availability and liveness likewise require network mechanisms rather than a transaction-local validity proof. Ethereum's documentation makes the validity-versus-availability distinction explicit. citeturn21search3

An application-level proof may contribute an inductive step to a global invariant, but network-wide safety follows only if **every** admissible transition is covered and the invariant holds at the base state.

**What should a minimal useful proof-carrying transaction standard contain?**

A first standard should define **claim semantics and binding, not mandate a cryptosystem**.

The minimum is:

```text
PCTE v1
 ├─ domain/version
 ├─ chain/network/protocol identity
 ├─ stable transaction-core commitment
 ├─ signed intent commitment
 ├─ typed claim descriptors
 │    ├─ claim type + version
 │    ├─ mandatory/optional flag
 │    ├─ program/code identity
 │    ├─ specification identity
 │    ├─ verifier/key identity
 │    ├─ state/dependency references
 │    ├─ public-input commitment
 │    ├─ effect/output commitment
 │    ├─ validity interval
 │    └─ evidence commitment
 ├─ explicit dependency graph
 └─ evidence or authenticated sidecar references
```

It should standardize canonical encoding, domain separation, signing rules, proof-stripping behavior, unknown claim handling, size/resource budgets, verifier versioning and test vectors.

The most valuable standardized initial predicates are likely **transfer effects, swap constraints, approval/delegation bounds, application postconditions and credential predicates**. These are semantically understandable, testable and do not require turning the base layer into a universal theorem prover.

**What should not be in the first version?**

The first version should **not** contain a consensus-wide arbitrary-predicate registry, mandatory generic zkVM execution proofs, mandatory recursion, a claim to prove oracle truth, generic “legal compliance,” global MEV fairness, censorship resistance, data availability, universal solver optimality or arbitrary cross-chain finality.

It should also not allow a transaction to select an unconstrained verifier and then claim “proof verified.” The relying policy—not the prover—must define what verifier, specification and claim type have authority.

Most importantly, a first version should not equate:

\[
\boxed{\text{valid proof}}
\]

with

\[
\boxed{\text{safe transaction}}.
\]

The state of the art supports a stronger and more useful architecture:

\[
\boxed{
\begin{array}{c}
\text{ordinary ledger validity}\\
+\\
\text{authenticated user intent}\\
+\\
\text{machine-checkable semantic claims}\\
+\\
\text{fresh state binding}\\
+\\
\text{selective cryptographic/formal evidence}\\
+\\
\text{recursive PCD only where history composition requires it}
\end{array}
}
\]

That architecture can add safety without forcing every transaction through a heavyweight proof system, add privacy where witnesses genuinely need to remain hidden, reduce replicated computation when the measured economics justify it, and reserve the strongest form of proof-carrying data for the problem it uniquely solves: **maintaining verifiable compliance as data and assets move through long, distributed, mutually distrustful computation histories.** citeturn22search2turn16search0
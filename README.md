# Moriarty

Moriarty is an experimental language and toolchain for **bounded financial contracts on Midnight**. Developers describe financial state, permitted actions, payment obligations and authorization rules. The goal is to compile those descriptions into Compact and require each transaction to carry a proof that its execution and the contract history it extends satisfy the agreement.

You can currently author and simulate contracts, inspect their effects, and generate restricted Compact execution kernels. The bounded [loan](deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/RESULT.md) and [swap](deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/RESULT.md) examples have executed on Midnight Preview with independently checked native transfers and historical state. Mandatory proof-carrying financial settlement is still under development. This repository is not a production SDK or an audited deployment.

**Midnight Preview is a hard acceptance gate.** Each supported financial capability needs execution on Preview, retained transaction IDs, finalized native effects and state readback checked against the language semantics. Local tests and compiled artifacts support that evidence. They do not close the network gate; mandatory proof and history checks must also pass before release.

## Financial semantics above Compact

Compact supports Midnight contracts and zero-knowledge circuits. Moriarty adds rules for financial operations: which asset an amount denotes, how interest rounds, when a payment becomes due, what a participant has authorized, and which obligations survive a transaction.

Developers express these rules in Moriarty’s domain-specific language (DSL). Its compiler and evaluator share a typed representation of the agreement. The intended proof system then connects that representation to the state changes and asset movements accepted by the ledger.

For a loan, calculating interest is only part of the work. A payment must discharge the correct debt, reach the authorized creditor and preserve the remaining principal. It must also resist replay. These requirements connect the agreement’s rules to its history and the ledger.

## Financial behavior defines the language

The financial design draws on the following work:

- **ACTUS**, the Algorithmic Contract Types Unified Standards, describes financial contracts through rules for events, state transitions and cash flows. It supplies reference behavior for obligations such as interest, principal repayment and maturity. Moriarty must reproduce the relevant financial behavior, including dates and rounding; naming a contract type is not enough.
- **The DeFi kernel study** is the project’s catalogue of decentralized-finance behaviors: swaps, liquidity, lending and composition. The [study](deliverables/moriarty-design-sprint-2026-09-06/README.md) supplies implementation and conformance requirements; applications do not connect to it as a runtime service.
- **Marlowe** is a financial-contract DSL designed to make contract behavior amenable to analysis. Moriarty adopts the goal of reasoning about an agreement before execution and is developing its authoring, proof and settlement architecture for Midnight.

The common language must accommodate both scheduled financial obligations and transactions authorized by desired outcomes. The [financial target study](deliverables/moriarty-design-sprint-2026-09-06/README.md) explains the source behaviors and their relationship to the proposed semantics. Full conformance remains unfinished.

## Language specification and formal semantics

Moriarty source files use the **`.mori`** extension. The specification separates what a program looks like from what it means:

| Layer | Specification method | What it defines |
| --- | --- | --- |
| Lexical structure | Separate token rules and regular expressions | Identifiers, literals, whitespace, comments and source locations |
| Syntax | EBNF using the ISO/IEC 14977 notation for the provisional successor profile | Valid combinations of declarations, actions and expressions |
| Static semantics | Typing and scoping judgments, illustrated by $\Gamma \vdash e : \tau$ | Name resolution, asset units, resource use and admissible bounds |
| Dynamic semantics | Executable TypeScript operational semantics, plus a bounded K subset for Transfer/Repay | State transitions, financial effects, obligations and rejection. K does not implement Core `/2` reads. |
| Correctness claims | Explicit properties over those semantics | What must be established about an agreement, execution and history |

[EBNF](https://www.iso.org/standard/26153.html) extends BNF with notation for repetition and optionality. It describes the grammar; it does not decide whether the source resembles Lisp or a language with braces. [ABNF, RFC 5234](https://datatracker.ietf.org/doc/html/rfc5234), is another BNF-family notation used for protocol specifications. Moriarty selects EBNF for its source grammar.

[K](https://kframework.org/docs/user_manual/) describes execution through configurations and rewrite rules. It is the selected framework for Moriarty's formal operational semantics. Typing judgments define admissible programs; contract properties and Hoare-style assertions state claims to prove. Denotational models can support particular financial analyses, but do not replace the execution definition.

The existing [atomic grammar](experiments/moriarty-language/spec/grammar.ebnf) and TypeScript evaluator use `moriarty-bounded-atomic/1`. The separate [successor syntax profile](experiments/moriarty-language/spec/successor/README.md) provides lexical rules, a parser and a formatter for `moriarty-successor-syntax/0`. Distinct later entries are `moriarty-expression-source/1`, pure `moriarty-financial-expression-source/1`, and `moriarty-financial-agreement-source/1`, `/2` and `/3`. These profiles are not interchangeable: the loan, swap and Compact workflow later in this README use the atomic profile. The complete `/3` grammar is reproduced below. The historical `/1` expression-source grammar remains at [its canonical file](experiments/moriarty-language/spec/successor/expression-source-grammar.ebnf).

The [bounded repayment K definition](experiments/moriarty-language/formal/k/README.md) supports Transfer-only execution as well as Transfer followed by Repay. The Transfer-only extension is tracked in [its scoped evidence](deliverables/transfer-only-k-2026-09-09/README.md). All 16 frozen cases match the independent financial expectations and the real `.mori` source preparation result, including complete accepted state/effects and exact rejection code/index. [Execution evidence](deliverables/bounded-k-2026-09-09/README.md) records the limited projection, failed attempts and checks.

The [reviewed expression specification](deliverables/sp01-expression-contract-2026-09-10/RESULT.md) now defines all 40 proposed constructors, their typing and reduction rules, and exact representations for finite byte/node bounds. GPT-6 Astra and Grok 4.6 approved that source scope. The [reviewed Boolean revision](deliverables/sp01-surface-core-contract-2026-09-10/RESULT.md) adds short-circuit And/Or rules and valid rejection spans while preserving the original source evidence. The full financial-operation, signing and history contract still needs completion.

The separate [executable expression runtime](deliverables/sp02-expression-runtime-2026-09-10/RESULT.md) now implements all 40 Core constructors and has independent GPT-6 and Grok approval. It checks types before execution, uses exact integer arithmetic, evaluates Boolean branches selectively and publishes no tentative state after rejection. Run `node deliverables/sp02-expression-runtime-2026-09-10/demo-01.mjs` to observe an accepted update and a rejected Ensure. The [reviewed source frontend](deliverables/sp02-expression-source-2026-09-10/RESULT.md) now elaborates actual `.mori` text into this runtime. Run `npm --prefix experiments/moriarty-language run expression-demo` to check and evaluate an ordinary state update from source. The factory accepts a trusted schema and one action; emitted financial operations are descriptors. Source-defined schemas, multiple named actions and a descriptor-to-kernel funded adapter exist in later agreement profiles. K correspondence remains open.

The [financial expression runtime](deliverables/sp02-financial-pure-expression-2026-09-10/RESULT.md) adds eight pure constructors in a separate versioned API, with independent GPT-6 and Grok approval. It supports shares, tagged variants, explicit numeric conversion, conditional values, UInt256 and dimensional arithmetic. The original expression profile remains unchanged. Its vault conversion cases exercise deposit, mint, withdrawal and redemption arithmetic; they do not execute those financial actions or establish their ledger acceptance.

The successor semantic freeze and full SP02/SP03 acceptance remain open. A K definition also needs correspondence arguments connecting it to the evaluator, Compact compiler, proof relation and Midnight ledger acceptance. Archived ZKIR K work does not establish Moriarty semantics.

### Successor source grammar (EBNF)

This is the complete syntax grammar for **`moriarty-financial-agreement-source/3`**, reproduced from the [canonical EBNF](experiments/moriarty-language/spec/successor/financial-agreement-source-v3-grammar.ebnf). It admits uninitialized state, record and operation declarations, multiple named actions, and six generic financial reads. The parser, formatter, checker and local funded evaluator support that profile. Historical full grammars remain separate: [`syntax/0`](experiments/moriarty-language/spec/successor/grammar.ebnf), [`expression-source/1`](experiments/moriarty-language/spec/successor/expression-source-grammar.ebnf), [`financial-expression-source/1`](experiments/moriarty-language/spec/successor/financial-expression-source-grammar.ebnf), [`agreement-source/1`](experiments/moriarty-language/spec/successor/financial-agreement-source-grammar.ebnf) and [`agreement-source/2`](experiments/moriarty-language/spec/successor/financial-agreement-source-v2-grammar.ebnf). The [source contract](experiments/moriarty-language/spec/successor/financial-agreement-source-v3.md) defines the `/3` schema binding, Core `/2` read constructors, diagnostics and work rules. The earlier [expression-source contract](experiments/moriarty-language/spec/successor/expression-source.md) still describes the trusted-schema one-action factory. Complete successor authoring, K correspondence and Preview financial settlement remain open.

The grammar uses **Extended Backus–Naur Form (EBNF)** with ISO/IEC 14977 notation. Production names use letters and digits; they are names in this specification, not Moriarty source keywords.

| Notation | Meaning |
| --- | --- |
| `=` and `;` | Define and end a production |
| `,` | Concatenation: match items in order |
| `\|` | Alternatives: match one choice |
| `[ ... ]` | Optional: zero or one occurrence |
| `{ ... }` | Repetition: zero or more occurrences |
| `( ... )` | Group an EBNF expression |
| `"..."` | Literal source text |
| `? ... ?` | A token or EOF condition defined by the lexical specification |
| `(* ... *)` | A comment in the grammar specification |

Quoted punctuation denotes Moriarty source text. Unquoted punctuation above belongs to EBNF; source comments instead use `//` or `/* ... */`.

The grammar below is the complete `/3` syntax, typeset from the canonical file, which remains authoritative. Its header comment records the fixed conditions: the source header must match exactly and older profiles remain separate; lexical rules are `lexical.md` plus profile-gated `[`, `]` and `?` punctuation; `record` and `operation` are profile-gated declaration spellings; state fields have no initializer; parentheses create no AST nodes; conditional is right-associative below Or; generic and financial-read names are excluded from `ordinaryPrimaryName`; integer lexical tokens remain unsigned canonical decimals with `-` as a separate token; the source contract supplies all type, domain, metadata and diagnostic rules. The simultaneous bounds are 65,536 source UTF-8 bytes, 8,192 tokens, 8,192 AST nodes, nesting depth 64, 256 declarations, 256 statements per action, 64 record fields, 64 ordinary call arguments, 128 collection-intrinsic arguments and 256 action parameters.

<details>
<summary>Verbatim EBNF for copying</summary>

```ebnf
(* ISO/IEC 14977 EBNF for moriarty-financial-agreement-source/3.
   Source header must match exactly; /1, /2 and older profiles remain separate.
   Lexical rules are lexical.md plus the financial-expression punctuation
   [ ] and ?. record and operation are profile-gated declaration spellings
   and are not added to the global keyword catalog.
   State fields in this profile have no initializer.
   Multiple actionDecl forms are admitted in declaration order.
   Zero actions is a semantic SOURCE_ACTION_COUNT rejection.
   Parentheses create no AST nodes. Conditional is right-associative below Or.
   Simultaneous bounds: source65536 bytes, tokens8192, AST8192, depth64,
   declarations256, statements256 per action, record fields64, ordinary call
   arguments64, collection intrinsic arguments128, action parameters256.
   Integer lexical tokens remain unsigned canonical decimals; - is separate.
   financialRead uses typeArgs; exactly one simple generic symbol is a
   static SOURCE_ARITY / SOURCE_TYPE_SHAPE rule, not a parse rejection.
   The source contract supplies all type/domain/metadata and diagnostic rules.
*)

program = profileDecl, agreementDecl, ? end of file ? ;

profileDecl = "profile", stringToken, ";" ;

agreementDecl = "agreement", identifier, "{", { declaration }, "}" ;

declaration = unitDecl
            | partyDecl
            | assetDecl
            | recordDecl
            | operationDecl
            | uninitializedStateDecl
            | actionDecl ;

unitDecl = "unit", identifier, ";" ;

partyDecl = "party", identifier, ";" ;

assetDecl = "asset", identifier, ":", type, ";" ;

recordDecl = "record", identifier, "{", { recordField }, "}" ;

recordField = identifier, ":", type, ";" ;

operationDecl = "operation", identifier, ":", type, ";" ;

uninitializedStateDecl = "state", identifier, ":", type, ";" ;

actionDecl = "action", identifier, "(", [ parameters ], ")",
              "{", { statement }, { postcondition }, "}" ;

parameters = parameter, { ",", parameter } ;

parameter = identifier, ":", type ;

statement = requirement | binding | update | emission ;

requirement = "requires", expression, ";" ;

binding = "let", identifier, "=", expression, ";" ;

update = "next", ".", identifier, "=", expression, ";" ;

emission = "emit", identifier, ( "{", [ effectFields ], "}" | expression ), ";" ;

effectFields = effectField, { ",", effectField } ;

effectField = identifier, ":", expression ;

postcondition = "ensures", expression, ";" ;

type = identifier, [ typeArgs ] ;

typeArgs = "<", typeArgument, { ",", typeArgument }, ">" ;

typeArgument = type | signedInteger ;
signedInteger = [ "-" ], integerToken ;

expression = conditional ;

conditional = disjunction, [ "?", expression, ":", conditional ] ;

disjunction = conjunction, { "or", conjunction } ;

conjunction = negation, { "and", negation } ;

negation = { "not" }, comparison ;

comparison = sum, [ comparisonOp, sum ] ;

comparisonOp = "==" | "!=" | "<" | "<=" | ">" | ">=" ;

sum = product, { ( "+" | "-" ), product } ;

product = postfix, { "*", postfix } ;

postfix = primary, { ".", identifier | "[", expression, "]" } ;

ordinaryPrimaryName = ? identifier token except some, none, collection, quantity, record, amount, shares, variant, project_variant, to_uint, outstanding, principal, accrued, balance, allowance_remaining, allowance_spent ? ;

primary = "-", integerToken
        | integerToken
        | stringToken
        | "true"
        | "false"
        | ordinaryPrimaryName, [ "(", [ arguments ], ")" ]
        | genericCall
        | dynamicOrLiteral
        | financialGeneric
        | financialRead
        | recordLiteral
        | "(", expression, ")" ;

genericCall = ( "some" | "none" | "collection" | "quantity" ),
              "<", typeArgument, { ",", typeArgument }, ">",
              "(", [ arguments ], ")" ;

dynamicOrLiteral = ( "amount" | "shares" ), [ typeArgs ],
                   "(", [ arguments ], ")" ;

financialGeneric = ( "variant" | "project_variant" | "to_uint" ),
                   typeArgs, "(", [ arguments ], ")" ;

financialRead = ( "outstanding" | "principal" | "accrued" | "balance"
                | "allowance_remaining" | "allowance_spent" ),
                typeArgs, "(", [ arguments ], ")" ;

recordLiteral = "record", "<", type, ">", "{", [ effectFields ], "}" ;

arguments = expression, { ",", expression } ;

identifier = ? ASCII identifier token defined in lexical.md ? ;

integerToken = ? canonical unsigned decimal token defined in lexical.md ? ;

stringToken = ? JSON string token defined in lexical.md ? ;
```

</details>

The [lexical specification](experiments/moriarty-language/spec/successor/lexical.md) defines the referenced tokens and their source spans. Identifiers match `[A-Za-z][A-Za-z0-9_]*`, are case-sensitive, and cannot be keywords. Integers are canonical unsigned decimals (`0` or a nonzero digit followed by digits); strings use JSON escapes and must decode to Unicode scalar values. Only ASCII space, tab, CR and LF separate tokens as whitespace. Source comments use `//` or non-nesting `/* ... */`; a bare `/` is not an operator. Input must be valid UTF-8, with no BOM stripping or Unicode normalization.

The [expression-source bounds](experiments/moriarty-language/spec/successor/expression-source.md) extend the base [syntax bounds](experiments/moriarty-language/spec/successor/syntax-profile.json) and apply together: 65,536 source UTF-8 bytes, 64 ASCII characters per identifier, 1,024 decoded UTF-8 bytes per string, 78 digits per integer, 8,192 tokens including EOF, 8,192 AST nodes, nesting depth 64, 256 declarations, 256 statements per action including `ensures`, 256 action parameters, 128 collection-intrinsic arguments, and 64 ordinary call arguments or effect fields. These parser limits are separate from the atomic execution bounds and from financial runtime limits.

Syntax acceptance does not imply execution. The [bounded funded source profile](experiments/moriarty-language/spec/successor/funded-source.md), `moriarty-funded-source/0`, uses the older `moriarty-successor-syntax/0` source header and [grammar](experiments/moriarty-language/spec/successor/grammar.ebnf) through a separate preparation API. It supports a typed subset of unit, party, asset and action declarations with explicit `Transfer` and `Repay` emissions, producing local `Prepared` candidates. It rejects state and const declarations, `requires`, `let`, `next`, `ensures`, projections and operators, although those forms appear in this grammar. Its numeric values are limited to UInt128. The [funded example](experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori) exercises that subset; the separate [syntax-only example](experiments/moriarty-language/spec/successor/examples/partial-payment.mori) does not implement a funded payment. Neither parsing nor local preparation establishes authorization, a proof, ledger acceptance or source/Core/K correspondence.

### Small-step semantics (implemented repayment subset)

This presentation uses **Felleisen–Hieb reduction semantics**: terms, evaluation contexts, primitive contractions, context closure and terminal answers. Felleisen and Hieb's [*Revised Report*](https://plv.mpi-sws.org/plerg/papers/felleisen-hieb-92-2up.pdf), §2 Definitions 2.1/2.3 and §3.1, supplies the presentation method; its metatheorems are not claims about Moriarty. The [downloaded open textbooks and Redex corpus](deliverables/reduction-semantics-textbooks-2026-09-09/CORPUS.md) and [source-linked practices](deliverables/reduction-semantics-textbooks-2026-09-09/BEST-PRACTICES.md) provide further references.

The [K definition](experiments/moriarty-language/formal/k/moriarty.k) implements the provisional **`moriarty-funded-repayment/0`** projection: one Transfer, optionally followed by one Repay; two initial balances, one allowance, one obligation and empty used-ID lists. It supports `AccrualFirst`, `PrincipalFirst` and `ProRata`, with explicit `none`, `floor` or `ceil` conversion rounding. The [numeric extension's evidence](deliverables/numeric-k-2026-09-09/README.md) records its execution and review status. Full successor expressions, arbitrary action sequences and correspondence proofs remain open.

The codec admits closed records and canonical UInt128 fields. K checks positive conversion mantissa and scale at most 18 before reaching the stage that computes powers or division. Invalid financial values such as zero mantissa or a huge scale reach K and reject; malformed records and unsupported collection/action shapes stop at the codec. Raw K terms outside this admitted domain are not covered.

**Terms and contexts.** $P$ is a packet and $H$ its codec-supplied digest. $s, r$ are sender/receiver row indices ($-1$ means absent). $x, q, u, c, v$ are intermediate integers. $\varepsilon$ is empty computation, $\rhd$ sequencing, and $\square$ one context hole. This is explanatory notation, not raw K syntax.

```math
\begin{array}{llcl}
\text{Instruction} & a & ::= & \mathsf{start}(P) \mid \mathsf{inspect}(P,s,r) \mid \mathsf{ensure}(H,b,\mathit{code},i) \\
& & \mid & \mathsf{convert}(P,s,r) \mid \mathsf{divide}(P,s,r,x) \mid \mathsf{round}(P,s,r,q,u) \\
& & \mid & \mathsf{fund}(P,s,r,c) \mid \mathsf{allocate}(P,s,r,c) \mid \mathsf{split}(P,s,r,c) \\
& & \mid & \mathsf{finishT}(P,s,r) \mid \mathsf{finishR}(P,s,r,c,v) \\[6pt]
\text{Computation} & k & ::= & \varepsilon \mid a \mid k \rhd k \\[6pt]
\text{Context} & E & ::= & \square \mid E \rhd k \\[6pt]
\text{Program} & t & ::= & \mathsf{run}_H(k) \mid \mathsf{prepared}(H,F) \mid \mathsf{rejected}(H,\mathit{code},i)
\end{array}
```

Sequences are identified up to associativity and the two unit equations $\varepsilon \rhd k = k = k \rhd \varepsilon$, so contexts select the first pending instruction and retain its suffix. There is no $k \rhd E$ context that skips an unfinished instruction, and no context enters packet data or terminal answers. Only programs reachable from $\mathsf{run}_H(\mathsf{start}(P))$ for admitted $P$ are in the domain.

**Pure helpers and guards.** Within a repayment rule, $n$ is nominal payment, $m$ conversion mantissa, $\ell$ scale, $\rho$ rounding mode, $p, a$ principal/accrued debt, $T$ transferred cash and $\lambda$ allocation rule, all read from $P$. Let $U = 2^{128} - 1$ and write $g(b, \mathit{code})$ for $\mathsf{ensure}(H, b, \mathit{code}, 1)$. $R(\rho, q, u)$ returns $q$, except that ceil with nonzero remainder returns $q + 1$.

```math
R(\rho,q,u) =
\begin{cases}
q + 1 & \rho = \mathtt{ceil} \text{ and } u \neq 0 \\
q & \text{otherwise}
\end{cases}
```

```math
\mathit{DP}(\lambda,n,p,a) =
\begin{cases}
n - \min(n,a) & \lambda = \mathtt{AccrualFirst} \\
\min(n,p) & \lambda = \mathtt{PrincipalFirst} \\
\left\lfloor \dfrac{n\,p}{p+a} \right\rfloor & \lambda = \mathtt{ProRata}
\end{cases}
```

The ProRata helper is used only after a positive outstanding obligation and a fitting $n\,p$ product have passed their guards. Integer division therefore has a positive denominator. Helpers abstract internal K equational steps; they are not financial action-work charges.

**Primitive contractions.** $\mathit{checks}(P,s,r)$ contains 15 ordered guards for Transfer-only or 21 initial guards for repayment. $\mathit{tail}(P,s,r)$ is $\mathsf{finishT}$ or $\mathsf{convert}$, respectively. Both are exact instruction-list abbreviations, not new program constructors. In the SPLIT rule, $d$ abbreviates $\mathit{DP}(\lambda,n,p,a)$.

```math
\begin{array}{rcll}
\mathsf{start}(P) & \leadsto & \mathsf{inspect}(P,\mathit{sender}(P),\mathit{receiver}(P)) & \text{(START)} \\[4pt]
\mathsf{inspect}(P,s,r) & \leadsto & \mathit{checks}(P,s,r) \rhd \mathit{tail}(P,s,r) & \text{(EXPAND)} \\[4pt]
\mathsf{ensure}(H,\mathsf{true},\mathit{code},i) & \leadsto & \varepsilon & \text{(CHECK)} \\[10pt]
\mathsf{convert}(P,s,r) & \leadsto & g(n\,m \le U,\ \mathtt{OVERFLOW}) \rhd \mathsf{divide}(P,s,r,n\,m) & \text{(CONVERT)} \\[4pt]
\mathsf{divide}(P,s,r,x) & \leadsto & \mathsf{round}\bigl(P,s,r,\ x\ \mathrm{div}\ 10^{\ell},\ x \bmod 10^{\ell}\bigr) & \text{(DIVIDE)} \\[4pt]
\mathsf{round}(P,s,r,q,u) & \leadsto & g(\rho \neq \mathtt{none} \lor u = 0,\ \mathtt{INEXACT\_CONVERSION}) & \text{(ROUND)} \\
& & \quad \rhd\ \mathsf{fund}(P,s,r,R(\rho,q,u)) & \\[4pt]
\mathsf{fund}(P,s,r,c) & \leadsto & g(c \le U,\ \mathtt{OVERFLOW}) \rhd g(c > 0,\ \mathtt{DUST}) & \text{(FUND)} \\
& & \quad \rhd\ g(c \le T,\ \mathtt{INSUFFICIENT\_UNALLOCATED}) & \\
& & \quad \rhd\ \mathsf{allocate}(P,s,r,c) & \\[4pt]
\mathsf{allocate}(P,s,r,c) & \leadsto & g(\lambda \neq \mathtt{ProRata} \lor n\,p \le U,\ \mathtt{OVERFLOW}) & \text{(ALLOCATE)} \\
& & \quad \rhd\ \mathsf{split}(P,s,r,c) & \\[4pt]
\mathsf{split}(P,s,r,c) & \leadsto & g(d \le p \land n - d \le a,\ \mathtt{ALLOCATION\_COMPONENT}) & \text{(SPLIT)} \\
& & \quad \rhd\ \mathsf{finishR}(P,s,r,c,d) &
\end{array}
```

The stage boundaries matter: a huge scale cannot trigger exponentiation before its state guard, and a product that exceeds UInt128 cannot be divided first to obtain an apparently fitting answer. The definition uses K instructions, rather than eager numeric helpers over the whole future continuation, for these boundaries.

**Contextual and terminal reductions.** $\to$ is generated by context closure and the following whole-program rules. ABORT discards the entire continuation, including finalization; rejected output has no tentative state or effects.

```math
\frac{a \leadsto k'}{\mathsf{run}_H(E[a]) \to \mathsf{run}_H(E[k'])}\ \text{(CONTEXT)}
```

```math
\begin{array}{rcll}
\mathsf{run}_H\bigl(E[\mathsf{ensure}(H,\mathsf{false},\mathit{code},i)]\bigr) & \to & \mathsf{rejected}(H,\mathit{code},i) & \text{(ABORT)} \\[6pt]
\mathsf{run}_H(\mathsf{finishT}(P,s,r)) & \to & \mathsf{prepared}(H,\mathit{FT}(P,s,r)) & \text{(PREPARE-T)} \\[4pt]
\mathsf{run}_H(\mathsf{finishR}(P,s,r,c,v)) & \to & \mathsf{prepared}(H,\mathit{FR}(P,s,r,c,v)) & \text{(PREPARE-R)}
\end{array}
```

$\mathit{FT}$ and $\mathit{FR}$ abbreviate the exact distinct scalar records returned by K's `preparedTransfer` and `prepared` constructors. Both include the input digest and receiver index. The codec reconstructs complete unchanged metadata and ordered effects. For Transfer-only, unchanged debt and allocation history are copied by the codec: they are not separately computed debt outputs or a proved K invariant. No context reduces inside $\mathsf{prepared}$ or $\mathsf{rejected}$.

Checks run in this order; the [K rules](experiments/moriarty-language/formal/k/moriarty.k) specify every predicate.

| Stage | Ordered checks | Error index |
| --- | --- | --- |
| State | Distinct balance keys; allowance sum; positive conversion mantissa and scale ≤18; debt sum/status; total work including reserve | `−1` (decoded as null) |
| Work | Ordinary remaining work covers one or two actions; spent work can increase by that count | `−1` |
| Transfer | Positive amount; distinct parties; sender exists and covers cash; exact allowance exists and covers cash; allowance-spent and receiver additions fit | `0` |
| Repay, when present | Positive nominal amount; matching obligation; outstanding status; nominal ≤ outstanding; prior transfer ID; matching payer/creditor/asset | `1` |
| Conversion | Product fits; `none` has zero remainder; rounded amount fits and is positive; converted cash ≤ transferred cash | `1` |
| Allocation | ProRata product fits; discharged principal/accrual fit their existing components | `1` |

A successful Transfer-only moves cash and gross allowance, appends only its Transfer ID, emits one Transfer and charges **one action-work unit**. It preserves the complete obligation, including a settled ProRata obligation, because no allocation occurs. A non-creditor recipient is legal for this operation.

A successful repayment charges **two action-work units**, emits Transfer then Repayment, and appends both IDs. With computed settlement $c$ and principal discharge $v$, the obligation becomes

```math
p' = p - v, \qquad a' = a - (n - v), \qquad o' = p + a - n, \qquad \text{Settled} \iff o' = 0 .
```

Only nominal $n$ discharges debt; cash $T$ and settlement $c$ have distinct roles. Closure reserve is unchanged.

For example, ProRata with principal 100, accrued 10 and nominal payment 7 gives $v = \lfloor 700/110 \rfloor = 6$, leaving principal 94, accrued 9 and outstanding 103. At mantissa 3, scale 1 and floor rounding, settlement is $\lfloor 21/10 \rfloor = 2$: transferring 2 can fund that nominal payment. `none` would reject the same fractional conversion; floor producing zero cash rejects DUST.
**Presentation bound.** Assuming pure helper termination on the admitted domain, successful Transfer-only takes **18 control steps** (START, EXPAND, 15 guards, PREPARE-T). Successful repayment takes **37** (eight stage contractions, 28 guards, PREPARE-R). Rejection ends at its first false guard and discards the remaining stages. Unlike the earlier single-expansion presentation, the failure step count depends on which numeric stages were reached. These are control-presentation counts, not internal K rewrites, execution fees or a mechanized termination proof.

The [initial execution](deliverables/bounded-k-2026-09-09/README.md), [additional repayment branches](deliverables/repayment-k-branches-2026-09-09/README.md), and [Transfer-only extension](deliverables/transfer-only-k-2026-09-09/README.md) retain their original source-bound evidence and earlier control counts. The numeric suite combines all 42 prior distinct inputs with 22 new independent cases. Finite comparisons do not establish full source/Core/K correspondence, SP03 completion, mandatory PCD or finalized financial settlement. `Prepared` remains a local proposal, not authorization or Midnight acceptance.

### Expression small-step semantics (40-constructor Core)

The [expression contract](experiments/moriarty-language/spec/successor/semantic-contract.md) and its [static semantics](experiments/moriarty-language/spec/successor/static-semantics.md) define the 40-constructor Core that `moriarty-expression-source/1` text elaborates to. Both documents are proposals with independent design approval; neither is a registered profile. The presentation below restates their §E and §S0 as a reduction system in the same explanatory notation as the repayment section. The contract's prose remains authoritative, the [runtime](deliverables/sp02-expression-runtime-2026-09-10/RESULT.md) is TypeScript rather than K, and no correspondence between the two is proved.

**Terms and frames.** An action is a statement list followed by an Ensure suffix. $K(e_1,\ldots,e_n)$ is an unentered constructor occurrence and $K^{\circ}$ the same occurrence once entered; $v$ ranges over values. The frame $\sigma = \langle \mathit{pre}, W, L, A, O, D \rangle$ holds the immutable pre-state, staged writes, locals, arguments, observations and emitted descriptors. $w$ is remaining work and $w_0$ the initial budget. Expression configurations are $\langle e, w\rangle$ and reduce under a fixed frame $\sigma$, which reads consult and which no expression rule changes; statement configurations are $\langle S, \sigma, w\rangle$. Every occurrence keeps its original span and node path.

```math
\begin{array}{llcl}
\text{Statement} & s & ::= & \mathsf{Require}(e) \mid \mathsf{Let}(x,e) \mid \mathsf{NextWrite}(f,e) \mid \mathsf{Emit}(O,e) \\[4pt]
\text{Suffix} & q & ::= & \mathsf{Ensure}(e) \\[4pt]
\text{Action} & \mathcal{A} & ::= & s_1;\ldots;s_m;\ q_1;\ldots;q_k \\[4pt]
\text{Context} & C & ::= & [\,] \\
 & & \mid & K^{\circ}(v_1,\ldots,v_{i-1},\,C,\,e_{i+1},\ldots,e_n) \qquad K \notin \{\mathsf{And},\mathsf{Or}\} \\
 & & \mid & \mathsf{And}^{\circ}(C,e_2) \;\mid\; \mathsf{Or}^{\circ}(C,e_2)
\end{array}
```

In the generic alternative $i$ ranges over the constructor's evaluated operands in listed order; metadata operands are never evaluated. There is no $\mathsf{And}^{\circ}(v,C)$ or $\mathsf{Or}^{\circ}(v,C)$ context: the right operand is reached only by the contractions below.

**Entry and context closure.** Entering an occurrence costs one unit of work. An occurrence that cannot be entered rejects without consuming work.

```math
\frac{w > 0}{\langle K(e_1,\ldots,e_n),\ w\rangle \to \langle K^{\circ}(e_1,\ldots,e_n),\ w-1\rangle}\ \text{(E-ENTER)}
\qquad
\frac{\langle e,\ w\rangle \to \langle e',\ w'\rangle}{\langle C[e],\ w\rangle \to \langle C[e'],\ w'\rangle}\ \text{(E-CONTEXT)}
```

```math
\langle C[K(e_1,\ldots,e_n)],\ 0\rangle \to \mathsf{Rejected}(\mathtt{WORK\_EXHAUSTED},\ \mathrm{span}(K),\ \mathrm{path}(K),\ w_0)\ \text{(E-EXHAUSTED)}
```

**Primitive rules.** For $K$ other than $\mathsf{And}$ and $\mathsf{Or}$, once every evaluated child has returned a value the constructor's rule $\mathcal{R}_K$ from the static semantics either yields a value or a rejection code. A rejection discards the entire continuation, staged writes, locals and descriptors; the reported work is the work actually consumed.

```math
\frac{\mathcal{R}_K(v_1,\ldots,v_n) = v}{\langle K^{\circ}(v_1,\ldots,v_n),\ w\rangle \to \langle v,\ w\rangle}\ \text{(E-PRIM)}
```

```math
\frac{\mathcal{R}_K(v_1,\ldots,v_n) = \mathtt{code}}{\langle C[K^{\circ}(v_1,\ldots,v_n)],\ w\rangle \to \mathsf{Rejected}(\mathtt{code},\ \mathrm{span}(K),\ \mathrm{path}(K),\ w_0 - w)}\ \text{(E-PRIM-FAIL)}
```

Every rejection raised before the first E-ENTER, by admission, schema validation or typing, reports work used 0. A span that is absent, malformed or out of range is reported as the synthetic span $[0,0)$ with the original node path kept; it is never clamped. An isolated pure expression, evaluated outside an action, ends in $\mathsf{ExpressionValue}(T, v, w)$ rather than a statement result.

**Boolean contractions.** After the left operand returns, the four contractions fire at zero cost. A skipped right operand is never entered and produces no work or failure. A selected right operand resumes as the original occurrence at child index 1 with its own span; if it is unentered and $w = 0$, E-EXHAUSTED applies to it.

```math
\begin{array}{rcll}
\mathsf{And}^{\circ}(\mathsf{false},\ e_2) & \to & \mathsf{false} & \text{(E-AND-F)} \\[2pt]
\mathsf{And}^{\circ}(\mathsf{true},\ e_2) & \to & e_2 & \text{(E-AND-T)} \\[2pt]
\mathsf{Or}^{\circ}(\mathsf{true},\ e_2) & \to & \mathsf{true} & \text{(E-OR-T)} \\[2pt]
\mathsf{Or}^{\circ}(\mathsf{false},\ e_2) & \to & e_2 & \text{(E-OR-F)}
\end{array}
```

**Statements.** Statements reduce in lexical order; each returns Unit and advances to the next. A statement's expression operand reduces by the E-rules under the current $\sigma$, with $w$ threaded through. $S$ is the remaining statement list, $\oplus$ overrides a record by staged writes, and $D \cdot d$ appends a descriptor. $\mathsf{ReadPre}(\mathsf{post},f)$ reads $\mathit{pre} \oplus W$ and is typable only inside an Ensure condition; that view never commits. EMIT also rejects `DESCRIPTOR_BOUND` when the descriptor list exceeds its aggregate bound, and FINISH rejects `VALUE_BOUND` when the assembled post-state exceeds its bound.

```math
\begin{array}{rcll}
\langle \mathsf{Require}^{\circ}(\mathsf{true});\ S,\ \sigma,\ w\rangle & \to & \langle S,\ \sigma,\ w\rangle & \text{(S-REQUIRE)} \\[2pt]
\langle \mathsf{Require}^{\circ}(\mathsf{false});\ S,\ \sigma,\ w\rangle & \to & \mathsf{Rejected}(\mathtt{GUARD\_FAILED},\ldots) & \text{(S-REQUIRE-FAIL)} \\[2pt]
\langle \mathsf{Let}^{\circ}(x,\ v);\ S,\ \sigma,\ w\rangle & \to & \langle S,\ \sigma[L \mathrel{+}= x \mapsto v],\ w\rangle & \text{(S-LET)} \\[2pt]
\langle \mathsf{NextWrite}^{\circ}(f,\ v);\ S,\ \sigma,\ w\rangle & \to & \langle S,\ \sigma[W \mathrel{+}= f \mapsto v],\ w\rangle & \text{(S-NEXT)} \\[2pt]
\langle \mathsf{Emit}^{\circ}(O,\ v);\ S,\ \sigma,\ w\rangle & \to & \langle S,\ \sigma[D \mathrel{:=} D \cdot \mathsf{Operation}_O(v)],\ w\rangle & \text{(S-EMIT)} \\[2pt]
\langle \mathsf{Ensure}^{\circ}(\mathsf{true});\ S,\ \sigma,\ w\rangle & \to & \langle S,\ \sigma,\ w\rangle & \text{(S-ENSURE)} \\[2pt]
\langle \mathsf{Ensure}^{\circ}(\mathsf{false});\ S,\ \sigma,\ w\rangle & \to & \mathsf{Rejected}(\mathtt{ENSURES\_FAILED},\ldots) & \text{(S-ENSURE-FAIL)} \\[2pt]
\langle \varepsilon,\ \sigma,\ w\rangle & \to & \mathsf{ExpressionPrepared}(\mathit{pre} \oplus W,\ D,\ w) & \text{(S-FINISH)}
\end{array}
```

**Work.** With $C(e)$ the work actually consumed by a successful pure expression and $B(e)$ the count of every constructor occurrence including unselected branches:

```math
\begin{array}{rcl}
C(\mathsf{And}(e_1,e_2)) & = & 1 + C(e_1) + [\,e_1 \Downarrow \mathsf{true}\,]\ C(e_2) \\[2pt]
C(\mathsf{Or}(e_1,e_2)) & = & 1 + C(e_1) + [\,e_1 \Downarrow \mathsf{false}\,]\ C(e_2) \\[2pt]
B(K(e_1,\ldots,e_n)) & = & 1 + \sum_i B(e_i)
\end{array}
```

$B$ is finite under the simultaneous source and Core bounds and conservatively bounds reduction work; it estimates neither validation overhead nor native proof cost.

**Typing judgment.** Admission precedes reduction: structure, schema, whole-action typing and snapshot validation all complete before the first E-ENTER, so a static error anywhere in the action precedes any runtime guard. The judgment is $\Sigma;\Gamma;\phi \vdash e : T$ with $\Sigma$ the bounded acyclic schema registry, $\Gamma = L \uplus A \uplus O \uplus F$ four disjoint namespaces, and $\phi$ the phase, $\mathsf{ensure}$ inside an Ensure condition and $\mathsf{stmt}$ elsewhere. Type equality is exact, including every index; there is no subtyping or implicit cast. Four representative rules:

```math
\frac{n \in \{64,128\} \qquad 0 \le v < 2^{n}}{\Sigma;\Gamma;\phi \vdash \mathsf{LitUInt}(n,\ v) : \mathsf{UInt}_{n}}\ \text{(L-U)}
```

```math
\frac{F[f] = T \qquad \mathit{view} = \mathsf{pre}\ \lor\ (\mathit{view} = \mathsf{post} \land \phi = \mathsf{ensure})}{\Sigma;\Gamma;\phi \vdash \mathsf{ReadPre}(\mathit{view},\ f) : T}\ \text{(READ-PRE)}
```

```math
\frac{\Sigma;\Gamma;\phi \vdash e_1 : T \qquad \Sigma;\Gamma;\phi \vdash e_2 : T \qquad T \in \{\mathsf{UInt64},\mathsf{UInt128},\mathsf{SInt128}\}}{\Sigma;\Gamma;\phi \vdash \mathsf{Add}(e_1,\ e_2) : T}\ \text{(ARITH-ADD-SCALAR)}
```

```math
\frac{\Sigma;\Gamma;\phi \vdash e_1 : \mathsf{Bool} \qquad \Sigma;\Gamma;\phi \vdash e_2 : \mathsf{Bool}}{\Sigma;\Gamma;\phi \vdash \mathsf{And}(e_1,\ e_2) : \mathsf{Bool}}\ \text{(CMP-AND)}
```

A `next` view rejects `TYPE_NEXT_READ` and a `post` view outside Ensure rejects `TYPE_POST_SCOPE`. ARITH-ADD-SCALAR reduces to the mathematical sum and rejects `ARITH_RANGE` when the result does not fit $T$; there is no widening then truncation. Add and Sub also admit two operands of the same indexed type Amount, Shares, Rate or Quantity with exactly equal indices, operating on the underlying quanta or mantissa with the same range check. CMP-AND checks both operands statically regardless of which the contractions later select. The complete rules for all 40 constructors, with every overload and rejection code, are in [static-semantics.md](experiments/moriarty-language/spec/successor/static-semantics.md), mirrored machine-readably in [expression-signatures.json](experiments/moriarty-language/spec/successor/expression-signatures.json).

**Funded composition (TypeScript).** S-FINISH still ends at $\mathsf{ExpressionPrepared}$. The later funded adapter maps emitted `Transfer` and `Repay` descriptors onto the retained repayment kernel, charges expression work E plus kernel action count N, and publishes ordinary `post` with complete `financialPost` and effects only after that kernel call succeeds. Ordinary `ensures` see ordinary post-state. The `/3` financial reads see the same validated financial pre-state throughout the action, including after `emit`; descriptors do not mutate that projection. This is executable TypeScript semantics. The bounded K kernel above implements Transfer/Repay on a limited projection and does not implement the new read constructors. A candidate mapping is also recorded in the unregistered [composition proposal](experiments/moriarty-language/spec/successor/composition-proposal.md); it does not replace the adapter.

**Typed financial reads.** Agreement source `/3` elaborates six generic reads to Core `/2` constructors. Each constructor has one declared unit or asset operand and one `Text` identity expression. Extra generic arguments parse; static checking requires exactly one simple declared symbol (`SOURCE_ARITY` / `SOURCE_TYPE_SHAPE`).

| Source | Core | Result |
| --- | --- | --- |
| `outstanding<U>(e)` | `ReadOutstanding` | `Quantity<Units<U,1>,0>` |
| `principal<U>(e)` | `ReadPrincipal` | `Quantity<Units<U,1>,0>` |
| `accrued<U>(e)` | `ReadAccrued` | `Quantity<Units<U,1>,0>` |
| `balance<A>(e)` | `ReadBalance` | `Amount<A>` |
| `allowance_remaining<A>(e)` | `ReadAllowanceRemaining` | `Amount<A>` |
| `allowance_spent<A>(e)` | `ReadAllowanceSpent` | `Amount<A>` |

$U$ is a declared unit and $A$ a declared asset. $\mathit{id}$ is the reduced `Text` identity. $\sigma_F$ is the immutable admitted financial pre-state; no read or `emit` writes it.

```math
\frac{U \in \Sigma.\mathit{units} \qquad \Sigma;\Gamma;\phi \vdash e : \mathsf{Text}}{\Sigma;\Gamma;\phi \vdash \mathsf{ReadOutstanding}(U,e) : \mathsf{Quantity}(\mathsf{Units}(U,1),0)}\ \text{(READ-OUTST)}
```

```math
\frac{A \in \Sigma.\mathit{assets} \qquad \Sigma;\Gamma;\phi \vdash e : \mathsf{Text}}{\Sigma;\Gamma;\phi \vdash \mathsf{ReadBalance}(A,e) : \mathsf{Amount}(A)}\ \text{(READ-BAL)}
```

Principal and accrued use the outstanding rule with their fields. Allowance remaining and spent use the balance rule with the matching allowance metric. Evaluation admits complete kernel state before any expression reduction. Reduction evaluates the identity once, looks up $\sigma_F$, and leaves $\sigma_F$ unchanged. It rejects `INVALID_IDENTIFIER` for a non-kernel identifier, `MISSING_OBLIGATION` / `MISSING_BALANCE` / `MISSING_ALLOWANCE` rather than defaulting to zero, `NOMINAL_UNIT` when an obligation denomination differs from $U$, and `ARITH_RANGE` when a Quantity exceeds signed128. Short-circuit `and` / `or` / `?` skips unselected runtime lookups; static checking still visits every branch and action. One read costs one reduction plus its identity child's actual reductions. Successful debit is E + N. These rules are TypeScript executable semantics. The bounded K kernel does not implement the new constructors.

## What a developer writes

The following examples and local workflow use `moriarty-bounded-atomic/1`; consult the successor profiles above for their separate syntax and funded subset.

An agreement is a source program; a contract instance gives that program its own state and participant bindings. An agreement declares typed state, observations, actions and effects. Actions contain guards, local calculations, state updates and explicit financial effects. Policies associate financial calculations with their rounding rules and required correctness claims.

An **obligation** is a duty that survives a transaction, such as an unpaid amount due. An **effect** records an action’s financial result: a transfer, fee, newly created due or settlement of a due. Recording an effect in the simulator does not move ledger assets.

**Observations** are external inputs such as time or a price. An instance’s initial configuration binds each observation to a provider and authentication policy. The demo uses a simulated clock; real observation authentication remains unfinished.

Amounts have named units. An amount of one asset cannot be added to another asset accidentally. Intermediate arithmetic is checked, including multiplication before division; overflow rejects rather than wrapping. Settlement bindings specify how nominal amounts convert into ledger asset quantities.

For example, this excerpt from the [swap agreement](experiments/moriarty-language/spec/examples/swap.mori) calculates output from pool reserves, applies a fee factor and checks the trader's minimum output:

```text
let effective_input = arg.amount_in * const.fee_numerator;
let numerator = effective_input * state.reserve_b;
let denominator = state.reserve_a * const.fee_denominator + effective_input;
let output_calculated = floor_div(numerator, denominator);
```

```text
guard arg.min_out <= output_calculated, "minimum output not met";
```

`arg`, `state` and `const` refer to action arguments, instance state and declared constants; `let` introduces an action-local value. Multiplication and division combine units, while addition and comparison require compatible types and units.

The complete agreement supplies the declarations, policies, remaining guards, state updates and transfers. The supplied loan and swap examples are bounded reference scenarios with fixed expectations, rather than deployable lending products or general-purpose exchanges. A policy's named proof claim is a requirement to discharge, not a proof merely because it appears in the source.

### Finite execution

Moriarty is designed to be Turing-incomplete. The current language has no loop construct or source recursion. Its registered profile bounds values, intermediate arithmetic, expression depth, collection sizes, work per action, contract lifetime and time horizon. Successive actions consume the contract's remaining execution allowance. At exhaustion, further actions reject. The frontend admits the exact registered [bounds document](experiments/moriarty-language/spec/bounds.json) by content hash; editing its limits or even reformatting its JSON is not a supported configuration change.

Execution limits do not cancel financial obligations. The loan example separates creating amounts due from settling them. Its demonstrated episode closes after those dues are settled, with 4,500 USD of principal still outstanding. Continuing that agreement requires an explicit mechanism that preserves obligations and lifecycle constraints; that continuation is not implemented.

These limits make termination and resource obligations explicit. They do not automatically prove that a financial agreement is correct, that all states are practical to enumerate, or that its proof fits a particular circuit. Those are separate claims to establish.

### Exact plans and outcome intents

Authorization has two forms:

- An **exact plan** fixes the action, state writes and effects that a participant authorizes.
- An **outcome intent** permits a plan to be chosen within constraints: maximum gross spending, minimum net receipts, allowed actions, recipients and calls. A solver is the component that proposes such a plan.

The local evaluator checks these constraints. Refunds do not erase gross spending, and fees count when checking net receipts. Cryptographic authorization and durable replay protection still need to be connected to ledger acceptance; the demo supplies simulated authentication.

## From source to settlement

The intended workflow is:

```mermaid
flowchart LR
    Source[Agreement source] --> Core[Checked typed Core]
    Core --> Simulation[Local simulation]
    Core --> Compact[Compact compilation]
    Simulation --> Plan[Proposed state and effects]
    Plan -.-> Proof[Authorization and history proof]
    Compact -.-> Acceptance[Ledger acceptance]
    Proof -.-> Acceptance
    Acceptance -.-> Settlement[Finalized state and asset movements]
```

Solid arrows show the implemented path. Dashed arrows show the proof and settlement integrations still to build.

The **Core** is the compiler's explicit representation of the agreement's operations. Source locations, semantic versions, canonical encodings and hashes connect it to the source and registered bounds. The evaluator derives a candidate state, obligation changes and ordered effects from that representation.

The current Compact mapper translates a restricted subset into pure execution kernels. The generated kernels are pure Compact circuits with no persistent ledger declarations or witness functions. They take state, arguments, observations, lifecycle allowances and arithmetic hints as inputs, then return the next numeric state and effect operands. The circuits constrain the hints; supplying a quotient or multiplication limb does not make it trusted. Their test wrappers store results for comparison; they do not move assets or implement the full acceptance protocol. Persistent text, text observations and lowering of the source `and`/`or` operators are among the current mapping restrictions. See the [mapping contract](experiments/moriarty-language/compact/MAPPING.md).

The remaining settlement adapter must bind those calculations to actual ledger inputs, outputs, custody and recipients. Comparing a final balance alone is insufficient: every relevant debit, credit, fee, change output and obligation must be accounted for.

## Proof-carrying transactions

**Proof-carrying data (PCD)** means a piece of data carries evidence that it was produced according to specified rules, including the validity of the predecessor data it depends on. For Moriarty, that data is a contract transition: the previous state, authorized action, observations, next state and effects.

The intended acceptance rule requires evidence of:

- **Contract properties:** the agreement's declared invariants hold over their stated domain.
- **Intent refinement:** the chosen execution stays within the participant's authorization.
- **Transition validity:** the next state and effects follow the contract semantics.
- **History compliance:** the predecessors originate from an allowed initial state and extend a compliant history.

In the intended protocol, deployment policy fixes the permitted claim specifications and verifier/key versions. The participant’s signed authorization commits to the mandatory claims. Acceptance must reject missing evidence, unsupported mandatory claims and unresolved dependencies. A prover cannot choose a permissive verifier or remove a signed requirement. Enforcement in the ledger acceptance path remains unimplemented.

Recursive proofs are the proposed mechanism for checking predecessor proofs inside a new proof. Recursion in the proof system does not add unbounded recursion to the source language. Each construction still needs explicit execution and composition bounds.

A valid history proof does not establish that an external price is true or that a previous state has not already been spent. Observation authentication, ledger consumption, transaction ordering and finality remain separate responsibilities. Likewise, zero-knowledge capability does not by itself provide private witness handoff between participants.

The project has not yet produced a native recursive Moriarty proof. The [native experiment](experiments/moriarty-native-ivc-r3/) uses Midnight’s Halo2-based incrementally verifiable computation (IVC) backend. Its prepared encoding constrains a fixed loan scenario to known states; it does not yet prove the general Moriarty transition relation. Connecting its complete verifier to Midnight's ledger verifier is an unresolved engineering boundary; a host-computed verification flag cannot substitute for that connection. The [PCD design](docs/research/2026-09-06-pcd-report-integration.md) and [verifier interface analysis](evidence/moriarty-completion-program-2026-09-07/MC04/wrapper-interface-source-02/README.md) describe these obligations.

## Try the local developer workflow

Use **Node.js 24**. The source simulator has no npm runtime dependencies and requires no wallet, faucet or network connection.

```sh
git clone https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty
npm --prefix experiments/moriarty-language run demo
```

The command needs no dependency installation or TypeScript compiler. The demo reads the actual [loan](experiments/moriarty-language/spec/examples/loan.mori) and [swap](experiments/moriarty-language/spec/examples/swap.mori) source files. It shows candidate transitions and rejected actions. To inspect complete structured inputs and results:

```sh
node experiments/moriarty-language/examples/simulate.mjs --json
```

The swap output includes:

```text
Transfer: 10000 AssetA_quantum
Transfer: 19743 AssetB_quantum
adverse input: GUARD_FAILED; acceptance without proof: PROOF_INVALID
```

Those rejection messages are expected. The demo deliberately tampers with an input and attempts acceptance without a proof backend.

In the JSON output, `genesis` is the initial instance configuration, `state` carries the revision and state hash, `action` holds its name and arguments, and `authority` contains the exact plan or outcome constraints. A candidate transition contains the derived writes, obligations and ordered effects.

Simulation does not sign, prove, submit or consume a ledger state. The acceptance entry point rejects without its required backend; that backend has not been supplied as a production implementation.

### Check an agreement of your own

The atomic frontend used by the loan and swap demo exposes JavaScript APIs. Save this as `check-agreement.mjs` in the repository root:

```js
import {readFileSync} from 'node:fs';
import {check} from './experiments/moriarty-language/src/frontend.ts';

const result = check(
  readFileSync(process.argv[2]),
  readFileSync('experiments/moriarty-language/spec/bounds.json')
);
console.log(JSON.stringify(result, null, 2));
if ('code' in result) process.exitCode = 1;
```

```sh
node check-agreement.mjs experiments/moriarty-language/spec/examples/loan.mori
```

Pass your own source file in place of the example. `check` returns a typed program or a diagnostic with a code and source span. `parse` returns the source tree, and `elaborate` returns the bound Core program. Simulating a new agreement also requires constructing its instance configuration, action inputs and authority; the demo script shows those API calls.

### Check and format financial expression source

The expression CLI takes an explicit source profile and a trusted schema. Run
these commands from the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/financial-vault-quote.schema.json experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-expression-source/1 experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-expression-source/1 --schema experiments/moriarty-language/spec/successor/examples/financial-vault-quote.schema.json --snapshots experiments/moriarty-language/spec/successor/examples/financial-vault-quote.snapshots.json experiments/moriarty-language/spec/successor/examples/financial-vault-quote.mori
npm --prefix experiments/moriarty-language run financial-expression-demo
```

`simulate` returns the validated pre-state, initial work, and the selected
source API's exact local candidate result, including its post-state,
descriptors, and remaining work where the candidate succeeds. A rejection is a
structured stderr record with no candidate state or effects. It is local only:
it does not run K, create a proof, or perform a ledger, wallet, service, or
network action.

`check` returns `SourceChecked` with static work bound `43` for this fixture.
`format` writes canonical source to stdout and leaves the input file unchanged.
The demo computes a local vault quote with explicit UInt256 intermediates and
shows rejection of a selected absent Option without publishing state or effects.
It performs expression evaluation; it does not transfer assets or submit a
transaction. The same CLI supports `moriarty-expression-source/1` with that
profile's source and schema. See the [CLI contract](experiments/moriarty-language/spec/successor/expression-cli.md)
for argument forms, diagnostic records and exit codes.

The original expression CLI still supports one action against a trusted schema.
Source-defined schemas, multiple named actions and kernel-backed financial
reads are later agreement profiles, documented below. Full SP02 acceptance
and K correspondence remain open.

### Check, format and simulate kernel-backed financial reads

Agreement source `/3` declares records, operations, uninitialized ordinary
state, multiple named actions and six generic reads of a validated local
kernel pre-state. Ordinary snapshots (`Pre`, `Args`, `Obs`) stay distinct
from that financial projection. `ensures` still see ordinary `post`. Reads
see financial pre-state even after `emit`. Run these from the repository
root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/3 experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/3 experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/3 --action repay --snapshots experiments/moriarty-language/spec/successor/examples/financial-state-payment.snapshots.json --repayment-state experiments/moriarty-language/spec/successor/examples/financial-state-payment.state.json experiments/moriarty-language/spec/successor/examples/financial-state-payment.mori
npm --prefix experiments/moriarty-language run financial-state-demo
```

`check` inspects every action and does not require state. `simulate` requires
`--action` once. Starting remaining work is 256. `repay` of 30 costs E 43 plus
N 2 and leaves outstanding 70; continuation `repay_installment` of 20 costs
E 45 plus N 2 and leaves 50; `repay_remaining` reads that 50, costs E 47 plus
N 2, and settles. A fourth `repay_remaining` fails the positive-payment
guard with no effects. Work debit is E + N, where E is the selected action's
actual expression reductions and N is the kernel action count. This is local
preparation, not a K execution, proof, or ledger settlement.

### Inspect the generated Compact

Read the committed [loan kernel](experiments/moriarty-language/compact/generated/loan/kernel.compact) or [swap kernel](experiments/moriarty-language/compact/generated/swap/kernel.compact), or regenerate them with Node.js:

```sh
node experiments/moriarty-language/compact/materialize-mapping.mjs
```

The generated files are under `experiments/moriarty-language/compact/generated/`. To compile and check them, use Python 3, an installed `tsc`, Compact compiler **0.31.1** with language **0.23.0**, and Compact runtime **0.16.0**:

```sh
python3 experiments/moriarty-language/compact/verify-mapping.py \
  --runtime-node-modules /path/to/node_modules \
  --output /tmp/moriarty-compact-check
```

Replace `/path/to/node_modules` with the directory containing `@midnight-ntwrk/compact-runtime` at that version. This check compiles with `--skip-zk` and generates no proving keys or proofs. Running the language package’s `build` or `typecheck` scripts also requires an installed `tsc`.

For the remaining frontend APIs and tests, continue with the [language package guide](experiments/moriarty-language/README.md). The [browser developer mock](experiments/moriarty-developer-mock/README.md) explores the proposed user flows with simulated authority and certificates; it is a separate prototype, not a browser frontend for the source compiler.

## What remains to build

The [DeFi action study and language proposal](deliverables/defi-language-design-2026-09-07/README.md) connect financial reference behaviors to the proposed syntax and K semantics.

The complete [roadmap](ROADMAP.md) lists the implementation sequence, acceptance criteria and remaining checks, with links to the maintained [research wiki](wiki/index.md).

| Area | Available now | Required next |
| --- | --- | --- |
| Language | Bounded grammar, types, canonical encoding, evaluator and executable reference examples | Complete implementation review and extend the semantics for all required financial cases |
| Compilation | Restricted source-derived Compact kernels and local result comparisons | Full effect mapping and compiler-to-ledger correspondence |
| Network integration | Reviewed Preview loan and swap effects; [corrected swap command](deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/RESULT.md) also has retained exit zero, bounded native fee comparison and separate containment | Loan raw-exit gap, remaining financial coverage and mandatory PCD acceptance |
| Recursive proofs | Native backend investigation and a prepared checked encoding | Produce and independently verify retained recursive proofs |
| Acceptance | Local semantic checks and interfaces that reject without a backend | Enforce all mandatory claims, authorization and replay protection in the actual ledger path |
| Privacy and composition | Specified handoff and composition requirements | Private witness transfer, proved split/join and preservation of residual obligations |
| Financial coverage | ACTUS and DeFi source requirements and representative local cases | Full behavioral conformance, including the held-out cases |

Midnight Preview execution is a hard release gate. The reviewed Preview loan establishes a bounded first-period test-asset payment with residual principal; the swap establishes a trade and reserve withdrawal at close. The original audits preserve each raw wrapper `FAILED`/driver `INCOMPLETE` record. A later [corrected swap run](deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/RESULT.md) has both audits accepting its financial effects, actual command exit zero and separately observed containment. The corrected Preview loan’s missing raw exit remains unresolved. A later [local loan command](deliverables/sp05-financial-integration-2026-09-09/local-command-execution-proposal-01/loan/RESULT.md) completed with independently reviewed exit-zero and cleanup evidence; its later [read-only historical-state capture](deliverables/sp05-financial-integration-2026-09-09/local-command-execution-proposal-01/loan/historical-readback-01/RESULT.md) resolved that local evidence gap with both scoped financial-result reviews. Complete SP05 reconciliation, full financial coverage and mandatory PCD acceptance remain open. The [completion plan](openspec/MORIARTY-COMPLETION-PROGRAM.md) defines the dependencies and evidence required to close these gaps.

## Research vault

The repository also serves as the agents’ research vault. The [vault overview](wiki/overview.md) and [research index](wiki/index.md) connect findings to their sources and decisions. The [vault guide](docs/OBSIDIAN.md) explains the WSL setup and agent workflow; Obsidian is an optional viewer.

## Agent startup

Agents working in this repository must load `moriarty-dev:develop` on every new
session and restore it after resume or compaction. Follow the
[startup procedure in AGENTS.md](AGENTS.md#required-startup-load-the-development-plugin),
including the guarded status command. Hosts without plugin discovery read the
[checked-in skill](plugins/moriarty-dev/skills/develop/SKILL.md) directly and use
the same CLI. This applies to research and reviews as well as implementation;
the user's current request determines the work.

## Repository guide

- [`experiments/moriarty-language/`](experiments/moriarty-language/): authoring language, evaluator, source examples and Compact mapping.
- [`experiments/moriarty-developer-mock/`](experiments/moriarty-developer-mock/): browser prototypes for developer interactions.
- [`experiments/moriarty-native-ivc-r3/`](experiments/moriarty-native-ivc-r3/): native recursive-proof experiments.
- [`experiments/moriarty-midnight-network/`](experiments/moriarty-midnight-network/): local and Preview network integration.
- [`wiki/`](wiki/index.md), [`docs/`](docs/) and [`deliverables/`](deliverables/): concepts, design rationale and financial source studies.
- [`openspec/`](openspec/MORIARTY-COMPLETION-PROGRAM.md): planned capabilities and acceptance requirements.
- [`evidence/`](evidence/) and [`raw/`](raw/): scoped experimental records and retained source material. Historical results keep their original limitations.

Superseded implementations and execution campaigns are available in the [historical archive](docs/ARCHIVE.md).

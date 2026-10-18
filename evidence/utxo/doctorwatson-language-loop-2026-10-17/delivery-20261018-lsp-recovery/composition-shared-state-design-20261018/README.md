# L08 shared-state interleaving proposal

Prepared against exact main `74a909d69639df8823a527a4d608a7516ccd69a2`, tree `f18c2346ff435f9ac95a7f6f56457c2edde3f71c`. **Unadopted design only.** No product, build, model, SDK or provider execution and no Git/shared-source mutation occurred.

The concrete proposal is a separate local `stage::shared::Session`: one admitted program, one complete shared head, two distinct action frames, and explicit segment invocations. The existing disjoint Pair remains unchanged. A pending action keeps its earlier captures while reading the current shared financial state on resume. This new rule needs design adoption; serial action replay or standalone edited continuations are not its implementation or acceptance evidence.

The example has two partial deliveries sharing reserve/CASH/tier. Both pending requests quote tier1. A-first assigns fees-a1/fees-b2; B-first assigns fees-a2/fees-b1. Sequential baselines instead give the second starter quote2. All four traces preserve marker77, observer11, outside duty5, closing delivery duties3/8, every original effect and cumulative successful work46. These are disclosed author examples, not independent oracles.

Read in order:

1. [Design and four adoption choices](DESIGN.md), including retained five-operator dispositions.
2. [Complete proposed interface/report fields](PROPOSED-API.rs.txt).
3. [Source and exact input scripts](examples/README.md).
4. [All twenty financial/frame/work observations and full source-step/effect rules](WORKED-OUTCOMES.md).
5. [Proposed EARS and retained normative mapping](PROPOSED-REQUIREMENTS.md).
6. [Finite implementation/actual-consumer slice](IMPLEMENTATION-SLICE.md) and two [proposed ordinary positive test sources](proposed-tests/shared_interleaving.rs).
7. [Nine missing independent artifacts and custody contract](ORACLE-REQUIRED.md).
8. [Actual file/line/hash matrix](SOURCE-MATRIX.md), [complete public source pins](SOURCE-PINS.json), and immutable `basis/` bodies.

Root obtains substantive agreeing votes on SI1–SI4 before implementation. Designated owners retain authority/hostile/resource decisions. The proposed failed-attempt accounting policy must not acquire approval through arithmetic examples. Actual authenticated shared-head lineage, full proof/correspondence, private handoff, unique consumption, ledger/resource/target evidence and all original parent/operator exits remain open. Nothing in this packet adds a public developer approval prerequisite.

The two proposed tests are UNRUN and cannot currently run: new API/feature/consumer and independent expected files are missing. The supplied API file is a signature/record specification, not Rust implementation bodies. Full Program/Plan/preimage expectations are separately missing and are not silently counted as observed by these report tests. No broad audit/orchestration expansion is proposed.

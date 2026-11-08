# Skeptical review: "Skeleton Coding" (arXiv 2610.38214v1, AWS Agentic AI, 28 Oct 2026)

Read in full (28 pp., App. A-E). Verdict: **pilot** (Section 3), not adopt.

## 1. Theory

**The algebra is correct** (checked against App. E).

- Thm 2.1 (E.1). S_A is the point-function class h_a = f* xor 1{a != 0, x = a}. S_B is the linear class f* xor <a,x>. With a uniform prior, S_A needs all 2^d - 1 nonzero inputs and S_B needs d basis vectors. This is the teaching-dimension gap (ref [17]) relabelled; a "skeleton" is any hypothesis class, and no developer writes S_A.
- Thm 2.2 (E.2). The bound 2^n(1-2δ)/(1-δ) follows from the prior (mass 1/2 on zero, 2^-(n+1) on each point function). The prior is built to give p_S(G) = 1/2, and E.2 itself concedes the result is exact identification under one prior, not fixed-error prediction. Each wrong candidate has error 2^-n under uniform inputs. The "4n local tests" side needs the tester to know the reference's internal transition tables.
- Prop 2.3 is a one-line Bayes identity. Thm 2.4 is an accounting identity: total cost = local tests + compatibility tests. "J_S = 0 under Hoare contracts" (Sec 2.4) just restates that composable contracts compose.
- Prop 3.1 (product vs sum of 1/α_i) and Prop 3.2 are correct under their assumptions: components independent, G = G_1 x ... x G_k, and judge errors that are fresh and independent. Prop 3.2 compares a per-component lower bound with a whole-program upper bound, not like for like. The text cites "Theorem 3.1" (p.3, Prop 3.2), but no such theorem exists.

**Do the assumptions hold?** Section 2 assumes exact Bayesian conditioning. Section 3 then says that prompting is not conditioning (q_w != p_0(·|C_E)). Fig 2 shows that extra correct contracts make small models worse. The theorems describe an idealized learner, not the agent. The Sec 2.1 claim that Eq. 3 explains Fig 1b uses the same Bayesian mixture that Sec 3 rejects. The exponential separations come from adversarial classes (point functions, parity) under priors chosen for the result. They say nothing about typical code. Formal checks are the one mechanism that does not depend on Bayesian assumptions (Sec 3). They are never tested (Sec 5).

## 2. Experiments

**Leakage is the main problem.** Every specification is generated from the ground-truth code h* (Sec 4, "Generation of specifications"; App D). A signature skeleton copies the reference's module tree, class names, attribute names, signatures and docstrings (App A vi-x). SCBench and SDDBench score with the projects' own test suites. Those suites import specific modules and internal functions, which a skeleton makes resolve. Behavioral specs do not give names (EARS, public-API tests). Part of the 29.0% -> 74.2% jump on SDDBench is probably API-name matching, not a structural insight. No renamed-skeleton or public-API-only ablation separates the two.

**ProgramBench is the cleanest test, and there the result is weakest.** ProgramBench tests are black-box CLI checks, so name matching does not apply. The gain is 64.0% -> 70.9% for local contracts (Fig 1b) and 73.7% for global invariants (Fig 10). The prose Design (EARS) document scores **73.6%** (Fig 10). That ties the best skeleton and beats signatures (71.7%), contradicting "code-like beats prose" (Fig 2, Fig 6 left).

**Information is not matched.** The structural conditions add more, and different, reference-derived content. Skeleton size is never reported; only prose docs get a size control (Fig 6 left). Fig 6 right compares a reference-derived Global Invariants skeleton with 20 queries against up to 160 curated I/O examples.

**Sample size and variance.**
- Scale: 20 ProgramBench tasks, 35 SCBench problems (final state only, so the long-horizon point of SlopCodeBench is gone), 13 SDDBench libraries, 5+4 repos (Fig 6 left), 5 repos x 2 models (Fig 9), 1 model (Fig 8).
- No seed counts. The error bars appear to span models, not runs. There are no significance tests.
- Per-model scatter is wide (Fig 2).
- Fig 9 coverage curves are not monotone and have wide bands.
- SDDBench is anonymized public libraries; contamination admitted (App B).
- One harness (Strands AI Functions, ref [30]).

**Headline numbers are selected after the fact.** "48.4%" compares the best skeleton with the best behavioral condition, chosen on the evaluation data (Sec 4.1). The abstract says erosion falls "24.7% on average". Fig 7 says "up to 24.7%", and the text gives 0.66 -> 0.56 and 0.41 -> 0.24. Lower erosion may just inherit the reference's factoring.

**Costs are not counted fairly.** Fig 5's "15-26x lower implementation cost" excludes specification generation (App D: "excluding specification generation and evaluation compute"). Who or what produced the skeletons is unstated, and human authoring effort, the stated objective (Sec 1), is never measured. Fig 1a ("no number of tests suffices") uses a fixed 25% held-out split. Sub-100% there has many causes; it does not show exponential test needs.

**What survives:** fixed interfaces help small models; per-component checks are cheaper. Neither is new ([46], [33], [16]).

## 3. Applicability to ZK circuits for a VM with recursive proofs

**What should transfer:**
- *Soundness is the real case for Thm 2.2.* A malicious prover chooses the bad witness, so the problem is worst-case exact identification, not average error. An underconstrained per-rule relation is a point-function bug: honest traces cannot find it. Here the adversarial constructions are realistic.
- *The decomposition is natural and cheap to write.* Per-rule step relations, the recursive node and the decider already exist as boundaries, and the Lean and Quint models already state the contracts. Written without a reference circuit, this is the honest setting the paper never tests.
- *Prop 3.1 with formal checks.* Prop 3.1 applies if each rule circuit can be checked against its Lean relation (equivalence check or an underconstraint/determinism checker).

**What should not transfer:**
- *Pass-rate evidence.* Honest-trace tests measure completeness only. They say nothing about soundness.
- *Informal EARS contracts with an LLM judge.* The β_i floor in Prop 3.2 is exactly where soundness bugs hide.
- *J_S = 0.* Soundness does not compose for free. Cross-rule invariants are global properties: public-input binding, state chaining, memory or lookup consistency, accumulator and folding soundness through recursion, and decider binding. These carry the most risk.
- *The paper's numbers.* The language, models, metrics and leakage all differ.
- *Witness layout.* A skeleton that fixes the column or witness layout makes much of the design choice for the agent.

**Minimum experiment (about 1-2 weeks):**
- **Components:** 8 per-rule step relations with Lean specs and no circuit code yet, or with the circuit code hidden.
- **Conditions:**
  - (A) Task plus Lean/Quint spec.
  - (B) A plus a hand-written skeleton: signatures, witness/column stubs, per-function contracts that cite Lean lemmas, and a global-invariant list. The author must not see any reference circuit.
  - (C) A skeleton derived from the reference. C minus B estimates leakage.
- **Runs:** 2 models, 5 seeds per cell, paired by component.
- **Primary metric:** the fraction of components with no soundness finding. Soundness findings come from witness-mutation fuzzing, an underconstraint detector and, where feasible, a formal equivalence check against Lean.
- **Secondary:** completeness on Quint traces, tokens, human skeleton-writing hours.
- **Analysis:** paired bootstrap or McNemar.
- **Decision rule:** adopt if B beats A on soundness and the authoring hours are lower than the agent time saved. Otherwise drop it.

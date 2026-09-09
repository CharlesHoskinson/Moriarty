# Independent GPT-6 Felleisen–Hieb README audit

Verdict: **PASS**, with one minor citation correction noted below. No blocking semantic findings.

Reviewer: delegated GPT-6 Astra (`gpt-6-astra`), independent of the author, continuing the prior independent README repayment review. Date: 2026-09-09. Checkout: `/home/charl/Moriarty/.worktrees/felleisen-hieb-readme`; base/HEAD `5c17c4e09faec801dbb5c2d5e103d2b11c07215b`. Approval covers only the README candidate hash listed below, within the lowered admitted repayment control domain.

## Scope, provenance, and checks

Read and applied the new checkout's `AGENTS.md` and `plugins/moriarty-dev/skills/develop/SKILL.md`, then ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`. Status reports unresolved loan-swap operational history and no pending transactions; no dependent work was dispatched.

Read the README diff and the complete unchanged K definition. Read selected extracted primary-paper pages: section 2's term/context grammar, Definition 2.1, evaluation contexts and Definition 2.3; section 3's abort explanation and section 3.1's top-level computation rule and Definition 3.1. This selected-page reading is not a full paper or metatheorem proof audit. The text is from the full local two-up PDF; inspected its capture receipt and verified its SHA-256. Receipt records HTTP 200 from the MPI academic mirror using Scrapling 0.4.15. The original author's host failure is retained separately. This reviewer made no network requests and does not claim fresh retrieval.

Commands included `git status --short`, `git rev-parse HEAD`, `git diff -- README.md`, `git diff HEAD -- experiments/moriarty-language/formal/k`, `git diff --check`, `sha256sum` on listed files, and targeted `sed`/`rg` reads. Standard-library Python assertions established that the entire text beginning with the guard-table introduction is byte-identical to the previously audited base, checked the outstanding-invariant fixture's first three predicates, and walked conceptual instruction lists for true-prefix/failure and all-true finalization.

No K compilation/execution, source execution, proof execution, installation, or product edit occurred. The K definition, codec, K README, and fixtures have the exact hashes of the prior independent audit at `/home/charl/.local/state/moriarty/readme-small-step-gpt6.md`; its six arithmetic checks and sixteen retained result comparisons remain applicable to those unchanged bytes. This review does not claim new empirical K evidence, bisimulation, mechanized correspondence, or metatheorems. Browser rendering is assigned separately to the root agent.

## Findings

- The presentation is a legitimate specialized Felleisen–Hieb-style reduction semantics for the declared lowered instruction language. It defines term categories, a hole/context grammar, primitive contractions, plugging, and the relation generated from evaluation-context closure plus whole-program rules. It does more than rename configuration arrows. Its scope is explicitly the implemented control layer, not source expression reduction.
- The paper distinguishes general compatible closure under arbitrary term contexts (Definition 2.1) from the standard reduction function under evaluation contexts (Definition 2.3). The README uses the latter mechanism: `E ::= □ | E ▷ k` places the hole only at the list head. Associativity and the two unit equations flatten lists and remove empty computations; they cannot commute an instruction past the hole. There is no right-context rule or descent into packet data. Syntactically redundant unit contexts may give multiple decompositions, but do not add different control outcomes.
- START and EXPAND correctly contract one instruction to the index-resolved inspection and then the exact ordered checks plus finish. Metafunction treatment of predicates and arithmetic explicitly abstracts K's equational work, so one README arrow is not claimed to count every K rewrite.
- CONTEXT lifts only primitive instruction contractions. Successful CHECK contracts the first guard to the unit, after which the suffix becomes active. False guards have no local contraction that could accidentally preserve their continuation.
- ABORT is a whole-program rule whose left side identifies the active false guard and whose right side is a bare terminal rejection. It discards the entire surrounding evaluation context, including finish. This use of a whole-program control rule is consistent with the selected section 3.1 precedent; it does not purport to instantiate the paper's lambda calculus or inherit its theorems.
- PREPARE is root-only and cannot run through an arbitrary context. Reachability from an admitted `run_H(start(P))` ensures the single finish instruction appears last and only becomes active after the ordered checks pass. The domain restriction also excludes fabricated mixed-packet or mismatched-digest instruction lists. Answers are outside the computation grammar and have no outgoing reductions.
- The unchanged financial table, arithmetic, codec boundary, finite-evidence qualifications, and distinction from successor grammar retain their prior PASS. The README expressly disclaims applying the paper's metatheorems to Moriarty and makes no bisimulation or formal correspondence claim.
- Minor citation correction: the paper labels 2.2 as a theorem, not a definition. Replace “Definitions 2.1–2.3” with “Definitions 2.1 and 2.3.” This does not affect the reduction semantics.

## Conceptual traces checked

Let `gj` denote the j-th ensure instruction; all carry the admitted packet digest. These are derivations of the explanatory rules, not newly executed K traces.

For the existing `outstanding-invariant` fixture, g1 (distinct balance pairs) and g2 (allowance sum) are true; g3 (debt invariant) is false. The later work check also fails, since remaining work is zero.

```text
run_H(start(P))
  → run_H(inspect(P,s,r))
  → run_H(g1 ▷ g2 ▷ g3 ▷ ... ▷ g21 ▷ finish(P,s,r))
  → run_H(g2 ▷ g3 ▷ ... ▷ g21 ▷ finish(P,s,r))
  → run_H(g3 ▷ ... ▷ g21 ▷ finish(P,s,r))
  → rejected(H, INVARIANT, -1)
```

The first two successful checks use contexts retaining their complete suffixes. ABORT then discards eighteen remaining guards and finish (nineteen instructions). It cannot advance to the later insufficient-work check or produce a financial result.

For an all-true admitted packet such as principal-partial:

```text
run_H(start(P))
  → run_H(inspect(P,s,r))
  → run_H(g1 ▷ ... ▷ g21 ▷ finish(P,s,r))
  →* run_H(finish(P,s,r))
  → prepared(H,F(P,s,r))
```

The middle closure contains exactly twenty-one CHECK/CONTEXT steps. Successful contraction replaces each head by epsilon, whose unit law exposes the next head. PREPARE has the required sole finish instruction. The resulting prepared answer has no further step. These counts concern the displayed control relation, not all K helper rewrites or charged action-work units.

## Exact SHA-256 hashes

- `README.md`: `bb39ad578b1b90ce41a6a6a97733b777ce865a1a5faaa60f321dd25fd8b87788`
- `experiments/moriarty-language/formal/k/moriarty.k`: `0e695ac53e7daa04de226e4f95a20d1f46bebaee02e89db7977a9b1be314c0cd`
- `experiments/moriarty-language/formal/k/codec.py`: `923120b9628bca2b0e5c1cdd2488b0811f432d663d76a9e93293f5f6c6180f1d`
- `experiments/moriarty-language/formal/k/README.md`: `0a98f2d0723ea6e03049cd3d185dd7258664a4dd761d5385b64a371e525dc47a`
- `experiments/moriarty-language/formal/k/fixtures/cases.json`: `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832`
- `raw/sources/felleisen-hieb-2026-09-09/felleisen-hieb-1992.pdf`: `7cb3694071ee9da37a2c1b631a950077289eee423fc79fe5528cf06462c48f4f`
- `raw/sources/felleisen-hieb-2026-09-09/felleisen-hieb-1992.txt`: `1864f39145f1ff15eb4982eff1e1fea06877043ca33a7a0844824ad7e7a61620`
- `raw/sources/felleisen-hieb-2026-09-09/capture-0.json`: `c91abe44649a12cf875ec87ed1b920d8d9490d20a02877ae8818673f95d78904`
- `raw/sources/felleisen-hieb-2026-09-09/author-host-unavailable.json`: `41dd8c36786adcaf3d972e1fb17f8d78720b71e979cd201bfa981da5c9df16ed`

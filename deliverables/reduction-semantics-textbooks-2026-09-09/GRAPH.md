# Textbook concept map

This is a scoped reading map, not a proof graph. Every proposed check remains specified-only. Complete downloads are described in [CORPUS.md](CORPUS.md). The interactive [graph](graph.html) and typed [JSON](graph.json) preserve source locators and evidence kinds.

```mermaid
flowchart LR
  plfa["PLFA — CC BY 4.0"]
  sf["Software Foundations PLF 7.1 — MIT"]
  redex["Redex manual 9.0 — Apache-2.0 OR MIT"]
  fh1992["Felleisen–Hieb 1992 — historical paper"]
  contexts["Evaluation contexts select the next redex"]
  closure["Primitive contraction versus context closure"]
  abort["Failure discards the evaluation context"]
  answers["Terminal answer versus stuck term"]
  domain["Admitted and reachable claim domain"]
  determinism["At most one successor"]
  termination["Finite control versus fuel and action budget"]
  state["Explicit state and final observations"]
  binding["Hole filling versus variable substitution"]
  testing["One-step, final-result and falsification tests"]
  check_order["First false check wins; no suffix step"]
  check_answers["Reachable non-answer steps; answers do not"]
  check_bound["Inspect 24-step abstract trace bound"]
  check_state["Compare full proposal; reject exposes no state/effects"]
  check_binding["Define scope only when successor binding executes"]
  check_relations["Compare all one-step successors and full final results"]
  readme["README control presentation"]
  k["Bounded repayment K rules"]
  codec["Codec boundary"]
  cases["16-case finite comparison"]
  successor["Successor semantics remain open"]
  roadmap["Formal correspondence remains open"]
  redex -->|explains| contexts
  redex -->|explains| closure
  redex -->|explains| abort
  redex -->|explains| state
  redex -->|explains| binding
  redex -->|explains| testing
  plfa -->|explains| contexts
  plfa -->|explains| answers
  plfa -->|explains| domain
  plfa -->|explains| determinism
  plfa -->|explains| termination
  plfa -->|explains| binding
  sf -->|explains| state
  sf -->|explains| determinism
  sf -->|explains| answers
  fh1992 -->|explains| closure
  fh1992 -->|explains| abort
  contexts -->|motivates| check_order
  closure -->|motivates| check_order
  abort -->|motivates| check_order
  answers -->|motivates| check_answers
  domain -->|motivates| check_answers
  determinism -->|motivates| check_answers
  termination -->|motivates| check_bound
  state -->|motivates| check_state
  abort -->|motivates| check_state
  binding -->|motivates| check_binding
  testing -->|motivates| check_relations
  determinism -->|motivates| check_relations
  check_order -->|would_check| readme
  check_order -->|would_check| k
  check_answers -->|would_check| readme
  check_bound -->|would_check| readme
  check_state -->|would_check| codec
  check_state -->|would_check| cases
  check_binding -->|would_check| successor
  check_relations -->|would_check| k
  check_relations -->|would_check| roadmap
  readme -->|already_specifies| abort
  readme -->|already_specifies| contexts
  check_bound -->|is_scoped_inspection_argument| termination
  cases -->|does_not_close| roadmap
```

26 nodes; 42 directed edges. Node types separate textbooks/manual/paper, concepts, proposed checks and repository files. Edge kinds separate source facts, observations, inferences, recommendations and open questions.

No whole-corpus semantic extraction, source compilation, theorem transfer or live runtime validation is claimed.

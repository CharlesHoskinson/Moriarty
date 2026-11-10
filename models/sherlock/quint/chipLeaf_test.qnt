// Deterministic checks for the chiplet leaf model (chipLeaf.qnt): reference semantics, an honest
// leaf, every tamper refused by the base relation, and each design-gap counterexample reproduced
// on its mutant. Run (one module at a time):
//   quint test chipLeaf_test.qnt --main chipLeaf_test
//   quint test chipLeaf_test.qnt --main chipLeaf_test_<mutant>
// Scenario program P1 (rows 0..5): ALLOC r0 -> r1; PUSH r1; LIT r0 := 3; SETTOP r0; POP -> r1; HALT.
// The leaf starts after the first two rows (warm 2): memory [(0, NIL), (0, NIL)], one frame at 1.
// Its four steps are LIT, SETTOP (appends the new top at address 2), POP (reads address 2 from
// MEM), HALT. Slot 2 is the POP step, the target of the read tampers.
module chipLeaf_case {
  const CASE_MUTANT: str
  import chipLeaf(S = 4, N_MEM = 2, N_PAGE = 8, N_ALU = 2, MUTANT = CASE_MUTANT) as m from "./chipLeaf"

  pure val P1: List[m::Ins] = [
    { op: m::OP_ALLOC, x: 0, y: 1 }, { op: m::OP_PUSH, x: 1, y: 0 }, { op: m::OP_LIT, x: 0, y: 3 },
    { op: m::OP_SETTOP, x: 0, y: 0 }, { op: m::OP_POP, x: 1, y: 0 }, m::HALT_INS,
  ]
  // P2: a LOAD of address 2, never written (len = 1; MEM rows hold addresses 1 and 2): an error.
  pure val P2: List[m::Ins] = [
    { op: m::OP_ALLOC, x: 0, y: 1 }, { op: m::OP_LIT, x: 0, y: 2 }, { op: m::OP_LOAD, x: 0, y: 1 },
    m::HALT_INS, m::HALT_INS, m::HALT_INS,
  ]
  // P3: two PUSH of r0 = 0 from genesis; the second appends (0, link 0), equal to the default leaf.
  pure val P3: List[m::Ins] = [
    { op: m::OP_PUSH, x: 0, y: 0 }, { op: m::OP_PUSH, x: 0, y: 0 }, { op: m::OP_POP, x: 1, y: 0 },
    m::HALT_INS, m::HALT_INS, m::HALT_INS,
  ]
  pure val R3: m::RefSt = m::warm(P3, 0)
  pure val R1: m::RefSt = m::warm(P1, 2)
  pure val R2: m::RefSt = m::warm(P2, 1)
  pure def leaf(prog: List[m::Ins], r0: m::RefSt, t: int, tk: str, lk: str): m::W =
    m::genLeaf(prog, r0, 4, t, tk, lk).w
  pure def sound(w0: m::W, r0: m::RefSt): bool = and {
    m::refineOk(w0, r0), m::framesOk(w0, r0), m::busOk(w0, r0), m::memOk(w0, r0), m::functionalOk(w0),
  }
  // A tamper is refused, or (mutant) accepted and refuted by a soundness predicate.
  pure def caught(w0: m::W, r0: m::RefSt): bool = m::accepts(w0) and not(sound(w0, r0))
  pure def refused(w0: m::W): bool = not(m::accepts(w0))
  pure def differs(prog: List[m::Ins], r0: m::RefSt, t: int, tk: str, lk: str): bool =
    leaf(prog, r0, t, tk, lk) != leaf(prog, r0, t, "none", lk)

  // ---- reference machine
  run refStackTest = m::init.expect(and {
    R1.stack == [{ v: 0, at: 1 }], R1.mem == [{ a: 0, b: m::NIL }, { a: 0, b: m::NIL }],
    m::refRun(P1, R1, 2).stack == [{ v: 3, at: 2 }],
    m::refRun(P1, R1, 2).mem[2] == { a: 3, b: m::NIL },
    m::refRun(P1, R1, 3).r1 == 3, m::refRun(P1, R1, 3).stack == [],
    m::refRun(P1, R1, 4).status == m::HALTED, m::refRun(P1, R1, 4).out == 3,
    m::refRun(P2, R2, 2).status == m::ERR, m::refRun(P2, R2, 3).status == m::PUBERR,
    m::implStack(R1.mem, 1, 1) == [0],
  })

  // ---- honest leaves (base only: mutants that change the core change the honest witness)
  run honestTest = m::init.expect(CASE_MUTANT != "none" or and {
    m::accepts(leaf(P1, R1, 0, "none", "cell")), sound(leaf(P1, R1, 0, "none", "cell"), R1),
    leaf(P1, R1, 0, "none", "cell").steps == 4,
    leaf(P1, R1, 0, "none", "cell").post.status == m::HALTED,
    m::accepts(leaf(P2, R2, 0, "none", "cell")), sound(leaf(P2, R2, 0, "none", "cell"), R2),
    leaf(P2, R2, 0, "none", "cell").post.status == m::PUBERR,
  })

  // ---- every effective tamper is refused by the base relation
  pure val READ_TAMPERS: Set[str] = Set("forged", "wrong_chip", "page_overlap", "dup_mem", "stale",
    "replay", "pad_match", "dec_lie", "mid_step", "wrong_pc")
  run baseRefusesTest = m::init.expect(CASE_MUTANT != "none" or and {
    READ_TAMPERS.forall(tk => differs(P1, R1, 2, tk, "cell") implies refused(leaf(P1, R1, 2, tk, "cell"))),
    Set("prog", "cell", "alu", "dec").forall(lk => refused(leaf(P1, R1, 1, "forged", lk)) or not(differs(P1, R1, 1, "forged", lk))),
    refused(leaf(P1, R1, 1, "page_forge", "cell")),
    refused(leaf(P2, R2, 1, "wrong_alu", "cell")),
    differs(P3, R3, 1, "append_via_page", "cell"), refused(leaf(P3, R3, 1, "append_via_page", "cell")),
    refused(leaf(P1, R1, 0, "orphan_bad", "cell")),
    refused(leaf(P1, R1, 3, "after_publish", "cell")) or not(differs(P1, R1, 3, "after_publish", "cell")),
    // harmless: valid unrequested rows are accepted and sound
    m::accepts(leaf(P1, R1, 0, "orphan_ok", "cell")), sound(leaf(P1, R1, 0, "orphan_ok", "cell"), R1),
    m::accepts(leaf(P1, R1, 0, "orphan_mem", "cell")), sound(leaf(P1, R1, 0, "orphan_mem", "cell"), R1),
    // the effective tampers in this scenario do change the witness
    differs(P1, R1, 2, "stale", "cell"), differs(P1, R1, 2, "page_overlap", "cell"),
    differs(P1, R1, 2, "mid_step", "cell"), differs(P1, R1, 1, "mid_step", "cell"),
  })

  // ---- design-gap counterexamples, each on its mutant
  run mutantCaughtTest = m::init.expect(
    if (CASE_MUTANT == "page_no_base_bound")
      caught(leaf(P1, R1, 2, "page_overlap", "cell"), R1) and caught(leaf(P3, R3, 1, "append_via_page", "cell"), R3)
    else if (CASE_MUTANT == "push_link_free") caught(leaf(P3, R3, 0, "none", "cell"), R3)
    else if (CASE_MUTANT == "base_per_row") caught(leaf(P1, R1, 2, "dup_mem", "cell"), R1)
    else if (CASE_MUTANT == "tag_unbound") caught(leaf(P1, R1, 2, "wrong_chip", "cell"), R1)
    else if (CASE_MUTANT == "table_ungated") caught(leaf(P1, R1, 2, "forged", "cell"), R1)
    else if (CASE_MUTANT == "no_inclusion") caught(leaf(P1, R1, 2, "forged", "cell"), R1)
    else if (CASE_MUTANT == "load_key_free") caught(leaf(P1, R1, 2, "stale", "cell"), R1)
    else if (CASE_MUTANT == "per_row_enable") caught(leaf(P1, R1, 1, "mid_step", "cell"), R1)
    else if (CASE_MUTANT == "rewrite_no_move") caught(leaf(P1, R1, 0, "none", "cell"), R1)
    else if (CASE_MUTANT == "root_all_rows") caught(leaf(P1, R1, 1, "orphan_mem", "cell"), R1)
    else if (CASE_MUTANT == "no_read_bound") caught(leaf(P2, R2, 0, "none", "cell"), R2)
    else if (CASE_MUTANT == "alu_gate_off") caught(leaf(P2, R2, 1, "wrong_alu", "cell"), R2)
    else if (CASE_MUTANT == "dec_dup") caught(leaf(P1, R1, 0, "none", "cell"), R1)
    else true)
}

module chipLeaf_test { import chipLeaf_case(CASE_MUTANT = "none").* }
module chipLeaf_test_page_no_base_bound { import chipLeaf_case(CASE_MUTANT = "page_no_base_bound").* }
module chipLeaf_test_base_per_row { import chipLeaf_case(CASE_MUTANT = "base_per_row").* }
module chipLeaf_test_tag_unbound { import chipLeaf_case(CASE_MUTANT = "tag_unbound").* }
module chipLeaf_test_table_ungated { import chipLeaf_case(CASE_MUTANT = "table_ungated").* }
module chipLeaf_test_no_inclusion { import chipLeaf_case(CASE_MUTANT = "no_inclusion").* }
module chipLeaf_test_load_key_free { import chipLeaf_case(CASE_MUTANT = "load_key_free").* }
module chipLeaf_test_per_row_enable { import chipLeaf_case(CASE_MUTANT = "per_row_enable").* }
module chipLeaf_test_rewrite_no_move { import chipLeaf_case(CASE_MUTANT = "rewrite_no_move").* }
module chipLeaf_test_root_all_rows { import chipLeaf_case(CASE_MUTANT = "root_all_rows").* }
module chipLeaf_test_no_read_bound { import chipLeaf_case(CASE_MUTANT = "no_read_bound").* }
module chipLeaf_test_alu_gate_off { import chipLeaf_case(CASE_MUTANT = "alu_gate_off").* }
module chipLeaf_test_push_link_free { import chipLeaf_case(CASE_MUTANT = "push_link_free").* }
module chipLeaf_test_dec_dup { import chipLeaf_case(CASE_MUTANT = "dec_dup").* }

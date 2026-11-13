// Test suite for HeaderOpenAir soundness, completeness, padding row freedom, and mutants M1-M6.
// Run: quint test qa/gemini-quint/x10-header/header_open_test.qnt --main header_open_test --max-samples 100 --seed 0x10

module header_open_test {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ) as sound from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = true,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ) as m1 from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = true,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ) as m2 from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = true,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ) as m3 from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = true,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ) as m4 from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = true,
    DROP_FIRST_CAP_ZERO = false
  ) as m5 from "./header_open"

  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = true
  ) as m6 from "./header_open"

  // ---- Completeness tests on the sound circuit.
  // 1. Honest header with non-zero payload and digests is accepted and matches sponge(w).
  run honestOpeningAcceptedTest =
    sound::init
      .then(sound::proveHonest(2, false, false, 0, 0))
      .expect(and {
        sound::st.accepted,
        sound::invOpen,
        not(sound::invNoAcceptedHonest),
        sound::openedWords(sound::st.tr) == sound::mkHeaderWords(2, false, false),
      })

  // 2. Honest header with zero payload (words 2..10 == 0) is accepted.
  run honestZeroPayloadAcceptedTest =
    sound::init
      .then(sound::proveHonest(1, true, false, 0, 0))
      .expect(and {
        sound::st.accepted,
        sound::invOpen,
        not(sound::invNoAcceptedZeroPayload),
        sound::isZeroPayload(sound::openedWords(sound::st.tr)),
      })

  // 3. Honest header with all-zero digests after the domain word is accepted.
  run honestAllZeroAfterDomainAcceptedTest =
    sound::init
      .then(sound::proveHonest(0, true, true, 0, 0))
      .expect(and {
        sound::st.accepted,
        sound::invOpen,
        1.to(sound::HW - 1).forall(i => sound::openedWords(sound::st.tr)[i] == 0),
      })

  // 4. Padding rows 6 and 7 are free and cannot change the 42 opened words or pv.
  run paddingRowsFreeAndPreserveOpeningTest =
    sound::init
      .then(sound::proveHonest(2, false, false, 7, 13))
      .expect(and {
        sound::st.accepted,
        sound::invOpen,
        sound::openedWords(sound::st.tr) == sound::mkHeaderWords(2, false, false),
        sound::st.pv == sound::sponge(sound::mkHeaderWords(2, false, false)),
      })

  // ---- Soundness refusal tests on the sound circuit.
  // Forging view_root (word 34 = row 4, col 2) under the honest post_d pv is refused.
  run forgedViewRootRefusedTest =
    sound::init
      .then(sound::proveTamperInput(4, 2, 1, true, 0, 0))
      .expect(and {
        not(sound::st.accepted),
        sound::invOpen,
      })

  // Tampering any output (post) cell while recomputing later rows is refused by Poseidon2SubAir.
  run postTamperRefusedTest =
    sound::init
      .then(sound::proveTamperPost(2, 8, 1, false))
      .expect(and {
        not(sound::st.accepted),
        sound::invOpen,
      })

  // Random interleaving of honest and adversarial steps preserves invOpen in sound module.
  run randomStepsSoundTest =
    sound::init
      .then(25.reps(_ => sound::step.expect(and {
        sound::invOpen,
        sound::invRawOpen,
        sound::invSelectorIdentity,
      })))

  // ---- Mutant detection tests (M1..M6 must each accept the malicious trace and violate invOpen).
  // M1 no_capacity_chain: tamper row 1 capacity input (col 8), recompute rows 1..5 honestly.
  run mutantM1NoCapacityChainCaughtTest =
    m1::init
      .then(m1::proveTamperInput(1, 8, 1, false, 0, 0))
      .expect(and {
        m1::st.accepted,
        not(m1::invOpen),
      })

  // M2 no_partial_keep: tamper row 5 rate input col 2 (outside the 2-word partial block), recompute row 5.
  run mutantM2NoPartialKeepCaughtTest =
    m2::init
      .then(m2::proveTamperInput(5, 2, 1, false, 0, 0))
      .expect(and {
        m2::st.accepted,
        not(m2::invOpen),
      })

  // M3 no_domain: tamper row 0 input 0 (domain word) away from SHK_SD2, recompute rows 0..5.
  run mutantM3NoDomainCaughtTest =
    m3::init
      .then(m3::proveTamperInput(0, 0, 1, false, 0, 0))
      .expect(and {
        m3::st.accepted,
        not(m3::invOpen),
      })

  // M4 output_on_row7: bind public values to row 7 (padding row) instead of row 5.
  run mutantM4OutputOnRow7CaughtTest =
    m4::init
      .then(m4::provePaddingPv(0, 0))
      .expect(and {
        m4::st.accepted,
        not(m4::invOpen),
      })

  // M5 selector_not_onehot: drop sum(s[r]) == 1; inject extra s[0]=1 on row 6 and s[1]=1 on row 7.
  run mutantM5SelectorNotOnehotCaughtTest =
    m5::init
      .then(m5::proveNonOneHotSelector)
      .expect(and {
        m5::st.accepted,
        not(m5::invOpen),
        not(m5::invSelectorIdentity),
      })

  // M6 first_capacity_free: tamper row 0 capacity input col 8 away from 0, recompute rows 0..5.
  run mutantM6FirstCapacityFreeCaughtTest =
    m6::init
      .then(m6::proveTamperInput(0, 8, 1, false, 0, 0))
      .expect(and {
        m6::st.accepted,
        not(m6::invOpen),
      })
}

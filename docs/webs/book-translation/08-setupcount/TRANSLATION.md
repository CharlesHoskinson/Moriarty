# Layer 1: setup count and Groth16

## Proposed reader explanation

A proof system needs public parameters that let a prover make proofs and a verifier check them. In Groth16, those parameters are tied to a particular compiled circuit. If the compiled constraints change, the circuit needs its own Phase 2 setup and keys. Changing the private or public values supplied to the same circuit does not, by itself, make a new circuit or call for another setup.

Some Groth16 workflows divide setup into two phases. A compatible Phase 1 Powers-of-Tau transcript can be reused as input to several circuits’ Phase 2 ceremonies. Phase 1 is shared setup material; it does not remove the circuit-specific Phase 2 work. The compatibility conditions matter, including the supported curve and parameter size. The snarkjs guide describes a common Phase 1 followed by circuit-specific Phase 2; Filecoin’s documentation describes one Phase 1 used across its circuits and a Phase 2 setup for each circuit.

That gives a conditional counting rule, not a Cardano census: for **K distinct compiled circuits** under one compatible Phase 1 domain, expect **K circuit-specific Phase 2 setups**, plus the shared Phase 1 setup. If the circuits require separate Phase 1 domains, count those separately too. Repeated uses of one unchanged circuit with different statements, accounts, amounts, or witnesses do not multiply the circuit count. A circuit change, such as a constraint or supported-bound change, may create a distinct circuit and require a new Phase 2 setup.

We do not have an audited inventory of distinct compiled Groth16 circuits covering all Cardano applications, or even a complete deployed-circuit inventory for this Webs catalogue. The catalogue’s 235 pre-dedup review identifiers and 20 families describe review scope, not compiled circuits. The actual count is therefore unknown. A page calculator should accept a hypothetical K and label its result as an illustration, never as the Cardano or Webs total.

A design can sometimes reduce the number of circuit-specific setups by putting several cases into one parameterized or bundled circuit. This only works for cases the one circuit actually expresses within its stated bounds. It can make the circuit larger or more complex; it does not make one bounded circuit cover every future Cardano application. Whether such a design is suitable or supported here remains an engineering and security question.

## Evidence labels and limits

- **BOOK_FACT:** The supplied manuscript calls Groth16 circuit-specific and contrasts it with universal-SRS systems. It also describes Layer 1 as interdependent with the other layers. References: `B08-EXACT-MANUSCRIPT-SECTIONS.json`, source `B07-proving-nothing.md`, lines 272–298 and 609–621; `B05-ch02-universal-versus-circuit-specific-setups.md`, “Universal versus Circuit-Specific Setups.”
- **PRIMARY_SOURCE_SUPPORT:** The supplied snarkjs README says projects can select from common Phase 1 for circuit-specific Phase 2, and its guide creates parameters from a compiled circuit. The supplied Filecoin Phase2 Attestations README says Phase 1 is run once for its circuits and Phase 2 is run per circuit. Groth’s supplied 2016 paper abstract supports the three-group-element proof and three-pairing verification description; it does not establish a Moriarty/Cardano circuit inventory. Inputs: `G02.txt`, `G04.txt`, `G05.txt`.
- **MORIARTY_MAPPING:** The supplied `V2-COPY.json` describes 235 pre-dedup review identifiers as not proven properties, lists retained families as requirements rather than supported ports, and says required setup count is unknown. It does not enumerate compiled Groth16 circuits.
- **INFERENCE:** For a hypothetical set of K distinct compiled circuits that can all use a compatible common Phase 1 transcript, count K Phase 2 ceremonies plus that shared Phase 1. The count is conditional on the circuit inventory and compatibility domain.
- **UNRESOLVED:** The exact deployed and planned Groth16 circuit inventory for Webs and for any stated Cardano-wide scope; phase-1 domain compatibility; whether any proposed generic or bundled circuit is actually compiled and used; what version/time boundary “entire Cardano” would mean.

The book excerpt’s proof-size and pairing claims are separately supported by the supplied Groth paper abstract at a high level. This draft does not audit those claims in full, establish setup security, or assess a specific ceremony’s correctness. The Opus 5.5 security reviewer owns final security fact-check and adopted technical wording.

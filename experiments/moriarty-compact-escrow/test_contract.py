"""Acceptance checks for the first Moriarty-to-Compact lowering experiment."""

from pathlib import Path
import re
import unittest


CONTRACT = Path(__file__).with_name("escrow.compact")


class MoriartyEscrowContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.source = CONTRACT.read_text() if CONTRACT.exists() else ""

    def test_has_exactly_three_terminal_phase_transitions(self) -> None:
        self.assertIn("phase = Phase.Funded", self.source)
        self.assertIn("phase = Phase.Released", self.source)
        self.assertIn("phase = Phase.Refunded", self.source)

    def test_release_and_refund_are_mutually_exclusive(self) -> None:
        self.assertGreaterEqual(self.source.count("assert(phase == Phase.Funded"), 2)
        self.assertEqual(self.source.count("phase = Phase.Waiting"), 1)

    def test_timeout_is_ledger_time_and_requires_no_private_witness(self) -> None:
        refund = re.search(
            r"export circuit refundAfterTimeout\(\): \[\] \{(?P<body>.*?)\n\}",
            self.source,
            re.DOTALL,
        )
        self.assertIsNotNone(refund)
        body = refund.group("body")
        self.assertIn("blockTimeGte(deadline)", body)
        self.assertNotIn("partySecret()", body)

    def test_value_effects_match_the_finite_state_machine(self) -> None:
        self.assertEqual(self.source.count("receiveUnshielded(tokenColor, amount)"), 1)
        sends = re.findall(r"sendUnshielded\(\s*tokenColor,\s*amount", self.source)
        self.assertEqual(len(sends), 2)

    def test_no_unbounded_or_ambient_authorization_features(self) -> None:
        for forbidden in ("Map<", "Set<", "List<", "ownPublicKey", "Contract<"):
            self.assertNotIn(forbidden, self.source)

    def test_private_authority_is_constrained_before_state_changes(self) -> None:
        self.assertIn("persistentHash<PartySecret>(partySecret())", self.source)
        self.assertIn("assert(buyerAuthority == authorityOfWitness()", self.source)


if __name__ == "__main__":
    unittest.main()

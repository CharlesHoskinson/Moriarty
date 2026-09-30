#!/usr/bin/env python3
"""Small interface-state probe. Evidence flags are fixture inputs, not verified proofs."""

from dataclasses import dataclass, field
import json


class Refused(Exception):
    pass


@dataclass
class Leg:
    entitlement: int
    payload: str
    attempts: dict[str, str] = field(default_factory=dict)
    queued: bool = False
    delivery: dict[str, int] = field(default_factory=dict)
    refunds: dict[str, int] = field(default_factory=dict)
    final_exclusive: bool = False

    def dispatch(self, attempt: str, payload: str) -> None:
        if payload != self.payload:
            raise Refused("payload_conflict")
        if attempt in self.attempts:
            return
        if any(s not in {"rejected_before_effect", "dropped_before_final"}
               for s in self.attempts.values()):
            raise Refused("unresolved_attempt")
        self.attempts[attempt] = "broadcast_unknown"

    def observe_attempt(self, attempt: str, phase: str) -> None:
        if attempt not in self.attempts:
            raise Refused("unknown_attempt_id")
        self.attempts[attempt] = phase

    def enqueue(self, attempt: str) -> None:
        if self.attempts.get(attempt) not in {"included", "final"}:
            raise Refused("enqueue_without_inclusion")
        self.queued = True

    def deliver(self, effect_id: str, amount: int) -> None:
        if amount <= 0:
            raise Refused("nonpositive_delivery")
        previous = self.delivery.get(effect_id)
        if previous is not None and previous != amount:
            raise Refused("effect_id_conflict")
        self.delivery[effect_id] = amount
        self.queued = False

    def prove_exclusive(self) -> None:
        # A real adapter must provide route-specific finality and non-delivery proof.
        self.final_exclusive = True

    def refund(self, refund_id: str, amount: int) -> None:
        if refund_id in self.refunds:
            if self.refunds[refund_id] == amount:
                return
            raise Refused("refund_id_conflict")
        if not self.final_exclusive or self.queued:
            raise Refused("remaining_delivery_unknown")
        if any(s in {"broadcast_unknown", "included"} for s in self.attempts.values()):
            raise Refused("attempt_unresolved")
        if amount < 0 or self.total_delivery + self.total_refund + amount > self.entitlement:
            raise Refused("refund_exceeds_residual")
        self.refunds[refund_id] = amount

    @property
    def total_delivery(self) -> int:
        return sum(self.delivery.values())

    @property
    def total_refund(self) -> int:
        return sum(self.refunds.values())

    @property
    def outcome(self) -> str:
        if self.total_delivery + self.total_refund > self.entitlement:
            return "breach"
        if self.queued or any(s in {"broadcast_unknown", "included"}
                              for s in self.attempts.values()):
            return "unknown"
        if self.total_delivery == self.entitlement and not self.refunds:
            return "delivered"
        if self.final_exclusive and self.total_delivery + self.total_refund == self.entitlement:
            return "closed_accounted"
        return "open"


def exercise() -> list[dict[str, str]]:
    rows = []

    def expect(name: str, fn, expected: str) -> None:
        try:
            fn()
            observed = "accepted"
        except Refused as error:
            observed = str(error)
        rows.append({"case": name, "expected": expected, "observed": observed,
                     "match": observed == expected})

    a = Leg(100, "digest-A")
    expect("initial dispatch", lambda: a.dispatch("attempt-1", "digest-A"), "accepted")
    expect("same attempt retry", lambda: a.dispatch("attempt-1", "digest-A"), "accepted")
    expect("changed bytes same effect", lambda: a.dispatch("attempt-1", "digest-B"), "payload_conflict")
    expect("duplicate while unknown", lambda: a.dispatch("attempt-2", "digest-A"), "unresolved_attempt")
    expect("timeout-only refund", lambda: a.refund("refund-1", 100), "remaining_delivery_unknown")
    a.observe_attempt("attempt-1", "final")
    a.enqueue("attempt-1")
    expect("enqueue is not delivery", lambda: rows.append({"case": "queued outcome", "expected": "unknown", "observed": a.outcome, "match": a.outcome == "unknown"}), "accepted")
    expect("refund while queued", lambda: a.refund("refund-1", 100), "remaining_delivery_unknown")
    a.deliver("fill-1", 40)
    expect("duplicate fill id", lambda: a.deliver("fill-1", 40), "accepted")
    expect("conflicting fill id", lambda: a.deliver("fill-1", 50), "effect_id_conflict")
    expect("partial full refund", lambda: a.refund("refund-1", 100), "remaining_delivery_unknown")
    a.prove_exclusive()
    expect("proved residual refund", lambda: a.refund("refund-1", 60), "accepted")
    expect("same refund id retry", lambda: a.refund("refund-1", 60), "accepted")
    expect("excess residual refund", lambda: a.refund("refund-2", 1), "refund_exceeds_residual")
    rows.append({"case": "accounted outcome", "expected": "closed_accounted", "observed": a.outcome, "match": a.outcome == "closed_accounted"})
    return rows


if __name__ == "__main__":
    results = exercise()
    print(json.dumps(results, indent=2))
    raise SystemExit(0 if all(row["match"] for row in results) else 1)

"""Check retained sample arithmetic only; do not invoke the product evaluator."""
from fractions import Fraction
from hashlib import sha256
import json
from pathlib import Path

here = Path(__file__).resolve().parent
repo = here.parents[2]
receipt = json.loads((here / "reference-calculations.json").read_text())
source = repo / receipt["source"]["path"]
assert sha256(source.read_bytes()).hexdigest() == receipt["source"]["sha256"]

loan = receipt["loan"]
interest = Fraction(int(loan["notional_micro_USD"])) * Fraction(loan["rate"]) * Fraction(loan["day_fraction"])
floored_interest = interest.numerator // interest.denominator
assert str(interest) == loan["exact_interest_micro_USD"]
assert str(floored_interest) == loan["floored_interest_micro_USD"]
assert str(interest - floored_interest) == loan["discarded_fraction_micro_USD"]
assert floored_interest + int(loan["principal_due_micro_USD"]) == int(loan["total_due_micro_USD"])
assert int(loan["notional_micro_USD"]) - int(loan["principal_due_micro_USD"]) == int(loan["remaining_contractual_notional_micro_USD"])

swap = receipt["swap"]
x, y = map(int, swap["reserves_before"])
dx = int(swap["input"])
multiplier = Fraction(swap["fee_multiplier"])
output = dx * multiplier * y / (x + dx * multiplier)
floored_output = output.numerator // output.denominator
assert str(output) == swap["exact_output"]
assert str(floored_output) == swap["floored_output"]
assert [str(x + dx), str(y - floored_output)] == swap["reserves_after"]
assert (floored_output >= 19700) == swap["min_out_19700_satisfied"]
assert (floored_output >= 19744) == swap["min_out_19744_satisfied"]

width = receipt["width_boundary"]
a, b = int(width["operand_a"]), int(width["operand_b"])
assert 0 <= a < 2**128 and 0 <= b < 2**128
assert a * b >= 2**128
print("PASS: retained sample arithmetic and source digest; no evaluator, conformance, ledger, or proof claim")

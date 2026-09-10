"""Check selected independent arithmetic derivations; not a Core evaluator."""
import json
from fractions import Fraction

checks = []
def check(name, condition):
    assert condition, name
    checks.append(name)

q, r = divmod(-5, 2)
check('signed-floor', (q, r) == (-3, 1))
check('signed-ceil', q + int(r != 0) == -2)
check('UInt64-product-overflow', (2**64-1)*2 == 36893488147419103230 > 2**64-1)
check('quantity-mul', Fraction(15,10)*Fraction(20,10) == Fraction(300,100))
check('quantity-floor-div', Fraction(6,10) <= Fraction(123,100)/Fraction(20,10) < Fraction(7,10))
check('retained-price-orientation', Fraction(19743,10**4) == Fraction(19743,10000))
check('staged-work', (1+1+1+1)+(1+1)+(1+1+1+1) == 10)
check('first-child-failure-work', 1+(1+1+1) == 4)
check('strict-and-failure-work', 1+1+1+(1+1+1) == 6)
check('aggregate-node-bound', 1+128*(1+16) == 2177 and 1+2*2177 == 4355 and 2178 <= 4096 < 4355)
print(json.dumps({'scope':'selected arithmetic/count derivations only; no typed execution or proof',
                  'checks':checks,'passed':len(checks)},indent=2))

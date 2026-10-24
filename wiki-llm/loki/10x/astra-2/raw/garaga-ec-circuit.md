import sympy
from garaga.curves import CURVES
from garaga.modulo\_circuit import ModuloCircuit, ModuloCircuitElement, PyFelt, WriteOps
from garaga.precompiled\_circuits.fp2 import Fp2Circuits
def is\_quad\_residue(n, p):
"""
Returns True if n is a quadratic residue mod p.
"""
return sympy.ntheory.residue\_ntheory.is\_quad\_residue(n, p)
def sqrt\_mod\_p(n, p):
"""
Finds the minimum non-negative integer m such that (m\*m) % p == n.
"""
return min(sympy.ntheory.residue\_ntheory.sqrt\_mod(n, p, all\_roots=True))
class IsOnCurveCircuit(Fp2Circuits):
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
compilation\_mode=compilation\_mode,
generic\_circuit=True,
)
self.curve = CURVES[curve\_id]
def set\_consts(
self,
a: PyFelt | ModuloCircuitElement,
b: PyFelt | ModuloCircuitElement,
b20: PyFelt | ModuloCircuitElement,
b21: PyFelt | ModuloCircuitElement,
):
self.a = self.write\_element(a) if isinstance(a, PyFelt) else a
self.b = self.write\_element(b) if isinstance(b, PyFelt) else b
self.b20 = self.write\_element(b20) if isinstance(b20, PyFelt) else b20
self.b21 = self.write\_element(b21) if isinstance(b21, PyFelt) else b21
def \_is\_on\_curve\_G1(
self, x: ModuloCircuitElement, y: ModuloCircuitElement
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
# y^2 = x^3 + ax + b
a = self.a
b = self.b
y2 = self.mul(y, y)
x3 = self.mul(x, self.mul(x, x))
if a.value != 0:
ax = self.mul(a, x)
x3\_ax\_b = self.add(x3, self.add(ax, b))
else:
x3\_ax\_b = self.add(x3, b)
return y2, x3\_ax\_b
def \_is\_on\_curve\_G2(
self,
x0: ModuloCircuitElement,
x1: ModuloCircuitElement,
y0: ModuloCircuitElement,
y1: ModuloCircuitElement,
):
# y^2 = x^3 + ax + b [Fp2]
a = self.a
b0 = self.b20
b1 = self.b21
y2 = self.fp2\_square([y0, y1])
x2 = self.fp2\_square([x0, x1])
x3 = self.fp2\_mul([x0, x1], x2)
ax = [self.mul(a, x0), self.mul(a, x1)]
ax\_b = [self.add(ax[0], b0), self.add(ax[1], b1)]
x3\_ax\_b = [self.add(x3[0], ax\_b[0]), self.add(x3[1], ax\_b[1])]
return y2, x3\_ax\_b
class DerivePointFromX(ModuloCircuit):
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
generic\_circuit=True,
compilation\_mode=compilation\_mode,
)
self.curve = CURVES[curve\_id]
def \_derive\_point\_from\_x(
self,
x: ModuloCircuitElement,
a: ModuloCircuitElement,
b: ModuloCircuitElement,
g: ModuloCircuitElement,
) -> list[ModuloCircuitElement]:
# y^2 = x^3 + ax + b
# Assumes a == 0.
x3 = self.mul(x, self.mul(x, x))
rhs = self.add(x3, self.add(self.mul(a, x), b))
grhs = self.mul(g, rhs)
# WRITE g\*rhs and rhs "square roots" to circuit.
# If rhs is a square, write zero to gx and the square root of rhs to x3\_ax\_b\_sqrt.
# Otherwise, write the square root of gx to gx\_sqrt and zero to x3\_ax\_b\_sqrt.
## %{
if is\_quad\_residue(rhs.value, self.field.p):
rhs\_sqrt = self.write\_element(
self.field(sqrt\_mod\_p(rhs.value, self.field.p)),
WriteOps.WITNESS,
)
grhs\_sqrt = self.write\_element(self.field.zero(), WriteOps.WITNESS)
else:
assert is\_quad\_residue(grhs.value, self.field.p) # Sanity check.
rhs\_sqrt = self.write\_element(self.field.zero(), WriteOps.WITNESS)
grhs\_sqrt = self.write\_element(
self.field(sqrt\_mod\_p(grhs.value, self.field.p)),
WriteOps.WITNESS,
)
## %}
should\_be\_rhs = self.mul(rhs\_sqrt, rhs\_sqrt)
should\_be\_grhs = self.mul(grhs\_sqrt, grhs\_sqrt)
return (rhs, grhs, should\_be\_rhs, should\_be\_grhs, rhs\_sqrt)
class DecompressG1Point(ModuloCircuit):
"""
A class to decompress a G1 point on an elliptic curve given the x-coordinate.
This class is a specialized ModuloCircuit that uses the curve parameters to compute the y-coordinate
from a given x-coordinate, ensuring that the point lies on the curve. The y-coordinate is determined
using the s\_bit, which is extracted from the compressed G1 point. The s\_bit indicates which of the two
possible y-coordinates (positive or negative) should be selected.
"""
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
generic\_circuit=True,
compilation\_mode=compilation\_mode,
)
self.curve = CURVES[curve\_id]
def derive\_y\_from\_x(
self,
b: ModuloCircuitElement,
x: ModuloCircuitElement,
) -> ModuloCircuitElement:
"""
Derive the two possible y-coordinates from the given x-coordinate on the elliptic curve.
Ensures that the point lies on the curve.
Assumes that the curve equation is y^2 = x^3 + b.
:param x: The x-coordinate as a ModuloCircuitElement.
:param b: The curve parameter b as a ModuloCircuitElement.
:return: The two possible y-coordinates as a ModuloCircuitElement.
No assumption on the order of the two y-coordinates.
:raises AssertionError: If the x-coordinate does not lie on the curve.
"""
assert (
CURVES[self.curve\_id].a == 0
), "This circuit is only supported for curves with a = 0"
assert (
self.compilation\_mode == 0
), "This circuit is only supported for CairoZero"
# y^2 = x^3 + b
x3 = self.mul(x, self.mul(x, x))
rhs = self.add(x3, b)
y = self.fp\_sqrt(rhs)
neg\_y = self.neg(y)
return (y, neg\_y)
class ECIPCircuits(ModuloCircuit):
def \_\_init\_\_(
self,
name: str,
curve\_id: int,
compilation\_mode: int = 0,
generic\_circuit: bool = True,
):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
generic\_circuit=generic\_circuit,
compilation\_mode=compilation\_mode,
)
self.curve = CURVES[curve\_id]
def \_slope\_intercept\_same\_point(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
A\_weirstrass: ModuloCircuitElement,
):
# Compute doubling slope m = (3x^2 + A) / 2y
xA0, yA0 = P
three = self.set\_or\_get\_constant(self.field(3))
mA0\_num = self.add(
self.mul(three, self.mul(xA0, xA0)),
A\_weirstrass,
)
mA0\_den = self.add(yA0, yA0)
m\_A0 = self.div(mA0\_num, mA0\_den)
# Compute intercept b = y - x\*m
b\_A0 = self.sub(yA0, self.mul(xA0, m\_A0))
# Compute A2 = -(2\*P)
xA2 = self.sub(self.mul(m\_A0, m\_A0), self.add(xA0, xA0))
yA2 = self.sub(self.mul(m\_A0, self.sub(xA0, xA2)), yA0)
yA2 = self.neg(yA2)
# Compute slope between A0 and A2
mA0A2\_num = self.sub(yA2, yA0)
mA0A2\_den = self.sub(xA2, xA0)
m\_A0A2 = self.div(mA0A2\_num, mA0A2\_den)
# coeff2 = (2 \* yA2) \* (xA0 - xA2) / (3 \* xA2^2 + A - 2 \* m \* yA2)
m\_yA2 = self.mul(m\_A0A2, yA2)
coeff2 = self.div(
self.mul(self.add(yA2, yA2), self.sub(xA0, xA2)),
self.add(
self.mul(three, self.mul(xA2, xA2)),
self.sub(A\_weirstrass, self.add(m\_yA2, m\_yA2)),
),
)
coeff0 = self.add(coeff2, self.add(m\_A0A2, m\_A0A2))
# Return slope intercept of A0 (for RHS) and coeff0/2 for LHS
return (
m\_A0,
b\_A0,
xA0,
yA0,
xA2,
yA2,
coeff0,
coeff2,
)
def \_accumulate\_eval\_point\_challenge\_signed\_same\_point(
self,
eval\_accumulator: ModuloCircuitElement,
slope\_intercept: tuple[ModuloCircuitElement, ModuloCircuitElement],
xA: ModuloCircuitElement,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
ep: ModuloCircuitElement,
en: ModuloCircuitElement,
sign\_ep: ModuloCircuitElement,
sign\_en: ModuloCircuitElement,
) -> ModuloCircuitElement:
assert isinstance(ep, ModuloCircuitElement) and isinstance(
en, ModuloCircuitElement
)
assert isinstance(sign\_ep, ModuloCircuitElement) and isinstance(
sign\_en, ModuloCircuitElement
)
m, b = slope\_intercept
xP, yP = P
num = self.sub(xA, xP)
# den\_tmp = m\*xP + b
den\_tmp = self.add(self.mul(m, xP), b)
# den\_pos = yP - (m\*xP + b) = yP - m\*xP - b
den\_pos = self.sub(yP, den\_tmp)
# den\_neg = -yP - m\*xP -b
den\_neg = self.sub(self.neg(yP), den\_tmp)
eval\_pos = self.mul(self.mul(sign\_ep, ep), self.div(num, den\_pos))
eval\_neg = self.mul(self.mul(sign\_en, en), self.div(num, den\_neg))
eval\_signed = self.add(eval\_pos, eval\_neg)
res = self.add(eval\_accumulator, eval\_signed)
return res
def \_RHS\_finalize\_acc(
self,
eval\_accumulator: ModuloCircuitElement,
slope\_intercept: tuple[ModuloCircuitElement, ModuloCircuitElement],
xA: ModuloCircuitElement,
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
):
m, b = slope\_intercept
xQ, yQ = Q
num = self.sub(xA, xQ)
# den\_tmp = m\*xQ + b
den\_tmp = self.add(self.mul(m, xQ), b)
# den\_neg = -yQ - m\*xQ -b
den\_neg = self.sub(self.neg(yQ), den\_tmp)
eval\_neg = self.div(num, den\_neg)
res = self.add(eval\_accumulator, eval\_neg)
return res
def \_eval\_function\_challenge\_single(
self,
A: tuple[ModuloCircuitElement, ModuloCircuitElement],
coeff: ModuloCircuitElement,
log\_div\_a\_num: list[ModuloCircuitElement],
log\_div\_a\_den: list[ModuloCircuitElement],
log\_div\_b\_num: list[ModuloCircuitElement],
log\_div\_b\_den: list[ModuloCircuitElement],
var\_name: str = "xA",
) -> ModuloCircuitElement:
xA, yA = A
F\_A = self.add(
self.div(
self.eval\_horner(
log\_div\_a\_num, xA, poly\_name="sumdlogdiv\_a\_num", var\_name=var\_name
),
self.eval\_horner(
log\_div\_a\_den, xA, poly\_name="sumdlogdiv\_a\_den", var\_name=var\_name
),
),
self.mul(
yA,
self.div(
self.eval\_horner(
log\_div\_b\_num,
xA,
poly\_name="sumdlogdiv\_b\_num",
var\_name=var\_name,
),
self.eval\_horner(
log\_div\_b\_den,
xA,
poly\_name="sumdlogdiv\_b\_den",
var\_name=var\_name,
),
),
),
)
return self.mul(coeff, F\_A)
def \_eval\_function\_challenge\_dupl(
self,
A0: tuple[ModuloCircuitElement, ModuloCircuitElement],
A2: tuple[ModuloCircuitElement, ModuloCircuitElement],
coeff0: ModuloCircuitElement,
coeff2: ModuloCircuitElement,
log\_div\_a\_num: list[ModuloCircuitElement],
log\_div\_a\_den: list[ModuloCircuitElement],
log\_div\_b\_num: list[ModuloCircuitElement],
log\_div\_b\_den: list[ModuloCircuitElement],
) -> ModuloCircuitElement:
# F = a(x) + y\*b(x), a and b being rational functions.
# computes coeff0\*F(A0) - coeff2\*F(A2)
xA0, yA0 = A0
xA2, yA2 = A2
F\_A0 = self.\_eval\_function\_challenge\_single(
A0,
coeff0,
log\_div\_a\_num,
log\_div\_a\_den,
log\_div\_b\_num,
log\_div\_b\_den,
var\_name="xA0",
)
F\_A2 = self.\_eval\_function\_challenge\_single(
A2,
coeff2,
log\_div\_a\_num,
log\_div\_a\_den,
log\_div\_b\_num,
log\_div\_b\_den,
var\_name="xA2",
)
# return coeff0\*F(A0) - coeff2\*F(A2)
res = self.sub(F\_A0, F\_A2)
return res
def \_init\_function\_challenge\_dupl(
self,
xA0: ModuloCircuitElement,
xA2: ModuloCircuitElement,
log\_div\_a\_num: list[ModuloCircuitElement],
log\_div\_a\_den: list[ModuloCircuitElement],
log\_div\_b\_num: list[ModuloCircuitElement],
log\_div\_b\_den: list[ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
# F = a(x) + y\*b(x), a and b being rational functions.
# computes F(A0) and F(A2)
# Precompute powers of xA0 and xA2 for evaluating the polynomials.
xA0\_powers = [xA0]
xA2\_powers = [xA2]
for i in range(len(log\_div\_b\_den) - 2):
xA0\_powers.append(self.mul(xA0\_powers[-1], xA0, comment=f"xA0^{i+2}"))
xA2\_powers.append(self.mul(xA2\_powers[-1], xA2, comment=f"xA2^{i+2}"))
A\_NUM\_A0 = self.eval\_poly(
log\_div\_a\_num, xA0\_powers, poly\_name="sumdlogdiv\_a\_num", var\_name="xA0"
)
A\_DEN\_A0 = self.eval\_poly(
log\_div\_a\_den, xA0\_powers, poly\_name="sumdlogdiv\_a\_den", var\_name="xA0"
)
B\_NUM\_A0 = self.eval\_poly(
log\_div\_b\_num, xA0\_powers, poly\_name="sumdlogdiv\_b\_num", var\_name="xA0"
)
B\_DEN\_A0 = self.eval\_poly(
log\_div\_b\_den, xA0\_powers, poly\_name="sumdlogdiv\_b\_den", var\_name="xA0"
)
A\_NUM\_A2 = self.eval\_poly(
log\_div\_a\_num, xA2\_powers, poly\_name="sumdlogdiv\_a\_num", var\_name="xA2"
)
A\_DEN\_A2 = self.eval\_poly(
log\_div\_a\_den, xA2\_powers, poly\_name="sumdlogdiv\_a\_den", var\_name="xA2"
)
B\_NUM\_A2 = self.eval\_poly(
log\_div\_b\_num, xA2\_powers, poly\_name="sumdlogdiv\_b\_num", var\_name="xA2"
)
B\_DEN\_A2 = self.eval\_poly(
log\_div\_b\_den, xA2\_powers, poly\_name="sumdlogdiv\_b\_den", var\_name="xA2"
)
# return F(A0) and F(A2), and the last power of xA0 and xA2 used in a\_den (also equal to b\_num)
a\_den\_degree = len(log\_div\_a\_den) - 1
assert a\_den\_degree == len(log\_div\_b\_num) - 1
return (
A\_NUM\_A0,
A\_DEN\_A0,
B\_NUM\_A0,
B\_DEN\_A0,
A\_NUM\_A2,
A\_DEN\_A2,
B\_NUM\_A2,
B\_DEN\_A2,
xA0\_powers[a\_den\_degree - 1],
xA2\_powers[a\_den\_degree - 1],
)
def \_accumulate\_function\_challenge\_dupl(
self,
a\_num\_acc\_A0: ModuloCircuitElement,
a\_den\_acc\_A0: ModuloCircuitElement,
b\_num\_acc\_A0: ModuloCircuitElement,
b\_den\_acc\_A0: ModuloCircuitElement,
a\_num\_acc\_A2: ModuloCircuitElement,
a\_den\_acc\_A2: ModuloCircuitElement,
b\_num\_acc\_A2: ModuloCircuitElement,
b\_den\_acc\_A2: ModuloCircuitElement,
xA0: ModuloCircuitElement,
xA2: ModuloCircuitElement,
xA0\_power: ModuloCircuitElement,
xA2\_power: ModuloCircuitElement,
next\_a\_num\_coeff: ModuloCircuitElement,
next\_a\_den\_coeff: ModuloCircuitElement,
next\_b\_num\_coeff: ModuloCircuitElement,
next\_b\_den\_coeff: ModuloCircuitElement,
):
# Update accumulators for A0
new\_a\_num\_acc\_A0 = self.add(a\_num\_acc\_A0, self.mul(next\_a\_num\_coeff, xA0\_power))
next\_xA0\_power = self.mul(xA0\_power, xA0)
new\_a\_den\_acc\_A0 = self.add(
a\_den\_acc\_A0, self.mul(next\_a\_den\_coeff, next\_xA0\_power)
)
new\_b\_num\_acc\_A0 = self.add(
b\_num\_acc\_A0, self.mul(next\_b\_num\_coeff, next\_xA0\_power)
)
next\_b\_den\_A0\_power = self.mul(next\_xA0\_power, xA0)
for \_ in range(2):
next\_b\_den\_A0\_power = self.mul(next\_b\_den\_A0\_power, xA0)
new\_b\_den\_acc\_A0 = self.add(
b\_den\_acc\_A0, self.mul(next\_b\_den\_coeff, next\_b\_den\_A0\_power)
)
# Update accumulators for A2
new\_a\_num\_acc\_A2 = self.add(a\_num\_acc\_A2, self.mul(next\_a\_num\_coeff, xA2\_power))
next\_xA2\_power = self.mul(xA2\_power, xA2)
new\_a\_den\_acc\_A2 = self.add(
a\_den\_acc\_A2, self.mul(next\_a\_den\_coeff, next\_xA2\_power)
)
new\_b\_num\_acc\_A2 = self.add(
b\_num\_acc\_A2, self.mul(next\_b\_num\_coeff, next\_xA2\_power)
)
next\_b\_den\_A2\_power = next\_xA2\_power
for \_ in range(3):
next\_b\_den\_A2\_power = self.mul(next\_b\_den\_A2\_power, xA2)
new\_b\_den\_acc\_A2 = self.add(
b\_den\_acc\_A2, self.mul(next\_b\_den\_coeff, next\_b\_den\_A2\_power)
)
return (
new\_a\_num\_acc\_A0,
new\_a\_den\_acc\_A0,
new\_b\_num\_acc\_A0,
new\_b\_den\_acc\_A0,
new\_a\_num\_acc\_A2,
new\_a\_den\_acc\_A2,
new\_b\_num\_acc\_A2,
new\_b\_den\_acc\_A2,
next\_xA0\_power,
next\_xA2\_power,
)
def \_finalize\_function\_challenge\_dupl(
self,
a\_num\_acc\_A0: ModuloCircuitElement,
a\_den\_acc\_A0: ModuloCircuitElement,
b\_num\_acc\_A0: ModuloCircuitElement,
b\_den\_acc\_A0: ModuloCircuitElement,
a\_num\_acc\_A2: ModuloCircuitElement,
a\_den\_acc\_A2: ModuloCircuitElement,
b\_num\_acc\_A2: ModuloCircuitElement,
b\_den\_acc\_A2: ModuloCircuitElement,
yA0: ModuloCircuitElement,
yA2: ModuloCircuitElement,
coeff\_A0: ModuloCircuitElement,
coeff\_A2: ModuloCircuitElement,
):
F\_A0 = self.add(
self.div(a\_num\_acc\_A0, a\_den\_acc\_A0),
self.mul(yA0, self.div(b\_num\_acc\_A0, b\_den\_acc\_A0)),
comment="a(x0) + y0 b(x0)",
)
F\_A2 = self.add(
self.div(a\_num\_acc\_A2, a\_den\_acc\_A2),
self.mul(yA2, self.div(b\_num\_acc\_A2, b\_den\_acc\_A2)),
comment="a(x2) + y2 b(x2)",
)
# return coeff0\*F(A0) - coeff2\*F(A2)
res = self.sub(self.mul(coeff\_A0, F\_A0), self.mul(coeff\_A2, F\_A2))
return res
class BasicEC(ModuloCircuit):
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
generic\_circuit=True,
compilation\_mode=compilation\_mode,
)
self.curve = CURVES[curve\_id]
def \_compute\_adding\_slope(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
):
xP, yP = P
xQ, yQ = Q
slope = self.div(self.sub(yP, yQ), self.sub(xP, xQ))
return slope
def \_compute\_doubling\_slope(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
A: ModuloCircuitElement,
):
xP, yP = P
# Compute doubling slope m = (3x^2 + A) / 2y
three = self.set\_or\_get\_constant(self.field(3))
m\_num = self.add(
self.mul(three, self.mul(xP, xP)),
A,
)
m\_den = self.add(yP, yP)
m = self.div(m\_num, m\_den)
return m
def add\_points(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
xP, yP = P
xQ, yQ = Q
slope = self.\_compute\_adding\_slope(P, Q)
slope\_sqr = self.mul(slope, slope)
nx = self.sub(self.sub(slope\_sqr, xP), xQ)
ny = self.sub(self.mul(slope, self.sub(xP, nx)), yP)
return (nx, ny)
def double\_point(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
A: ModuloCircuitElement,
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
xP, yP = P
slope = self.\_compute\_doubling\_slope(P, A)
slope\_sqr = self.mul(slope, slope)
nx = self.sub(self.sub(slope\_sqr, xP), xP)
ny = self.sub(self.mul(slope, self.sub(xP, nx)), yP)
return (nx, ny)
def scalar\_mul\_2\_pow\_k(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
A: ModuloCircuitElement,
k: int,
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
for \_ in range(k):
P = self.double\_point(P, A)
return P
def \_is\_on\_curve\_G1\_weirstrass(
self,
x: ModuloCircuitElement,
y: ModuloCircuitElement,
A: ModuloCircuitElement,
b: ModuloCircuitElement,
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
# y^2 = x^3 + ax + b
y2 = self.mul(y, y)
x3 = self.mul(x, self.mul(x, x))
ax = self.mul(A, x)
x3\_ax\_b = self.add(x3, self.add(ax, b))
return y2, x3\_ax\_b
def negate\_point(
self, P: tuple[ModuloCircuitElement, ModuloCircuitElement]
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Negate a point in G1."""
x, y = P
return (x, self.neg(y))
def add\_points(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Add two points in G1."""
xP, yP = P
xQ, yQ = Q
slope = self.\_compute\_adding\_slope(P, Q)
slope\_sqr = self.mul(slope, slope)
nx = self.sub(self.sub(slope\_sqr, xP), xQ)
ny = self.sub(self.mul(slope, self.sub(xP, nx)), yP)
return (nx, ny)
def double\_point\_a\_eq\_0(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Double a point in G1 when curve parameter a=0."""
xP, yP = P
three = self.set\_or\_get\_constant(self.field(3))
slope = self.div(
self.mul(three, self.mul(xP, xP)), self.add(yP, yP) # 3x^2 # 2y
)
slope\_sqr = self.mul(slope, slope)
nx = self.sub(self.sub(slope\_sqr, xP), xP)
ny = self.sub(self.mul(slope, self.sub(xP, nx)), yP)
return (nx, ny)
def triple\_point(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
A: ModuloCircuitElement,
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Triple a point in G1."""
xP, yP = P
# compute λ1 = (3p.x²+a)/2p.y,
xx = self.square(xP)
three\_xx = self.sum([xx, xx, xx])
m\_num = self.add(three\_xx, A)
λ1 = self.div(m\_num, self.double(yP))
# xr = λ1²-2p.x
x2 = self.sub(self.square(λ1), self.add(xP, xP))
# omit y2 computation, and
# compute λ2 = 2p.y/(x2 − p.x) − λ1.
x1x2 = self.sub(xP, x2)
λ2 = self.div(self.double(yP), x1x2)
λ2 = self.sub(λ2, λ1)
# xr = λ²-p.x-x2
xr = self.sub(self.square(λ2), self.add(xP, x2))
# yr = λ(p.x-xr) - p.y
yr = self.sub(self.mul(λ2, self.sub(xP, xr)), yP)
return (xr, yr)
def double\_n\_times(
self, P: tuple[ModuloCircuitElement, ModuloCircuitElement], n: int
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Double a point n times in G1."""
Q = P
for \_ in range(n):
Q = self.double\_point\_a\_eq\_0(Q)
return Q
def n\_quadruple\_and\_add(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
adds: list[tuple[ModuloCircuitElement, ModuloCircuitElement]],
A\_weirstrass: ModuloCircuitElement,
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Quadruple a point n times and add it to the point."""
Q = P
for add in adds:
Q = self.double\_point(Q, A\_weirstrass)
Q = self.double\_and\_add(Q, add)
return Q
def double\_and\_add(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
add: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Double a point and add it to the point."""
Px, Py = P
Qx, Qy = add
# compute λ1 = (q.y-p.y)/(q.x-p.x)
yqyp = self.sub(Qy, Py)
xqxp = self.sub(Qx, Px)
λ1 = self.div(yqyp, xqxp)
# compute x2 = λ1²-p.x-q.x
x2 = self.sub(self.square(λ1), self.add(Px, Qx))
# omit y2 computation
# compute -λ2 = λ1+2\*p.y/(x2-p.x)
ypyp = self.double(Py)
x2xp = self.sub(x2, Px)
λ2 = self.div(ypyp, x2xp)
λ2 = self.add(λ1, λ2)
# compute x3 = (-λ2)²-p.x-x2
x3 = self.sub(self.square(λ2), self.add(Px, x2))
# compute y3 = -λ2\*(x3 - p.x)-p.y
y3 = self.sub(self.mul(λ2, self.sub(x3, Px)), Py)
return (x3, y3)
class BasicECG2(Fp2Circuits):
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(
name=name,
curve\_id=curve\_id,
generic\_circuit=True,
compilation\_mode=compilation\_mode,
)
self.curve = CURVES[curve\_id]
def \_is\_on\_curve\_G2\_weirstrass(
self,
x0: ModuloCircuitElement,
x1: ModuloCircuitElement,
y0: ModuloCircuitElement,
y1: ModuloCircuitElement,
a: ModuloCircuitElement,
b0: ModuloCircuitElement,
b1: ModuloCircuitElement,
):
# y^2 = x^3 + ax + b [Fp2]
y2 = self.fp2\_square([y0, y1])
x2 = self.fp2\_square([x0, x1])
x3 = self.fp2\_mul([x0, x1], x2)
ax = [self.mul(a, x0), self.mul(a, x1)]
ax\_b = [self.add(ax[0], b0), self.add(ax[1], b1)]
x3\_ax\_b = [self.add(x3[0], ax\_b[0]), self.add(x3[1], ax\_b[1])]
return y2, x3\_ax\_b
def \_compute\_adding\_slope(
self,
P: tuple[
tuple[ModuloCircuitElement, ModuloCircuitElement],
tuple[ModuloCircuitElement, ModuloCircuitElement],
],
Q: tuple[
tuple[ModuloCircuitElement, ModuloCircuitElement],
tuple[ModuloCircuitElement, ModuloCircuitElement],
],
):
xP, yP = P
xQ, yQ = Q
slope = self.fp2\_div(self.vector\_sub(yP, yQ), self.vector\_sub(xP, xQ))
return slope
def \_compute\_doubling\_slope\_a\_eq\_0(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
):
xP, yP = P
# Compute doubling slope m = (3x^2 + A) / 2y
three = self.set\_or\_get\_constant(self.field(3))
m\_num = self.vector\_scale(self.fp2\_square(xP), three)
m\_den = self.vector\_add(yP, yP)
m = self.fp2\_div(m\_num, m\_den)
return m
def add\_points(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
xP, yP = P
xQ, yQ = Q
slope = self.\_compute\_adding\_slope(P, Q)
slope\_sqr = self.fp2\_square(slope)
nx = self.vector\_sub(self.vector\_sub(slope\_sqr, xP), xQ)
ny = self.vector\_sub(self.fp2\_mul(slope, self.vector\_sub(xP, nx)), yP)
return (nx, ny)
def double\_point\_a\_eq\_0(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
xP, yP = P
slope = self.\_compute\_doubling\_slope\_a\_eq\_0(P)
slope\_sqr = self.fp2\_square(slope)
nx = self.vector\_sub(self.vector\_sub(slope\_sqr, xP), xP)
ny = self.vector\_sub(self.fp2\_mul(slope, self.vector\_sub(xP, nx)), yP)
return (nx, ny)
def double\_n\_times(self, P, n):
Q = P
for \_ in range(n):
Q = self.double\_point\_a\_eq\_0(Q)
return Q
def negate\_point(
self, P: tuple[list[ModuloCircuitElement], list[ModuloCircuitElement]]
) -> tuple[list[ModuloCircuitElement], list[ModuloCircuitElement]]:
x, y = P
return (x, self.vector\_neg(y))
def double\_and\_add(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
add: tuple[ModuloCircuitElement, ModuloCircuitElement],
) -> tuple[ModuloCircuitElement, ModuloCircuitElement]:
"""Double a point and add it to the point."""
Px, Py = P
Qx, Qy = add
# compute λ1 = (q.y-p.y)/(q.x-p.x)
yqyp = self.vector\_sub(Qy, Py)
xqxp = self.vector\_sub(Qx, Px)
λ1 = self.fp2\_div(yqyp, xqxp)
# compute x2 = λ1²-p.x-q.x
x2 = self.vector\_sub(self.fp2\_square(λ1), self.vector\_add(Px, Qx))
# omit y2 computation
# compute -λ2 = λ1+2\*p.y/(x2-p.x)
ypyp = self.vector\_add(Py, Py)
x2xp = self.vector\_sub(x2, Px)
λ2 = self.fp2\_div(ypyp, x2xp)
λ2 = self.vector\_add(λ1, λ2)
# compute x3 = (-λ2)²-p.x-x2
x3 = self.vector\_sub(self.fp2\_square(λ2), self.vector\_add(Px, x2))
# compute y3 = -λ2\*(x3 - p.x)-p.y
y3 = self.vector\_sub(self.fp2\_mul(λ2, self.vector\_sub(x3, Px)), Py)
return (x3, y3)
class FakeGLVCircuits(BasicEC):
def \_\_init\_\_(self, name: str, curve\_id: int, compilation\_mode: int = 0):
super().\_\_init\_\_(name, curve\_id, compilation\_mode)
def prepare\_points\_glv\_fake\_glv(
self, Px, P0y, P1y, Qx, Q0y, Phi\_P0y, Phi\_P1y, Phi\_Q0y, Gen, third\_root
):
P0 = (Px, P0y)
P1 = (Px, P1y)
Q0 = (Qx, Q0y)
S0 = self.add\_points(P0, Q0)
S1 = (S0[0], self.neg(S0[1]))
S2 = self.add\_points(P1, Q0)
Phi\_P0x = self.mul(Px, third\_root)
Phi\_P0 = (Phi\_P0x, Phi\_P0y)
Phi\_P1 = (Phi\_P0x, Phi\_P1y)
Phi\_Q0x = self.mul(Q0[0], third\_root)
Phi\_Q0 = (Phi\_Q0x, Phi\_Q0y)
Phi\_S0 = self.add\_points(Phi\_P0, Phi\_Q0)
Phi\_S1 = (Phi\_S0[0], self.neg(Phi\_S0[1]))
Phi\_S2 = self.add\_points(Phi\_P1, Phi\_Q0)
Phi\_S3 = (Phi\_S2[0], self.neg(Phi\_S2[1]))
B1 = self.add\_points(S1, Phi\_S1) # P + Q + Φ(P) + Φ(Q)
B2 = self.add\_points(S1, Phi\_S2) # P + Q + Φ(P) - Φ(Q)
B3 = self.add\_points(S1, Phi\_S3) # P + Q - Φ(P) + Φ(Q)
B4 = self.add\_points(S1, Phi\_S0) # P + Q - Φ(P) - Φ(Q)
B5 = self.add\_points(S2, Phi\_S1) # P - Q + Φ(P) + Φ(Q)
B6 = self.add\_points(S2, Phi\_S2) # P - Q + Φ(P) - Φ(Q)
B7 = self.add\_points(S2, Phi\_S3) # P - Q - Φ(P) + Φ(Q)
B8 = self.add\_points(S2, Phi\_S0) # P - Q - Φ(P) - Φ(Q)
B9y = self.neg(B8[1]) # -P + Q + Φ(P) + Φ(Q)
B10y = self.neg(B7[1]) # -P + Q + Φ(P) - Φ(Q)
B11y = self.neg(B6[1]) # -P + Q - Φ(P) + Φ(Q)
B12y = self.neg(B5[1]) # -P + Q - Φ(P) - Φ(Q)
B13y = self.neg(B4[1]) # -P - Q + Φ(P) + Φ(Q)
B14y = self.neg(B3[1]) # -P - Q + Φ(P) - Φ(Q)
B15y = self.neg(B2[1]) # -P - Q - Φ(P) + Φ(Q)
B16y = self.neg(B1[1]) # -P - Q - Φ(P) - Φ(Q)
Acc = self.add\_points(B1, Gen)
return (
B1,
B2,
B3,
B4,
B5,
B6,
B7,
B8,
B9y,
B10y,
B11y,
B12y,
B13y,
B14y,
B15y,
B16y,
Phi\_P0x,
Phi\_Q0x,
Acc,
)
def prepare\_points\_fake\_glv(
self,
P: tuple[ModuloCircuitElement, ModuloCircuitElement],
Q: tuple[ModuloCircuitElement, ModuloCircuitElement],
s2\_sign: ModuloCircuitElement,
A\_weirstrass: ModuloCircuitElement,
):
P = (P[0], P[1])
Q = (Q[0], Q[1])
table\_P = [None, None, None]
# table\_P[0] = (P[0], self.neg(P[0])) # Unused
table\_P[1] = P
table\_P[2] = self.triple\_point(P, A\_weirstrass)
table\_R = [None, None, None]
R\_signed = [Q[0], self.mul(s2\_sign, Q[1])]
table\_R[0] = self.negate\_point(R\_signed)
table\_R[1] = R\_signed
table\_R[2] = self.triple\_point(R\_signed, A\_weirstrass)
T1 = self.add\_points(table\_P[2], table\_R[2])
T2 = Acc = self.add\_points(table\_P[1], table\_R[1])
T3 = self.add\_points(table\_P[2], table\_R[1])
T4 = self.add\_points(table\_P[1], table\_R[2])
T5y = self.neg(T2[1])
T6y = self.neg(T1[1])
T7y = self.neg(T4[1])
T8y = self.neg(T3[1])
T9 = self.add\_points(table\_P[2], table\_R[0])
\_neg\_table\_R\_verify\_2 = self.negate\_point(table\_R[2])
T10 = self.add\_points(table\_P[1], \_neg\_table\_R\_verify\_2)
T11 = self.add\_points(table\_P[2], \_neg\_table\_R\_verify\_2)
T12 = self.add\_points(table\_R[0], table\_P[1])
T13y = self.neg(T10[1])
T14y = self.neg(T9[1])
T15y = self.neg(T12[1])
T16y = self.neg(T11[1])
# P0y = table\_P[0][1]
return (
T1,
T2,
T3,
T4,
T5y,
T6y,
T7y,
T8y,
T9,
T10,
T11,
T12,
T13y,
T14y,
T15y,
T16y,
table\_R[2],
table\_R[0][1], # R0y
)
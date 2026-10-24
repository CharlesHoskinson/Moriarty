use core::array::ArrayTrait;
use core::circuit::{
CircuitElement, CircuitInput, CircuitInputs, CircuitModulus, circuit\_add, circuit\_inverse,
circuit\_mul, circuit\_sub, u384, u96,
};
use core::num::traits::{One, Zero};
use core::panic\_with\_felt252;
use core::poseidon::hades\_permutation;
use core::result::ResultTrait;
use corelib\_imports::bounded\_int::{downcast, upcast};
use corelib\_imports::integer::{U128sFromFelt252Result, u128s\_from\_felt252};
use garaga::basic\_field\_ops::{is\_opposite\_mod\_p, mul\_mod\_p, neg\_mod\_p, sub\_mod\_p};
use garaga::circuits::ec;
use garaga::core::circuit::{
AddInputResultTrait2, CircuitOutputsTrait, EvalCircuitTrait, u288IntoCircuitInputValue,
};
use garaga::definitions::{
G1Point, G1PointZero, get\_G, get\_a, get\_b, get\_curve\_order\_modulus, get\_eigenvalue, get\_min\_one,
get\_min\_one\_order, get\_modulus, get\_n, get\_nG\_glv\_fake\_glv, get\_third\_root\_of\_unity,
};
use garaga::ec::selectors;
#[generate\_trait]
pub impl G1PointImpl of G1PointTrait {
fn assert\_on\_curve\_excluding\_infinity(self: @G1Point, curve\_index: usize) {
if self.is\_on\_curve\_excluding\_infinity(curve\_index) == false {
panic\_with\_felt252('point not on curve');
}
}
fn is\_on\_curve\_excluding\_infinity(self: @G1Point, curve\_index: usize) -> bool {
let (check) = ec::run\_IS\_ON\_CURVE\_G1\_circuit(
\*self, get\_a(curve\_index), get\_b(curve\_index), curve\_index,
);
check.is\_zero()
}
fn negate(self: @G1Point, curve\_index: usize) -> G1Point {
let modulus = get\_modulus(curve\_index);
G1Point { x: \*self.x, y: neg\_mod\_p(\*self.y, modulus) }
}
fn assert\_in\_subgroup\_excluding\_infinity(self: @G1Point, curve\_index: usize) {
self.assert\_on\_curve\_excluding\_infinity(curve\_index);
match curve\_index {
0 => {}, // BN254 (cofactor 1)
1 => {
// https://github.com/Consensys/gnark-crypto/blob/ff4c0ddbe1ef37d1c1c6bec8c36fc43a84c86be5/ecc/bls12-381/g1.go#L492
let modulus = get\_modulus(curve\_index);
let phi\_p = G1Point {
x: mul\_mod\_p(get\_third\_root\_of\_unity(curve\_index), \*self.x, modulus),
y: \*self.y,
};
let x\_sq\_phi\_P = scalar\_mul\_by\_bls12\_381\_seed\_square(phi\_p);
if !ec\_safe\_add(\*self, x\_sq\_phi\_P, curve\_index).is\_infinity() {
panic\_with\_felt252('bls12-381 pt not in subgroup');
}
}, // BLS12-381
2 => {}, // SECP256K1 (cofactor 1)
3 => {}, // SECP256R1 (cofactor 1)
4 => { panic\_with\_felt252('ED25519 not implemented'); },
5 => {}, // GRUMPKIN (cofactor 1)
\_ => { panic\_with\_felt252('invalid curve index') },
}
}
fn is\_infinity(self: @G1Point) -> bool {
return self.is\_zero();
}
fn update\_hash\_state(
self: @G1Point, s0: felt252, s1: felt252, s2: felt252,
) -> (felt252, felt252, felt252) {
let base: felt252 = 79228162514264337593543950336; // 2\*\*96
let self = \*self;
let in\_1 = s0 + self.x.limb0.into() + base \* self.x.limb1.into();
let in\_2 = s1 + self.x.limb2.into() + base \* self.x.limb3.into();
let (s0, s1, s2) = hades\_permutation(in\_1, in\_2, s2);
let in\_1 = s0 + self.y.limb0.into() + base \* self.y.limb1.into();
let in\_2 = s1 + self.y.limb2.into() + base \* self.y.limb3.into();
let (s0, s1, s2) = hades\_permutation(in\_1, in\_2, s2);
return (s0, s1, s2);
}
}
// Adds two elliptic curve points on a given curve.
// Does not check the input points are on the curve.
pub fn ec\_safe\_add(p: G1Point, q: G1Point, curve\_index: usize) -> G1Point {
if p.is\_infinity() {
return q;
}
if q.is\_infinity() {
return p;
}
let modulus = get\_modulus(curve\_index);
let same\_x = sub\_mod\_p(p.x, q.x, modulus).is\_zero();
if same\_x {
let opposite\_y = is\_opposite\_mod\_p(p.y, q.y, modulus);
if opposite\_y {
return G1PointZero::zero();
} else {
let (res) = ec::run\_DOUBLE\_EC\_POINT\_circuit(p, get\_a(curve\_index), modulus);
return res;
}
} else {
let (res) = ec::run\_ADD\_EC\_POINT\_circuit(p, q, modulus);
return res;
}
}
pub fn \_ec\_safe\_add(
p: G1Point, q: G1Point, modulus: CircuitModulus, curve\_index: usize,
) -> G1Point {
if p.is\_infinity() {
return q;
}
if q.is\_infinity() {
return p;
}
let same\_x = sub\_mod\_p(p.x, q.x, modulus).is\_zero();
if same\_x {
let opposite\_y = is\_opposite\_mod\_p(p.y, q.y, modulus);
if opposite\_y {
return G1PointZero::zero();
} else {
let (res) = ec::run\_DOUBLE\_EC\_POINT\_circuit(p, get\_a(curve\_index), modulus);
return res;
}
} else {
let (res) = ec::run\_ADD\_EC\_POINT\_circuit(p, q, modulus);
return res;
}
}
#[derive(Drop, Serde)]
pub struct GlvFakeGlvHint {
pub Q: G1Point,
pub u1: felt252, // Encoded as 2^128 \* sign + abs(u1)
pub u2: felt252, // Encoded as 2^128 \* sign + abs(u2)
pub v1: felt252, // Encoded as 2^128 \* sign + abs(v1)
pub v2: felt252 // Encoded as 2^128 \* sign + abs(v2)
}
#[derive(Drop, Serde)]
pub struct FakeGlvHint {
pub Q: G1Point,
pub s1: u128, // (s1)\_u128 (always positive)
pub s2: felt252 // Encoded as 2^128 \* sign + abs(s2)\_u128
}
pub fn msm\_g1(
points: Span, scalars: Span, curve\_index: usize, mut hint: Span,
) -> G1Point {
match curve\_index {
0 | 1 | 2 => { return msm\_glv\_fake\_glv(points, scalars, curve\_index, ref hint); },
\_ => { return msm\_fake\_glv(points, scalars, curve\_index, ref hint); },
}
}
pub fn msm\_fake\_glv(
mut points: Span, mut scalars: Span, curve\_index: usize, ref hint: Span,
) -> G1Point {
let n = scalars.len();
assert(n == points.len(), 'n\_pts!=n\_scalars');
if n == 0 {
panic\_with\_felt252('Msm size must be >= 1');
}
assert(hint.len() == n \* 10, 'wrong fakeglv hint size');
let order\_modulus = get\_curve\_order\_modulus(curve\_index);
let curve\_order: u256 = get\_n(curve\_index);
let modulus = get\_modulus(curve\_index);
let A\_weirstrass = get\_a(curve\_index);
let order\_minus\_one = get\_min\_one\_order(curve\_index);
let base\_min\_one = get\_min\_one(curve\_index);
let mut acc = G1PointZero::zero();
while hint.len() != 0 {
let pt = \*points.pop\_front().unwrap();
let scalar = \*scalars.pop\_front().unwrap();
assert(scalar < curve\_order, 'unreduced scalar');
let scalar\_u384: u384 = scalar.into();
let fake\_glv\_hint: FakeGlvHint = Serde::deserialize(ref hint).unwrap();
let temp = \_scalar\_mul\_fake\_glv(
pt,
scalar\_u384,
order\_modulus,
modulus,
A\_weirstrass,
order\_minus\_one,
base\_min\_one,
fake\_glv\_hint,
curve\_index,
);
acc = \_ec\_safe\_add(acc, temp, modulus, curve\_index);
}
return acc;
}
pub fn \_scalar\_mul\_fake\_glv(
point: G1Point,
scalar: u384,
order\_modulus: CircuitModulus,
modulus: CircuitModulus,
A\_weirstrass: u384,
order\_minus\_one: u384,
base\_min\_one: u384,
hint: FakeGlvHint,
curve\_index: usize,
) -> G1Point {
if scalar.is\_zero() || point.is\_infinity() {
return G1PointZero::zero();
}
point.assert\_on\_curve\_excluding\_infinity(curve\_index);
if scalar.is\_one() {
return point;
}
if scalar == order\_minus\_one {
return G1Point { x: point.x, y: neg\_mod\_p(point.y, modulus) };
}
hint.Q.assert\_on\_curve\_excluding\_infinity(curve\_index);
let (\_s2\_sign\_order, \_s2\_sign\_base, \_s2\_abs): (u384, u384, u128) =
match u128s\_from\_felt252(hint.s2) {
U128sFromFelt252Result::Narrow(low) => {
(
u384 { limb0: 1, limb1: 0, limb2: 0, limb3: 0 },
u384 { limb0: 1, limb1: 0, limb2: 0, limb3: 0 },
low,
)
},
U128sFromFelt252Result::Wide((\_, low)) => { (order\_minus\_one, base\_min\_one, low) },
};
// Verify decomposition : s1 + scalar\*s2 = 0 mod n && s2!=0.
let s1 = CircuitElement::> {};
let S = CircuitElement::> {};
let s2 = CircuitElement::> {};
let s2\_sign = CircuitElement::> {};
let check = circuit\_add(s1, circuit\_mul(S, circuit\_mul(s2\_sign, s2)));
let s1\_u384: u384 = hint.s1.into();
let s2\_u384: u384 = \_s2\_abs.into();
let outputs = (check,)
.new\_inputs()
.next\_2(s1\_u384)
.next\_2(scalar)
.next\_2(s2\_u384)
.next\_2(\_s2\_sign\_order)
.done\_2()
.eval(order\_modulus)
.unwrap();
assert(\_s2\_abs != 0, 'FakeGLV: s2=0');
assert(outputs.get\_output(check).is\_zero(), 'Wrong FakeGLV decomposition');
let (T1, T2, T3, T4, T5y, T6y, T7y, T8y, T9, T10, T11, T12, T13y, T14y, T15y, T16y, R2, R0y) =
ec::run\_PREPARE\_FAKE\_GLV\_PTS\_circuit(
point, hint.Q, \_s2\_sign\_base, A\_weirstrass, modulus,
);
// ['T6', 'T7', 'T10', 'T11', 'T8', 'T5', 'T12', 'T9', 'T14', 'T15', 'T2', 'T3', 'T16', 'T13',
// 'T4', 'T1']
let mut Ts: Array = array![
G1Point { x: T1.x, y: T6y }, G1Point { x: T4.x, y: T7y }, T10, T11,
G1Point { x: T3.x, y: T8y }, G1Point { x: T2.x, y: T5y }, T12, T9,
G1Point { x: T9.x, y: T14y }, G1Point { x: T12.x, y: T15y }, T2, T3,
G1Point { x: T11.x, y: T16y }, G1Point { x: T10.x, y: T13y }, T4, T1,
];
let (mut selectors, s1lsb, s2lsb) = selectors::build\_selectors\_inlined\_fake\_glv(
hint.s1, \_s2\_abs,
);
// Add corrected point for the last bits 2-1 at position 16 in Ts array.
let last\_selector = \*selectors.pop\_front().unwrap();
let last\_selector\_pt = \*Ts[last\_selector];
let (last\_selector\_pt\_corrected) = ec::run\_ADD\_EC\_POINT\_circuit(last\_selector\_pt, R2, modulus);
Ts.append(last\_selector\_pt\_corrected);
let Ts = Ts.span();
// now the first selector should be 16 and will select the corrected point in the last
// iteration.
// First iteration (bit 128)
let selector\_y = \*selectors.pop\_back().unwrap();
let Bi = \*Ts[selector\_y];
let (Acc) = ec::run\_DOUBLE\_EC\_POINT\_circuit(T2, A\_weirstrass, modulus);
let (mut Acc) = ec::run\_ADD\_EC\_POINT\_circuit(Acc, Bi, modulus);
// assert(selectors.len() == 63, 'wrong number of selectors');
// 7 iterations\* 9 \* 2 bits = 63 \* 2 = 126 bits.
while let Some(selector\_y) = selectors.multi\_pop\_back::<9>() {
let [
selector\_y8,
selector\_y7,
selector\_y6,
selector\_y5,
selector\_y4,
selector\_y3,
selector\_y2,
selector\_y1,
selector\_y0,
] =
selector\_y
.unbox();
let Bi = \*Ts.at(selector\_y0);
let Bi1 = \*Ts.at(selector\_y1);
let Bi2 = \*Ts.at(selector\_y2);
let Bi3 = \*Ts.at(selector\_y3);
let Bi4 = \*Ts.at(selector\_y4);
let Bi5 = \*Ts.at(selector\_y5);
let Bi6 = \*Ts.at(selector\_y6);
let Bi7 = \*Ts.at(selector\_y7);
let Bi8 = \*Ts.at(selector\_y8);
let (\_Acc) = ec::run\_QUADRUPLE\_AND\_ADD\_9\_circuit(
Acc, Bi, Bi1, Bi2, Bi3, Bi4, Bi5, Bi6, Bi7, Bi8, A\_weirstrass, modulus,
);
Acc = \_Acc;
}
let (Acc) = if s1lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(
Acc, G1Point { x: point.x, y: neg\_mod\_p(point.y, modulus) }, modulus,
)
} else {
(Acc,)
};
let (Acc) = if s2lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(Acc, G1Point { x: hint.Q.x, y: R0y }, modulus)
} else {
(Acc,)
};
// Assert.
assert(Acc == R2, 'wrong FakeGLV result');
return hint.Q;
}
fn msm\_glv\_fake\_glv(
mut points: Span, mut scalars: Span, curve\_index: usize, ref hint: Span,
) -> G1Point {
let n = scalars.len();
assert!(n == points.len(), "scalars and points length mismatch");
if n == 0 {
panic\_with\_felt252('Msm size must be >= 1');
}
assert(hint.len() == n \* 12, 'wrong glv&fakeglv hint size');
let eigen = get\_eigenvalue(curve\_index);
let third\_root\_of\_unity = get\_third\_root\_of\_unity(curve\_index);
let modulus = get\_modulus(curve\_index);
let order\_modulus = get\_curve\_order\_modulus(curve\_index);
let curve\_order: u256 = get\_n(curve\_index);
let min\_one = get\_min\_one\_order(curve\_index);
let nG = get\_nG\_glv\_fake\_glv(curve\_index);
let mut acc = G1PointZero::zero();
while hint.len() != 0 {
let pt = \*points.pop\_front().unwrap();
let scalar = \*scalars.pop\_front().unwrap();
assert(scalar < curve\_order, 'unreduced scalar');
let scalar\_u384: u384 = scalar.into();
let glv\_fake\_glv\_hint: GlvFakeGlvHint = Serde::deserialize(ref hint).unwrap();
match pt.is\_infinity() {
true => { acc = acc; },
false => {
pt.assert\_on\_curve\_excluding\_infinity(curve\_index);
let temp = \_scalar\_mul\_glv\_and\_fake\_glv(
pt,
scalar\_u384,
order\_modulus,
modulus,
glv\_fake\_glv\_hint,
eigen,
third\_root\_of\_unity,
min\_one,
nG,
curve\_index,
);
acc = \_ec\_safe\_add(acc, temp, modulus, curve\_index);
},
}
}
return acc;
}
#[inline(always)]
// /!\ Assumes point is on curve (not infinity) and scalar is reduced mod n. /!\
pub fn \_scalar\_mul\_glv\_and\_fake\_glv(
point: G1Point,
scalar: u384,
order\_modulus: CircuitModulus,
modulus: CircuitModulus,
hint: GlvFakeGlvHint,
\_lambda: u384,
third\_root\_of\_unity: u384,
minus\_one: u384,
n\_bits\_G: G1Point,
curve\_index: usize,
) -> G1Point {
if scalar.is\_zero() {
return G1PointZero::zero();
}
if scalar.is\_one() {
return point;
}
hint.Q.assert\_on\_curve\_excluding\_infinity(curve\_index);
if scalar == minus\_one {
return G1Point { x: point.x, y: neg\_mod\_p(point.y, modulus) };
}
let one\_u384: u384 = u384 { limb0: 1, limb1: 0, limb2: 0, limb3: 0 };
// Retrieve the u1, u2, v1, v2 values from the hint
// They are encoded as 2^128 \* sign + abs(value)
// If high\_128 limbs are != 0, we consider the value is negative, otherwise it is positive.
// We also precompute the negated value of the y coordinate of the points based on the sign in
// the same match condition for efficiency.
let (\_u1\_sign, \_u1\_abs, P0y, P1y): (u384, u96, u384, u384) = match u128s\_from\_felt252(hint.u1) {
U128sFromFelt252Result::Narrow(low) => {
(one\_u384, downcast(low).unwrap(), neg\_mod\_p(point.y, modulus), point.y)
},
U128sFromFelt252Result::Wide((\_, low)) => {
(minus\_one, downcast(low).unwrap(), point.y, neg\_mod\_p(point.y, modulus))
},
};
let (\_u2\_sign, \_u2\_abs, Phi\_P0y, Phi\_P1y): (u384, u96, u384, u384) =
match u128s\_from\_felt252(hint.u2) {
U128sFromFelt252Result::Narrow(low) => {
(one\_u384, downcast(low).unwrap(), neg\_mod\_p(point.y, modulus), point.y)
},
U128sFromFelt252Result::Wide((\_, low)) => {
(minus\_one, downcast(low).unwrap(), point.y, neg\_mod\_p(point.y, modulus))
},
};
let (\_v1\_sign, \_v1\_abs, Q0y): (u384, u96, u384) = match u128s\_from\_felt252(hint.v1) {
U128sFromFelt252Result::Narrow(low) => {
(one\_u384, downcast(low).unwrap(), neg\_mod\_p(hint.Q.y, modulus))
},
U128sFromFelt252Result::Wide((\_, low)) => { (minus\_one, downcast(low).unwrap(), hint.Q.y) },
};
let (\_v2\_sign, \_v2\_abs, Phi\_Q0y): (u384, u96, u384) = match u128s\_from\_felt252(hint.v2) {
U128sFromFelt252Result::Narrow(low) => {
(one\_u384, downcast(low).unwrap(), neg\_mod\_p(hint.Q.y, modulus))
},
U128sFromFelt252Result::Wide((\_, low)) => { (minus\_one, downcast(low).unwrap(), hint.Q.y) },
};
// At this point ~ 631 steps.
// Check that the GLV/FakeGLV decomposition is valid:
// # s\*(v1 + λ\*v2) + u1 + λ\*u2 = 0
let s = CircuitElement::> {};
let lambda = CircuitElement::> {};
let u1\_abs = CircuitElement::> {};
let u1\_sign = CircuitElement::> {};
let u2\_abs = CircuitElement::> {};
let u2\_sign = CircuitElement::> {};
let v1\_abs = CircuitElement::> {};
let v1\_sign = CircuitElement::> {};
let v2\_abs = CircuitElement::> {};
let v2\_sign = CircuitElement::> {};
let u1 = circuit\_mul(u1\_abs, u1\_sign);
let u2 = circuit\_mul(u2\_abs, u2\_sign);
let v1 = circuit\_mul(v1\_abs, v1\_sign);
let v2 = circuit\_mul(v2\_abs, v2\_sign);
// u1 + λ\*u2
let lambda\_u2 = circuit\_mul(lambda, u2);
let u1\_lambda\_u2 = circuit\_add(u1, lambda\_u2);
// s\*(v1 + λ\*v2)
let lambda\_v2 = circuit\_mul(lambda, v2);
let v1\_lambda\_v2 = circuit\_add(v1, lambda\_v2);
let s\_v1\_lambda\_v2 = circuit\_mul(s, v1\_lambda\_v2);
// s\*(v1 + λ\*v2) + u1 + λ\*u2
let res = circuit\_add(s\_v1\_lambda\_v2, u1\_lambda\_u2);
let outputs = (res,)
.new\_inputs()
.next\_2(scalar)
.next\_2(\_lambda)
.next\_2([\_u1\_abs, 0, 0, 0])
.next\_2(\_u1\_sign)
.next\_2([\_u2\_abs, 0, 0, 0])
.next\_2(\_u2\_sign)
.next\_2([\_v1\_abs, 0, 0, 0])
.next\_2(\_v1\_sign)
.next\_2([\_v2\_abs, 0, 0, 0])
.next\_2(\_v2\_sign)
.done\_2()
.eval(order\_modulus)
.unwrap();
assert(outputs.get\_output(res).is\_zero(), 'Wrong GLV/FakeGLV decomposition');
// At this point, ~ 836 steps
// Note : P != Q (scalar != 1)
let P0 = G1Point { x: point.x, y: P0y };
let Q0 = G1Point { x: hint.Q.x, y: Q0y };
let (
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
mut Acc,
) =
ec::run\_PREPARE\_GLV\_FAKE\_GLV\_PTS\_circuit(
point.x,
P0y,
P1y,
hint.Q.x,
Q0y,
Phi\_P0y,
Phi\_P1y,
Phi\_Q0y,
get\_G(curve\_index),
third\_root\_of\_unity,
modulus,
);
let Bs: Span = array![
G1Point { x: B1.x, y: B16y }, B8, G1Point { x: B3.x, y: B14y }, B6,
G1Point { x: B5.x, y: B12y }, B4, G1Point { x: B7.x, y: B10y }, B2,
G1Point { x: B2.x, y: B15y }, B7, G1Point { x: B4.x, y: B13y }, B5,
G1Point { x: B6.x, y: B11y }, B3, G1Point { x: B8.x, y: B9y }, B1,
]
.span();
// At this point, ~ 1643 steps.
// println!("Bs: {:?}", Bs);
// println!("Acc: {:?}", Acc);
let mut \_u1: u128 = upcast(\_u1\_abs);
let mut \_u2: u128 = upcast(\_u2\_abs);
let mut \_v1: u128 = upcast(\_v1\_abs);
let mut \_v2: u128 = upcast(\_v2\_abs);
let (mut selectors, u1lsb, u2lsb, v1lsb, v2lsb) = selectors::build\_selectors\_inlined(
\_u1, \_u2, \_v1, \_v2,
);
// Process 8 bits per iteration
while let Some(selector\_y) = selectors.multi\_pop\_back::<8>() {
let [
selector\_y7,
selector\_y6,
selector\_y5,
selector\_y4,
selector\_y3,
selector\_y2,
selector\_y1,
selector\_y0,
] =
(\*selector\_y)
.unbox();
let Bi = \*Bs.at(selector\_y0);
let Bi1 = \*Bs.at(selector\_y1);
let Bi2 = \*Bs.at(selector\_y2);
let Bi3 = \*Bs.at(selector\_y3);
let Bi4 = \*Bs.at(selector\_y4);
let Bi5 = \*Bs.at(selector\_y5);
let Bi6 = \*Bs.at(selector\_y6);
let Bi7 = \*Bs.at(selector\_y7);
Acc = double\_and\_add\_8(Acc, Bi, Bi1, Bi2, Bi3, Bi4, Bi5, Bi6, Bi7, modulus);
}
// println!("Acc final: {:?}", Acc);
let (Acc) = if u1lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(Acc, P0, modulus)
} else {
(Acc,)
};
let (Acc) = if u2lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(Acc, G1Point { x: Phi\_P0x, y: Phi\_P0y }, modulus)
} else {
(Acc,)
};
let (Acc) = if v1lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(Acc, Q0, modulus)
} else {
(Acc,)
};
let (Acc) = if v2lsb == 0 {
ec::run\_ADD\_EC\_POINT\_circuit(Acc, G1Point { x: Phi\_Q0x, y: Phi\_Q0y }, modulus)
} else {
(Acc,)
};
assert(Acc == n\_bits\_G, 'Wrong Glv&FakeGLV result');
return hint.Q;
}
fn scalar\_mul\_by\_bls12\_381\_seed\_square(q: G1Point) -> G1Point {
let a = get\_a(1);
let modulus = get\_modulus(1);
let (z) = ec::run\_DOUBLE\_EC\_POINT\_circuit(q, a, modulus); // z := g1.double(q)
let (z) = ec::run\_ADD\_EC\_POINT\_circuit(q, z, modulus); // z = g1.add(q, z)
let (z) = ec::run\_DOUBLE\_EC\_POINT\_circuit(z, a, modulus); // z = g1.double(z)
let z = double\_and\_add(z, q, modulus); // z = g1.doubleAndAdd(z, q)
let z = double\_n\_times(z, 2, modulus, a); // z = g1.doubleN(z, 2)
let z = double\_and\_add(z, q, modulus); // z = g1.doubleAndAdd(z, q)
let z = double\_n\_times(z, 8, modulus, a); // z = g1.doubleN(z, 8)
let z = double\_and\_add(z, q, modulus); // z = g1.doubleAndAdd(z, q)
let (t0) = ec::run\_DOUBLE\_EC\_POINT\_circuit(z, a, modulus); // t0 := g1.double(z)
let (t0) = ec::run\_ADD\_EC\_POINT\_circuit(z, t0, modulus); // t0 = g1.add(z, t0)
let (t0) = ec::run\_DOUBLE\_EC\_POINT\_circuit(t0, a, modulus); // t0 = g1.double(t0)
let t0 = double\_and\_add(t0, z, modulus); // t0 = g1.doubleAndAdd(t0, z)
let t0 = double\_n\_times(t0, 2, modulus, a); // t0 = g1.doubleN(t0, 2)
let t0 = double\_and\_add(t0, z, modulus); // t0 = g1.doubleAndAdd(t0, z)
let t0 = double\_n\_times(t0, 8, modulus, a); // t0 = g1.doubleN(t0, 8)
let t0 = double\_and\_add(t0, z, modulus); // t0 = g1.doubleAndAdd(t0, z)
let t0 = double\_n\_times(t0, 31, modulus, a); // t0 = g1.doubleN(t0, 31)
let (z) = ec::run\_ADD\_EC\_POINT\_circuit(t0, z, modulus); // z = g1.add(t0, z)
let z = double\_n\_times(z, 32, modulus, a); // z = g1.doubleN(z, 32)
let z = double\_and\_add(z, q, modulus); // z = g1.doubleAndAdd(z, q)
let z = double\_n\_times(z, 32, modulus, a); // z = g1.doubleN(z, 32)
return z;
}
pub fn double\_n\_times(p: G1Point, n: usize, modulus: CircuitModulus, a: u384) -> G1Point {
let mut res = p;
for \_ in 0..n {
let (tmp) = ec::run\_DOUBLE\_EC\_POINT\_circuit(res, a, modulus);
res = tmp;
}
return res;
}
// Computes 2\*P + Q
pub fn double\_and\_add(p: G1Point, q: G1Point, modulus: CircuitModulus) -> G1Point {
let px = CircuitElement::> {};
let py = CircuitElement::> {};
let qx = CircuitElement::> {};
let qy = CircuitElement::> {};
// Compute lambda1 for P + Q
let num\_lambda1 = circuit\_sub(qy, py);
let den\_lambda1 = circuit\_sub(qx, px);
let lambda1 = circuit\_mul(num\_lambda1, circuit\_inverse(den\_lambda1));
// Compute x-coordinate of P + Q
let lambda1\_sq = circuit\_mul(lambda1, lambda1);
let x2 = circuit\_sub(lambda1\_sq, px);
let x2 = circuit\_sub(x2, qx);
// Compute lambda2 for 2\*P
let den\_lambda2 = circuit\_sub(x2, px);
let num\_lambda2 = circuit\_add(py, py);
let term2\_lambda2 = circuit\_mul(num\_lambda2, circuit\_inverse(den\_lambda2));
let lambda2 = circuit\_add(lambda1, term2\_lambda2);
// Compute final coordinates of 2\*P + Q
let lambda2\_sq = circuit\_mul(lambda2, lambda2);
let tx\_temp = circuit\_sub(lambda2\_sq, px);
let result\_x = circuit\_sub(tx\_temp, x2);
let tx\_sub\_px = circuit\_sub(result\_x, px);
let term1\_ty = circuit\_mul(lambda2, tx\_sub\_px);
let result\_y = circuit\_sub(term1\_ty, py);
let outputs = (result\_x, result\_y)
.new\_inputs()
.next\_2(p.x)
.next\_2(p.y)
.next\_2(q.x)
.next\_2(q.y)
.done\_2()
.eval(modulus)
.expect('double\_and\_add failed');
return G1Point { x: outputs.get\_output(result\_x), y: outputs.get\_output(result\_y) };
}
// Computes 2\*(2\*(2\*(2\*(2\*(2\*(2\*(2\*P + Q0) + Q1) + Q2) + Q3) + Q4) + Q5) + Q6) + Q7
#[inline(always)]
pub fn double\_and\_add\_8(
p: G1Point,
q0: G1Point,
q1: G1Point,
q2: G1Point,
q3: G1Point,
q4: G1Point,
q5: G1Point,
q6: G1Point,
q7: G1Point,
modulus: CircuitModulus,
) -> G1Point {
let px = CircuitElement::> {};
let py = CircuitElement::> {};
let q0x = CircuitElement::> {};
let q0y = CircuitElement::> {};
let q1x = CircuitElement::> {};
let q1y = CircuitElement::> {};
let q2x = CircuitElement::> {};
let q2y = CircuitElement::> {};
let q3x = CircuitElement::> {};
let q3y = CircuitElement::> {};
let q4x = CircuitElement::> {};
let q4y = CircuitElement::> {};
let q5x = CircuitElement::> {};
let q5y = CircuitElement::> {};
let q6x = CircuitElement::> {};
let q6y = CircuitElement::> {};
let q7x = CircuitElement::> {};
let q7y = CircuitElement::> {};
// --- Step 1: (2 \* CurrentP) + Q0 ---
let num\_lambda1\_0 = circuit\_sub(q0y, py);
let den\_lambda1\_0 = circuit\_sub(q0x, px);
let lambda1\_0 = circuit\_mul(num\_lambda1\_0, circuit\_inverse(den\_lambda1\_0));
let lambda1\_sq\_0 = circuit\_mul(lambda1\_0, lambda1\_0);
let x2\_0 = circuit\_sub(lambda1\_sq\_0, px);
let x2\_0 = circuit\_sub(x2\_0, q0x);
let den\_lambda2\_0 = circuit\_sub(x2\_0, px);
let num\_lambda2\_0 = circuit\_add(py, py);
let term2\_lambda2\_0 = circuit\_mul(num\_lambda2\_0, circuit\_inverse(den\_lambda2\_0));
let lambda2\_0 = circuit\_add(lambda1\_0, term2\_lambda2\_0);
let lambda2\_sq\_0 = circuit\_mul(lambda2\_0, lambda2\_0);
let tx\_temp\_0 = circuit\_sub(lambda2\_sq\_0, px);
let T1x = circuit\_sub(tx\_temp\_0, x2\_0);
let tx\_sub\_px\_0 = circuit\_sub(T1x, px);
let term1\_ty\_0 = circuit\_mul(lambda2\_0, tx\_sub\_px\_0);
let T1y = circuit\_sub(term1\_ty\_0, py);
// --- Step 2: (2 \* CurrentP) + Q1 ---
let num\_lambda1\_1 = circuit\_sub(q1y, T1y);
let den\_lambda1\_1 = circuit\_sub(q1x, T1x);
let lambda1\_1 = circuit\_mul(num\_lambda1\_1, circuit\_inverse(den\_lambda1\_1));
let lambda1\_sq\_1 = circuit\_mul(lambda1\_1, lambda1\_1);
let x2\_1 = circuit\_sub(lambda1\_sq\_1, T1x);
let x2\_1 = circuit\_sub(x2\_1, q1x);
let den\_lambda2\_1 = circuit\_sub(x2\_1, T1x);
let num\_lambda2\_1 = circuit\_add(T1y, T1y);
let term2\_lambda2\_1 = circuit\_mul(num\_lambda2\_1, circuit\_inverse(den\_lambda2\_1));
let lambda2\_1 = circuit\_add(lambda1\_1, term2\_lambda2\_1);
let lambda2\_sq\_1 = circuit\_mul(lambda2\_1, lambda2\_1);
let tx\_temp\_1 = circuit\_sub(lambda2\_sq\_1, T1x);
let T2x = circuit\_sub(tx\_temp\_1, x2\_1);
let tx\_sub\_px\_1 = circuit\_sub(T2x, T1x);
let term1\_ty\_1 = circuit\_mul(lambda2\_1, tx\_sub\_px\_1);
let T2y = circuit\_sub(term1\_ty\_1, T1y);
// --- Step 3: (2 \* CurrentP) + Q2 ---
let num\_lambda1\_2 = circuit\_sub(q2y, T2y);
let den\_lambda1\_2 = circuit\_sub(q2x, T2x);
let lambda1\_2 = circuit\_mul(num\_lambda1\_2, circuit\_inverse(den\_lambda1\_2));
let lambda1\_sq\_2 = circuit\_mul(lambda1\_2, lambda1\_2);
let x2\_2 = circuit\_sub(lambda1\_sq\_2, T2x);
let x2\_2 = circuit\_sub(x2\_2, q2x);
let den\_lambda2\_2 = circuit\_sub(x2\_2, T2x);
let num\_lambda2\_2 = circuit\_add(T2y, T2y);
let term2\_lambda2\_2 = circuit\_mul(num\_lambda2\_2, circuit\_inverse(den\_lambda2\_2));
let lambda2\_2 = circuit\_add(lambda1\_2, term2\_lambda2\_2);
let lambda2\_sq\_2 = circuit\_mul(lambda2\_2, lambda2\_2);
let tx\_temp\_2 = circuit\_sub(lambda2\_sq\_2, T2x);
let T3x = circuit\_sub(tx\_temp\_2, x2\_2);
let tx\_sub\_px\_2 = circuit\_sub(T3x, T2x);
let term1\_ty\_2 = circuit\_mul(lambda2\_2, tx\_sub\_px\_2);
let T3y = circuit\_sub(term1\_ty\_2, T2y);
// --- Step 4: (2 \* CurrentP) + Q3 ---
let num\_lambda1\_3 = circuit\_sub(q3y, T3y);
let den\_lambda1\_3 = circuit\_sub(q3x, T3x);
let lambda1\_3 = circuit\_mul(num\_lambda1\_3, circuit\_inverse(den\_lambda1\_3));
let lambda1\_sq\_3 = circuit\_mul(lambda1\_3, lambda1\_3);
let x2\_3 = circuit\_sub(lambda1\_sq\_3, T3x);
let x2\_3 = circuit\_sub(x2\_3, q3x);
let den\_lambda2\_3 = circuit\_sub(x2\_3, T3x);
let num\_lambda2\_3 = circuit\_add(T3y, T3y);
let term2\_lambda2\_3 = circuit\_mul(num\_lambda2\_3, circuit\_inverse(den\_lambda2\_3));
let lambda2\_3 = circuit\_add(lambda1\_3, term2\_lambda2\_3);
let lambda2\_sq\_3 = circuit\_mul(lambda2\_3, lambda2\_3);
let tx\_temp\_3 = circuit\_sub(lambda2\_sq\_3, T3x);
let T4x = circuit\_sub(tx\_temp\_3, x2\_3);
let tx\_sub\_px\_3 = circuit\_sub(T4x, T3x);
let term1\_ty\_3 = circuit\_mul(lambda2\_3, tx\_sub\_px\_3);
let T4y = circuit\_sub(term1\_ty\_3, T3y);
// --- Step 5: (2 \* CurrentP) + Q4 ---
let num\_lambda1\_4 = circuit\_sub(q4y, T4y);
let den\_lambda1\_4 = circuit\_sub(q4x, T4x);
let lambda1\_4 = circuit\_mul(num\_lambda1\_4, circuit\_inverse(den\_lambda1\_4));
let lambda1\_sq\_4 = circuit\_mul(lambda1\_4, lambda1\_4);
let x2\_4 = circuit\_sub(lambda1\_sq\_4, T4x);
let x2\_4 = circuit\_sub(x2\_4, q4x);
let den\_lambda2\_4 = circuit\_sub(x2\_4, T4x);
let num\_lambda2\_4 = circuit\_add(T4y, T4y);
let term2\_lambda2\_4 = circuit\_mul(num\_lambda2\_4, circuit\_inverse(den\_lambda2\_4));
let lambda2\_4 = circuit\_add(lambda1\_4, term2\_lambda2\_4);
let lambda2\_sq\_4 = circuit\_mul(lambda2\_4, lambda2\_4);
let tx\_temp\_4 = circuit\_sub(lambda2\_sq\_4, T4x);
let T5x = circuit\_sub(tx\_temp\_4, x2\_4);
let tx\_sub\_px\_4 = circuit\_sub(T5x, T4x);
let term1\_ty\_4 = circuit\_mul(lambda2\_4, tx\_sub\_px\_4);
let T5y = circuit\_sub(term1\_ty\_4, T4y);
// --- Step 6: (2 \* CurrentP) + Q5 ---
let num\_lambda1\_5 = circuit\_sub(q5y, T5y);
let den\_lambda1\_5 = circuit\_sub(q5x, T5x);
let lambda1\_5 = circuit\_mul(num\_lambda1\_5, circuit\_inverse(den\_lambda1\_5));
let lambda1\_sq\_5 = circuit\_mul(lambda1\_5, lambda1\_5);
let x2\_5 = circuit\_sub(lambda1\_sq\_5, T5x);
let x2\_5 = circuit\_sub(x2\_5, q5x);
let den\_lambda2\_5 = circuit\_sub(x2\_5, T5x);
let num\_lambda2\_5 = circuit\_add(T5y, T5y);
let term2\_lambda2\_5 = circuit\_mul(num\_lambda2\_5, circuit\_inverse(den\_lambda2\_5));
let lambda2\_5 = circuit\_add(lambda1\_5, term2\_lambda2\_5);
let lambda2\_sq\_5 = circuit\_mul(lambda2\_5, lambda2\_5);
let tx\_temp\_5 = circuit\_sub(lambda2\_sq\_5, T5x);
let T6x = circuit\_sub(tx\_temp\_5, x2\_5);
let tx\_sub\_px\_5 = circuit\_sub(T6x, T5x);
let term1\_ty\_5 = circuit\_mul(lambda2\_5, tx\_sub\_px\_5);
let T6y = circuit\_sub(term1\_ty\_5, T5y);
// --- Step 7: (2 \* CurrentP) + Q6 ---
let num\_lambda1\_6 = circuit\_sub(q6y, T6y);
let den\_lambda1\_6 = circuit\_sub(q6x, T6x);
let lambda1\_6 = circuit\_mul(num\_lambda1\_6, circuit\_inverse(den\_lambda1\_6));
let lambda1\_sq\_6 = circuit\_mul(lambda1\_6, lambda1\_6);
let x2\_6 = circuit\_sub(lambda1\_sq\_6, T6x);
let x2\_6 = circuit\_sub(x2\_6, q6x);
let den\_lambda2\_6 = circuit\_sub(x2\_6, T6x);
let num\_lambda2\_6 = circuit\_add(T6y, T6y);
let term2\_lambda2\_6 = circuit\_mul(num\_lambda2\_6, circuit\_inverse(den\_lambda2\_6));
let lambda2\_6 = circuit\_add(lambda1\_6, term2\_lambda2\_6);
let lambda2\_sq\_6 = circuit\_mul(lambda2\_6, lambda2\_6);
let tx\_temp\_6 = circuit\_sub(lambda2\_sq\_6, T6x);
let T7x = circuit\_sub(tx\_temp\_6, x2\_6);
let tx\_sub\_px\_6 = circuit\_sub(T7x, T6x);
let term1\_ty\_6 = circuit\_mul(lambda2\_6, tx\_sub\_px\_6);
let T7y = circuit\_sub(term1\_ty\_6, T6y);
// --- Step 8: (2 \* CurrentP) + Q7 ---
let num\_lambda1\_7 = circuit\_sub(q7y, T7y);
let den\_lambda1\_7 = circuit\_sub(q7x, T7x);
let lambda1\_7 = circuit\_mul(num\_lambda1\_7, circuit\_inverse(den\_lambda1\_7));
let lambda1\_sq\_7 = circuit\_mul(lambda1\_7, lambda1\_7);
let x2\_7 = circuit\_sub(lambda1\_sq\_7, T7x);
let x2\_7 = circuit\_sub(x2\_7, q7x);
let den\_lambda2\_7 = circuit\_sub(x2\_7, T7x);
let num\_lambda2\_7 = circuit\_add(T7y, T7y);
let term2\_lambda2\_7 = circuit\_mul(num\_lambda2\_7, circuit\_inverse(den\_lambda2\_7));
let lambda2\_7 = circuit\_add(lambda1\_7, term2\_lambda2\_7);
let lambda2\_sq\_7 = circuit\_mul(lambda2\_7, lambda2\_7);
let tx\_temp\_7 = circuit\_sub(lambda2\_sq\_7, T7x);
let T8x = circuit\_sub(tx\_temp\_7, x2\_7);
let tx\_sub\_px\_7 = circuit\_sub(T8x, T7x);
let term1\_ty\_7 = circuit\_mul(lambda2\_7, tx\_sub\_px\_7);
let T8y = circuit\_sub(term1\_ty\_7, T7y);
let outputs = (T8x, T8y)
.new\_inputs()
.next\_2(p.x)
.next\_2(p.y)
.next\_2(q0.x)
.next\_2(q0.y)
.next\_2(q1.x)
.next\_2(q1.y)
.next\_2(q2.x)
.next\_2(q2.y)
.next\_2(q3.x)
.next\_2(q3.y)
.next\_2(q4.x)
.next\_2(q4.y)
.next\_2(q5.x)
.next\_2(q5.y)
.next\_2(q6.x)
.next\_2(q6.y)
.next\_2(q7.x)
.next\_2(q7.y)
.done\_2()
.eval(modulus)
.expect('double\_and\_add\_8 failed');
return G1Point { x: outputs.get\_output(T8x), y: outputs.get\_output(T8y) };
}
from dataclasses import dataclass
from functools import lru\_cache
from garaga import garaga\_rs
from garaga import modulo\_circuit\_structs as structs
from garaga.algebra import PyFelt
from garaga.curves import CURVES, STARK, CurveID, WeierstrassCurve, get\_base\_field
from garaga.hints import fake\_glv
from garaga.points import G1Point
@dataclass(slots=True)
class MSMCalldataBuilder:
curve\_id: CurveID
points: list[G1Point]
scalars: list[int]
def \_\_post\_init\_\_(self):
assert all(
point.curve\_id == self.curve\_id for point in self.points
), "All points must be on the same curve."
assert len(self.points) == len(
self.scalars
), f"Number of points and scalars must be equal, got {len(self.points)} points and {len(self.scalars)} scalars"
assert all(
0 <= s <= CURVES[self.curve\_id.value].n for s in self.scalars
), f"Scalars must be in [0, {self.curve\_id.name}'s order] == [0, {CURVES[self.curve\_id.value].n}]."
def \_\_hash\_\_(self) -> int:
return hash((self.curve\_id, tuple(self.points), tuple(self.scalars)))
@property
def field(self):
return get\_base\_field(self.curve\_id)
@property
def msm\_size(self):
return len(self.scalars)
@property
def curve(self) -> WeierstrassCurve:
return CURVES[self.curve\_id.value]
@lru\_cache(maxsize=2)
def build\_msm\_hint(self) -> structs.StructArray:
"""
Returns the MSMHint
"""
hints\_structs = []
for i, (point, scalar) in enumerate(zip(self.points, self.scalars)):
if self.curve.is\_endomorphism\_available():
Q, u1, u2, v1, v2 = fake\_glv.get\_glv\_fake\_glv\_hint(point, scalar)
hints\_structs.append(
structs.Struct(
struct\_name=f"GlvFakeGlvHint",
name=f"glv\_fake\_glv\_hint\_{i}",
elmts=[
structs.G1PointCircuit.from\_G1Point("Q", Q),
structs.felt252(name=f"u1\_{i}", elmts=[PyFelt(u1, STARK)]),
structs.felt252(name=f"u2\_{i}", elmts=[PyFelt(u2, STARK)]),
structs.felt252(name=f"v1\_{i}", elmts=[PyFelt(v1, STARK)]),
structs.felt252(name=f"v2\_{i}", elmts=[PyFelt(v2, STARK)]),
],
)
)
else:
Q, s1, s2 = fake\_glv.get\_fake\_glv\_hint(point, scalar)
hints\_structs.append(
structs.Struct(
struct\_name=f"FakeGlvHint",
name=f"fake\_glv\_hint\_{i}",
elmts=[
structs.G1PointCircuit.from\_G1Point("Q", Q),
structs.felt252(name=f"s1\_{i}", elmts=[PyFelt(s1, STARK)]),
structs.felt252(name=f"s2\_{i}", elmts=[PyFelt(s2, STARK)]),
],
)
)
return structs.StructSpan(name="msm\_hint", elmts=hints\_structs)
def \_get\_input\_structs(
self,
) -> list[structs.Cairo1SerializableStruct]:
""" """
inputs = []
inputs.append(self.build\_msm\_hint()) # msm\_hint
inputs.append(
structs.StructSpan(
name="points",
elmts=[
structs.G1PointCircuit.from\_G1Point(f"point{i}", point)
for i, point in enumerate(self.points)
],
)
)
inputs.append(
structs.StructSpan(
name="scalars",
elmts=[
structs.u256(name=f"scalar{i}", elmts=[PyFelt(s, 2\*\*256)])
for i, s in enumerate(self.scalars)
],
)
)
return inputs
def to\_cairo\_1\_test(self, test\_name: str = None, ignored=False):
# print(
# f"Generating MSM test for {self.curve\_id.name} with {len(self.scalars)} points"
# )
test\_name = test\_name or f"test\_msm\_{self.curve\_id.name}\_{len(self.scalars)}P"
inputs = garaga\_rs.msm\_calldata\_builder(
[value for point in self.points for value in [point.x, point.y]],
self.scalars,
self.curve\_id.value,
True,
False,
)
Q = structs.G1PointCircuit.from\_G1Point(
"Q", G1Point.msm(self.points, self.scalars)
)
T = "u384" if self.curve\_id == CurveID.BLS12\_381 else "u288"
ignore\_str = "\n#[ignore]" if ignored else ""
code = f"""
#[test]{ignore\_str}
fn {test\_name}() {{
let mut data = array![{','.join([hex(value) for value in inputs])}].span();
let points = Serde::deserialize(ref data).unwrap();
let scalars = Serde::deserialize(ref data).unwrap();
let curve\_id = Serde::deserialize(ref data).unwrap();
let res = msm\_g1(points, scalars, curve\_id, data);
assert!(res == {Q.serialize(raw=True)});
}}
"""
return code
def \_serialize\_to\_calldata\_rust(
self,
include\_points\_and\_scalars=True,
serialize\_as\_pure\_felt252\_array=False,
) -> list[int]:
return garaga\_rs.msm\_calldata\_builder(
[value for point in self.points for value in [point.x, point.y]],
self.scalars,
self.curve\_id.value,
include\_points\_and\_scalars,
serialize\_as\_pure\_felt252\_array,
)
def serialize\_to\_calldata(
self,
include\_points\_and\_scalars=True,
serialize\_as\_pure\_felt252\_array=False,
use\_rust=False,
) -> list[int]:
if use\_rust:
return self.\_serialize\_to\_calldata\_rust(
include\_points\_and\_scalars,
serialize\_as\_pure\_felt252\_array,
)
msm\_hint, points, scalars = self.\_get\_input\_structs()
call\_data: list[int] = []
if include\_points\_and\_scalars:
call\_data.extend(points.serialize\_to\_calldata())
call\_data.extend(scalars.serialize\_to\_calldata())
call\_data.append(self.curve\_id.value)
call\_data.extend(msm\_hint.serialize\_to\_calldata()[1:])
if serialize\_as\_pure\_felt252\_array:
return [len(call\_data)] + call\_data
else:
return call\_data
if \_\_name\_\_ == "\_\_main\_\_":
import random
c = CurveID.SECP256K1
order = CURVES[c.value].n
msm = MSMCalldataBuilder(
curve\_id=c,
points=[G1Point.gen\_random\_point(c) for \_ in range(1)],
scalars=[random.randint(0, order) for \_ in range(1)],
)
cd = msm.serialize\_to\_calldata(
include\_points\_and\_scalars=True,
serialize\_as\_pure\_felt252\_array=False,
)
print(cd)
print(len(cd))
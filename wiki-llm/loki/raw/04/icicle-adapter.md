//! icicle
use ff::Field;
use group::ff::PrimeField;
use halo2curves::bn256::Fr as Bn256Fr;
use crate::{arithmetic::FftGroup, plonk::{Calculation, CalculationInfo, ValueSource}, poly::{ExtendedLagrangeCoeff, Polynomial}};
use std::{any::{Any, TypeId}, ptr};
pub use halo2curves::CurveAffine;
use icicle\_runtime::{get\_available\_memory, memory::{DeviceSlice, DeviceVec, HostOrDeviceSlice, HostSlice}, set\_default\_device, set\_device, stream::IcicleStream};
use icicle\_bn254::curve::{CurveCfg, G1Projective, ScalarField};
use icicle\_core::{
curve::Affine, msm, ntt::{ntt\_inplace, NTTConfig, NTTDir}, traits::MontgomeryConvertible, vec\_ops::{add\_scalars, inv\_scalars, mul\_scalars, scalar\_add, scalar\_mul, scalar\_sub, sub\_scalars, VecOpsConfig}
};
use maybe\_rayon::iter::IntoParallelRefIterator;
use maybe\_rayon::iter::ParallelIterator;
use std::{env, mem};
/// load cpu or cuda backend
pub fn try\_load\_and\_set\_backend\_device(device\_type: &str) {
if device\_type != "CPU" {
icicle\_runtime::runtime::load\_backend\_from\_env\_or\_default().unwrap();
}
println!("Setting device {}", device\_type);
let device = icicle\_runtime::Device::new(device\_type, 0 /\* =device\_id\*/);
set\_device(&device).unwrap();
set\_default\_device(&device).unwrap();
}
/// should use cpu for fft
pub fn should\_use\_cpu\_fft(size: usize) -> bool {
size <= (1
<< u8::from\_str\_radix(&env::var("ICICLE\_SMALL\_K\_FFT").unwrap\_or("2".to\_string()), 10).unwrap())
}
/// is icicle support the field
pub fn is\_gpu\_supported\_field(\_sample\_element: &G) -> bool {
match TypeId::of::() {
id if id == TypeId::of::() => true,
\_ => false,
}
}
fn repr\_from\_u32(u32\_arr: &[u32; 8]) -> ::Base {
let t: &[<::Base as PrimeField>::Repr] =
unsafe { mem::transmute(&u32\_arr[..]) };
PrimeField::from\_repr(t[0]).unwrap()
}
/// process value source
pub fn process\_value\_source(
value: &ValueSource,
value\_types: &mut Vec,
value\_indices: &mut Vec,
) {
let (code, idx0, idx1) = match value {
ValueSource::Constant(v) => (0, \*v, 0),
ValueSource::Intermediate(v) => (1, \*v, 0),
ValueSource::Fixed(a, b) => (2, \*a, \*b),
ValueSource::Advice(a, b) => (3, \*a, \*b),
ValueSource::Instance(a, b) => (4, \*a, \*b),
ValueSource::Challenge(v) => (5, \*v, 0),
ValueSource::Beta() => (6, 0, 0),
ValueSource::Gamma() => (7, 0, 0),
ValueSource::Theta() => (8, 0, 0),
ValueSource::Y() => (9, 0, 0),
ValueSource::PreviousValue() => (10, 0, 0),
};
value\_types.push(code);
value\_indices.push(idx0 as u32);
value\_indices.push(idx1 as u32);
}
/// create calculation data
pub fn create\_calculation\_data(
calculations: &[CalculationInfo],
constants: &[C::ScalarExt],
rotations: &[i32],
num\_intermediates: usize,
) -> (Vec, Vec, Vec, Vec, Vec, Vec, u32, u32, Vec, Vec, Vec, Vec)
{
let size = calculations.len();
let mut icicle\_calculations = Vec::with\_capacity(size);
let mut targets = Vec::with\_capacity(size);
let mut value\_types = Vec::with\_capacity(size \* 2);
let mut value\_indices = Vec::with\_capacity(size \* 4);
let mut horner\_value\_types: Vec = Vec::new();
let mut horner\_value\_indices: Vec = Vec::new();
let mut horner\_offsets: Vec = vec![0; size];
let mut horner\_sizes: Vec = vec![0; size];
for (i, info) in calculations.iter().enumerate() {
let calc\_code = match info.calculation {
Calculation::Add(\_, \_) => 0,
Calculation::Sub(\_, \_) => 1,
Calculation::Mul(\_, \_) => 2,
Calculation::Square(\_) => 3,
Calculation::Double(\_) => 4,
Calculation::Negate(\_) => 5,
Calculation::Horner(\_, \_, \_) => 6,
Calculation::Store(\_) => 7,
};
icicle\_calculations.push(calc\_code);
targets.push(info.target as u32);
let (first, second) = match &info.calculation {
Calculation::Add(a, b)
| Calculation::Sub(a, b)
| Calculation::Mul(a, b) => (a, b),
Calculation::Square(a)
| Calculation::Double(a)
| Calculation::Negate(a)
| Calculation::Store(a) => (a, a),
Calculation::Horner(a, b\_values, c) => {
horner\_offsets[i] = horner\_value\_types.len() as u32;
horner\_sizes[i] = b\_values.len() as u32;
for b in b\_values {
process\_value\_source(b, &mut horner\_value\_types, &mut horner\_value\_indices);
}
(a, c)
},
};
process\_value\_source(first, &mut value\_types, &mut value\_indices);
process\_value\_source(second, &mut value\_types, &mut value\_indices);
}
let icicle\_rotations: Vec = rotations.iter().map(|&x| x).collect();
let icicle\_constants = icicle\_scalars\_from\_c\_scalars::(constants);
(icicle\_calculations, targets, value\_types, value\_indices, icicle\_constants, icicle\_rotations, size as u32, num\_intermediates as u32, horner\_value\_types, horner\_value\_indices, horner\_offsets, horner\_sizes)
}
/// unsafe flatten
pub fn unsafe\_flatten(poly\_vec: &[Polynomial]) -> Vec {
let total\_size: usize = poly\_vec.iter().map(|poly| poly.num\_coeffs()).sum();
let mut result: Vec = Vec::with\_capacity(total\_size);
unsafe {
let result\_ptr = result.as\_mut\_ptr();
let mut offset = 0;
for poly in poly\_vec {
let poly\_len = poly.num\_coeffs();
let poly\_ptr = poly.as\_ptr();
// Perform direct memory copy
ptr::copy\_nonoverlapping(poly\_ptr, result\_ptr.add(offset), poly\_len);
offset += poly\_len;
}
// Manually set the length of result
result.set\_len(total\_size);
}
result
}
/// create gate data
pub fn create\_gate\_data(
advice: &[Polynomial],
instance: &[Polynomial],
challenges: &[C::ScalarExt],
beta: C::ScalarExt,
gamma: C::ScalarExt,
theta: C::ScalarExt,
y: C::ScalarExt,
) -> (DeviceVec, DeviceVec, Vec, Vec, Vec, Vec, Vec)
{
let icicle\_instance = device\_vec\_from\_poly\_vec::(instance, &IcicleStream::default());
let icicle\_advice = device\_vec\_from\_poly\_vec::(advice, &IcicleStream::default());
let icicle\_challenges = icicle\_scalars\_from\_c\_scalars::(challenges);
let icicle\_beta = icicle\_scalars\_from\_c\_scalars::(&[beta]);
let icicle\_gamma = icicle\_scalars\_from\_c\_scalars::(&[gamma]);
let icicle\_theta = icicle\_scalars\_from\_c\_scalars::(&[theta]);
let icicle\_y = icicle\_scalars\_from\_c\_scalars::(&[y]);
(icicle\_advice, icicle\_instance, icicle\_challenges, icicle\_beta, icicle\_gamma, icicle\_theta, icicle\_y)
}
/// device vec from poly vec
pub fn device\_vec\_from\_poly\_vec(
poly\_vec: &[Polynomial],
stream: &IcicleStream,
) -> DeviceVec {
if poly\_vec.len() == 0 {
return DeviceVec::::device\_malloc\_async(0, stream).unwrap();
}
let total\_len: usize = poly\_vec.iter().map(|poly| poly.num\_coeffs()).sum();
let mut device\_vec = DeviceVec::::device\_malloc\_async(total\_len, stream).unwrap();
let mut offset = 0;
for poly in poly\_vec {
let values: &[C::Scalar] = poly.as\_ref();
let len = values.len();
let icicle\_values: &[ScalarField] = unsafe {
std::slice::from\_raw\_parts(values.as\_ptr() as \*const ScalarField, len)
};
let h\_icicle\_values = HostSlice::from\_slice(icicle\_values);
device\_vec[offset..offset + len].copy\_from\_host\_async(h\_icicle\_values, stream).unwrap();
offset += len;
}
ScalarField::from\_mont(&mut device\_vec, stream);
device\_vec
}
/// device vec from c scalars
pub fn device\_vec\_from\_c\_scalars(coeffs: &[G], stream: &IcicleStream) -> DeviceVec {
let icicle\_scalars = unsafe { &\*(coeffs as \*const \_ as \*const [ScalarField]) };
// Create a HostSlice from the mutable slice
let icicle\_host\_slice = HostSlice::from\_slice(icicle\_scalars);
let mut icicle\_scalars = DeviceVec::::device\_malloc\_async(coeffs.len(), stream).unwrap();
icicle\_scalars
.copy\_from\_host\_async(icicle\_host\_slice, stream)
.unwrap();
// Convert from Montgomery representation using the Icicle type's conversion method
ScalarField::from\_mont(&mut icicle\_scalars, stream);
icicle\_scalars
}
/// c scalars from device vec
pub fn c\_scalars\_from\_device\_vec(device\_vec: &mut DeviceVec, stream: &IcicleStream) -> Vec {
let len = device\_vec.len();
let mut host\_vec = Vec::with\_capacity(len);
unsafe { host\_vec.set\_len(len); }
let host\_slice = HostSlice::from\_mut\_slice(&mut host\_vec[..]);
// Convert from Icicle's representation back to Montgomery representation
ScalarField::to\_mont(&mut device\_vec[..], stream);
device\_vec.copy\_to\_host\_async(host\_slice, stream).unwrap();
stream.synchronize().unwrap();
unsafe { std::mem::transmute::<\_, Vec>(host\_vec) }
}
/// icicle invert
pub fn icicle\_invert(coeffs: &[G], stream: &IcicleStream) -> Vec {
let mut d\_result = device\_vec\_from\_c\_scalars(coeffs, stream);
inplace\_invert(&mut d\_result, stream);
c\_scalars\_from\_device\_vec(&mut d\_result, stream)
}
/// icicle scalars from c scalars
pub fn icicle\_scalars\_from\_c\_scalars(coeffs: &[G]) -> Vec {
coeffs.par\_iter().map(|coef| {
let repr: [u32; 8] = unsafe { mem::transmute\_copy(&coef.to\_repr()) };
ScalarField::from(repr)
}).collect()
}
/// icicle scalar from c scalar
pub fn icicle\_scalar\_from\_c\_scalar(coef: &G) -> ScalarField {
let repr: [u32; 8] = unsafe { mem::transmute\_copy(&coef.to\_repr()) };
ScalarField::from(repr)
}
/// icicle points from c
pub fn icicle\_points\_from\_c(bases: &[C]) -> Vec> {
bases.par\_iter().map(|p| {
let coordinates = p.coordinates().unwrap();
let x\_repr: [u32; 8] = unsafe { mem::transmute\_copy(&coordinates.x().to\_repr()) };
let y\_repr: [u32; 8] = unsafe { mem::transmute\_copy(&coordinates.y().to\_repr()) };
Affine::::from\_limbs(x\_repr, y\_repr)
}).collect()
}
fn c\_from\_icicle\_point(point: &G1Projective) -> C::Curve {
let (x, y) = {
let affine: Affine = Affine::::from(\*point);
(
repr\_from\_u32::(&affine.x.into()),
repr\_from\_u32::(&affine.y.into()),
)
};
let affine = C::from\_xy(x, y);
affine.unwrap().to\_curve()
}
/// multiexp on device
pub fn multiexp\_on\_device(coeffs: &[C::Scalar], bases: &DeviceSlice>, stream: &IcicleStream) -> C::Curve {
let coeffs = device\_vec\_from\_c\_scalars::(coeffs, stream);
let mut msm\_results = DeviceVec::::device\_malloc\_async(1, stream).unwrap();
let mut cfg = msm::MSMConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
msm::msm(&coeffs, bases, &cfg, &mut msm\_results[..]).unwrap();
let mut msm\_host\_result = [G1Projective::zero(); 1];
msm\_results
.copy\_to\_host\_async(HostSlice::from\_mut\_slice(&mut msm\_host\_result[..]), stream)
.unwrap();
c\_from\_icicle\_point::(&msm\_host\_result[0])
}
/// fft on device
pub fn fft\_on\_device + ff::PrimeField>(
scalars: &mut [G],
inverse: bool,
stream: &IcicleStream,
) {
let dir = if inverse { NTTDir::kInverse } else { NTTDir::kForward };
let mut cfg = NTTConfig::::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let mut icicle\_scalars = device\_vec\_from\_c\_scalars(scalars, stream);
ntt\_inplace::(
&mut icicle\_scalars,
dir,
&cfg,
).unwrap();
let c\_scalars = &c\_scalars\_from\_device\_vec::(&mut icicle\_scalars, stream)[..];
scalars.copy\_from\_slice(c\_scalars);
}
/// fft on device
pub fn fft\_on\_device\_vec + ff::PrimeField>(
scalars: &mut [G],
inverse: bool,
stream: &IcicleStream,
) -> DeviceVec {
let dir = if inverse { NTTDir::kInverse } else { NTTDir::kForward };
let mut cfg = NTTConfig::::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let mut icicle\_scalars = device\_vec\_from\_c\_scalars(scalars, stream);
ntt\_inplace::(
&mut icicle\_scalars,
dir,
&cfg,
).unwrap();
icicle\_scalars
}
/// print available memory
pub fn print\_available\_memory(label: &str) {
let (total, free) = get\_available\_memory().unwrap();
let used = total - free;
println!("{:?}", label);
println!("Total memory: {} MB", total / 1024 / 1024);
println!("Free memory: {} MB", free / 1024 / 1024);
println!("Used memory: {} MB", used / 1024 / 1024);
println!("--------------------------------");
}
/// inplace invert
pub fn inplace\_invert(device\_vec: &mut DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
inv\_scalars(d\_copy, device\_vec, &cfg).unwrap();
}
/// inplace mul
pub fn inplace\_mul(device\_vec: &mut DeviceSlice, other: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
mul\_scalars(d\_copy, other, device\_vec, &cfg).unwrap();
}
/// inplace add
pub fn inplace\_add(device\_vec: &mut DeviceSlice, other: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
add\_scalars(d\_copy, other, device\_vec, &cfg).unwrap();
}
/// inplace sub
pub fn inplace\_sub(device\_vec: &mut DeviceSlice, other: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
sub\_scalars(d\_copy, other, device\_vec, &cfg).unwrap();
}
/// inplace scalar add
pub fn inplace\_scalar\_add(device\_vec: &mut DeviceSlice, scalar: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
scalar\_add(scalar, d\_copy, device\_vec, &cfg).unwrap();
}
/// inplace scalar mul
pub fn inplace\_scalar\_mul(device\_vec: &mut DeviceSlice, scalar: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
scalar\_mul(scalar, d\_copy, device\_vec, &cfg).unwrap();
}
/// inplace scalar sub
pub fn inplace\_scalar\_sub(device\_vec: &mut DeviceSlice, scalar: &DeviceSlice, stream: &IcicleStream) {
let mut cfg = VecOpsConfig::default();
cfg.stream\_handle = stream.into();
cfg.is\_async = true;
let d\_copy = unsafe {
DeviceSlice::from\_mut\_slice(std::slice::from\_raw\_parts\_mut(
device\_vec.as\_mut\_ptr(),
device\_vec.len(),
))
};
scalar\_sub(scalar, d\_copy, device\_vec, &cfg).unwrap();
}
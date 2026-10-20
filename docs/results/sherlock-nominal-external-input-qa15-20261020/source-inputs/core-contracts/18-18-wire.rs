pub trait OwnerWire { fn owner_wire(&self) -> Value; }
impl OwnerWire for str { fn owner_wire(&self) -> Value { Value::String(self.to_owned()) } }
macro_rules! decimal_wire { ($($ty:ty),+ $(,)?) => { $(
    impl OwnerWire for $ty { fn owner_wire(&self) -> Value { Value::String(self.to_string()) } }
)+ }; }
decimal_wire!(usize, u128, i64);
macro_rules! json_wire { ($($ty:ty),+ $(,)?) => { $(
    impl OwnerWire for $ty { fn owner_wire(&self) -> Value { json!(self) } }
)+ }; }
json_wire!(bool, u8, u32, String, crate::Type);
impl<T: OwnerWire> OwnerWire for Option<T> { fn owner_wire(&self) -> Value {
    self.as_ref().map_or(Value::Null, OwnerWire::owner_wire)
} }
impl<T: OwnerWire> OwnerWire for [T] { fn owner_wire(&self) -> Value {
    Value::Array(self.iter().map(OwnerWire::owner_wire).collect())
} }
impl<T: OwnerWire> OwnerWire for Vec<T> { fn owner_wire(&self) -> Value { self.as_slice().owner_wire() } }
impl<T: OwnerWire> OwnerWire for Arc<T> { fn owner_wire(&self) -> Value { self.as_ref().owner_wire() } }
impl<T: OwnerWire + ?Sized> OwnerWire for &T { fn owner_wire(&self) -> Value { T::owner_wire(*self) } }
impl<A: OwnerWire, B: OwnerWire> OwnerWire for (A, B) { fn owner_wire(&self) -> Value {
    Value::Array(vec![self.0.owner_wire(), self.1.owner_wire()])
} }

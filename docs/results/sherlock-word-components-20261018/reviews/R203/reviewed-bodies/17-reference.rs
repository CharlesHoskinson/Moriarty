// main.k:250-272. Translate uiChecked's range failure to error 10 using
// checked arithmetic on the typed U128 input domain.
fn binary_result(op: BinaryOp, left: Value, right: Value) -> Result<Value, u8> {
    match (op, left, right) {
        (BinaryOp::Add, Value::UInt(a), Value::UInt(b)) => {
            a.checked_add(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Sub, Value::UInt(a), Value::UInt(b)) => {
            a.checked_sub(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Mul, Value::UInt(a), Value::UInt(b)) => {
            a.checked_mul(b).map(Value::UInt).ok_or(10)
        }
        (BinaryOp::Lt, Value::UInt(a), Value::UInt(b)) => Ok(Value::Bool(a < b)),
        (BinaryOp::Eq, a, b) => Ok(Value::Bool(a == b)),
        (BinaryOp::Cons, Value::UInt(head), Value::List(mut tail)) => {
            tail.insert(0, head);
            Ok(Value::List(tail))
        }
        (BinaryOp::Branch, Value::Tree(left), Value::Tree(right)) => {
            Ok(Value::Tree(Tree::Branch(Arc::new(left), Arc::new(right))))
        }
        _ => Err(11),
    }
}


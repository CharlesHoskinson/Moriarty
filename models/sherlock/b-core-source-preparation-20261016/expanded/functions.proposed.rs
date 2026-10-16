// PROVISIONAL, UNINSTALLED private row projections only.
// Required lexical types: ConstRow, DataRow and LogicalKind from the pinned
// 104-type proposal ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1.
// Root owns eventual module placement; no module path or table interface is chosen.
// These are factors of the row-to-kind part of R2 logical_kind.kind(ref),
// not the full ValueRef dereference operation. No validation is performed.
// No tag ordinals, representation, admission, C-INV policy or ABI is selected.
// Compilation and execution are UNRUN.

fn const_row_kind(row: &ConstRow) -> LogicalKind {
    match row {
        ConstRow::U128 { .. } => LogicalKind::UInt,
        ConstRow::Bool { .. } => LogicalKind::Bool,
        ConstRow::Nil { .. } => LogicalKind::Nil,
        ConstRow::Cons { .. } => LogicalKind::Cons,
        ConstRow::Leaf { .. } => LogicalKind::Leaf,
        ConstRow::Branch { .. } => LogicalKind::Branch,
        ConstRow::U128OutOfRange { .. } => LogicalKind::Marker,
    }
}

fn data_row_kind(row: &DataRow) -> LogicalKind {
    match row {
        DataRow::U128 { .. } => LogicalKind::UInt,
        DataRow::Bool { .. } => LogicalKind::Bool,
        DataRow::Nil => LogicalKind::Nil,
        DataRow::Cons { .. } => LogicalKind::Cons,
        DataRow::Leaf { .. } => LogicalKind::Leaf,
        DataRow::Branch { .. } => LogicalKind::Branch,
    }
}

// Additional private leaves follow the separately pinned Opus leaf contract
// 31b5f2854e93a2010cbd3c1d69395b8eea6a7658a9d7d0bd8f87862a509edecd.
// Additional required lexical types: UInt128, Bool and SortTag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckedArith {
    Value { n: UInt128 },
    Overflow,
}

fn checked_add(a: UInt128, b: UInt128) -> CheckedArith {
    match a.checked_add(b) {
        Some(n) => CheckedArith::Value { n },
        None => CheckedArith::Overflow,
    }
}

fn checked_sub(a: UInt128, b: UInt128) -> CheckedArith {
    match a.checked_sub(b) {
        Some(n) => CheckedArith::Value { n },
        None => CheckedArith::Overflow,
    }
}

fn checked_mul(a: UInt128, b: UInt128) -> CheckedArith {
    match a.checked_mul(b) {
        Some(n) => CheckedArith::Value { n },
        None => CheckedArith::Overflow,
    }
}

fn arity(kind: &LogicalKind) -> Option<usize> {
    match kind {
        LogicalKind::UInt => Some(0),
        LogicalKind::Bool => Some(0),
        LogicalKind::Nil => Some(0),
        LogicalKind::Cons => Some(2),
        LogicalKind::Leaf => Some(1),
        LogicalKind::Branch => Some(2),
        LogicalKind::Marker => None,
        LogicalKind::ConstructorExtensionReserved => None,
    }
}

fn sort_of_kind(kind: &LogicalKind) -> Option<SortTag> {
    match kind {
        LogicalKind::UInt => Some(SortTag::UInt),
        LogicalKind::Bool => Some(SortTag::Bool),
        LogicalKind::Nil => Some(SortTag::List),
        LogicalKind::Cons => Some(SortTag::List),
        LogicalKind::Leaf => Some(SortTag::Tree),
        LogicalKind::Branch => Some(SortTag::Tree),
        LogicalKind::Marker => None,
        LogicalKind::ConstructorExtensionReserved => None,
    }
}

fn same_logical_kind(a: &LogicalKind, b: &LogicalKind) -> bool {
    match a {
        LogicalKind::UInt => matches!(b, LogicalKind::UInt),
        LogicalKind::Bool => matches!(b, LogicalKind::Bool),
        LogicalKind::Nil => matches!(b, LogicalKind::Nil),
        LogicalKind::Cons => matches!(b, LogicalKind::Cons),
        LogicalKind::Leaf => matches!(b, LogicalKind::Leaf),
        LogicalKind::Branch => matches!(b, LogicalKind::Branch),
        LogicalKind::Marker => matches!(b, LogicalKind::Marker),
        LogicalKind::ConstructorExtensionReserved => {
            matches!(b, LogicalKind::ConstructorExtensionReserved)
        }
    }
}

fn same_sort_tag(a: &SortTag, b: &SortTag) -> bool {
    match a {
        SortTag::UInt => matches!(b, SortTag::UInt),
        SortTag::Bool => matches!(b, SortTag::Bool),
        SortTag::List => matches!(b, SortTag::List),
        SortTag::Tree => matches!(b, SortTag::Tree),
    }
}

// A borrowed resolved row only; the caller owns ValueRef resolution and bounds.
#[derive(Clone, Copy)]
enum ValueRow<'a> {
    Const(&'a ConstRow),
    Data(&'a DataRow),
}

fn value_row_kind(row: ValueRow<'_>) -> LogicalKind {
    match row {
        ValueRow::Const(row) => const_row_kind(row),
        ValueRow::Data(row) => data_row_kind(row),
    }
}

fn value_row_u128(row: ValueRow<'_>) -> Option<UInt128> {
    match row {
        ValueRow::Const(row) => match row {
            ConstRow::U128 { n, .. } => Some(*n),
            ConstRow::Bool { .. } => None,
            ConstRow::Nil { .. } => None,
            ConstRow::Cons { .. } => None,
            ConstRow::Leaf { .. } => None,
            ConstRow::Branch { .. } => None,
            ConstRow::U128OutOfRange { .. } => None,
        },
        ValueRow::Data(row) => match row {
            DataRow::U128 { n } => Some(*n),
            DataRow::Bool { .. } => None,
            DataRow::Nil => None,
            DataRow::Cons { .. } => None,
            DataRow::Leaf { .. } => None,
            DataRow::Branch { .. } => None,
        },
    }
}

fn value_row_bool(row: ValueRow<'_>) -> Option<Bool> {
    match row {
        ValueRow::Const(row) => match row {
            ConstRow::U128 { .. } => None,
            ConstRow::Bool { b, .. } => Some(*b),
            ConstRow::Nil { .. } => None,
            ConstRow::Cons { .. } => None,
            ConstRow::Leaf { .. } => None,
            ConstRow::Branch { .. } => None,
            ConstRow::U128OutOfRange { .. } => None,
        },
        ValueRow::Data(row) => match row {
            DataRow::U128 { .. } => None,
            DataRow::Bool { b } => Some(*b),
            DataRow::Nil => None,
            DataRow::Cons { .. } => None,
            DataRow::Leaf { .. } => None,
            DataRow::Branch { .. } => None,
        },
    }
}

fn shallow_differ(a: ValueRow<'_>, b: ValueRow<'_>) -> bool {
    !same_logical_kind(&value_row_kind(a), &value_row_kind(b))
        || value_row_u128(a) != value_row_u128(b)
        || value_row_bool(a) != value_row_bool(b)
}

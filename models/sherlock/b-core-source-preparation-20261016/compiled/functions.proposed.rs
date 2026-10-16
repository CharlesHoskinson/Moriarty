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

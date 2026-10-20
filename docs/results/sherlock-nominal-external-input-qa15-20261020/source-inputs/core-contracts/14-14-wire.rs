impl OwnerWire for LocatorC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Occurrence { owner_is_function, function, ast_path } => json!({"Occurrence": {"owner_is_function": owner_is_function.owner_wire(), "function": function.owner_wire(), "ast_path": ast_path.owner_wire()}}),
            Self::Table { table, row } => json!({"Table": {"table": table.owner_wire(), "row": row.owner_wire()}}),
            Self::Field { name } => json!({"Field": {"name": name.owner_wire()}}),
        }
    }
}

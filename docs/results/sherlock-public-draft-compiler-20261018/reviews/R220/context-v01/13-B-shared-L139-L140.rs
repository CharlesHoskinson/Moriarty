#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LocatorC { Occurrence { owner_is_function: bool, function: usize, ast_path: Vec<usize> }, Table { table: String, row: usize }, Field { name: String } }

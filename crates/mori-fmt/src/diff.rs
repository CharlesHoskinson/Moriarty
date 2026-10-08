//! The regions a formatting run changes, grouped as `cargo fmt --check` shows
//! them: each change with up to three lines of context, nearby changes merged.

use similar::{ChangeTag, TextDiff};

/// Lines of unchanged context around each change.
const CONTEXT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Context,
    Removed,
    Added,
}

/// One changed region.
#[derive(Debug)]
pub struct Hunk {
    /// The 1-based line in the original where the region starts.
    pub line: usize,
    /// Each line of the region, without its line break.
    pub lines: Vec<(Change, String)>,
}

/// The changed regions between `original` and `formatted`, empty when equal.
pub fn diff(original: &str, formatted: &str) -> Vec<Hunk> {
    let diff = TextDiff::from_lines(original, formatted);
    diff.grouped_ops(CONTEXT)
        .iter()
        .map(|group| {
            let line = group.first().map_or(0, |op| op.old_range().start) + 1;
            let lines = group
                .iter()
                .flat_map(|op| diff.iter_changes(op))
                .map(|change| {
                    let kind = match change.tag() {
                        ChangeTag::Equal => Change::Context,
                        ChangeTag::Delete => Change::Removed,
                        ChangeTag::Insert => Change::Added,
                    };
                    let text = change.value().trim_end_matches(['\n', '\r']).to_owned();
                    (kind, text)
                })
                .collect();
            Hunk { line, lines }
        })
        .collect()
}

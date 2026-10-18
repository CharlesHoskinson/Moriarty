//! Suggested fixes: close matches and corrected source lines.

use mori_span::Span;

/// The candidate closest to `word`, if one is within two edits and shorter
/// edits than its own length.
pub fn suggest<'a>(word: &str, candidates: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    candidates
        .into_iter()
        .map(|candidate| (edit_distance(word, candidate), candidate))
        .filter(|&(distance, candidate)| distance <= 2 && distance < candidate.len())
        .min_by_key(|&(distance, _)| distance)
        .map(|(_, candidate)| candidate)
}

/// The source line containing `span`, with the span replaced by
/// `replacement` and surrounding whitespace removed.
pub fn corrected_line(source: &str, span: Span, replacement: &str) -> String {
    let start = span.start as usize;
    let end = span.end as usize;
    let line_start = source[..start].rfind('\n').map_or(0, |i| i + 1);
    let line_end = source[end..].find('\n').map_or(source.len(), |i| end + i);
    let line = format!(
        "{}{replacement}{}",
        &source[line_start..start],
        &source[end..line_end]
    );
    line.trim().to_owned()
}

/// Levenshtein distance, counting a swap of adjacent letters as one edit.
fn edit_distance(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut rows = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in rows.iter_mut().enumerate() {
        row[0] = i;
    }
    rows[0] = (0..=b.len()).collect();
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (rows[i - 1][j] + 1)
                .min(rows[i][j - 1] + 1)
                .min(rows[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(rows[i - 2][j - 2] + 1);
            }
            rows[i][j] = best;
        }
    }
    rows[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::suggest;

    #[test]
    fn suggests_within_two_edits() {
        let keywords = ["const", "asset", "action"];
        assert_eq!(suggest("cosnt", keywords), Some("const"));
        assert_eq!(suggest("aset", keywords), Some("asset"));
        assert_eq!(suggest("let", keywords), None);
    }
}

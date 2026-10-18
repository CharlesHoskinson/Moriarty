    fn wrap(&self, text: &str, opts: textwrap::Options<'_>) -> String {
        if self.wrap_lines {
            textwrap::fill(text, opts)
        } else {
            // Format without wrapping, but retain the indentation options
            // Implementation based on `textwrap::indent`
            let mut result = String::with_capacity(2 * text.len());
            let trimmed_indent = opts.subsequent_indent.trim_end();
            for (idx, line) in text.split_terminator('\n').enumerate() {
                if idx > 0 {
                    result.push('\n');
                }
                if idx == 0 {
                    if line.trim().is_empty() {
                        result.push_str(opts.initial_indent.trim_end());
                    } else {
                        result.push_str(opts.initial_indent);
                    }
                } else if line.trim().is_empty() {
                    result.push_str(trimmed_indent);
                } else {
                    result.push_str(opts.subsequent_indent);
                }
                result.push_str(line);
            }
            if text.ends_with('\n') {
                // split_terminator will have eaten the final '\n'.
                result.push('\n');
            }
            result
        }
    }

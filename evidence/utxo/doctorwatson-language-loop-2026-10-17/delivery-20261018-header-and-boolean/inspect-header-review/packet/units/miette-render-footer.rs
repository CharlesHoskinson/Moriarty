    fn render_footer(&self, f: &mut impl fmt::Write, diagnostic: &(dyn Diagnostic)) -> fmt::Result {
        if let Some(help) = diagnostic.help() {
            let width = self.termwidth.saturating_sub(2);
            let initial_indent = "  help: ".style(self.theme.styles.help).to_string();
            let mut opts = textwrap::Options::new(width)
                .initial_indent(&initial_indent)
                .subsequent_indent("        ")
                .break_words(self.break_words);
            if let Some(word_separator) = self.word_separator {
                opts = opts.word_separator(word_separator);
            }
            if let Some(word_splitter) = self.word_splitter.clone() {
                opts = opts.word_splitter(word_splitter);
            }

            writeln!(f, "{}", self.wrap(&help.to_string(), opts))?;
        }
        Ok(())
    }

    fn render_report_inner(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);
        self.render_header(f, diagnostic, false)?;
        self.render_causes(f, diagnostic, src)?;
        self.render_snippets(f, diagnostic, src)?;
        self.render_footer(f, diagnostic)?;
        self.render_related(f, diagnostic, src)?;
        if let Some(footer) = &self.footer {
            writeln!(f)?;
            let width = self.termwidth.saturating_sub(2);
            let mut opts = textwrap::Options::new(width)
                .initial_indent("  ")
                .subsequent_indent("  ")
                .break_words(self.break_words);
            if let Some(word_separator) = self.word_separator {
                opts = opts.word_separator(word_separator);
            }
            if let Some(word_splitter) = self.word_splitter.clone() {
                opts = opts.word_splitter(word_splitter);
            }

            writeln!(f, "{}", self.wrap(footer, opts))?;
        }
        Ok(())
    }

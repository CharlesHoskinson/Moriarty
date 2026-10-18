    fn render_related(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);

        if let Some(related) = diagnostic.related() {
            let severity_style = match diagnostic.severity() {
                Some(Severity::Error) | None => self.theme.styles.error,
                Some(Severity::Warning) => self.theme.styles.warning,
                Some(Severity::Advice) => self.theme.styles.advice,
            };

            let mut inner_renderer = self.clone();
            // Re-enable the printing of nested cause chains for related errors
            inner_renderer.with_cause_chain = true;
            if self.show_related_as_nested {
                let width = self.termwidth.saturating_sub(2);
                let mut related = related.peekable();
                while let Some(rel) = related.next() {
                    let is_last = related.peek().is_none();
                    let char = if !is_last {
                        self.theme.characters.lcross
                    } else {
                        self.theme.characters.lbot
                    };
                    let initial_indent = format!(
                        "  {}{}{} ",
                        char, self.theme.characters.hbar, self.theme.characters.rarrow
                    )
                    .style(severity_style)
                    .to_string();
                    let rest_indent = format!(
                        "  {}   ",
                        if is_last {
                            ' '
                        } else {
                            self.theme.characters.vbar
                        }
                    )
                    .style(severity_style)
                    .to_string();

                    let mut opts = textwrap::Options::new(width)
                        .initial_indent(&initial_indent)
                        .subsequent_indent(&rest_indent)
                        .break_words(self.break_words);
                    if let Some(word_separator) = self.word_separator {
                        opts = opts.word_separator(word_separator);
                    }
                    if let Some(word_splitter) = self.word_splitter.clone() {
                        opts = opts.word_splitter(word_splitter);
                    }

                    let mut inner = String::new();

                    let mut inner_renderer = self.clone();
                    inner_renderer.footer = None;
                    inner_renderer.with_cause_chain = false;
                    inner_renderer.termwidth -= rest_indent.width();
                    inner_renderer.render_report_inner(&mut inner, rel, src)?;

                    // If there was no header, remove the leading newline
                    let inner = inner.trim_matches('\n');
                    writeln!(f, "{}", self.wrap(inner, opts))?;
                }
            } else {
                for rel in related {
                    writeln!(f)?;
                    match rel.severity() {
                        Some(Severity::Error) | None => write!(f, "Error: ")?,
                        Some(Severity::Warning) => write!(f, "Warning: ")?,
                        Some(Severity::Advice) => write!(f, "Advice: ")?,
                    };
                    inner_renderer.render_header(f, rel, true)?;
                    let src = rel.source_code().or(parent_src);
                    inner_renderer.render_causes(f, rel, src)?;
                    inner_renderer.render_snippets(f, rel, src)?;
                    inner_renderer.render_footer(f, rel)?;
                    inner_renderer.render_related(f, rel, src)?;
                }
            }
        }
        Ok(())
    }

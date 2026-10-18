    fn render_causes(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        parent_src: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let src = diagnostic.source_code().or(parent_src);

        let (severity_style, severity_icon) = match diagnostic.severity() {
            Some(Severity::Error) | None => (self.theme.styles.error, &self.theme.characters.error),
            Some(Severity::Warning) => (self.theme.styles.warning, &self.theme.characters.warning),
            Some(Severity::Advice) => (self.theme.styles.advice, &self.theme.characters.advice),
        };

        let initial_indent = format!("  {} ", severity_icon.style(severity_style));
        let rest_indent = format!("  {} ", self.theme.characters.vbar.style(severity_style));
        let width = self.termwidth.saturating_sub(2);
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

        writeln!(f, "{}", self.wrap(&diagnostic.to_string(), opts))?;

        if !self.with_cause_chain {
            return Ok(());
        }

        if let Some(mut cause_iter) = diagnostic
            .diagnostic_source()
            .map(DiagnosticChain::from_diagnostic)
            .or_else(|| diagnostic.source().map(DiagnosticChain::from_stderror))
            .map(|it| it.peekable())
        {
            while let Some(error) = cause_iter.next() {
                let is_last = cause_iter.peek().is_none();
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

                match error {
                    ErrorKind::Diagnostic(diag) => {
                        let mut inner = String::new();

                        let mut inner_renderer = self.clone();
                        // Don't print footer for inner errors
                        inner_renderer.footer = None;
                        // Cause chains are already flattened, so don't double-print the nested error
                        inner_renderer.with_cause_chain = false;
                        // Since everything from here on is indented, shrink the virtual terminal
                        inner_renderer.termwidth -= rest_indent.width();
                        inner_renderer.render_report_inner(&mut inner, diag, src)?;

                        // If there was no header, remove the leading newline
                        let inner = inner.trim_start_matches('\n');
                        writeln!(f, "{}", self.wrap(inner, opts))?;
                    }
                    ErrorKind::StdError(err) => {
                        writeln!(f, "{}", self.wrap(&err.to_string(), opts))?;
                    }
                }
            }
        }

        Ok(())
    }

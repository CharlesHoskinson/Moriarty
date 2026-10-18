    fn render_snippets(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        opt_source: Option<&dyn SourceCode>,
    ) -> fmt::Result {
        let source = match opt_source {
            Some(source) => source,
            None => return Ok(()),
        };
        let labels = match diagnostic.labels() {
            Some(labels) => labels,
            None => return Ok(()),
        };

        let mut labels = labels.collect::<Vec<_>>();
        labels.sort_unstable_by_key(|l| l.inner().offset());

        let mut contexts = Vec::with_capacity(labels.len());
        for right in labels.iter().cloned() {
            let right_conts =
                match source.read_span(right.inner(), self.context_lines, self.context_lines) {
                    Ok(cont) => cont,
                    Err(err) => {
                        writeln!(
                            f,
                            "  [{} `{}` (offset: {}, length: {}): {:?}]",
                            "Failed to read contents for label".style(self.theme.styles.error),
                            right
                                .label()
                                .unwrap_or("<none>")
                                .style(self.theme.styles.link),
                            right.offset().style(self.theme.styles.link),
                            right.len().style(self.theme.styles.link),
                            err.style(self.theme.styles.warning)
                        )?;
                        return Ok(());
                    }
                };

            if contexts.is_empty() {
                contexts.push((right, right_conts));
                continue;
            }

            let (left, left_conts) = contexts.last().unwrap();
            if left_conts.line() + left_conts.line_count() >= right_conts.line() {
                // The snippets will overlap, so we create one Big Chunky Boi
                let left_end = left.offset() + left.len();
                let right_end = right.offset() + right.len();
                let new_end = std::cmp::max(left_end, right_end);

                let new_span = LabeledSpan::new(
                    left.label().map(String::from),
                    left.offset(),
                    new_end - left.offset(),
                );
                // Check that the two contexts can be combined
                if let Ok(new_conts) =
                    source.read_span(new_span.inner(), self.context_lines, self.context_lines)
                {
                    contexts.pop();
                    // We'll throw the contents away later
                    contexts.push((new_span, new_conts));
                    continue;
                }
            }

            contexts.push((right, right_conts));
        }
        for (ctx, _) in contexts {
            self.render_context(f, source, &ctx, &labels[..])?;
        }

        Ok(())
    }

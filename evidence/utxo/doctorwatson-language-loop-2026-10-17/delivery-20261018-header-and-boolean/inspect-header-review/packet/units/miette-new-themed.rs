    pub fn new_themed(theme: GraphicalTheme) -> Self {
        Self {
            links: LinkStyle::Link,
            termwidth: 200,
            theme,
            footer: None,
            context_lines: 1,
            tab_width: 4,
            wrap_lines: true,
            with_cause_chain: true,
            with_primary_span_start: true,
            break_words: true,
            word_separator: None,
            word_splitter: None,
            highlighter: MietteHighlighter::default(),
            link_display_text: None,
            show_related_as_nested: false,
        }
    }

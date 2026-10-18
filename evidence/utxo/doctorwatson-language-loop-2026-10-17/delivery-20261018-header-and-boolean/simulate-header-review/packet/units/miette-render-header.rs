    fn render_header(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
        is_nested: bool,
    ) -> fmt::Result {
        let severity_style = match diagnostic.severity() {
            Some(Severity::Error) | None => self.theme.styles.error,
            Some(Severity::Warning) => self.theme.styles.warning,
            Some(Severity::Advice) => self.theme.styles.advice,
        };
        let mut header = String::new();
        let mut need_newline = is_nested;
        if self.links == LinkStyle::Link && diagnostic.url().is_some() {
            let url = diagnostic.url().unwrap(); // safe
            let code = if let Some(code) = diagnostic.code() {
                format!("{} ", code)
            } else {
                "".to_string()
            };
            let display_text = self.link_display_text.as_deref().unwrap_or("(link)");
            let link = format!(
                "\u{1b}]8;;{}\u{1b}\\{}{}\u{1b}]8;;\u{1b}\\",
                url,
                code.style(severity_style),
                display_text.style(self.theme.styles.link)
            );
            write!(header, "{}", link)?;
            writeln!(f, "{}", header)?;
            need_newline = true;
        } else if let Some(code) = diagnostic.code() {
            write!(header, "{}", code.style(severity_style),)?;
            if self.links == LinkStyle::Text && diagnostic.url().is_some() {
                let url = diagnostic.url().unwrap(); // safe
                write!(header, " ({})", url.style(self.theme.styles.link))?;
            }
            writeln!(f, "{}", header)?;
            need_newline = true;
        }
        if need_newline {
            writeln!(f)?;
        }
        Ok(())
    }

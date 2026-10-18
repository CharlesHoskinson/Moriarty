    pub fn with_links(mut self, links: bool) -> Self {
        self.links = if links {
            LinkStyle::Link
        } else {
            LinkStyle::Text
        };
        self
    }

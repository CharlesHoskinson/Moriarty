//! Building the document for a parsed file.
//!
//! Every source token is printed through [`Formatter::token`], which first
//! prints the comments written before it, so comments keep their order.
//! Commas are not printed from the source: lists print their own separators,
//! with a trailing comma only when broken.

use mori_ast::{Ast, NodeIdx, NodeTag};
use mori_lexer::{TokenIdx, TokenKind};

use crate::doc::{DocId, Docs};

pub struct Formatter<'a> {
    ast: &'a Ast<'a>,
    source: &'a str,
    docs: Docs,
    /// The next comment to print, by index in source order.
    next_comment: usize,
    /// The end of the last source text printed, token or comment.
    last_end: u32,
}

impl<'a> Formatter<'a> {
    pub fn new(ast: &'a Ast<'a>) -> Self {
        Self {
            ast,
            source: ast.source,
            docs: Docs::default(),
            next_comment: 0,
            last_end: 0,
        }
    }

    /// Lays out the whole file within `width` columns.
    pub fn format(mut self, width: usize) -> String {
        let root = self.file();
        self.docs.print(root, width)
    }

    fn file(&mut self) -> DocId {
        let ast = self.ast;
        let mut parts = Vec::new();

        // profile "moriarty-beta/1";
        let profile = TokenIdx::new(0);
        parts.extend(self.leading(self.start(profile), true));
        parts.push(self.token(profile));
        parts.push(self.docs.text(" "));
        parts.push(self.token(TokenIdx::new(1)));
        parts.push(self.token(TokenIdx::new(2)));
        parts.extend(self.trailing());

        // agreement NAME {
        let name = ast.main_token(NodeIdx::ROOT);
        let keyword = TokenIdx::new(name.index() - 1);
        let open = TokenIdx::new(name.index() + 1);
        // Exactly one blank line after the profile line.
        parts.push(self.docs.hard_line());
        parts.push(self.docs.hard_line());
        parts.extend(self.leading(self.start(keyword), false));
        parts.push(self.token(keyword));
        parts.push(self.docs.text(" "));
        parts.push(self.token(name));
        parts.push(self.docs.text(" "));
        parts.push(self.token(open));
        parts.extend(self.trailing());

        let close = ast.last_token(NodeIdx::ROOT);
        let items: Vec<NodeIdx> = ast.list(NodeIdx::ROOT).collect();
        let mut body = Vec::new();
        for (index, &item) in items.iter().enumerate() {
            body.push(self.docs.hard_line());
            // Exactly one blank line between items. Comments above an item stay
            // attached to it.
            if index > 0 {
                body.push(self.docs.hard_line());
            }
            body.extend(self.leading(self.start(ast.first_token(item)), false));
            body.push(self.item(item));
            body.extend(self.trailing());
        }
        for comment in self.leading_inside(self.start(close)) {
            body.push(comment);
        }
        if !body.is_empty() {
            // Comments flushed before `}` end with a line break; keep them inside.
            let nested = self.nest_lines(body);
            parts.push(nested);
            parts.push(self.docs.hard_line());
        }
        parts.push(self.token(close));
        parts.extend(self.trailing());

        // Comments after the agreement, each on its own line.
        let rest = self.leading(u32::MAX, true);
        if !rest.is_empty() {
            parts.push(self.docs.hard_line());
            parts.extend(rest);
        }
        self.docs.concat(&parts)
    }

    /// Indents `parts`, which start with a line break.
    fn nest_lines(&mut self, parts: Vec<DocId>) -> DocId {
        self.docs.nest(&parts)
    }

    fn item(&mut self, item: NodeIdx) -> DocId {
        let ast = self.ast;
        let main = ast.main_token(item);
        let mut parts = vec![self.token(main), self.docs.text(" ")];
        parts.push(self.token(ast.token_after(main, 1)));
        if ast.tag(item) == NodeTag::Action {
            for offset in [2, 3] {
                parts.push(self.docs.text(" "));
                parts.push(self.token(ast.token_after(main, offset)));
            }
            parts.push(self.token(ast.token_after(main, 4)));
            return self.docs.concat(&parts);
        }

        let mut before_equals = ast.token_after(main, 1);
        if let Some(ty) = ast.lhs_node(item) {
            parts.push(self.token(ast.token_after(main, 2)));
            parts.push(self.docs.text(" "));
            parts.push(self.ty(ty));
            before_equals = ast.last_token(ty);
        }
        let value = ast.rhs_node(item).expect("declarations have a value");
        parts.push(self.docs.text(" "));
        parts.push(self.token(ast.token_after(before_equals, 1)));
        parts.push(self.docs.text(" "));
        parts.push(self.expr(value));
        parts.push(self.token(ast.last_token(item)));
        self.docs.concat(&parts)
    }

    /// `NAME` or `NAME<ARG, ...>`, always on one line.
    fn ty(&mut self, ty: NodeIdx) -> DocId {
        let ast = self.ast;
        let name = ast.main_token(ty);
        let mut parts = vec![self.token(name)];
        let args: Vec<NodeIdx> = ast.list(ty).collect();
        if !args.is_empty() {
            parts.push(self.token(ast.token_after(name, 1)));
            for (index, &arg) in args.iter().enumerate() {
                if index > 0 {
                    parts.push(self.docs.text(", "));
                }
                parts.push(self.ty(arg));
            }
            parts.push(self.token(ast.last_token(ty)));
        }
        self.docs.concat(&parts)
    }

    fn expr(&mut self, node: NodeIdx) -> DocId {
        let ast = self.ast;
        let main = ast.main_token(node);
        match ast.tag(node) {
            NodeTag::Add | NodeTag::Sub | NodeTag::Mul => {
                let lhs = self.expr(self.operand(node, 0));
                let space = self.docs.text(" ");
                let operator = self.token(main);
                let space_after = self.docs.text(" ");
                let rhs = self.expr(self.operand(node, 1));
                self.docs.concat(&[lhs, space, operator, space_after, rhs])
            }
            NodeTag::Paren => {
                let open = self.token(main);
                let inner = self.expr(self.operand(node, 0));
                let close = self.token(ast.last_token(node));
                self.docs.concat(&[open, inner, close])
            }
            NodeTag::Quantity => {
                let number = self.token(main);
                let space = self.docs.text(" ");
                let asset = self.token(ast.token_after(main, 1));
                self.docs.concat(&[number, space, asset])
            }
            NodeTag::String
            | NodeTag::Number
            | NodeTag::Bool
            | NodeTag::Tag
            | NodeTag::Reference => self.token(main),
            NodeTag::Array => {
                let items: Vec<NodeIdx> = ast.list(node).collect();
                self.list(main, &items, ast.last_token(node), false)
            }
            NodeTag::Record => {
                let fields: Vec<NodeIdx> = ast.list(node).collect();
                self.list(main, &fields, ast.last_token(node), true)
            }
            NodeTag::Call => {
                let mut parts = vec![self.token(main)];
                let mut segment = main;
                while ast.tokens.kind(ast.token_after(segment, 1)) == TokenKind::Dot {
                    parts.push(self.token(ast.token_after(segment, 1)));
                    segment = ast.token_after(segment, 2);
                    parts.push(self.token(segment));
                }
                let fields: Vec<NodeIdx> = ast.list(node).collect();
                let open = ast.token_after(segment, 1);
                parts.push(self.list(open, &fields, ast.last_token(node), false));
                self.docs.concat(&parts)
            }
            NodeTag::Field => {
                let key = self.token(main);
                let colon = self.token(ast.token_after(main, 1));
                let space = self.docs.text(" ");
                let value = self.expr(self.operand(node, 0));
                self.docs.concat(&[key, colon, space, value])
            }
            tag => unreachable!("{tag:?} is not an expression"),
        }
    }

    /// `OPEN item, item CLOSE`: flat when it fits, otherwise one item per
    /// line with a trailing comma. Records pad with spaces when flat.
    fn list(&mut self, open: TokenIdx, items: &[NodeIdx], close: TokenIdx, padded: bool) -> DocId {
        let open_doc = self.token(open);
        let mut inner = Vec::new();
        for (index, &item) in items.iter().enumerate() {
            // Inside the brackets, a space only for padded lists; between
            // items, a space when flat.
            inner.push(if index > 0 || padded {
                self.docs.line()
            } else {
                self.docs.soft_line()
            });
            let item_doc = self.expr(item);
            inner.push(item_doc);
            let separator = if index + 1 < items.len() {
                self.docs.text(",")
            } else {
                self.docs.if_break(",")
            };
            inner.push(separator);
            // Separators are printed here, so skip the source comma, keeping
            // any comment written before it after the printed one.
            let after = self.ast.token_after(self.ast.last_token(item), 1);
            if self.ast.tokens.kind(after) == TokenKind::Comma {
                inner.extend(self.trailing());
                inner.extend(self.leading_inside(self.start(after)));
                self.last_end = self.end(after);
            }
            inner.extend(self.trailing());
        }
        // Comments before the closer stay inside the list.
        for comment in self.leading_inside(self.start(close)) {
            inner.push(comment);
        }
        if inner.is_empty() {
            let close_doc = self.token(close);
            return self.docs.concat(&[open_doc, close_doc]);
        }
        let nested = self.docs.nest(&inner);
        let line = if padded && !items.is_empty() {
            self.docs.line()
        } else {
            self.docs.soft_line()
        };
        let close_doc = self.token(close);
        self.docs.group(&[open_doc, nested, line, close_doc])
    }

    /// Prints `token`, after the comments written before it.
    fn token(&mut self, token: TokenIdx) -> DocId {
        let mut parts = self.leading(self.start(token), false);
        let text = self.ast.token_text(token);
        parts.push(self.docs.text(text));
        self.last_end = self.end(token);
        if parts.len() == 1 {
            parts[0]
        } else {
            self.docs.concat(&parts)
        }
    }

    /// Comments before `before`, each followed by a line break if it is a line
    /// comment or by a space if it is a block comment. With `keep_blank`, a
    /// blank line the author left before a comment or before `before` is kept.
    fn leading(&mut self, before: u32, keep_blank: bool) -> Vec<DocId> {
        let mut parts = Vec::new();
        while let Some(comment) = self.comment_before(before) {
            if keep_blank && self.blank_line_between(self.last_end, comment.start) {
                parts.push(self.docs.hard_line());
            }
            let text = comment.source_text(self.source);
            parts.push(self.docs.text(text));
            if text.starts_with("//") || text.contains('\n') {
                parts.push(self.docs.hard_line());
            } else {
                parts.push(self.docs.text(" "));
            }
            self.last_end = comment.end;
            self.next_comment += 1;
        }
        if keep_blank && before != u32::MAX && self.blank_line_between(self.last_end, before) {
            parts.push(self.docs.hard_line());
        }
        parts
    }

    /// Comments before a closing token, each on its own line inside the list.
    fn leading_inside(&mut self, before: u32) -> Vec<DocId> {
        let mut parts = Vec::new();
        while let Some(comment) = self.comment_before(before) {
            parts.push(self.docs.hard_line());
            parts.push(self.docs.text(comment.source_text(self.source)));
            self.last_end = comment.end;
            self.next_comment += 1;
        }
        parts
    }

    /// Comments on the same line as the last thing printed. A line comment
    /// ends the line, so it breaks the enclosing group.
    fn trailing(&mut self) -> Vec<DocId> {
        let mut parts = Vec::new();
        while self.next_comment < self.ast.tokens.comments.len() {
            let comment = self.ast.tokens.comment_span(self.next_comment);
            let gap = &self.source[self.last_end as usize..comment.start as usize];
            if gap.contains('\n') || !gap.trim().is_empty() {
                break;
            }
            let text = comment.source_text(self.source);
            parts.push(self.docs.text(" "));
            parts.push(self.docs.text(text));
            if text.starts_with("//") || text.contains('\n') {
                parts.push(self.docs.break_parent());
            }
            self.last_end = comment.end;
            self.next_comment += 1;
        }
        parts
    }

    fn comment_before(&self, before: u32) -> Option<mori_span::Span> {
        (self.next_comment < self.ast.tokens.comments.len())
            .then(|| self.ast.tokens.comment_span(self.next_comment))
            .filter(|comment| comment.start < before)
    }

    /// Whether the source between `from` and `to` has an empty line.
    fn blank_line_between(&self, from: u32, to: u32) -> bool {
        let gap = &self.source[from as usize..(to as usize).min(self.source.len())];
        let mut lines = gap.split('\n');
        lines.next();
        let mut middle: Vec<&str> = lines.collect();
        middle.pop();
        middle.iter().any(|line| line.trim().is_empty())
    }

    fn operand(&self, node: NodeIdx, which: usize) -> NodeIdx {
        let raw = if which == 0 {
            self.ast.lhs(node)
        } else {
            self.ast.rhs(node)
        };
        NodeIdx::from_raw(raw)
    }

    fn start(&self, token: TokenIdx) -> u32 {
        self.ast.tokens.start(token)
    }

    fn end(&self, token: TokenIdx) -> u32 {
        self.ast.tokens.span(token, self.source).end
    }
}

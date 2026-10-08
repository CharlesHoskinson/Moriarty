//! A Wadler-style document and its layout.
//!
//! A document is text, line breaks, indentation and groups. Laying it out
//! puts each group on one line when it fits in the remaining width, together
//! with whatever follows it on that line, and breaks it otherwise. Documents
//! live in an arena of rows; lists of children live in [`Docs::children`].

/// Index of a document in [`Docs`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocId(u32);

#[derive(Debug, Clone, Copy)]
enum Doc {
    /// `Docs::text[start..end]`.
    Text { start: u32, end: u32 },
    /// A space when flat, a newline when broken.
    Line,
    /// Nothing when flat, a newline when broken.
    SoftLine,
    /// Always a newline. Every enclosing group breaks.
    HardLine,
    /// Nothing, but every enclosing group breaks, such as after a line comment.
    BreakParent,
    /// `Docs::text[start..end]` only when the enclosing group is broken.
    IfBreak { start: u32, end: u32 },
    /// Children in order.
    Concat { start: u32, end: u32 },
    /// Children in order, indented one level after each newline.
    Nest { start: u32, end: u32 },
    /// Children laid out flat or broken together.
    Group { start: u32, end: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

const INDENT: usize = 2;

#[derive(Default)]
pub struct Docs {
    docs: Vec<Doc>,
    /// Whether a document contains a hard break, which forces its groups to break.
    hard: Vec<bool>,
    children: Vec<DocId>,
    text: String,
}

impl Docs {
    pub fn text(&mut self, text: &str) -> DocId {
        let (start, end) = self.push_text(text);
        self.push(Doc::Text { start, end }, text.contains('\n'))
    }

    pub fn line(&mut self) -> DocId {
        self.push(Doc::Line, false)
    }

    pub fn soft_line(&mut self) -> DocId {
        self.push(Doc::SoftLine, false)
    }

    pub fn hard_line(&mut self) -> DocId {
        self.push(Doc::HardLine, true)
    }

    pub fn break_parent(&mut self) -> DocId {
        self.push(Doc::BreakParent, true)
    }

    pub fn if_break(&mut self, text: &str) -> DocId {
        let (start, end) = self.push_text(text);
        self.push(Doc::IfBreak { start, end }, false)
    }

    pub fn concat(&mut self, parts: &[DocId]) -> DocId {
        let (start, end, hard) = self.push_children(parts);
        self.push(Doc::Concat { start, end }, hard)
    }

    pub fn nest(&mut self, parts: &[DocId]) -> DocId {
        let (start, end, hard) = self.push_children(parts);
        self.push(Doc::Nest { start, end }, hard)
    }

    pub fn group(&mut self, parts: &[DocId]) -> DocId {
        let (start, end, hard) = self.push_children(parts);
        self.push(Doc::Group { start, end }, hard)
    }

    /// Lays out `root` within `width` columns.
    pub fn print(&self, root: DocId, width: usize) -> String {
        let mut out = String::new();
        let mut column = 0;
        let mut stack = vec![(0, Mode::Break, root)];
        while let Some((indent, mode, id)) = stack.pop() {
            match self.doc(id) {
                Doc::Text { start, end } => {
                    let text = self.slice(start, end);
                    out.push_str(text);
                    column = match text.rfind('\n') {
                        Some(newline) => text[newline + 1..].chars().count(),
                        None => column + text.chars().count(),
                    };
                }
                Doc::Line if mode == Mode::Flat => {
                    out.push(' ');
                    column += 1;
                }
                Doc::SoftLine if mode == Mode::Flat => {}
                Doc::Line | Doc::SoftLine | Doc::HardLine => {
                    newline(&mut out, indent);
                    column = indent;
                }
                Doc::BreakParent => {}
                Doc::IfBreak { start, end } => {
                    if mode == Mode::Break {
                        let text = self.slice(start, end);
                        out.push_str(text);
                        column += text.chars().count();
                    }
                }
                Doc::Concat { start, end } => {
                    for &child in self.children[start as usize..end as usize].iter().rev() {
                        stack.push((indent, mode, child));
                    }
                }
                Doc::Nest { start, end } => {
                    for &child in self.children[start as usize..end as usize].iter().rev() {
                        stack.push((indent + INDENT, mode, child));
                    }
                }
                Doc::Group { start, end } => {
                    let flat = !self.hard[id.0 as usize]
                        && self.fits(id, width.saturating_sub(column), &stack);
                    let mode = if flat { Mode::Flat } else { Mode::Break };
                    for &child in self.children[start as usize..end as usize].iter().rev() {
                        stack.push((indent, mode, child));
                    }
                }
            }
        }
        out.truncate(out.trim_end().len());
        out.push('\n');
        out
    }

    /// Whether `group`, laid out flat, and the rest of its line fit in `width`.
    fn fits(&self, group: DocId, width: usize, rest: &[(usize, Mode, DocId)]) -> bool {
        let mut remaining = width as isize;
        let mut pending = vec![(Mode::Flat, group)];
        let mut rest = rest.iter().rev();
        loop {
            let (mode, id) = match pending.pop() {
                Some(next) => next,
                None => match rest.next() {
                    Some(&(_, mode, id)) => (mode, id),
                    None => return true,
                },
            };
            match self.doc(id) {
                Doc::Text { start, end } => {
                    let text = self.slice(start, end);
                    if text.contains('\n') {
                        return false;
                    }
                    remaining -= text.chars().count() as isize;
                }
                Doc::Line => match mode {
                    Mode::Flat => remaining -= 1,
                    Mode::Break => return true,
                },
                Doc::SoftLine if mode == Mode::Break => return true,
                Doc::HardLine => return true,
                Doc::SoftLine | Doc::BreakParent => {}
                Doc::IfBreak { start, end } => {
                    if mode == Mode::Break {
                        remaining -= self.slice(start, end).chars().count() as isize;
                    }
                }
                Doc::Concat { start, end }
                | Doc::Nest { start, end }
                | Doc::Group { start, end } => {
                    let mode = if self.hard[id.0 as usize] {
                        Mode::Break
                    } else {
                        mode
                    };
                    for &child in self.children[start as usize..end as usize].iter().rev() {
                        pending.push((mode, child));
                    }
                }
            }
            if remaining < 0 {
                return false;
            }
        }
    }

    fn doc(&self, id: DocId) -> Doc {
        self.docs[id.0 as usize]
    }

    fn slice(&self, start: u32, end: u32) -> &str {
        &self.text[start as usize..end as usize]
    }

    fn push(&mut self, doc: Doc, hard: bool) -> DocId {
        let id = DocId(u32::try_from(self.docs.len()).expect("documents fit in u32"));
        self.docs.push(doc);
        self.hard.push(hard);
        id
    }

    fn push_text(&mut self, text: &str) -> (u32, u32) {
        let start = self.text.len();
        self.text.push_str(text);
        (offset(start), offset(self.text.len()))
    }

    fn push_children(&mut self, parts: &[DocId]) -> (u32, u32, bool) {
        let start = self.children.len();
        self.children.extend_from_slice(parts);
        let hard = parts.iter().any(|part| self.hard[part.0 as usize]);
        (offset(start), offset(self.children.len()), hard)
    }
}

/// Ends the line, dropping trailing spaces, and indents the next.
fn newline(out: &mut String, indent: usize) {
    out.truncate(out.trim_end_matches(' ').len());
    out.push('\n');
    out.extend(std::iter::repeat_n(' ', indent));
}

fn offset(index: usize) -> u32 {
    u32::try_from(index).expect("document text fits in u32")
}

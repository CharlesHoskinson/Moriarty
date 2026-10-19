//! Struct-of-arrays syntax tree for `moriarty-beta/1` source.
//!
//! Following Zig's `std.zig.Ast`, every node is a fixed-size row: a tag, its
//! main token and two `u16` operands whose meaning depends on the tag (see
//! [`NodeTag`]). Variable-length children live in [`Ast::extra`] and are
//! referenced by a half-open range. Node `0` is always the root, so `0` is
//! free to mean "no child".

mod print;

use mori_lexer::{TokenIdx, TokenKind, Tokens};
use mori_span::Span;
use soa_rs::{Soa, Soars};

/// What a node is, and how to read its `lhs` and `rhs`.
///
/// "Range" means `lhs..rhs` indexes [`Ast::extra`], whose entries are node
/// indices. Tokens that follow the main token at a fixed distance are not
/// stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NodeTag {
    /// `agreement NAME { ... }`. Main token: the name. Range: items.
    Agreement,
    /// `KIND NAME [: TYPE] = VALUE ;`. Main token: the kind keyword; the name
    /// follows it. `lhs`: type node or `0`. `rhs`: value node.
    Declaration,
    /// `action NAME uses INTENT ;`. Main token: `action`; the name and intent
    /// are the tokens one and three after it.
    Action,
    /// `NAME [< TYPE, ... >]`. Main token: the name. Range: type arguments.
    Type,

    /// `lhs + rhs`. Main token: the operator.
    Add,
    /// `lhs - rhs`. Main token: the operator.
    Sub,
    /// `lhs * rhs`. Main token: the operator.
    Mul,
    /// `( lhs )`. Main token: `(`.
    Paren,

    /// Main token: the string.
    String,
    /// A bare number. Main token: the number.
    Number,
    /// `NUMBER ASSET`, such as `10.00 USD`. Main token: the number; the asset
    /// name follows it.
    Quantity,
    /// `true` or `false`. Main token: the keyword.
    Bool,
    /// `None` or `SuccessOnly`. Main token: the keyword.
    Tag,
    /// A reference to an earlier declaration. Main token: the name.
    Reference,
    /// `[ ITEM, ... ]`. Main token: `[`. Range: items.
    Array,
    /// `{ FIELD, ... }`. Main token: `{`. Range: field nodes.
    Record,
    /// `NAME(.NAME)* ( FIELD, ... )`. Main token: the first name segment.
    /// Range: field nodes.
    Call,
    /// `KEY : lhs` inside a record or call. Main token: the key.
    Field,
}

/// One node row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Node {
    pub tag: NodeTag,
    pub main_token: TokenIdx,
    pub lhs: u16,
    pub rhs: u16,
}

/// Index of a node in an [`Ast`]. Every node has a distinct main token, so
/// there are never more nodes than tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIdx(u16);

impl NodeIdx {
    pub const ROOT: Self = Self(0);

    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("node index exceeds u16"))
    }

    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }

    pub fn raw(self) -> u16 {
        self.0
    }
}

/// A parsed source file: its tokens, nodes and extra data.
#[derive(Debug)]
pub struct Ast<'src> {
    pub source: &'src str,
    pub tokens: Tokens,
    pub nodes: Soa<Node>,
    pub extra: Vec<u16>,
}

impl<'src> Ast<'src> {
    pub fn tag(&self, node: NodeIdx) -> NodeTag {
        self.nodes.tag()[node.index()]
    }

    pub fn main_token(&self, node: NodeIdx) -> TokenIdx {
        self.nodes.main_token()[node.index()]
    }

    pub fn lhs(&self, node: NodeIdx) -> u16 {
        self.nodes.lhs()[node.index()]
    }

    pub fn rhs(&self, node: NodeIdx) -> u16 {
        self.nodes.rhs()[node.index()]
    }

    /// The child stored in `lhs`, or `None` when it is `0`.
    pub fn lhs_node(&self, node: NodeIdx) -> Option<NodeIdx> {
        child(self.lhs(node))
    }

    /// The child stored in `rhs`, or `None` when it is `0`.
    pub fn rhs_node(&self, node: NodeIdx) -> Option<NodeIdx> {
        child(self.rhs(node))
    }

    /// The nodes in the extra range `lhs..rhs` of a list node.
    pub fn list(&self, node: NodeIdx) -> impl Iterator<Item = NodeIdx> + '_ {
        self.extra[self.list_range(node)]
            .iter()
            .copied()
            .map(NodeIdx::from_raw)
    }

    /// Every child of `node`, in source order.
    pub fn children(&self, node: NodeIdx) -> impl Iterator<Item = NodeIdx> + '_ {
        let (pair, list) = match self.tag(node) {
            NodeTag::Agreement
            | NodeTag::Type
            | NodeTag::Array
            | NodeTag::Record
            | NodeTag::Call => ([None, None], self.list_range(node)),
            NodeTag::Declaration | NodeTag::Add | NodeTag::Sub | NodeTag::Mul => {
                ([self.lhs_node(node), self.rhs_node(node)], 0..0)
            }
            NodeTag::Paren | NodeTag::Field => ([self.lhs_node(node), None], 0..0),
            NodeTag::Action
            | NodeTag::String
            | NodeTag::Number
            | NodeTag::Quantity
            | NodeTag::Bool
            | NodeTag::Tag
            | NodeTag::Reference => ([None, None], 0..0),
        };
        pair.into_iter()
            .flatten()
            .chain(self.extra[list].iter().copied().map(NodeIdx::from_raw))
    }

    /// The first token of `node`'s source text.
    pub fn first_token(&self, node: NodeIdx) -> TokenIdx {
        let main = self.main_token(node);
        match self.tag(node) {
            // `agreement NAME {`: the main token is the name.
            NodeTag::Agreement => TokenIdx::new(main.index() - 1),
            NodeTag::Add | NodeTag::Sub | NodeTag::Mul => self.first_token(self.operand(node, 0)),
            _ => main,
        }
    }

    /// The last token of `node`'s source text.
    pub fn last_token(&self, node: NodeIdx) -> TokenIdx {
        let main = self.main_token(node);
        match self.tag(node) {
            // A parsed file ends with the agreement's `}` and then end of file.
            NodeTag::Agreement => TokenIdx::new(self.tokens.len() - 2),
            NodeTag::Declaration => self.token_after(self.last_token(self.operand(node, 1)), 1),
            NodeTag::Action => self.token_after(main, 4),
            NodeTag::Type => match self.list(node).last() {
                Some(argument) => self.token_after(self.last_token(argument), 1),
                None => main,
            },
            NodeTag::Add | NodeTag::Sub | NodeTag::Mul => self.last_token(self.operand(node, 1)),
            NodeTag::Paren => self.token_after(self.last_token(self.operand(node, 0)), 1),
            NodeTag::Field => self.last_token(self.operand(node, 0)),
            NodeTag::Quantity => self.token_after(main, 1),
            NodeTag::Array | NodeTag::Record => self.closer(node, main),
            NodeTag::Call => {
                let mut segment = main;
                while self.tokens.kind(self.token_after(segment, 1)) == TokenKind::Dot {
                    segment = self.token_after(segment, 2);
                }
                self.closer(node, self.token_after(segment, 1))
            }
            NodeTag::String
            | NodeTag::Number
            | NodeTag::Bool
            | NodeTag::Tag
            | NodeTag::Reference => main,
        }
    }

    /// The source span of `node`, from its first token to its last.
    pub fn span(&self, node: NodeIdx) -> Span {
        let first = self.tokens.span(self.first_token(node), self.source);
        let last = self.tokens.span(self.last_token(node), self.source);
        Span::new(first.start, last.end)
    }

    /// The closing token of a list opened at `open`, after any trailing comma.
    fn closer(&self, node: NodeIdx, open: TokenIdx) -> TokenIdx {
        match self.list(node).last() {
            None => self.token_after(open, 1),
            Some(item) => {
                let after = self.token_after(self.last_token(item), 1);
                match self.tokens.kind(after) {
                    TokenKind::Comma => self.token_after(after, 1),
                    _ => after,
                }
            }
        }
    }

    /// The required `lhs` (`0`) or `rhs` (`1`) child of `node`.
    fn operand(&self, node: NodeIdx, which: usize) -> NodeIdx {
        let raw = if which == 0 {
            self.lhs(node)
        } else {
            self.rhs(node)
        };
        child(raw).expect("this node always has the operand")
    }

    fn list_range(&self, node: NodeIdx) -> std::ops::Range<usize> {
        usize::from(self.lhs(node))..usize::from(self.rhs(node))
    }

    /// The source text of a token.
    pub fn token_text(&self, token: TokenIdx) -> &'src str {
        self.tokens
            .span(token, self.source)
            .source_text(self.source)
    }

    /// The token `offset` positions after `token`.
    pub fn token_after(&self, token: TokenIdx, offset: usize) -> TokenIdx {
        TokenIdx::new(token.index() + offset)
    }

    /// Renders the tree as indented text, one node per line.
    pub fn print(&self) -> String {
        print::print(self)
    }
}

fn child(raw: u16) -> Option<NodeIdx> {
    (raw != 0).then_some(NodeIdx(raw))
}

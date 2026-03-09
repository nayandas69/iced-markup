use crate::attribute::Attribute;
use syn::{punctuated::Punctuated, token, Expr, Ident, Pat, Path};

/// Represents a single widget node in the markup tree.
///
/// Syntax: `name(args) ![attributes] | style { children }`
pub struct Node {
    /// The name of the widget (e.g., `column`, `row`, `text`).
    pub name: Path,

    /// Constructor arguments.
    pub args: Punctuated<Expr, token::Comma>,

    /// Modifiers applied to the widget.
    pub attributes: Vec<Attribute>,

    /// Thematic pipe for styling (Optional).
    pub style: Option<Expr>,

    /// Nested content (widgets or control flow).
    pub children: Vec<Content>,
}

/// Represents the types of content allowed inside a widget's block.
pub enum Content {
    /// A nested widget.
    Widget(Node),
    /// A named slot (e.g., `@header: text("Hi") {}`).
    Slot(Ident, Box<Self>),
    /// A native Rust-like `if` block.
    If(IfBlock),
    /// A native Rust-like `for` block.
    For(ForBlock),
    /// A raw expression (e.g., a variable or function call).
    Expr(Expr),
}

/// Represents an `if` control flow block.
pub struct IfBlock {
    pub condition: Expr,
    pub then_branch: Vec<Content>,
    pub else_branch: Option<Vec<Content>>,
}

/// Represents a `for` control flow block.
pub struct ForBlock {
    pub pat: Pat,
    pub expr: Expr,
    pub body: Vec<Content>,
}

/// The root of a markup expression.
pub struct Markup {
    pub root: Node,
}

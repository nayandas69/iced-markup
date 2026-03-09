use syn::{Expr, Ident};

/// Represents a widget modifier or property.
///
/// Syntax: `name: value` inside `![ ... ]`
pub struct Attribute {
    /// The name of the modifier (e.g., `spacing`, `padding`, `on_press`).
    pub name: Ident,

    /// Whether this is an event shorthand (using the `+` prefix).
    pub is_event: bool,

    /// The value to pass to the modifier function.
    pub value: Expr,
}

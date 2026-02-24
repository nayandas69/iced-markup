//! # iced-markup
//!
//! A declarative markup DSL (Domain-Specific Language) for building
//! [Iced](https://iced.rs) GUI applications in Rust.
//!
//! Instead of writing deeply nested builder chains by hand, you write a
//! concise, JSX-inspired syntax inside the [`view!`] macro and get
//! idiomatic Iced widget code at compile time — zero runtime overhead.
//!
//! ## Quick example
//!
//! ```ignore
//! use iced::widget::{column, row, text, button};
//! use iced_markup::view;
//!
//! fn view(&self) -> iced::Element<Msg> {
//!     view! {
//!         column ![spacing: 20, padding: 40] {
//!             text("Welcome to Iced!") {},
//!             row ![spacing: 10] {
//!                 button("Increment") ![on_press: Msg::Increment] {},
//!                 button("Decrement") ![on_press: Msg::Decrement] {},
//!             },
//!         }
//!     }
//!     .into()
//! }
//! ```
//!
//! Which expands to:
//!
//! ```ignore
//! iced::widget::column![
//!     iced::widget::text("Welcome to Iced!"),
//!     iced::widget::row![
//!         iced::widget::button("Increment").on_press(Msg::Increment),
//!         iced::widget::button("Decrement").on_press(Msg::Decrement),
//!     ].spacing(10),
//! ].spacing(20).padding(40)
//! ```
//!
//! ## Architecture
//!
//! The crate is structured as a classic compiler pipeline:
//!
//! 1. **[`parser`]** — Reads the token stream and produces a typed AST.
//! 2. **[`ast`]** — Defines every node type (widgets, components,
//!    expressions, attributes, children).
//! 3. **[`codegen`]** — Implements `ToTokens` for every AST node, emitting
//!    valid Iced Rust code.
//!
//! The public entry point is the [`view!`] procedural macro defined below.

// Internal modules that form the compiler pipeline.
mod ast;
mod codegen;
mod parser;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

use crate::ast::Markup;

/// The main procedural macro that transforms declarative markup into Iced widget code.
/// I will write complete full code and then explain it in detail.
#[proc_macro]
#[proc_macro_error]
pub fn view(input: TokenStream) -> TokenStream {
    // Parse the input tokens into our AST.
    let markup = parse_macro_input!(input as Markup);

    // Generate the Iced widget code from the AST.
    let tokens = markup.to_token_stream();
    
    // Return the generated tokens as a TokenStream.
    TokenStream::from(quote! { #tokens })
}


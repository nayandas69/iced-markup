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
//! ```no_run
//! use iced::widget::{column, row, text, button};
//! use iced_markup::view;
//!
//! #[derive(Clone)]
//! enum Msg { Click }
//!
//! fn view() -> iced::Element<'static, Msg> {
//!     view! {
//!         column ![spacing: 20, padding: 40] {
//!             text("Welcome to Iced!") {},
//!             row ![spacing: 10] {
//!                 button("Increment") ![on_press: Msg::Click] {},
//!                 button("Decrement") ![on_press: Msg::Click] {},
//!             },
//!         }
//!     }
//!     .into()
//! }
//! ```
//!
//! ## Advanced Features
//!
//! `iced_markup` provides several advanced features to simplify complex UIs:
//!
//! ### 1. Native Control Flow
//! Use real `if` and `for` blocks inside your markup:
//!
//! ```no_run
//! # use iced_markup::view;
//! # use iced::widget::{column, text, Column};
//! # use iced::{Theme, Renderer};
//! # let show_admin = true;
//! # let items = vec!["A"];
//! # enum Msg { Item }
//! let _: Column<'_, Msg, Theme, Renderer> = view! {
//!     column {
//!         if show_admin {
//!             text("Admin Panel") {}
//!         },
//!         for item in items {
//!             text(item) {}
//!         }
//!     }
//! };
//! ```
//!
//! ### 2. Thematic Pipes (`|`)
//! Apply styles or themes with a sleek syntax:
//!
//! ```no_run
//! # use iced_markup::view;
//! # use iced::widget::{text, Column, column};
//! # use iced::{Theme, Renderer};
//! # enum Msg { Item }
//! let _: Column<'_, Msg, Theme, Renderer> = view! {
//!     column {
//!         text("Hello") | |_: &Theme| iced::widget::text::Style { color: Some(iced::Color::WHITE) } {}
//!     }
//! };
//! ```
//!
//! ### 3. Event Shorthands (`+`)
//! Use `+click`, `+input`, or `+submit` for common events:
//!
//! ```no_run
//! # use iced_markup::view;
//! # use iced::widget::{button, Column, column};
//! # use iced::{Theme, Renderer};
//! # #[derive(Clone)] enum Msg { Save }
//! let _: Column<'_, Msg, Theme, Renderer> = view! {
//!     column {
//!         button("Save") ![+click: Msg::Save] {}
//!     }
//! };
//! ```
//!
//! ### 4. Component Slots (`@`)
//! Use named slots for flexible widget configuration:
//!
//! ```no_run
//! # use iced_markup::view;
//! # use iced::widget::{button, Column, column};
//! # use iced::{Theme, Renderer};
//! # #[derive(Clone)] enum Msg { Click }
//! let _: Column<'_, Msg, Theme, Renderer> = view! {
//!     column {
//!         button("Submit") {
//!             @on_press: Msg::Click
//!         }
//!     }
//! };
//! ```
//!
//! ## Architecture
//!
//! The crate is structured as a classic compiler pipeline:
//!
//! 1. **[`parser`]** — Reads the token stream and produces a typed AST.
//! 2. **[`node`]** — Defines every node type (widgets, attributes, control flow).
//! 3. **[`codegen`]** — Implements `ToTokens` for every AST node, emitting
//!    optimized Iced Rust code.
//!
//! The public entry point is the [`view!`] procedural macro defined below.

mod attribute;
mod codegen;
mod node;
mod parser;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::node::Markup;

/// The `view!` procedural macro transforms a declarative, JSX-inspired markup syntax
/// into idiomatic [Iced](https://iced.rs) widget code at compile time.
///
/// This provides a much more readable and maintainable way to define GUI layouts
/// compared to deeply nested builder chains, while maintaining zero runtime overhead.
///
/// # Syntax breakdown
///
/// ```text
/// view! {
///     widget_name(constructor_args) ![attribute: value] {
///         child_widget,
///     }
/// }
/// ```
///
/// - **`widget_name`**: The Iced widget to create (e.g., `column`, `row`, `text`). Only widgets that support `.push()` (like `Column` and `Row`) should have children in `{}` blocks.
/// - **`constructor_args`**: (Optional) Arguments passed to the widget's creation function.
/// - **`attributes`**: (Optional) Modifiers applied via `![...]` (e.g., `![spacing: 10]`).
/// - **`children`**: (Optional) Nested widgets inside `{...}` separated by commas.
///
/// # Example
///
/// ```no_run
/// # use iced_markup::view;
/// # use iced::widget::{column, text, button, Column};
/// # use iced::{Theme, Renderer};
/// # #[derive(Clone)] enum Message { Clicked }
/// let _: Column<'_, Message, Theme, Renderer> = view! {
///     column ![spacing: 20] {
///         text("Hello World") {},
///         button("Click me") ![on_press: Message::Clicked] {},
///     }
/// };
/// ```
///
/// Expands roughly to:
///
/// ```no_run
/// # use iced::widget::{column, text, button, Column};
/// # use iced::{Theme, Renderer};
/// # #[derive(Clone)] enum Message { Clicked }
/// let _: Column<'_, Message, Theme, Renderer> = iced::widget::column([])
///     .spacing(20)
///     .push(iced::widget::text("Hello World"))
///     .push(iced::widget::button("Click me").on_press(Message::Clicked));
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn view(input: TokenStream) -> TokenStream {
    // 1. Parse: Convert the input tokens into our AST structure.
    let markup = parse_macro_input!(input as Markup);

    // 2. Codegen: Transform the AST into valid Rust code.
    let tokens = markup.to_token_stream();

    // 3. Output: Return the generated code back to the compiler.
    TokenStream::from(tokens)
}

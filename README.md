<div align="center">

<h1>iced-markup</h1>

<p>
<a href="https://github.com/nayandas69/iced-markup">
<img src="https://img.shields.io/badge/status-experimental-orange.svg">
</a>
<a href="https://github.com/nayandas69/iced-markup/actions/workflows/ci.yml">
<img src="https://github.com/nayandas69/iced-markup/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://github.com/nayandas69/iced-markup/actions/workflows/docs.yml">
<img src="https://github.com/nayandas69/iced-markup/actions/workflows/docs.yml/badge.svg">
</a>
<a href="https://github.com/nayandas69/iced-markup/actions/workflows/release.yml">
<img src="https://github.com/nayandas69/iced-markup/actions/workflows/release.yml/badge.svg">
</a>
</p>

<p>
A declarative markup DSL for building
<a href="https://iced.rs">Iced</a> GUI applications in Rust.
</p>

</div>

Write concise, JSX-inspired syntax inside the `view!` macro and get idiomatic Iced widget code at compile time with zero runtime overhead.

> [!IMPORTANT]
> This is a pre-release (v0.1.0). It's stable enough to use, but expect syntax shifts.
> ⚠️ This crate is experimental. APIs may change and features are incomplete.

[Check out the official guide](https://nayandas69.github.io/iced-markup)


## The Idea

Iced is great, but building layout-heavy UIs with 50 levels of `.push()` is a nightmare to read. `iced-markup` is a procedural macro that takes a JSX-like syntax and expands it into standard, zero-cost Iced code at compile time.

## Installation

To use the markup macro in your project, add it via cargo:

```bash
cargo add iced-markup
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
iced-markup = "0.1.0"
iced = "0.13"
```

## The Markup Pattern

Everything follows a predictable shape:
`widget_name(constructor_args) ![attributes] | style { children }`

| Part | What it does |
|------|--------------|
| `name` | The Iced widget (column, text, etc) |
| `(...)` | Optional creation arguments |
| `![...]` | Method calls (spacing, padding, +events) |
| `\| ...` | The styling pipe |
| `{...}` | Nested children (for layout widgets) |

### Real code looks like this:

```rust
use iced_markup::view;
use iced::widget::{Column, Theme, Renderer};

fn ui() -> Column<'static, Message, Theme, Renderer> {
    view! {
        column ![spacing: 20] {
            text("Hello world") {},
            button("Click") ![+click: Message::DoSomething] {},
        }
    }
}
```

## How the nesting works

You can nest widgets infinitely. If a widget supports `.push()`, you put its children inside `{}`.

```rust
view! {
    scrollable {
        column ![spacing: 10] {
            row ![spacing: 5] {
                text("Left") {},
                text("Right") {},
            },
            text("Bottom") {}
        }
    }
}
```

## Control Flow & Logic

You don't need macros or `.map()` for lists. Use standard Rust syntax inside the curly braces.

```rust
view! {
    column {
        if user.is_admin {
            text("Admin Zone") | |_| text::primary {}
        },
        for i in 1..=5 {
            text(format!("Item {}", i)) {}
        }
    }
}
```

## Event Shorthands

Tired of typing `.on_input()` or `.on_press()`? Use the `+` prefix in attributes.

- `+click` -> `on_press`
- `+input` -> `on_input`
- `+submit` -> `on_submit`

```rust
button("Push me") ![+click: Msg::Trigger] {}
```

## Parameters & Slots

If you need to set a special property using the `@` syntax (like a scroll handler), do it inside the block:

```rust
scrollable {
    @on_scroll: Message::Scrolled
}
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
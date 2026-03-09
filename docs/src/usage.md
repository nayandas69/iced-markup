# Usage Guide

The logic behind `iced-markup` is simple: we map Iced builder calls to a JSX-inspired syntax.

## Basic Syntax

The general shape of a widget is:
`widget_name(constructor_args) ![attributes] | style { children }`

### Widgets

Every widget corresponds to a function in `iced::widget`. 

```rust
view! {
    text("Hello") {}
}
```

### Attributes

Attributes are mapped to method calls on the widget. They go inside `![...]`.

```rust
view! {
    column ![spacing: 20, padding: 40] {
        text("Welcome") {}
    }
}
```

### Event Shorthands

Use the `+` prefix for common events:

- `+click`: `on_press`
- `+input`: `on_input`
- `+submit`: `on_submit`

```rust
button("Submit") ![+click: Message::Submit] {}
```

### Styling Pipes

Use the `|` operator to apply styles easily.

```rust
text("Warning") | |_| text::danger {}
```

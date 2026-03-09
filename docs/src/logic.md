# Dynamic Logic

`iced-markup` isn't just a static template; it's high-performance code generation.

## Native Control Flow

You can use real Rust expressions directly inside the macro. This is much cleaner than building conditional vectors manually.

### Conditional Rendering

```rust
view! {
    column {
        if user.is_online {
            text("Online") | |_| text::success {}
        } else {
            text("Offline") | |_| text::muted {}
        }
    }
}
```

### List Rendering

```rust
view! {
    column {
        for item in &self.items {
            text(item) {}
        }
    }
}
```

## Component Slots

The `@` slot syntax allows you to inject special handlers or named parameters into a widget.

```rust
scrollable {
    @on_scroll: Message::Scrolled,
    column {
        // ...
    }
}
```

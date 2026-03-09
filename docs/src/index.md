# Introduction

Welcome to the official `iced-markup` guide.

`iced-markup` is a declarative, JSX-inspired Domain Specific Language (DSL) built for the [Iced](https://iced.rs) GUI library in Rust. 

The goal of this project is to take the pain out of building complex, nested layouts by replacing deeply nested builder chains with a clean, readable markup syntax that expands at compile-time—meaning zero runtime cost and maximum performance.

## Why use iced-markup?

- **Readability**: Your UI code actually looks like a UI, not a giant tower of `.push()` calls.
- **Speed**: It uses procedural macros to generate the exact code you would have written by hand.
- **Native Logic**: Use real Rust `if` and `for` blocks inside your UI definitions.
- **Power**: Shorthands for events, styling pipes, and component slots make high-level UI construction effortless.

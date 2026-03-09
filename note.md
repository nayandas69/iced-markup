# Developer Notes

Quick reference for common commands during development.

## Building
Build the project in debug mode:
```bash
cargo check
```

## Formatting
Ensure code follows standard Rust style:
```bash
cargo fmt
```

Check if formatting is correct (CI check):
```bash
cargo fmt -- --check
```

## Testing
Run all tests including integration and documentation tests:
```bash
cargo test
```

Run only documentation tests:
```bash
cargo test --doc
```

## Examples
Run the built-in counter example to verify UI logic:
```bash
cargo run --example counter
```

## Documentation
Build and open the documentation locally.

**Prerequisite**: Install `mdBook` if you haven't already:
```bash
cargo install mdbook
```

Once installed, run:
```bash
# From the project root
cd docs
mdbook build --open
```
This will build the documentation and open it in your default web browser. You can edit the markdown files in the `docs` directory to update the documentation content.
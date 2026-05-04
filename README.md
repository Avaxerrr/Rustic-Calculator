# Rustic Calculator

A small Rust desktop calculator built with Slint.

## Structure

- `ui/main_window.slint` contains the visual layout and button callbacks.
- `src/calculator.rs` contains calculator state and arithmetic rules.
- `src/app.rs` connects the Slint UI to the calculator state.
- `src/main.rs` starts the application.
- `build.rs` compiles the Slint UI during `cargo build`.

This is closer to a small MVU/MVVM-style split than classic MVC: the Slint file is the view, `Calculator` is the model/domain state, and `app.rs` is the thin binding layer between them.

## Run

```powershell
cargo run
```

## Check

```powershell
cargo test
cargo clippy
```

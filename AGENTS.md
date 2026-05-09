# Project Guidelines

This project is a Rust desktop calculator and unit converter built with Slint. Keep changes small, readable, and organized around clear responsibilities.

## Architecture

- Keep domain logic in Rust, not in Slint UI files.
- Treat Slint files as view/layout code with callbacks and bindings only.
- Keep app wiring in `src/app.rs` and avoid mixing UI event wiring with calculation/conversion rules.
- Keep calculator rules in `src/calculator.rs`.
- Keep converter state in `src/converter_app.rs`.
- Keep conversion formulas/data in `src/features/converters.rs`.
- Keep platform/window behavior in focused modules such as `src/window_state.rs` and `src/native_window.rs`.

## Slint Organization

- `ui/main_window.slint` should remain the root window and composition layer.
- Reusable controls belong in `ui/components/`.
- Screen-level UI belongs in `ui/views/`.
- Do not let one Slint file grow into a catch-all for unrelated views or controls.
- Prefer adding or updating a focused component/view over copying similar layout blocks.
- Preserve existing dark Windows-style UI conventions unless a task explicitly changes the design.

## Rust Code

- Prefer simple structs and functions over broad abstractions.
- Add abstractions only when they remove real duplication or clarify ownership.
- Keep state mutations explicit and easy to test.
- Avoid stringly typed logic when a typed enum or structured value would make behavior safer.
- Keep public APIs small and purpose-specific.

## Separation Of Concerns

- UI should display state and emit user intents.
- Rust state modules should decide what those intents mean.
- Formatting/parsing should live near the state or feature that owns it, not scattered through callbacks.
- Platform-specific code should stay isolated behind `cfg` gates or platform modules.

## Testing And Verification

Run these before considering a change complete:

```powershell
cargo build --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

For production/release changes, also run:

```powershell
cargo build --release --locked
```

For UI changes, launch the app and visually check calculator, converter, sidebar, history, shortcuts, and About screens.

## Git Hygiene

- Do not commit generated build output from `target/`.
- Keep screenshots only when they are intentionally used by docs, such as `docs/screenshots/rustic-calculator.png`.
- Do not include local scratch folders, mockups, or tool caches.
- Keep commits focused around one logical change.

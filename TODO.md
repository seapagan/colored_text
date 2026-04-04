# TODO

- Add a writer-aware rendering API so `ColorMode::Auto` can use the actual
  output target instead of always consulting `stdout()`.
- Revisit whether to add `rust-version` to `Cargo.toml` once we want to commit
  to an explicit MSRV policy.
- Support 3-character shorthand hex colors like `#f80` in addition to 6-digit
  hex input.

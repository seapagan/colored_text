# TODO

- Revisit whether to add `rust-version` to `Cargo.toml` once we want to commit
  to an explicit MSRV policy.
- Consider caching resolved terminal color capabilities for hot render paths.
  Design explicit cache invalidation for tests and callers that mutate the
  process environment, and document the cache behavior plus any opt-out path.

# Contributing

Keep changes focused and preserve the existing public API and rendering policy.
Use standard Rust formatting and document every new public item. The project
prefers zero runtime dependencies; justify any proposed runtime dependency.

## Rust support

`Cargo.toml`'s `rust-version` is the single source of truth for the MSRV. The
library, all targets/features, examples, dev dependencies, tests, doctests,
documentation, builds, and package verification support Rust 1.78.0 with the
unchanged v4 lockfile. Exact stable checks from 1.70.0 upward proved that
1.70.0 through 1.77.0 reject lockfile v4; 1.78.0 passes the complete compiler
gate. The immediately preceding 1.77.0 fails because its Cargo cannot read
lockfile v4. No dependency downgrades were needed to select the boundary.

Before updating dependencies, inspect their compiler requirements and release
notes, retain a reproducible lockfile, and run `cargo make msrv`. An MSRV increase
requires an explicit support-policy decision and documentation update. Consumer
projects and the bootstrap requirements of developer tools do not set this
crate's MSRV.

## Local tools and checks

Install current stable Rust with rustfmt, Clippy, and llvm-tools-preview, plus
cargo-make, cargo-nextest, cargo-llvm-cov, cargo-audit, cargo-deny, actionlint,
and Zizmor. The MSRV task uses cargo-make's embedded Duckscript on every
platform, including native Windows. It reads Cargo metadata from `Cargo.toml`,
requires an exact stable `major.minor.patch` rust-version, and invokes that
toolchain for formatting, checking, Clippy, tests, doctests, documentation,
builds, and packaging. No external scripting runtime is required. Install the
exact declared Rust toolchain with Clippy and rustfmt before running the task:

```console
rustup toolchain install 1.78.0 --profile minimal --component clippy,rustfmt
cargo make verify
```

The full gate checks formatting, all targets/features, Clippy and rustdoc with
warnings denied, unit/integration tests and doctests, builds, packaging, the
exact declared MSRV, advisories/licenses/sources, workflow syntax/security,
and coverage. Packaging requires a clean committed tree. During development,
run individual tasks and use `cargo package --locked --allow-dirty` as an
interim check; run the canonical gate again after committing.

```console
cargo make coverage
cargo make coverage-html
```

Coverage includes the new resolver module. The measured pre-change baseline
was 100% lines, 100% functions, and 99.64% regions. The line threshold is 100%;
keep functions and regions at or above that baseline as well. LCOV uses remapped
repository-relative paths at `target/llvm-cov/coverage.lcov`; the HTML report is
under `target/llvm-cov/html`. `test-html` remains an alias for the HTML task.
`cargo make test` runs the test suite; `cargo make coverage` enforces the
100% line, 100% function, and 99.64% region floors. `cargo make verify` includes
both. `coverage-html` generates a diagnostic report without threshold checks,
so it remains usable when coverage falls below policy.

CI runs the common library checks on Linux, macOS, and Windows with current
stable Rust. A separate Linux job derives and verifies the exact MSRV from
`Cargo.toml`. Dependency/workflow audits and coverage run on Linux. Actions are
pinned to full commit SHAs; verify upstream stable releases and compatibility
before updating pins.

## Commits

Use short imperative Conventional Commit subjects and focused commits. Sign
commits using the existing Git signing setup and add a sign-off with
`git commit -s`. Keep API/test work separate from CI changes. Never manually
edit the generated `CHANGELOG.md` or commit local agent instructions or plans.

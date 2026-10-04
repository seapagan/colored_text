# Contributing

Keep changes focused and preserve the existing public API and rendering policy.
Use standard Rust formatting and document every new public item. The project
prefers zero runtime dependencies; justify any proposed runtime dependency.

## Published package and full source

The crates.io package retains the library source, examples, and user-facing
documentation required to build and use `colored_text`. Repository development
files, integration tests, fixtures, and tooling are not included. Unit-test
modules within the library source remain part of `src/`.

For the complete source tree, including the full test suite and development
tooling, use the [GitHub repository](https://github.com/seapagan/colored_text)
or one of GitHub's source archives.

## Rust support

`Cargo.toml`'s `rust-version` is the single source of truth for the MSRV. The
package/source MSRV is Rust 1.70.0. With a compatible lockfile, Rust 1.69.0
fails with `E0658` on production uses of `std::io::IsTerminal` and
`Option::is_some_and`. Rust 1.70.0 passes library and example builds,
documentation, package verification, and the full repository compiler gate:
all targets/features, dev dependencies, tests, doctests, formatting, and Clippy.
Cargo 1.70 regenerated the lockfile as v3 with identical dependency versions.
Keep the lockfile readable by the declared MSRV; regenerate it with that
toolchain's Cargo and inspect the diff. Current stable also accepts v3.

Before updating dependencies, inspect their compiler requirements and release
notes, retain a reproducible lockfile, and run `cargo make msrv`. An MSRV increase
requires an explicit support-policy decision and documentation update. Consumer
projects, test dependencies, lockfile format, and the bootstrap requirements of
developer tools do not set the package/source MSRV.

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
rustup toolchain install 1.70.0 --profile minimal --component clippy,rustfmt
cargo make verify
```

The full gate checks formatting, all targets/features, Clippy and rustdoc with
warnings denied, unit/integration tests and doctests, builds, packaging, the
exact declared MSRV, advisories/licenses/sources, workflow syntax/security,
and coverage, plus the optional Python support tasks below. Packaging requires
a clean committed tree. During development, run individual tasks and use
`cargo package --locked --allow-dirty` as an
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

### Optional Python support tooling

Python 3.10+ is required only for support tooling. The checker, its hardened
standard-library `unittest` regression suite, and Ruff configuration are ported
from [Keyhold main](https://github.com/seapagan/keyhold/tree/0cbd7108cd46aae929b5d39b77ecf3290fa913e9).
Lizard must be exactly 1.23.0 for parser compatibility. Ruff and mypy check the
support code; for example, install the optional tools with uv:

```sh
uv tool install 'lizard==1.23.0'
uv tool install 'ruff==0.16.10'
uv tool install 'mypy==2.4.0'
```

`cargo make verify` includes `python-format` (Ruff format check), `python-lint`
(Ruff lint), `python-type` (strict mypy), `python-test` (`unittest`), and
`complexity`. Tasks skip when their required optional executable is absent;
`python-test` and `complexity` also skip when no Python 3.10+ interpreter is available. Installed
tools that fail remain failures. Windows uses `python` for tests and complexity,
with native executable-presence checks for Ruff and mypy. Unrelated Rust tasks
do not require Python tooling. The existing Linux quality job installs Python,
Ruff, mypy, and the exact Lizard version and runs every support task.

Run `cargo make complexity` for the Codacy-aligned report. Git discovers
non-ignored tracked and untracked Rust (`*.rs`) and Python (`*.py`) sources,
**including maintained tests**. Keyhold's Codacy `tests/**` exclusion is
intentionally not copied. Deleted files are omitted, and conflicted index
entries are deduplicated.

The defaults are overrideable through the environment:

| Variable | Default |
| --- | --- |
| `COMPLEXITY_LIZARD_VERSION` | `1.23.0` |
| `COMPLEXITY_MAX_CCN` | `10` |
| `COMPLEXITY_MAX_FUNCTION_NLOC` | `50` |
| `COMPLEXITY_MAX_PARAMETERS` | `8` |
| `COMPLEXITY_MAX_FILE_NLOC` | `500` |

The default threshold values are supplied by `Makefile.toml`; direct invocation
of `scripts/check_complexity.py` therefore requires the corresponding environment
variables to be set.

Equality is allowed; only values above a threshold are findings. Complexity
findings are advisory and do not fail verification. Malformed or unexpected
Lizard output, source-set mismatches, version mismatches, invalid configuration,
missing required inputs after task start, and other tooling/setup failures are
fatal. The regression fixture disables local Git commit signing so inherited
global signing/GPG configuration cannot break its temporary repository.

## Commits

Use short imperative Conventional Commit subjects and focused commits. Sign
commits using the existing Git signing setup and add a sign-off with
`git commit -s`. Keep API/test work separate from CI changes. Never manually
edit the generated `CHANGELOG.md` or commit local agent instructions or plans.

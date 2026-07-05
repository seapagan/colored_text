# Changelog

This is an auto-generated log of all the changes that have been made to the
project since the first release.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1](https://github.com/seapagan/colored_text/releases/tag/0.5.1) (2026-07-05)

Note: this is an observable behavior change.

When color output is enabled but no terminal color-depth hints are available,
`Auto`/`Always` now resolve to `Ansi16` instead of `TrueColor`.
Explicit truecolor configuration and truecolor-capable environment hints are
unchanged.

**Bug Fixes**

- Fix: use ansi16 for unknown color depth ([#12](https://github.com/seapagan/colored_text/pull/12)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.5.0...0.5.1) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.5.0...0.5.1.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.5.0...0.5.1.patch)

## [0.5.0](https://github.com/seapagan/colored_text/releases/tag/0.5.0) (2026-07-05)

**New Features**

- Add terminal color capability detection ([#11](https://github.com/seapagan/colored_text/pull/11)) by [seapagan](https://github.com/seapagan)
- Feat: add ansi256 color support ([#10](https://github.com/seapagan/colored_text/pull/10)) by [seapagan](https://github.com/seapagan)

**Documentation**

- Docs: add note about bright color and theme support ([#9](https://github.com/seapagan/colored_text/pull/9)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.4.1...0.5.0) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.4.1...0.5.0.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.4.1...0.5.0.patch)

## [0.4.1](https://github.com/seapagan/colored_text/releases/tag/0.4.1) (2026-04-04)

**Documentation**

- Correct the install section version in README.md ([#8](https://github.com/seapagan/colored_text/pull/8)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.4.0...0.4.1) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.4.0...0.4.1.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.4.0...0.4.1.patch)

## [0.4.0](https://github.com/seapagan/colored_text/releases/tag/0.4.0) (2026-04-04) **[`YANKED`]**

**This release has been removed for the following reason and should not be used:**

- README was not updated for this version, so the install command was wrong.

**New Features**

- Feat: support shorthand hex colors ([#7](https://github.com/seapagan/colored_text/pull/7)) by [seapagan](https://github.com/seapagan)
- Feat: add writer-aware rendering ([#6](https://github.com/seapagan/colored_text/pull/6)) by [seapagan](https://github.com/seapagan)

**Refactoring**

- Feat: modernize color composition and crate internals ([#5](https://github.com/seapagan/colored_text/pull/5)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.3.0...0.4.0) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.3.0...0.4.0.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.3.0...0.4.0.patch)

## [0.3.0](https://github.com/seapagan/colored_text/releases/tag/0.3.0) (2025-02-13)

**New Features**

- Strip ansi codes when not running in a terminal ([#4](https://github.com/seapagan/colored_text/pull/4)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.2.0...0.3.0) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.2.0...0.3.0.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.2.0...0.3.0.patch)

## [0.2.0](https://github.com/seapagan/colored_text/releases/tag/0.2.0) (2025-02-10)

**New Features**

- Implement hsl() functionality ([#3](https://github.com/seapagan/colored_text/pull/3)) by [seapagan](https://github.com/seapagan)
- Support the 'NO_COLOR' env variable ([#2](https://github.com/seapagan/colored_text/pull/2)) by [seapagan](https://github.com/seapagan)
- Add inverse() and strikethrough() ([#1](https://github.com/seapagan/colored_text/pull/1)) by [seapagan](https://github.com/seapagan)

[`Full Changelog`](https://github.com/seapagan/colored_text/compare/0.1.1...0.2.0) | [`Diff`](https://github.com/seapagan/colored_text/compare/0.1.1...0.2.0.diff) | [`Patch`](https://github.com/seapagan/colored_text/compare/0.1.1...0.2.0.patch)

## [0.1.1](https://github.com/seapagan/colored_text/releases/tag/0.1.1) (2025-02-10)

Initial Release
---

*This changelog was generated using [github-changelog-md](http://changelog.seapagan.net/) by [Seapagan](https://github.com/seapagan)*

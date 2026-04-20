# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.4.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.4.0...clorinde-v1.4.1) - 2026-04-20

### Fixed

- Parsing of utf8 quoted identifiers ([#245](https://github.com/halcyonnouveau/clorinde/pull/245))

## [1.4.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.3.1...clorinde-v1.4.0) - 2026-03-24

### Added

- [**breaking**] remove deprecated `--serialize` CLI flag and config field

### Fixed

- clean up stale container on setup and box large error variants
- restore default package section when manifest dependencies are specified without explicit package config

## [1.3.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.3.0...clorinde-v1.3.1) - 2026-02-18

### Fixed

- context-aware semicolon parsing in SQL queries ([#91](https://github.com/halcyonnouveau/clorinde/pull/91))

## [1.3.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.2.0...clorinde-v1.3.0) - 2026-01-11

### Added

- *(#214)* consolidate custom type config into types.custom section
- *(#213)* add type-attributes-mapping to support attrs on generated type definitions

### Fixed

- strip Default trait from enums

## [1.2.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.1.1...clorinde-v1.2.0) - 2025-12-02

### Added

- add borrowed-type support for custom type mappings

## [1.1.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.1.0...clorinde-v1.1.1) - 2025-09-26

### Changed

- updated deps and removed fallible_iterator from generated cargo - now relying on reexported fallible_iterator from postgres/tokio-postgres

### Fixed

- replace unstable features for stable rust compatibility

## [1.1.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.1...clorinde-v1.1.0) - 2025-08-13

### Added

- [**breaking**] add custom attrs for borrowed structs ([#177](https://github.com/halcyonnouveau/clorinde/pull/177))
- Add metadata ([#145](https://github.com/halcyonnouveau/clorinde/pull/145))

## [1.0.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0...clorinde-v1.0.1) - 2025-08-02

### Fixed

- use default type arg for cargo_toml::Manifest ([#167](https://github.com/halcyonnouveau/clorinde/pull/167))
- specify concrete type for cargo_toml::Package ([#165](https://github.com/halcyonnouveau/clorinde/pull/165))

## [1.0.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.5...clorinde-v1.0.0) - 2025-07-04

### Added

- add custom attributes syntax for structs and queries ([#151](https://github.com/halcyonnouveau/clorinde/pull/151))

## [1.0.0-beta.5](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.4...clorinde-v1.0.0-beta.5) - 2025-06-23

### Added

- [**breaking**] add safer/better use of prepare and immutable bind ([#143](https://github.com/halcyonnouveau/clorinde/pull/143))
- Allow configuring the output location of static files ([#141](https://github.com/halcyonnouveau/clorinde/pull/141))

### Fixed

- running without config file

## [1.0.0-beta.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.3...clorinde-v1.0.0-beta.4) - 2025-06-20

### Added

- [**breaking**] split custom field attributes into `attributes` and `attributes-borrowed`
- optimise codegen with async client and pipelining

## [1.0.0-beta.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.2...clorinde-v1.0.0-beta.3) - 2025-06-20

### Fixed

- feature for WASM compilation ([#137](https://github.com/halcyonnouveau/clorinde/pull/137))

## [1.0.0-beta.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.1...clorinde-v1.0.0-beta.2) - 2025-06-19

### Added

- [**breaking**] remove ignore_underscore_files from cli args
- [**breaking**] make Config #[non_exhaustive]

## [1.0.0-beta.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.0...clorinde-v1.0.0-beta.1) - 2025-06-18

### Added

- deprecate serialize config, update docs to replacement
- better cli validation

## [1.0.0-beta.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.16.0...clorinde-v1.0.0-beta.0) - 2025-06-18

### Added

- *(breaking)* remove time feature flag ([#120](https://github.com/halcyonnouveau/clorinde/pull/120))
- *(breaking)* allow config to specify entire generated cargo manifest ([#119](https://github.com/halcyonnouveau/clorinde/pull/119))
- add params-only mode to make bind() private ([#118](https://github.com/halcyonnouveau/clorinde/pull/118))
- fresh command for temporary databases ([#115](https://github.com/halcyonnouveau/clorinde/pull/115))
- support nullity composite arrays ([#116](https://github.com/halcyonnouveau/clorinde/pull/116))
- ability to change container image for schema command

### Fixed

- style config serialize from toml

## [0.16.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.2...clorinde-v0.16.0) - 2025-06-05

### Added

- add style setting to configure enum variant style
- add search_path option to Live action and set in Postgres client ([#110](https://github.com/halcyonnouveau/clorinde/pull/110))
- allow user custom rust types for all pg types ([#109](https://github.com/halcyonnouveau/clorinde/pull/109))

## [0.15.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.1...clorinde-v0.15.2) - 2025-05-29

### Fixed

- codegen directory rename on windows

## [0.15.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.0...clorinde-v0.15.1) - 2025-05-26

### Added

- directory based query modules ([#99](https://github.com/halcyonnouveau/clorinde/pull/99))

## [0.15.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.4...clorinde-v0.15.0) - 2025-05-12

### Added

- allow custom field attributes in type mappings ([#96](https://github.com/halcyonnouveau/clorinde/pull/96))
- Ignore files with names prefixed with `_` ([#90](https://github.com/halcyonnouveau/clorinde/pull/90))

### Fixed

- ensure generated crate is only deleted when new generation succeeds ([#95](https://github.com/halcyonnouveau/clorinde/pull/95))

## [0.14.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.3...clorinde-v0.14.4) - 2025-04-14

### Added

- improve error handling with `try_get` in query extractors

### Fixed

- *(codegen)* make "chrono" and "time" features mutually exclusive ([#88](https://github.com/halcyonnouveau/clorinde/pull/88))

## [0.14.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.2...clorinde-v0.14.3) - 2025-04-03

### Added

- overwrite generated dep from config
- better error message when docker not installed
- add builder pattern for config

## [0.14.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.1...clorinde-v0.14.2) - 2025-03-28

### Fixed

- context aware bind parsing

## [0.14.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.0...clorinde-v0.14.1) - 2025-03-27

### Fixed

- `time` feature defined multiple times

## [0.14.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.2...clorinde-v0.14.0) - 2025-03-21

### Added

- add `types.type-traits-mapping` to set traits on specific postgres types

## [0.13.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.1...clorinde-v0.13.2) - 2025-03-07

### Fixed

- add serde to chrono and uuid features

## [0.13.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.0...clorinde-v0.13.1) - 2025-02-27

### Fixed

- adding custom deps without type mapping (#61)

### Breaking

- `ctypes` is no longer a default custom type mapping crate

## [0.13.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.1...clorinde-v0.13.0) - 2025-02-25

### Added

- add derive traits (#58)

## [0.12.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.0...clorinde-v0.12.1) - 2025-02-22

### Added

- add query doc strings (#55)

## [0.12.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.4...clorinde-v0.12.0) - 2025-02-16

### Added

- Some CLI improvements (#54)
- feat; add `use-workspace-deps` option ([#50](https://github.com/halcyonnouveau/clorinde/pull/50))

### Fixed

- fix; config defaults

### Refactor

- refactor; type register for better custom type support

## [0.11.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.3...clorinde-v0.11.4) - 2025-02-07

### Added

- feat; add static files config ([#49](https://github.com/halcyonnouveau/clorinde/pull/49))
- feat; add prompt for generating on a non-default directory

## [0.11.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.2...clorinde-v0.11.3) - 2025-01-29

### Fixed

- publish to specific repo wasn't supported in clorinde.toml (#40)

### Refactor

- use quote crate instead of codegen_template and run `cargo fmt` after generation (#35)

### Added

- add citext and other extension friends (#44)

## [0.11.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.1...clorinde-v0.11.2) - 2025-01-23

### Fixed

- lifetimes and generics (#36)

## [0.11.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.0...clorinde-v0.11.1) - 2025-01-21

### Fixed

- add serde for serialize without json (#27)
- Don't force enable optional dependencies if wasm-async is enabled (#19)
- Detect borrowed type based on std Rust types ([#17](https://github.com/halcyonnouveau/clorinde/pull/17))
- Only depend on "ctypes" crate if it is referenced ([#18](https://github.com/halcyonnouveau/clorinde/pull/18))

### Other

- rename settings parameter to config (#24)

### Refactor

- remove async-trait dependency (#28)

## [0.11.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.2...clorinde-v0.11.0) - 2025-01-12

### Added

- add bpchar to string types (#14)
- clorinde.toml adds to generated crate package (#11)
- add optional time feature

## [0.10.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.1...clorinde-v0.10.2) - 2025-01-07

### Fixed

- Don't generate imports specific to async for the sync client
- Clippy warnings in generated code
- fix warning placement

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0](https://github.com/cornucopia-rs/cornucopia/compare/cornucopia-v0.9.0...cornucopia-v0.10.0) - 2026-05-15

### Added

- add nullable-variant field attributes to type mappings
- [**breaking**] remove deprecated `--serialize` CLI flag and config field
- *(#214)* consolidate custom type config into types.custom section
- *(#213)* add type-attributes-mapping to support attrs on generated type definitions
- add borrowed-type support for custom type mappings
- [**breaking**] add custom attrs for borrowed structs ([#177](https://github.com/cornucopia-rs/cornucopia/pull/177))
- Add metadata ([#145](https://github.com/cornucopia-rs/cornucopia/pull/145))
- add custom attributes syntax for structs and queries ([#151](https://github.com/cornucopia-rs/cornucopia/pull/151))
- [**breaking**] add safer/better use of prepare and immutable bind ([#143](https://github.com/cornucopia-rs/cornucopia/pull/143))
- Allow configuring the output location of static files ([#141](https://github.com/cornucopia-rs/cornucopia/pull/141))
- [**breaking**] split custom field attributes into `attributes` and `attributes-borrowed`
- optimise codegen with async client and pipelining
- [**breaking**] remove ignore_underscore_files from cli args
- [**breaking**] make Config #[non_exhaustive]
- deprecate serialize config, update docs to replacement
- better cli validation
- *(breaking)* remove time feature flag ([#120](https://github.com/cornucopia-rs/cornucopia/pull/120))
- *(breaking)* allow config to specify entire generated cargo manifest ([#119](https://github.com/cornucopia-rs/cornucopia/pull/119))
- add params-only mode to make bind() private ([#118](https://github.com/cornucopia-rs/cornucopia/pull/118))
- fresh command for temporary databases ([#115](https://github.com/cornucopia-rs/cornucopia/pull/115))
- support nullity composite arrays ([#116](https://github.com/cornucopia-rs/cornucopia/pull/116))
- ability to change container image for schema command
- add style setting to configure enum variant style
- add search_path option to Live action and set in Postgres client ([#110](https://github.com/cornucopia-rs/cornucopia/pull/110))
- allow user custom rust types for all pg types ([#109](https://github.com/cornucopia-rs/cornucopia/pull/109))
- add lto to release build
- directory based query modules ([#99](https://github.com/cornucopia-rs/cornucopia/pull/99))
- allow custom field attributes in type mappings ([#96](https://github.com/cornucopia-rs/cornucopia/pull/96))
- Ignore files with names prefixed with `_` ([#90](https://github.com/cornucopia-rs/cornucopia/pull/90))
- improve error handling with `try_get` in query extractors
- overwrite generated dep from config
- better error message when docker not installed
- add builder pattern for config
- add `types.type-traits-mapping` to set traits on specific postgres types
- add derive traits ([#58](https://github.com/cornucopia-rs/cornucopia/pull/58))
- add query doc strings ([#55](https://github.com/cornucopia-rs/cornucopia/pull/55))
- Some CLI improvements ([#54](https://github.com/cornucopia-rs/cornucopia/pull/54))
- feat; add `use-workspace-deps` option ([#50](https://github.com/cornucopia-rs/cornucopia/pull/50))
- feat; add static files config ([#49](https://github.com/cornucopia-rs/cornucopia/pull/49))
- feat; add prompt for generating on a non-default directory
- add citext and other extension friends ([#44](https://github.com/cornucopia-rs/cornucopia/pull/44))
- add documentation book
- add bpchar to string types ([#14](https://github.com/cornucopia-rs/cornucopia/pull/14))
- clorinde.toml adds to generated crate package ([#11](https://github.com/cornucopia-rs/cornucopia/pull/11))
- add optional time feature

### Changed

- updated deps and removed fallible_iterator from generated cargo - now relying on reexported fallible_iterator from postgres/tokio-postgres

### Fixed

- detect workspace dependencies in Cargo.toml parsing ([#248](https://github.com/cornucopia-rs/cornucopia/pull/248))
- Parsing of utf8 quoted identifiers ([#245](https://github.com/cornucopia-rs/cornucopia/pull/245))
- clean up stale container on setup and box large error variants
- restore default package section when manifest dependencies are specified without explicit package config
- context-aware semicolon parsing in SQL queries ([#91](https://github.com/cornucopia-rs/cornucopia/pull/91))
- strip Default trait from enums
- replace unstable features for stable rust compatibility
- use default type arg for cargo_toml::Manifest ([#167](https://github.com/cornucopia-rs/cornucopia/pull/167))
- specify concrete type for cargo_toml::Package ([#165](https://github.com/cornucopia-rs/cornucopia/pull/165))
- running without config file
- feature for WASM compilation ([#137](https://github.com/cornucopia-rs/cornucopia/pull/137))
- dont publish example codegen
- style config serialize from toml
- codegen directory rename on windows
- ensure generated crate is only deleted when new generation succeeds ([#95](https://github.com/cornucopia-rs/cornucopia/pull/95))
- *(codegen)* make "chrono" and "time" features mutually exclusive ([#88](https://github.com/cornucopia-rs/cornucopia/pull/88))
- context aware bind parsing
- `time` feature defined multiple times
- add serde to chrono and uuid features
- adding custom deps without type mapping ([#61](https://github.com/cornucopia-rs/cornucopia/pull/61))
- fix; config defaults
- fix; rename hard-link
- fix; workflow run codegen
- update test path in benches
- publish to specific repo wasn't supported in clorinde.toml ([#40](https://github.com/cornucopia-rs/cornucopia/pull/40))
- lifetimes and generics ([#36](https://github.com/cornucopia-rs/cornucopia/pull/36))
- add serde for serialize without json ([#27](https://github.com/cornucopia-rs/cornucopia/pull/27))
- Don't force enable optional dependencies if wasm-async is enabled ([#19](https://github.com/cornucopia-rs/cornucopia/pull/19))
- fix features
- Don't generate imports specific to async for the sync client
- Clippy warnings in generated code
- *(bench)* CodegenSettings was missing config (this doesn't seem to be used?)
- fix warning placment
- fix code! macro usage
- fix path
- fix persist

### Other

- update README for the cornucopia/clorinde merge
- move introduction page to root dir
- Fix broken link to examples page ([#102](https://github.com/cornucopia-rs/cornucopia/pull/102))
- remove deprecation warning for `--serialize`
- deprecate `--serialize` and update time flag
- add section for `types.derive-traits-mapping`
- fix links
- fix link
- add query comments
- docs; fix bench docs
- docs; yet another re-pass of the book
- docs; add note for ToSql/FromSql for custom types
- Detect borrowed type based on std Rust types ([#17](https://github.com/cornucopia-rs/cornucopia/pull/17))
- Only depend on "ctypes" crate if it is referenced ([#18](https://github.com/cornucopia-rs/cornucopia/pull/18))
- rename settings parameter to config ([#24](https://github.com/cornucopia-rs/cornucopia/pull/24))
- bump version
- allow no config file
- run cargo fmt
- update cargo.toml
- update readme
- update config file
- update workflows
- book init
- cool hat
- get_type_mapping pub crate
- add custom type mapping
- cleanup
- rename to clorinde
- Merge remote-tracking branch 'fork/crate_codegen'
- update auto_build example
- move tests to single dir
- re export postgres for sync
- add more wasm support
- time to chrono
- re export tokio-pg and deadpool
- add wasm feature
- Clippy fix
- Update deadpool
- Update deadpool
- Revert "Update deadpool and remove async_trait"
- Update deadpool and remove async_trait
- Update dependencies
- Add retrocompatible import paths
- Merge branch 'main' into crate_codegen
- Clippy fix
- Update dependencies
- Change name of benchmark in fixture.
- Add minor comments.
- Update lockfile.
- Remove dependencies that were added by mistake.
- For some reason, `toml` now uses double quotes for multiline.
- Fix integration test.
- Minor aesthetic change.
- Fix merge typo.
- Adapt `test_integration` to dual sync-async support.
- Merge branch 'main' into workspace_improvements
- Docker should be the default for integration tests.
- Upgrade toml.
- Upgrade dependencies.
- Improve `test_integration` internal organization.
- clippy fixes
- Allow drop copy in our copy test.
- Slightly simplify `run` feature in integration tests.
- Rename `codegen_test` to `test_codegen` and `integration` to `test_integration` for better visibility.
- Add `integration` documentation.
- Add `codegen_test` documentation.
- Renamed `usage` bench to `execution`
- Reword error message.
- Copy documentation into a README file.
- Move published crates into a dedicated `crates/` folder.
- Document dependencies.
- Prevent running sync `codegen_test` twice (until we add a real async `codegen_test`)
- Add optional podman support to integration test.
- Remove unused lock file.
- Add some documentation for benchmarks.
- Remove ad-hoc `moving` function and replcace with `std::drop`, which is exactly the same.
- Rename `bench` crate to `benches`.
- Remove unused asset.

### Refactor

- refactor; type register for better custom type support
- use quote crate instead of codegen_template and run `cargo fmt` after generation ([#35](https://github.com/cornucopia-rs/cornucopia/pull/35))
- remove async-trait dependency ([#28](https://github.com/cornucopia-rs/cornucopia/pull/28))

<!--
Entries below this point are from the upstream clorinde fork at the time of merging, preserved for
historical reference. Cornucopia changes resume above starting with v1.0.
-->

## [clorinde-v1.4.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.4.0...clorinde-v1.4.1) - 2026-04-20

### Fixed

- Parsing of utf8 quoted identifiers ([#245](https://github.com/halcyonnouveau/clorinde/pull/245))

## [clorinde-v1.4.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.3.1...clorinde-v1.4.0) - 2026-03-24

### Added

- [**breaking**] remove deprecated `--serialize` CLI flag and config field

### Fixed

- clean up stale container on setup and box large error variants
- restore default package section when manifest dependencies are specified without explicit package config

## [clorinde-v1.3.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.3.0...clorinde-v1.3.1) - 2026-02-18

### Fixed

- context-aware semicolon parsing in SQL queries ([#91](https://github.com/halcyonnouveau/clorinde/pull/91))

## [clorinde-v1.3.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.2.0...clorinde-v1.3.0) - 2026-01-11

### Added

- *(#214)* consolidate custom type config into types.custom section
- *(#213)* add type-attributes-mapping to support attrs on generated type definitions

### Fixed

- strip Default trait from enums

## [clorinde-v1.2.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.1.1...clorinde-v1.2.0) - 2025-12-02

### Added

- add borrowed-type support for custom type mappings

## [clorinde-v1.1.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.1.0...clorinde-v1.1.1) - 2025-09-26

### Changed

- updated deps and removed fallible_iterator from generated cargo - now relying on reexported fallible_iterator from postgres/tokio-postgres

### Fixed

- replace unstable features for stable rust compatibility

## [clorinde-v1.1.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.1...clorinde-v1.1.0) - 2025-08-13

### Added

- [**breaking**] add custom attrs for borrowed structs ([#177](https://github.com/halcyonnouveau/clorinde/pull/177))
- Add metadata ([#145](https://github.com/halcyonnouveau/clorinde/pull/145))

## [clorinde-v1.0.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0...clorinde-v1.0.1) - 2025-08-02

### Fixed

- use default type arg for cargo_toml::Manifest ([#167](https://github.com/halcyonnouveau/clorinde/pull/167))
- specify concrete type for cargo_toml::Package ([#165](https://github.com/halcyonnouveau/clorinde/pull/165))

## [clorinde-v1.0.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.5...clorinde-v1.0.0) - 2025-07-04

### Added

- add custom attributes syntax for structs and queries ([#151](https://github.com/halcyonnouveau/clorinde/pull/151))

## [clorinde-v1.0.0-beta.5](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.4...clorinde-v1.0.0-beta.5) - 2025-06-23

### Added

- [**breaking**] add safer/better use of prepare and immutable bind ([#143](https://github.com/halcyonnouveau/clorinde/pull/143))
- Allow configuring the output location of static files ([#141](https://github.com/halcyonnouveau/clorinde/pull/141))

### Fixed

- running without config file

## [clorinde-v1.0.0-beta.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.3...clorinde-v1.0.0-beta.4) - 2025-06-20

### Added

- [**breaking**] split custom field attributes into `attributes` and `attributes-borrowed`
- optimise codegen with async client and pipelining

## [clorinde-v1.0.0-beta.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.2...clorinde-v1.0.0-beta.3) - 2025-06-20

### Fixed

- feature for WASM compilation ([#137](https://github.com/halcyonnouveau/clorinde/pull/137))

## [clorinde-v1.0.0-beta.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.1...clorinde-v1.0.0-beta.2) - 2025-06-19

### Added

- [**breaking**] remove ignore_underscore_files from cli args
- [**breaking**] make Config #[non_exhaustive]

## [clorinde-v1.0.0-beta.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v1.0.0-beta.0...clorinde-v1.0.0-beta.1) - 2025-06-18

### Added

- deprecate serialize config, update docs to replacement
- better cli validation

## [clorinde-v1.0.0-beta.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.16.0...clorinde-v1.0.0-beta.0) - 2025-06-18

### Added

- *(breaking)* remove time feature flag ([#120](https://github.com/halcyonnouveau/clorinde/pull/120))
- *(breaking)* allow config to specify entire generated cargo manifest ([#119](https://github.com/halcyonnouveau/clorinde/pull/119))
- add params-only mode to make bind() private ([#118](https://github.com/halcyonnouveau/clorinde/pull/118))
- fresh command for temporary databases ([#115](https://github.com/halcyonnouveau/clorinde/pull/115))
- support nullity composite arrays ([#116](https://github.com/halcyonnouveau/clorinde/pull/116))
- ability to change container image for schema command

### Fixed

- style config serialize from toml

## [clorinde-v0.16.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.2...clorinde-v0.16.0) - 2025-06-05

### Added

- add style setting to configure enum variant style
- add search_path option to Live action and set in Postgres client ([#110](https://github.com/halcyonnouveau/clorinde/pull/110))
- allow user custom rust types for all pg types ([#109](https://github.com/halcyonnouveau/clorinde/pull/109))

## [clorinde-v0.15.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.1...clorinde-v0.15.2) - 2025-05-29

### Fixed

- codegen directory rename on windows

## [clorinde-v0.15.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.15.0...clorinde-v0.15.1) - 2025-05-26

### Added

- directory based query modules ([#99](https://github.com/halcyonnouveau/clorinde/pull/99))

## [clorinde-v0.15.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.4...clorinde-v0.15.0) - 2025-05-12

### Added

- allow custom field attributes in type mappings ([#96](https://github.com/halcyonnouveau/clorinde/pull/96))
- Ignore files with names prefixed with `_` ([#90](https://github.com/halcyonnouveau/clorinde/pull/90))

### Fixed

- ensure generated crate is only deleted when new generation succeeds ([#95](https://github.com/halcyonnouveau/clorinde/pull/95))

## [clorinde-v0.14.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.3...clorinde-v0.14.4) - 2025-04-14

### Added

- improve error handling with `try_get` in query extractors

### Fixed

- *(codegen)* make "chrono" and "time" features mutually exclusive ([#88](https://github.com/halcyonnouveau/clorinde/pull/88))

## [clorinde-v0.14.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.2...clorinde-v0.14.3) - 2025-04-03

### Added

- overwrite generated dep from config
- better error message when docker not installed
- add builder pattern for config

## [clorinde-v0.14.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.1...clorinde-v0.14.2) - 2025-03-28

### Fixed

- context aware bind parsing

## [clorinde-v0.14.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.14.0...clorinde-v0.14.1) - 2025-03-27

### Fixed

- `time` feature defined multiple times

## [clorinde-v0.14.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.2...clorinde-v0.14.0) - 2025-03-21

### Added

- add `types.type-traits-mapping` to set traits on specific postgres types

## [clorinde-v0.13.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.1...clorinde-v0.13.2) - 2025-03-07

### Fixed

- add serde to chrono and uuid features

## [clorinde-v0.13.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.13.0...clorinde-v0.13.1) - 2025-02-27

### Fixed

- adding custom deps without type mapping (#61)

### Breaking

- `ctypes` is no longer a default custom type mapping crate

## [clorinde-v0.13.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.1...clorinde-v0.13.0) - 2025-02-25

### Added

- add derive traits (#58)

## [clorinde-v0.12.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.12.0...clorinde-v0.12.1) - 2025-02-22

### Added

- add query doc strings (#55)

## [clorinde-v0.12.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.4...clorinde-v0.12.0) - 2025-02-16

### Added

- Some CLI improvements (#54)
- feat; add `use-workspace-deps` option ([#50](https://github.com/halcyonnouveau/clorinde/pull/50))

### Fixed

- fix; config defaults

### Refactor

- refactor; type register for better custom type support

## [clorinde-v0.11.4](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.3...clorinde-v0.11.4) - 2025-02-07

### Added

- feat; add static files config ([#49](https://github.com/halcyonnouveau/clorinde/pull/49))
- feat; add prompt for generating on a non-default directory

## [clorinde-v0.11.3](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.2...clorinde-v0.11.3) - 2025-01-29

### Fixed

- publish to specific repo wasn't supported in clorinde.toml (#40)

### Refactor

- use quote crate instead of codegen_template and run `cargo fmt` after generation (#35)

### Added

- add citext and other extension friends (#44)

## [clorinde-v0.11.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.1...clorinde-v0.11.2) - 2025-01-23

### Fixed

- lifetimes and generics (#36)

## [clorinde-v0.11.1](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.11.0...clorinde-v0.11.1) - 2025-01-21

### Fixed

- add serde for serialize without json (#27)
- Don't force enable optional dependencies if wasm-async is enabled (#19)
- Detect borrowed type based on std Rust types ([#17](https://github.com/halcyonnouveau/clorinde/pull/17))
- Only depend on "ctypes" crate if it is referenced ([#18](https://github.com/halcyonnouveau/clorinde/pull/18))

### Other

- rename settings parameter to config (#24)

### Refactor

- remove async-trait dependency (#28)

## [clorinde-v0.11.0](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.2...clorinde-v0.11.0) - 2025-01-12

### Added

- add bpchar to string types (#14)
- clorinde.toml adds to generated crate package (#11)
- add optional time feature

## [clorinde-v0.10.2](https://github.com/halcyonnouveau/clorinde/compare/clorinde-v0.10.1...clorinde-v0.10.2) - 2025-01-07

### Fixed

- Don't generate imports specific to async for the sync client
- Clippy warnings in generated code
- fix warning placement

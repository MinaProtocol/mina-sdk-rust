# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- Constructor input validation: panics on zero retries or zero timeout
- HTTP status code check before parsing JSON response body
- Retry on HTTP 5xx errors (previously only retried on connection failures)
- Reverse multiply support: `3_u64 * currency` now works
- `ClientConfig::default()` test to assert public API contract
- CHANGELOG.md
- Badges in README (crates.io, docs.rs, CI, license)
- Integration test instructions in README
- Troubleshooting section in README
- Doc build with `-D warnings` in CI

### Changed
- Slimmed `tokio` runtime features from `full` to `rt` + `time`
- Explicit type re-exports instead of glob `pub use types::*`
- Excluded non-essential files from crates.io package

### Fixed
- `GET_ACCOUNT` query uses correct `TokenId` type (was `UInt64`)
- Account query split into with/without token variants for schema compatibility
- Integration tests skip gracefully when env vars are empty strings

## [0.1.0] - 2025-11-20

### Added
- Initial release
- Async GraphQL client with configurable retry and timeout
- Query methods: sync_status, daemon_status, network_id, account, best_chain, peers, pooled_user_commands
- Mutation methods: send_payment, send_delegation, set_snark_worker, set_snark_work_fee
- `Currency` type with nanomina arithmetic, decimal parsing, and Display
- `execute_query()` for custom GraphQL queries
- `tracing` instrumentation
- Unit tests with wiremock
- Integration tests against live Mina daemon
- CI, integration, release, and schema-drift workflows
- Apache 2.0 license

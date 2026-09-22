# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_Nothing yet._

## [0.6.0] - 2026-09-22

### Added

- Add `x86_64-apple-darwin` (Intel macOS) target to the release build matrix in `.github/workflows/release.yml`, cross-compiling on `macos-latest` runners (#3).
- Add `scripts/doctor.py` and pre-commit hook integration to verify no unresolved template placeholders survive before commits are finalized (#1).
- Add `.github/dependabot.yml` for automated weekly dependency maintenance across `github-actions` and `cargo` ecosystems (#2).
- Add Keep a Changelog tracking in `CHANGELOG.md` and release extraction tooling in `scripts/release.py`.

### Changed

- Dynamically derive binary and asset names from `Cargo.toml` in `.github/workflows/release.yml`, removing all hardcoded template placeholders (#1).
- Pin all GitHub Actions in `.github/workflows/release.yml` and `.github/workflows/rust.yml` to immutable commit SHAs with version comments (#2).
- Bump `reqwest` from 0.11 to 0.13 (#9).
- Bump `scraper` from 0.18 to 0.27 (#10).
- Bump `chrono` to 0.4.45 (#8).
- Bump `serde` to 1.0.229 and `serde_json` to 1.0.151 (#11, #12).
- Bump `clap` to 4.5.60 and `anyhow` to 1.0.104 (#13, #14).
- Bump actions (`actions/checkout`, `actions/upload-artifact`, `actions/download-artifact`, `softprops/action-gh-release`) to latest releases (#4-#7).
- Update `.gemini/prompts/setup-new-tool.md` and `README.md` to reflect automated derivation and verification gates.

### Fixed

- Prevent release pipeline failures and misnamed asset publications caused by unreplaced placeholders (#1).
- Prevent supply-chain vulnerabilities from mutable floating action tags (#2).
- Fix lack of pre-built release binaries for Intel macOS users (#3).

<!-- markdownlint-disable MD049 -->
---
*Last Updated: 2026-09-22* | *Last Reviewed: 2026-09-22*

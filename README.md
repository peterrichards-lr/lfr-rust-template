# Liferay Rust Tool Template

A modular template for building high-performance, cross-platform CLI tools for Liferay DXP, Liferay Cloud (LXC), and Client Extensions.

## New in v0.7.0

- **Version Gate:** The release workflow rejects tags that don't match the `Cargo.toml` version, so `--version` always reports the released version.
- **`--version` Flag:** The CLI exposes `--version`, and the Homebrew formula test checks it.
- **Distribution Fixes:** The distribution prompt keeps `formula.rb.example` and `scoop.json.example` for future releases and skips Scoop updates the bucket already automates.

## New in v0.6.0

- **Release Management & Changelog:** Adopted release management architecture from `ai-agent-template` with `CHANGELOG.md` and automated release note extraction via `scripts/release.py`.
- **Dynamic Binary Derivation:** Release workflows dynamically derive binary and asset names from `Cargo.toml`, removing template placeholders.
- **Doctor Verification Gate:** Added `scripts/doctor.py` and pre-commit hook integration to guard against unreplaced placeholders.
- **Intel macOS Release Target:** Cross-compiles `x86_64-apple-darwin` binaries on `macos-latest` runners alongside Apple Silicon.
- **Pinned Actions & Dependabot:** All GitHub Actions pinned to immutable commit SHAs with automated weekly Dependabot updates.
- **Modernized Dependencies:** Upgraded core and optional dependencies (`reqwest 0.13`, `scraper 0.27`, `serde 1.0`, `clap 4.5`, etc.).

## Features

- **Cross-Platform:** GitHub Actions pre-configured for Windows, Linux, and macOS (ARM/Intel).
- **Liferay Aware:** Logic for path resolution and product version detection from `gradle.properties`.
- **Modern CLI:** Built on `clap` for a professional command-line experience.

## Project Structure

```plaintext
.
├── .cargo/config.toml        # Cargo aliases (lint, fmt-check)
├── .gemini/prompts/          # Automated Gemini CLI workflows
├── .github/
│   ├── dependabot.yml        # Automated dependency maintenance
│   └── workflows/            # Multi-OS CI/CD (Release, Rust)
├── .githooks/pre-commit      # Shared cross-platform git hook
├── scripts/
│   ├── doctor.py             # Placeholder verification gate
│   └── release.py            # Changelog extraction and tagging
├── src/
│   ├── main.rs               # Command routing
│   ├── core/
│   │   ├── mod.rs            # Core traits
│   │   └── env.rs            # Project discovery logic
│   ├── utils/
│   │   ├── mod.rs            # Utility re-exports
│   │   ├── git.rs            # Git wrappers
│   │   └── xml.rs            # Recursive XML logic
│   └── cli.rs                # Command definitions
├── CHANGELOG.md              # Keep a Changelog history
├── formula.rb.example        # Homebrew template
├── scoop.json.example        # Scoop template
├── .gitignore                # Tracks Cargo.lock for reliable CI
├── Cargo.toml                # Feature-based dependencies
└── LICENSE (MIT)
```

## Prerequisites

- **Rust:** `cargo`, `rustc`, `rustfmt`, `clippy`.
- **Git Hooks:** To ensure consistent code style, activate the shared pre-commit hooks:
  - Run: `git config core.hooksPath .githooks`.
  - On macOS/Linux: `chmod +x .githooks/pre-commit`.
  - This hook automatically runs `scripts/doctor.py`, `cargo fmt`, and `cargo clippy` before each commit.

## Getting Started

1. Click **"Use this template"** on GitHub to create your new repository.
2. **Automated Initialization:** Once cloned, ask Gemini to set up your new tool:
   ```bash
   "Please execute .gemini/prompts/setup-new-tool.md to initialize my new project"
   ```
   Gemini will automatically gather your project metadata (name, description, features) and update core configuration files (`Cargo.toml` and this `README.md`) for you. Release workflows automatically derive binary names directly from `Cargo.toml`.
3. **Manual Customization:**
   - Customize subcommands in `src/cli.rs`.
   - Update `LICENSE` if necessary.
4. **First Release:** Follow [Releasing](#releasing). Pushing a tag such as `v0.1.0` builds and publishes binaries for all major operating systems.

## Installation (End-Users)

Once a release is published and distribution channels are updated, users can install the tool using these commands:

### Homebrew (macOS / Linux)

```bash
brew tap [github-user]/homebrew-tap
brew install [tool-name]
```

### Scoop (Windows)

```bash
scoop bucket add [github-user] https://github.com/[github-user]/scoop-bucket
scoop install [tool-name]
```

## Development

```bash
# Build locally
cargo build

# Run with arguments
cargo run -- --help
```

## Releasing

1. Bump `version` in `Cargo.toml` and run `cargo build` to update `Cargo.lock`.
2. Move the `[Unreleased]` entries in `CHANGELOG.md` into a new `## [x.y.z] - YYYY-MM-DD` section.
3. Commit, then create an annotated tag and push it:
   ```bash
   python3 scripts/release.py --tag vX.Y.Z
   git push origin vX.Y.Z
   ```
   The release workflow fails if the tag isn't annotated, doesn't match `Cargo.toml`, or has no `CHANGELOG.md` section.
4. For a tool's first release, add it to the distribution channels (see below). After that, the Homebrew tap and Scoop bucket pick up new releases automatically within a couple of hours.

## Distribution (macOS, Linux, Windows)

To avoid "Unidentified Developer" warnings on macOS and ensure a secure, user-level installation on Windows, we recommend building from source via **Homebrew** or **Scoop**.

### Repository Visibility & Authentication

By default, Homebrew assumes your **homebrew-tap** and the tool's source repository are **public**. 

If you wish to keep your distribution repositories **private**:
1. Users must have a **GitHub Personal Access Token (PAT)** with `repo` scope.
2. Users should export this token in their environment: 
   ```bash
   export HOMEBREW_GITHUB_API_TOKEN=your_token_here
   ```
3. Without a token, `brew tap` and `brew install` will fail for private repositories.

### Automated Distribution via Gemini

This template includes an automated prompt for Gemini CLI to handle updating your Homebrew tap and Scoop bucket repositories with new releases.

Once a tool is in the tap and bucket, both update themselves from new GitHub releases, so this is only needed for a tool's first release. Ask Gemini:

```bash
"Please execute the steps in .gemini/prompts/update-distribution-channels.md to update my distribution repositories"
```

Gemini will automatically:
1. Extract metadata from `Cargo.toml`.
2. Calculate the SHA256 hash of the release tarball.
3. Generate and write the Homebrew formula (`formula.rb.example`) and Scoop manifest (`scoop.json.example`).
4. Commit and push the updates to your local `homebrew-tap` and `scoop-bucket` repositories.

<!-- markdownlint-disable MD049 -->
---
*Last Updated: 2026-11-11* | *Last Reviewed: 2026-11-11*

# Squaredle CLI

[![CI](https://github.com/HEKPYTO/Squardle-Solver-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/HEKPYTO/Squardle-Solver-cli/actions/workflows/ci.yml)

A fast, beautiful, and robust terminal utility written in Rust to fetch and decode the daily puzzle from [Squaredle.app](https://squaredle.app). This is the CLI companion to [SQUAREDLE-SOLVER](https://github.com/HEKPYTO/SQUAREDLE-SOLVER), a web UI solver that uses the same decoding algorithm.

> This project is not affiliated with or endorsed by Squaredle.app.

## Features

- **Daily Fetch:** Retrieves today's puzzle configuration via the official Squaredle API.
- **Auto-Decoder:** Implements the proprietary 62-character substitution cipher (with a -12 shift) and Base64 decoding to reveal all words.
- **Colored Grid Output:** Renders the puzzle board neatly styled.
- **Grouped Word Lists:** Organizes the decoded words by their character lengths into an easy-to-read table layout.
- **Summary Mode:** Provides a clean statistical breakdown of available words.
- **JSON Support:** Scriptable JSON output using the `--json` flag.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run from this directory:

```bash
cargo install --path .
```

## Usage

Run the default command to fetch and display today's puzzle:

```bash
squaredle-cli daily
```

Or omit the subcommand to get the same default action:

```bash
squaredle-cli
```

### Options

- `--json`: Outputs the puzzle and word lists as a raw JSON string for integration with other tools.
- `--no-color`: Disables colored terminal output (also respects the `NO_COLOR` environment variable).

## Docker

Pull and run from GitHub Container Registry:

```bash
docker pull ghcr.io/hekpyto/squardle-solver-cli:latest
docker run --rm ghcr.io/hekpyto/squardle-solver-cli
docker run --rm ghcr.io/hekpyto/squardle-solver-cli --json
```

Or build locally:

```bash
docker build -t squaredle-cli .
docker run --rm squaredle-cli
```

## Development

```bash
# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings
```

## Example Output

```text
┌─────────────────────────────┐
│     Daily Squaredle         │
│     2026-02-25              │
└─────────────────────────────┘

  Grid:
  ┌───┬───┬───┬───┐
  │ R │ U │ S │ T │
  ├───┼───┼───┼───┤
  │ A │ W │ E │ S │
  ├───┼───┼───┼───┤
  │ O │ M │ E │ L │
  ├───┼───┼───┼───┤
  │ A │ P │ P │ S │
  └───┴───┴───┴───┘

  Words (20 total):
  ╔══════════════════════════════╗
  ║ 5-letter (4)                 ║
  ╠══════════════════════════════╣
  ║ APPLE  SWEAT  STARS  RULES   ║
  ╚══════════════════════════════╝

  Summary:
  ─────────────────────────────
  Total words: 20
  5-letter: 4 │ 4-letter: 16
```

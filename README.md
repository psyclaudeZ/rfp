# rfp - Rust File Picker

[![CI main](https://github.com/psyclaudeZ/rfp/actions/workflows/ci.yml/badge.svg)](https://github.com/psyclaudeZ/rfp/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust rewrite of [fpp](https://github.com/facebook/PathPicker) that lets you interactively select files from piped input and open them in your editor.

## Installation

### macOS

```bash
brew tap psyclaudeZ/rfp
brew install rfp
```

### Other platforms

```bash
cargo install rfp
```

## Usage

Pipe any command output to rfp and it will parse out file paths for interactive selection:

```bash
find . -name "*.rs" | rfp
git grep -i 'TODO' | rfp
git ls-files | rfp
```

Press `?` for help on motions and functions.

## Acknowledgements

This project began as a learning exercise to explore Rust and is heavily inspired by [Facebook PathPicker](https://github.com/facebook/PathPicker/), a tool I've been using since it was born.

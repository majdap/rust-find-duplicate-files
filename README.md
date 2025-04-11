# rust_duplicate_files

A recursive duplicate file finder utility written in Rust.

## Overview

This CLI utility was created to find files with duplicate names across a codebase. Originally designed for use with large Angular monorepos with multiple sub-projects that frequently had duplicate component/service names, it helps identify potential naming conflicts and overlaps.

## Installation

### From Source

```bash
git clone https://github.com/majdap/rust-find-duplicate-files.git
cd rust-find-duplicate-files
cargo install --path .
```

## Usage

```bash
# Basic usage
rust_duplicate_files /path/to/search

# With custom regex pattern
rust_duplicate_files /path/to/search --pattern "\.component\.ts$"

# Excluding directories
rust_duplicate_files /path/to/search --exclude "node_modules|dist"

# Help
rust_duplicate_files --help

```

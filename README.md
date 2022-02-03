Commentator is a fucking fast source code comments finder CLI and Rust SDK (crate).

**work in progress.**

[![Check](https://github.com/g4s8/commentator/actions/workflows/check.yml/badge.svg)](https://github.com/g4s8/commentator/actions/workflows/check.yml)
[![GitHub release (latest by date)](https://img.shields.io/github/downloads/g4s8/commentator/latest/total?label=download%40latest)](https://github.com/g4s8/commentator/releases/latest)
[![Crates.io (latest)](https://img.shields.io/crates/dv/commentator?label=crates.io%40latest)](https://crates.io/crates/commentator)

# Motivation

Existing source code comments extractors (see References) forcomments extracting are quite slow,
not always accurate (don't find all comments) or doesn't provide
SDK. This tool fixes all of this.

# Usage

 - Get crate: [crates.io/crates/commentator](https://crates.io/crates/commentator)
 - Get CLI: [releases@latest](https://github.com/g4s8/commentator/releases/latest)

This library could be used as CLI or from code.

To build CLI from sources (you need Rust and Cargo installed):
```bash
# clone repo
git clone https://github.com/g4s8/commentator.git
cd commentator
# build with cargo
cargo build --release --bin commentator --features feat-bin
# move binary to your $PATH
sudo mv ./target/release/commentator /usr/local/bin
```

Or download from release pages: https://github.com/g4s8/commentator/releases/tag/0.1.0

## CLI usage

`commentator` require file name argument and supports these options:
 - `--format` - output format: either `plain` or `json`
 - `--lang` - language comment specification, one of:
   - `c`,`java`, `go`, `cpp` - for C-like comment syntax
   - `rust` - Rust comments syntax
   - `bash` - for Bash, Python and Ruby
   - `html` - for HTML, XML
 - `--trim` - trim comment symbols and whitespaces, align to the first
   sentence indent.

Example:
```bash
./commentator --format=json --lang=java filename.java
```

## SDK usage

TODO: SDK usage

# References

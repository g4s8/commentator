Commentator is a fucking fast source code comments finder CLI and Rust SDK (crate).

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

SDK allows you to find, parse and trim comments from source code files. It's designed to be performance and memory-effecient:
you can push source code to tokenizer line by line, and take parsed comment after each push operation, when you finish with tokenizer
you need to notify about the end of file.

See documentation for more details: https://docs.rs/commentator/0.2.3/commentator/

Example:
```rust
let mut t = Tokenizer::new(&spec::StandardSpec::C);
t.update(1, "/*\n");
t.update(2, " * Entry point.\n");
t.update(3, " */\n");
t.update(4, "public static void main(String... args) {\n");
t.update(5, "  System.out.println(\"hello world\");\n");
t.update(6, "}\n");
t.finish();
let mut cmt = t.take().unwrap();
cmt.trim(&spec::StandardSpec::C);
assert_eq!(cmt.text, "Entry point.");
assert!(t.take().is_none());
```

# References

 - [github.com/jonschlinkert/extract-comments](https://github.com/jonschlinkert/extract-comments) - supports only JavaScript, not a binary, not fast.
 - [github.com/nknapp/multilang-extract-comments](https://github.com/nknapp/multilang-extract-comments) - no all comment cases could be extracted (didn't find all comments in test files `./test-files`), not a binary tool (require `npm` and `node` to run), not fast.
 - [tree-sitter.github.io/tree-sitter](https://tree-sitter.github.io/tree-sitter/) -too complex for this case, doesn't have a binary CLI.
 - (feel free to submit other tools)

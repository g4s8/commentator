Commentator is a fucking fast source code comments finder CLI and Rust SDK (crate).

**work in progress.**

TODO: badges

# Motivation

Existing source code comments extractors (see References) forcomments extracting are quite slow,
not always accurate (don't find all comments) or doesn't provide
SDK. This tool fixes all of this.

# Usage

This library could be used as CLI or from code.

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

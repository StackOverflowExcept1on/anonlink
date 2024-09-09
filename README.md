# anonlink

[![Build Status](https://github.com/StackOverflowExcept1on/anonlink/actions/workflows/ci.yml/badge.svg)](https://github.com/StackOverflowExcept1on/anonlink/actions/workflows/ci.yml)
[![Latest Version](https://img.shields.io/crates/v/anonlink.svg)](https://crates.io/crates/anonlink)

Command line tool to automatically patch link.exe to remove Rich PE Header

### Why?

TL;DR: when you building exe file with MSVC toolchain, Microsoft leaks some info about your development tools, such as
version of compiler and number of C/C++ source files

For more advanced
users: [The Undocumented Microsoft "Rich" Header](https://bytepointer.com/articles/the_microsoft_rich_header.htm)


### Installing from [crates.io](https://crates.io/crates/anonlink)

```bat
cargo install anonlink
```

### Building

```bat
cargo build --release
```

### Usage

```bat
:: run as administrator!
cargo run --release
```

```
linker path: C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64\link.exe
Found call instruction at address 140051A96
Found add instruction at address 140051AAA
Patching bytes [03, CF] => [90, 90]
```

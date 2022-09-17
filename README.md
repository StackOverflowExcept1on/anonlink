# anonlink

[![Build Status](https://github.com/StackOverflowExcept1on/anonlink/actions/workflows/main.yml/badge.svg)](https://github.com/StackOverflowExcept1on/anonlink/actions/workflows/main.yml)
[![Latest Version](https://img.shields.io/crates/v/anonlink.svg)](https://crates.io/crates/anonlink)

Command line tool to automatically patch link.exe to remove Rich PE Header

### Why?

TL;DR: when you building exe file with MSVC toolchain, Microsoft leaks some info about your development tools, such as
version of compiler and number of C/C++ source files

For more advanced
users: [The Undocumented Microsoft "Rich" Header](https://bytepointer.com/articles/the_microsoft_rich_header.htm)

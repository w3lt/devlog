---
title: Installation
description: Install devlog from crates.io or from a source checkout.
---

## From crates.io

```bash
cargo install d3vlog --locked
```

The crate is published as **`d3vlog`**, but it installs a binary named
**`devlog`**.

## From source

```bash
git clone https://github.com/w3lt/devlog
cd devlog
cargo install --path .
```

## Requirements

You need a Rust toolchain new enough for the 2024 edition, Rust **1.85+**, and a
C compiler.

`devlog` uses [`rusqlite`](https://crates.io/crates/rusqlite), which compiles a
bundled copy of SQLite, so you do **not** need SQLite installed on your system.

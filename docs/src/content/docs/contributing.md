---
title: Contributing
description: How to contribute to devlog and where to find the license.
---

Issues and pull requests are welcome.

```bash
git clone https://github.com/w3lt/devlog
cd devlog
cargo build
cargo run -- list
```

Please keep changes small and focused. Before opening a pull request, run:

```bash
cargo fmt
cargo clippy
```

## License

`devlog` is licensed under the **Apache License, Version 2.0**. See
[`LICENSE`](https://github.com/w3lt/devlog/blob/main/LICENSE) or the
[Apache License 2.0 text](https://www.apache.org/licenses/LICENSE-2.0) for the
full text.

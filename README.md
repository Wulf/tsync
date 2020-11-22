# cargo-tsync

[![Repository](https://img.shields.io/badge/Repository-Wulf%2Fcargo--tsync-333?style=for-the-badge)](https://github.com/Wulf/cargo-tsync)

[![License: MIT OR Apache-2.0](https://img.shields.io/crates/v/cargo-tsync.svg?style=for-the-badge)](https://crates.io/crates/cargo-tsync)

A utility to generate types for other typed languages.

Currently, only typescript is supported.


# Install

```
cargo install cargo-tsync
```

# Usage

Mark structs with `#[tsync]` as below:

```rs
/// src/main.rs

#[tsync]
struct Book {
  name: String,
  chapters: Vec<Chapter>
}

#[tsync]
struct Chapter {
  title: String,
  pages: u32
}
```

Then use the tool:

```sh
cargo tsync -i **/*.rs -o types.d.ts
```

And voilà!

```ts
/// types.d.ts

interface Book {
  name: string
  chapters: Array<Chapter>
}

interface Chapter {
  title: string
  pages: number
}
```

# Docs

See `cargo tsync --help` for more information.

Feel free to open tickets for support or feature requests.

# License

This tool is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
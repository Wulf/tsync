# tsync

<a href="https://crates.io/crates/tsync"><img src="https://img.shields.io/crates/v/tsync.svg?style=for-the-badge" height="20" alt="License: MIT OR Apache-2.0" /></a>

A utility to generate types for other typed languages.

Currently, only typescript is supported.


# Install

There are two parts to this:

1. The CLI tool:

   ```
   cargo install tsync
   ```

2. The dependency for rust projects (to use the `#[tsync]` attribute; see usage below)

   ```
   /// Cargo.toml
 
   tsync = "X.Y.Z"
   ```

# Usage

Mark structs with `#[tsync]` as below:

```rust
/// rust_file.rs
use tsync::tsync;

/// Doc comments are preserved too!
#[tsync]
struct Book {
  name: String,
  chapters: Vec<Chapter>,
  user_reviews: Option<Vec<String>>
}

#[tsync]
struct Chapter {
  title: String,
  pages: u32
}


#[tsync]
/// Time in UTC seconds
type UTC = usize;
```

Then use the CLI tool:

```sh
tsync -i rust_file.rs -o types.d.ts
```

And voilà!

```ts
/// types.d.ts

/* This file is generated and managed by tsync */

// Doc comments are preserved too!
interface Book {
  name: string
  chapters: Array<Chapter>
  user_reviews: Array<string> | undefined
}

interface Chapter {
  title: string
  pages: number
}

// Time in UTC seconds
type UTC = number

```

_**Note**: globs don't recurse on all platforms so try double or triple globbing!_
```sh
tsync -i **/*.rs -o types.d.ts
tsync -i **/**/*.rs -o types.d.ts
tsync -i **/**/**/*.rs -o types.d.ts
```

_**Note**: it might help to create multiple typing files for your project:_
```sh
tsync -i src/models/**/*.rs -o models.d.ts
tsync -i src/api/**/*.rs -o api.d.ts
```

# Errors

A list of files which can't be opened or parsed successfully are listed after executing `tsync`. For other errors, using the `--debug` flag may help find errors.

# Docs

See `tsync --help` for more information.

Feel free to open tickets for support or feature requests.

# License

This tool is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.

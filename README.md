# tsync

<a href="https://crates.io/crates/tsync"><img src="https://img.shields.io/crates/v/tsync.svg?style=for-the-badge" height="20" alt="License: MIT OR Apache-2.0" /></a>

A utility to generate typescript types from rust code.

# Install

There are two parts to this:

1. The CLI tool (or see "Usage as a library"):

   ```
   cargo install tsync
   ```

2. The dependency for rust projects (to use the `#[tsync]` attribute; see usage below)

   ```
   cargo add tsync@1
   ```

# Usage

Mark structs with `#[tsync]` as below:

```rust
/// src/main.rs
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
tsync -i ./src -o types.d.ts
```

And voilà!

```ts
/// types.d.ts

/* This file is generated and managed by tsync */

// Doc comments are preserved too!
interface Book {
  name: string
  chapters: Array<Chapter>
  user_reviews?: Array<string>
}

interface Chapter {
  title: string
  pages: number
}

// Time in UTC seconds
type UTC = number
```

**Supported Conversions & Examples**

| Rust code with `#[tsync]`                                                                               | Typescript output                                                                                                                                            |
|---------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`struct`](./test/struct/rust.rs)                                                                       | [.d.ts file](./test/struct/typescript.d.ts) or .ts file                                                                                                      |
| [`type`](./test/type/rust.rs)                                                                           | [.d.ts file](./test/type/typescript.d.ts) or .ts file                                                                                                        |
| [`enum`](./test/enum/rust.rs)                                                                           | [.d.ts file](./test/enum/typescript.d.ts) or [.ts file](./test/enum/typescript.ts)                                                                           |
| [`const`](./test/const/rust.rs) (or [limited](https://github.com/Wulf/tsync/issues/10) `json!` support) | [.ts file](./test/const/typescript.ts)<br>**Note: if you specify a `.d.ts` extension for your output, rust `const`s with `#[tsync]` are ignored**            |


## Multiple Inputs

You can specify many inputs (directories and/or files) using the `-i` flag multiple times, like so:

```sh
tsync -i directory1 -i directory2 -o types.d.ts
```

## Multiple Outputs

It might help to create multiple typing files for your project. It's easy, just call tsync multiple times:

```sh
tsync -i src/models -o models.d.ts
tsync -i src/api -o api.d.ts
```

# Usage as a library

In the case that installing `tsync` globally isn't an option (or causes other concerns), you can use it as a library.

1. Add the library to your project:

   ```sh
   cargo add tsync@1
   ```

2. Create a new binary in your project which uses the crate (for example, `bin/tsync.rs`):
   
   ```rust
   // bin/tsync.rs

   use std::path::PathBuf;
   
   pub fn main() {
   let dir = env!("CARGO_MANIFEST_DIR");
   
       let inputs = vec![PathBuf::from_iter([dir, "backend"])];
       let output = PathBuf::from_iter([dir, "frontend/src/types/rust.d.ts"]);
   
       tsync::generate_typescript_defs(inputs, output, false);
   }
   ```

3. Create a `Cargo.toml` binary entry:
   
   ```toml
   [[bin]]
   name = "tsync"
   path = "bin/tsync.rs"
   ```

4. Execute!

   ```sh
   cargo run --bin tsync
   ```

**Protip**: to use `cargo tsync`, create an alias in `.cargo/config`:

   ```toml
   [alias]
   tsync="run --bin tsync"
   ```

# Errors

A list of files which can't be opened or parsed successfully are listed after executing `tsync`. For other errors, try using the `--debug` flag to pinpoint issues. Please use the Github issue tracker to report any issues.

# Docs

See `tsync --help` for more information.

Feel free to open tickets for support or feature requests.

# Development/Testing

Use `./test/test_all.sh` to run tests.
After running the test, there should be no unexpected changes to files in `./test` (use `git status` and `git diff` to see if there were any changes).

# License

This tool is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.

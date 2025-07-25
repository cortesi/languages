![Discord](https://img.shields.io/discord/1381424110831145070?style=flat-square&logo=rust&link=https%3A%2F%2Fdiscord.gg%2FfHmRmuBDxF)
[![Crates.io](https://img.shields.io/crates/v/languages)](https://crates.io/crates/languages)
[![docs.rs](https://img.shields.io/docsrs/languages)](https://docs.rs/languages)

# languages


> GitHub's language data, compiled into a tiny, fast Rust library. 🦀

This crate provides an efficient way to look up language information from
GitHub's Linguist `languages.yml` file. The data is parsed at compile-time and
baked directly into your binary, making lookups instantaneous with zero runtime
overhead.

-----

## Features

  - **Fast**: All data is stored in static `HashMap`s for instant, case-insensitive lookups.
  - **Simple API**: Get language info by name, alias, extension, or CodeMirror mode.
  - **Self-Contained**: No need to read files or parse YAML at runtime.

-----

## Quickstart

1.  Add `languages` to your `Cargo.toml`:

    ```toml
    [dependencies]
    languages = "0.1.0" # Replace with the latest version
    ```

2.  Use the lookup functions:

    <!-- snips: examples/readme.rs#example -->
    ```rust
    let lang = languages::from_extension("rs").unwrap();
    assert_eq!(lang.name, "Rust");
    assert_eq!(lang.language_type, "programming");
    assert_eq!(lang.color, Some("#dea584"));

    // Look up by name (case-insensitive)
    let python = languages::from_name("Python").unwrap();
    assert!(python.extensions.unwrap().contains(&".py"));

    // Look up by alias
    let cpp = languages::from_name("cpp").unwrap();
    assert_eq!(cpp.name, "C++");
    ```

-----

## How It Works

This crate uses a `build.rs` script that parses `languages.yml` and generates
the necessary Rust code. To update the language data to the latest version from
GitHub, simply run the `download_languages.sh` script and recompile your
project.

# Related Projects

This library was written to be used in the
[snips](https://github.com/cortesi/snips) tool, which is also used to maintain
the code examples in this README.


# License and Acknowledgements

The code for the languages crate is licensed under the MIT License.

The language data is sourced from the [GitHub
Linguist](https://github.com/github-linguist/linguist) project, which is
distributed under the MIT license and is copyright of GitHub, Inc.

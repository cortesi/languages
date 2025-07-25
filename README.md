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

    ```rust
    // Look up by extension
    let lang = languages::from_extension("rs").unwrap();
    assert_eq!(lang.name, "Rust");
    assert_eq!(lang.language_type, "programming");
    assert_eq!(lang.color, Some("#dea584"));

    // Look up by name (case-insensitive)
    let python = languages::from_name("Python").unwrap();
    assert_eq!(python.extensions, Some(&[".py", ".pyw", ".pyi", ".ipynb"]));

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

fn main() {
    // snips-start: example
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
    // snips-end: example
}

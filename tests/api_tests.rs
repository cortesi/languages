#[test]
fn test_from_name_basic() {
    let lang = languages::from_name("Rust").expect("Could not find Rust");
    assert_eq!(lang.name, "Rust");
    assert_eq!(lang.language_type, "programming");
}

#[test]
fn test_from_name_case_insensitive() {
    let lang1 = languages::from_name("python").expect("Could not find python");
    let lang2 = languages::from_name("Python").expect("Could not find Python");
    let lang3 = languages::from_name("PYTHON").expect("Could not find PYTHON");
    assert_eq!(lang1.language_id, lang2.language_id);
    assert_eq!(lang2.language_id, lang3.language_id);
}

#[test]
fn test_from_name_alias() {
    let lang = languages::from_name("cpp").expect("Could not find C++ by alias");
    assert_eq!(lang.name, "C++");
}

#[test]
fn test_from_extension() {
    let lang = languages::from_extension("rs").expect("Could not find by extension 'rs'");
    assert_eq!(lang.name, "Rust");
}

#[test]
fn test_from_extension_case_insensitive() {
    let lang1 = languages::from_extension("Py").expect("Could not find by extension 'Py'");
    let lang2 = languages::from_extension("py").expect("Could not find by extension 'py'");
    assert_eq!(lang1.name, "Python");
    assert_eq!(lang2.name, "Python");
}

#[test]
fn test_from_codemirror_mode() {
    let lang = languages::from_codemirror_mode("javascript").expect("Could not find by codemirror mode");
    assert_eq!(lang.name, "JavaScript");
}

#[test]
fn test_not_found() {
    assert!(languages::from_name("NotALang").is_none());
    assert!(languages::from_extension("notanext").is_none());
    assert!(languages::from_codemirror_mode("notamode").is_none());
}

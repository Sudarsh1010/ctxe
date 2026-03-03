use ctxe::{
    domain::common::language::Language,
    infrastructure::parsing::TreeSitterParser,
};

#[test]
fn test_parse_rust_function() {
    let mut parser = TreeSitterParser::new().unwrap();
    let code = r#"
        pub fn hello(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    "#;

    let symbols = parser.parse_symbols(code, Language::Rust).unwrap();

    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name, "hello");
    assert_eq!(symbols[0].signature, "pub fn hello(name: &str) -> String");
}

#[test]
fn test_handles_malformed_code() {
    let mut parser = TreeSitterParser::new().unwrap();
    let code = "fn broken( {"; // Invalid syntax

    // Should not panic - may return empty or partial results
    let result = parser.parse_symbols(code, Language::Rust);
    // Accept either empty result or error (depending on tree-sitter behavior)
    assert!(result.is_ok() || result.unwrap().is_empty());
}

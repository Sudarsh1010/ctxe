use ctxe::{
    domain::code::service::compressor::{AstCompressor, CompressionLevel},
    domain::common::language::Language,
    infrastructure::parsing::TreeSitterParser,
};

#[test]
fn test_compress_signatures_only() {
    let mut parser = TreeSitterParser::new().unwrap();
    let compressor = AstCompressor;

    let code = r#"
        /// Adds two numbers
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        /// Subtracts two numbers
        pub fn sub(a: i32, b: i32) -> i32 {
            a - b
        }
    "#;

    let symbols = parser.parse_symbols(code, Language::Rust).unwrap();
    let result = compressor
        .compress(code, &symbols, CompressionLevel::SignaturesOnly, None)
        .unwrap();

    assert!(
        result
            .compressed
            .contains("pub fn add(a: i32, b: i32) -> i32")
    );
    assert!(
        result
            .compressed
            .contains("pub fn sub(a: i32, b: i32) -> i32")
    );
    assert!(!result.compressed.contains("a + b")); // Body removed
    assert!(result.reduction_percent > 50.0); // Should achieve >50% reduction
}

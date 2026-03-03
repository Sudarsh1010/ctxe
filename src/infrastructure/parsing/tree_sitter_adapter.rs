use std::collections::HashMap;

use tree_sitter::{Node, Parser as TsParser, Tree};

use crate::domain::{
    code::{
        service::parser::Parser as ParserTrait,
        value::symbol::{SourceSpan, Symbol, SymbolKind},
    },
    common::language::Language,
};

pub struct TreeSitterParser {
    parser: TsParser,
    languages: HashMap<Language, tree_sitter::Language>,
}

impl TreeSitterParser {
    pub fn new() -> crate::Result<Self> {
        let parser = TsParser::new();
        let mut languages = HashMap::new();

        languages.insert(Language::Rust, tree_sitter_rust::LANGUAGE.into());
        languages.insert(Language::Go, tree_sitter_go::LANGUAGE.into());
        languages.insert(
            Language::JavaScript,
            tree_sitter_javascript::LANGUAGE.into(),
        );
        languages.insert(
            Language::TypeScript,
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        );
        languages.insert(Language::Python, tree_sitter_python::LANGUAGE.into());

        Ok(Self { parser, languages })
    }

    pub fn parse_tree(
        &mut self,
        code: &str,
        language: Language,
    ) -> crate::Result<Tree> {
        let lang = self.languages.get(&language).ok_or_else(|| {
            crate::Error::UnsupportedLanguage(language.as_str().to_string())
        })?;

        self.parser.set_language(&lang).map_err(|e| {
            crate::Error::Parse(format!("Failed to set language: {e}"))
        })?;

        self.parser.parse(code, None).ok_or_else(|| {
            crate::Error::Parse("Failed to parse the code:".to_string())
        })
    }

    fn extract_symbols_from_tree(
        &self,
        tree: &Tree,
        code: &str,
        language: Language,
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        self.walk_node(tree.root_node(), code, language, &mut symbols);
        symbols
    }

    fn walk_node(
        &self,
        node: Node,
        code: &str,
        language: Language,
        symbols: &mut Vec<Symbol>,
    ) {
        if language == Language::Rust {
            match node.kind() {
                "function_item" => {
                    if let Some(symbol) =
                        self.extract_function(node, code, language)
                    {
                        symbols.push(symbol)
                    }
                }

                "struct_item" => {
                    if let Some(symbol) =
                        self.extract_struct(node, code, language)
                    {
                        symbols.push(symbol)
                    }
                }

                // TODO: add other kind, try to simply the extraction (keep this in mind, multi-lang support)
                _ => {}
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node(child, code, language, symbols);
        }
    }

    fn extract_function(
        &self,
        node: Node,
        code: &str,
        language: Language,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = &code[name_node.byte_range()];

        // Extract signature: from "fn" to end of parameteres/return type
        let signature = &code[node.byte_range()];

        let location = SourceSpan {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_line: node.start_position().row as u32,
            end_line: node.end_position().row as u32,
        };

        Some(Symbol::new(
            name.to_string(),
            SymbolKind::Function,
            signature.trim().to_string(),
            language,
            location,
        ))
    }

    fn extract_struct(
        &self,
        node: Node,
        code: &str,
        language: Language,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = &code[name_node.byte_range()];
        let signature = &code[node.byte_range()];

        let location = SourceSpan {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_line: node.start_position().row as u32,
            end_line: node.end_position().row as u32,
        };

        Some(Symbol::new(
            name.to_string(),
            SymbolKind::Struct,
            signature.trim().to_string(),
            language,
            location,
        ))
    }
}

impl ParserTrait for TreeSitterParser {
    fn parse_symbols(
        &mut self,
        code: &str,
        language: Language,
    ) -> crate::Result<Vec<Symbol>> {
        let tree = self.parse_tree(code, language)?;
        Ok(self.extract_symbols_from_tree(&tree, code, language))
    }

    fn extract_signature(
        &mut self,
        _code: &str,
        symbol: &Symbol,
    ) -> crate::Result<String> {
        // TODO:
        // for now return the signature we already extracted
        // needs to be researched more on this
        Ok(symbol.signature.clone())
    }
}

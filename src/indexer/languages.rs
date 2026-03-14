use crate::models::SymbolKind;
use std::path::Path;
use tree_sitter::Language;

/// Language information for tree-sitter parsing
#[derive(Debug, Clone)]
pub struct LangInfo {
    /// Language name (e.g., "Rust", "Python")
    pub name: &'static str,
    /// File extensions (e.g., ["rs"], ["py", "pyi"])
    pub extensions: &'static [&'static str],
    /// Lazy grammar loader function
    pub grammar: fn() -> Language,
    /// Supported SymbolKinds for this language
    pub symbol_kinds: &'static [SymbolKind],
    /// Tree-sitter node type -> SymbolKind mappings
    pub node_mappings: &'static [(&'static str, SymbolKind)],
}

impl LangInfo {
    /// Get the tree-sitter Language for this language
    pub fn get_grammar(&self) -> Language {
        (self.grammar)()
    }

    /// Check if this language supports a given SymbolKind
    pub fn supports_kind(&self, kind: SymbolKind) -> bool {
        self.symbol_kinds.contains(&kind)
    }

    /// Get SymbolKind for a tree-sitter node type
    pub fn node_to_kind(&self, node_type: &str) -> Option<SymbolKind> {
        self.node_mappings
            .iter()
            .find(|(node, _)| *node == node_type)
            .map(|(_, kind)| *kind)
    }
}

/// Macro to define all supported languages in one place
macro_rules! define_languages {
    (
        $(
            $name:ident => {
                ext: [$($ext:literal),*],
                grammar: $grammar_fn:expr,
                kinds: [$($kind:ident),*],
                nodes: { $($node:literal => $node_kind:ident),* }
            }
        ),* $(,)?
    ) => {
        /// All supported languages
        pub static LANGUAGES: &[LangInfo] = &[
            $(
                LangInfo {
                    name: stringify!($name),
                    extensions: &[$($ext),*],
                    grammar: $grammar_fn,
                    symbol_kinds: &[$(SymbolKind::$kind),*],
                    node_mappings: &[$(($node, SymbolKind::$node_kind)),*],
                }
            ),*
        ];

        /// Get language info by file extension (without the dot)
        pub fn from_extension(ext: &str) -> Option<&'static LangInfo> {
            let ext_lower = ext.to_lowercase();
            LANGUAGES.iter().find(|lang| {
                lang.extensions.iter().any(|e| e.to_lowercase() == ext_lower)
            })
        }

        /// Get language info from a file path
        pub fn from_path(path: &Path) -> Option<&'static LangInfo> {
            let ext = path.extension()?.to_str()?;
            from_extension(ext)
        }

        /// Get all supported language names
        pub fn all_language_names() -> &'static [&'static str] {
            &[$(stringify!($name)),*]
        }

        /// Get SymbolKind from language and node type
        pub fn get_symbol_kind(lang: &LangInfo, node_type: &str) -> Option<SymbolKind> {
            lang.node_to_kind(node_type)
        }
    };
}

// Define all 24 supported languages
define_languages! {
    Agda => {
        ext: ["agda"],
        grammar: || tree_sitter_agda::LANGUAGE.into(),
        kinds: [Function, Module],
        nodes: { "function" => Function, "module" => Module }
    },
    Bash => {
        ext: ["sh", "bash", "zsh"],
        grammar: || tree_sitter_bash::LANGUAGE.into(),
        kinds: [Function, Variable],
        nodes: { "function_definition" => Function, "variable_assignment" => Variable }
    },
    C => {
        ext: ["c", "h"],
        grammar: || tree_sitter_c::LANGUAGE.into(),
        kinds: [Function, Struct, Enum, Constant, TypeAlias],
        nodes: {
            "function_definition" => Function,
            "struct_specifier" => Struct,
            "enum_specifier" => Enum,
            "type_definition" => TypeAlias,
            "preproc_def" => Constant
        }
    },
    Cpp => {
        ext: ["cpp", "cc", "cxx", "hpp", "hxx"],
        grammar: || tree_sitter_cpp::LANGUAGE.into(),
        kinds: [Function, Struct, Enum, Class, Method, Constant, TypeAlias],
        nodes: {
            "function_definition" => Function,
            "struct_specifier" => Struct,
            "enum_specifier" => Enum,
            "class_specifier" => Class,
            "method_declaration" => Method,
            "type_definition" => TypeAlias
        }
    },
    CSharp => {
        ext: ["cs"],
        grammar: || tree_sitter_c_sharp::LANGUAGE.into(),
        kinds: [Function, Class, Struct, Enum, Interface, Method, Constant],
        nodes: {
            "method_declaration" => Function,
            "class_declaration" => Class,
            "struct_declaration" => Struct,
            "enum_declaration" => Enum,
            "interface_declaration" => Interface,
            "field_declaration" => Constant
        }
    },
    Css => {
        ext: ["css", "scss", "sass"],
        grammar: || tree_sitter_css::LANGUAGE.into(),
        kinds: [Constant],
        nodes: { "rule_set" => Constant }
    },
    EmbeddedTemplate => {
        ext: ["erb", "ejs"],
        grammar: || tree_sitter_embedded_template::LANGUAGE.into(),
        kinds: [],
        nodes: {}
    },
    Go => {
        ext: ["go"],
        grammar: || tree_sitter_go::LANGUAGE.into(),
        kinds: [Function, Struct, Interface, Method, Constant, TypeAlias],
        nodes: {
            "function_declaration" => Function,
            "method_declaration" => Method,
            "type_declaration" => Struct,
            "interface_type" => Interface,
            "const_declaration" => Constant,
            "type_spec" => TypeAlias
        }
    },
    Haskell => {
        ext: ["hs"],
        grammar: || tree_sitter_haskell::LANGUAGE.into(),
        kinds: [Function, Struct, TypeAlias],
        nodes: {
            "function" => Function,
            "data_type" => Struct,
            "type_synonym" => TypeAlias
        }
    },
    Html => {
        ext: ["html", "htm"],
        grammar: || tree_sitter_html::LANGUAGE.into(),
        kinds: [],
        nodes: {}
    },
    Java => {
        ext: ["java"],
        grammar: || tree_sitter_java::LANGUAGE.into(),
        kinds: [Function, Class, Interface, Enum, Method, Constant],
        nodes: {
            "method_declaration" => Function,
            "class_declaration" => Class,
            "interface_declaration" => Interface,
            "enum_declaration" => Enum,
            "field_declaration" => Constant
        }
    },
    JavaScript => {
        ext: ["js", "mjs", "cjs", "jsx"],
        grammar: || tree_sitter_javascript::LANGUAGE.into(),
        kinds: [Function, Class, Method, Variable, Constant],
        nodes: {
            "function_declaration" => Function,
            "function_expression" => Function,
            "arrow_function" => Function,
            "class_declaration" => Class,
            "method_definition" => Method,
            "variable_declaration" => Variable,
            "lexical_declaration" => Variable
        }
    },
    Jsdoc => {
        ext: ["jsdoc"],
        grammar: || tree_sitter_jsdoc::LANGUAGE.into(),
        kinds: [],
        nodes: {}
    },
    Json => {
        ext: ["json"],
        grammar: || tree_sitter_json::LANGUAGE.into(),
        kinds: [],
        nodes: {}
    },
    Julia => {
        ext: ["jl"],
        grammar: || tree_sitter_julia::LANGUAGE.into(),
        kinds: [Function, Struct, Module, Constant],
        nodes: {
            "function_definition" => Function,
            "struct_definition" => Struct,
            "module_definition" => Module,
            "const_definition" => Constant
        }
    },
    Ocaml => {
        ext: ["ml", "mli"],
        grammar: || tree_sitter_ocaml::LANGUAGE_OCAML.into(),
        kinds: [Function, Module, TypeAlias],
        nodes: {
            "value_definition" => Function,
            "module_definition" => Module,
            "type_definition" => TypeAlias
        }
    },
    Php => {
        ext: ["php"],
        grammar: || tree_sitter_php::LANGUAGE_PHP.into(),
        kinds: [Function, Class, Interface, Trait, Method, Constant],
        nodes: {
            "function_definition" => Function,
            "class_declaration" => Class,
            "interface_declaration" => Interface,
            "trait_declaration" => Trait,
            "method_declaration" => Method
        }
    },
    Python => {
        ext: ["py", "pyi"],
        grammar: || tree_sitter_python::LANGUAGE.into(),
        kinds: [Function, Class, Method, Import, Variable],
        nodes: {
            "function_definition" => Function,
            "class_definition" => Class,
            "import_statement" => Import,
            "import_from_statement" => Import,
            "assignment" => Variable
        }
    },
    Regex => {
        ext: ["regex"],
        grammar: || tree_sitter_regex::LANGUAGE.into(),
        kinds: [],
        nodes: {}
    },
    Ruby => {
        ext: ["rb", "rake"],
        grammar: || tree_sitter_ruby::LANGUAGE.into(),
        kinds: [Function, Class, Module, Method, Constant],
        nodes: {
            "method" => Function,
            "class" => Class,
            "module" => Module,
            "singleton_method" => Method,
            "constant" => Constant
        }
    },
    Rust => {
        ext: ["rs"],
        grammar: || tree_sitter_rust::LANGUAGE.into(),
        kinds: [Function, Struct, Enum, Trait, Impl, Module, Constant, TypeAlias],
        nodes: {
            "function_item" => Function,
            "struct_item" => Struct,
            "enum_item" => Enum,
            "trait_item" => Trait,
            "impl_item" => Impl,
            "mod_item" => Module,
            "const_item" => Constant,
            "type_item" => TypeAlias
        }
    },
    Scala => {
        ext: ["scala", "sc"],
        grammar: || tree_sitter_scala::LANGUAGE.into(),
        kinds: [Function, Class, Trait, Constant, Method],
        nodes: {
            "function_definition" => Function,
            "class_definition" => Class,
            "trait_definition" => Trait,
            "object_definition" => Constant,
            "method_definition" => Method
        }
    },
    TypeScript => {
        ext: ["ts"],
        grammar: || tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        kinds: [Function, Class, Interface, Method, TypeAlias, Variable],
        nodes: {
            "function_declaration" => Function,
            "class_declaration" => Class,
            "interface_declaration" => Interface,
            "method_definition" => Method,
            "type_alias_declaration" => TypeAlias,
            "lexical_declaration" => Variable
        }
    },
    Tsx => {
        ext: ["tsx"],
        grammar: || tree_sitter_typescript::LANGUAGE_TSX.into(),
        kinds: [Function, Class, Interface, Method, TypeAlias, Variable],
        nodes: {
            "function_declaration" => Function,
            "class_declaration" => Class,
            "interface_declaration" => Interface,
            "method_definition" => Method,
            "type_alias_declaration" => TypeAlias,
            "lexical_declaration" => Variable
        }
    },
    Verilog => {
        ext: ["v", "vh"],
        grammar: || tree_sitter_verilog::LANGUAGE.into(),
        kinds: [Function, Module],
        nodes: {
            "module_declaration" => Module,
            "function_declaration" => Function
        }
    },
}

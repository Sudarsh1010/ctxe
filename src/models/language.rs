use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Agda,
    Bash,
    C,
    Cpp,
    CSharp,
    Css,
    EmbeddedTemplate,
    Go,
    Haskell,
    Html,
    Java,
    JavaScript,
    Jsdoc,
    Json,
    Julia,
    Ocaml,
    Php,
    Python,
    Regex,
    Ruby,
    Rust,
    Scala,
    TypeScript,
    Tsx,
    Verilog,
    Unknown,
}

impl Language {
    pub fn from_extension(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        Self::from_path(ext.to_lowercase().as_str())
    }

    pub fn from_path(path: &str) -> Self {
        match path.rsplit(".").next() {
            Some("agda") => Language::Agda,
            Some("sh") | Some("bash") | Some("zsh") => Language::Bash,
            Some("c") | Some("h") => Language::C,
            Some("cpp") | Some("cc") | Some("cxx") | Some("hpp")
            | Some("hxx") => Language::Cpp,
            Some("cs") => Language::CSharp,
            Some("css") | Some("scss") | Some("sass") => Language::Css,
            Some("erb") | Some("ejs") => Language::EmbeddedTemplate,
            Some("go") => Language::Go,
            Some("hs") => Language::Haskell,
            Some("html") | Some("htm") => Language::Html,
            Some("java") => Language::Java,
            Some("js") | Some("mjs") | Some("cjs") | Some("jsx") => {
                Language::JavaScript
            }
            Some("jsdoc") => Language::Jsdoc,
            Some("json") => Language::Json,
            Some("jl") => Language::Julia,
            Some("ml") | Some("mli") => Language::Ocaml,
            Some("php") => Language::Php,
            Some("py") | Some("pyi") => Language::Python,
            Some("regex") => Language::Regex,
            Some("rb") | Some("rake") => Language::Ruby,
            Some("rs") => Language::Rust,
            Some("scala") | Some("sc") => Language::Scala,
            Some("ts") => Language::TypeScript,
            Some("tsx") => Language::Tsx,
            Some("v") | Some("vh") => Language::Verilog,
            _ => Language::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Agda => "agda",
            Self::Bash => "bash",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::CSharp => "csharp",
            Self::Css => "css",
            Self::EmbeddedTemplate => "embedded_template",
            Self::Go => "go",
            Self::Haskell => "haskell",
            Self::Html => "html",
            Self::Java => "java",
            Self::JavaScript => "javascript",
            Self::Jsdoc => "jsdoc",
            Self::Json => "json",
            Self::Julia => "julia",
            Self::Ocaml => "ocaml",
            Self::Php => "php",
            Self::Python => "python",
            Self::Regex => "regex",
            Self::Ruby => "ruby",
            Self::Rust => "rust",
            Self::Scala => "scala",
            Self::TypeScript => "typescript",
            Self::Tsx => "tsx",
            Self::Verilog => "verilog",
            Self::Unknown => "unknown",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_agda_extensions() {
        assert_eq!(
            Language::from_path("agda"),
            Language::Agda,
            "Agda should be mapped from .agda extension"
        );
        assert_eq!(Language::from_path("test.agda"), Language::Agda);
    }

    #[test]
    fn test_bash_extensions() {
        assert_eq!(
            Language::from_path("sh"),
            Language::Bash,
            "Bash should be mapped from .sh extension"
        );
        assert_eq!(
            Language::from_path("bash"),
            Language::Bash,
            "Bash should be mapped from .bash extension"
        );
        assert_eq!(
            Language::from_path("zsh"),
            Language::Bash,
            "Bash should be mapped from .zsh extension"
        );
    }

    #[test]
    fn test_c_extensions() {
        assert_eq!(
            Language::from_path("c"),
            Language::C,
            "C should be mapped from .c extension"
        );
        assert_eq!(
            Language::from_path("h"),
            Language::C,
            "C should be mapped from .h header extension"
        );
    }

    #[test]
    fn test_cpp_extensions() {
        assert_eq!(
            Language::from_path("cpp"),
            Language::Cpp,
            "Cpp should be mapped from .cpp extension"
        );
        assert_eq!(
            Language::from_path("cc"),
            Language::Cpp,
            "Cpp should be mapped from .cc extension"
        );
        assert_eq!(
            Language::from_path("cxx"),
            Language::Cpp,
            "Cpp should be mapped from .cxx extension"
        );
        assert_eq!(
            Language::from_path("hpp"),
            Language::Cpp,
            "Cpp should be mapped from .hpp header extension"
        );
        assert_eq!(
            Language::from_path("hxx"),
            Language::Cpp,
            "Cpp should be mapped from .hxx header extension"
        );
    }

    #[test]
    fn test_csharp_extensions() {
        assert_eq!(
            Language::from_path("cs"),
            Language::CSharp,
            "CSharp should be mapped from .cs extension"
        );
    }

    #[test]
    fn test_css_extensions() {
        assert_eq!(
            Language::from_path("css"),
            Language::Css,
            "Css should be mapped from .css extension"
        );
        assert_eq!(
            Language::from_path("scss"),
            Language::Css,
            "Css should be mapped from .scss extension"
        );
        assert_eq!(
            Language::from_path("sass"),
            Language::Css,
            "Css should be mapped from .sass extension"
        );
    }

    #[test]
    fn test_embedded_template_extensions() {
        assert_eq!(
            Language::from_path("erb"),
            Language::EmbeddedTemplate,
            "EmbeddedTemplate should be mapped from .erb extension"
        );
        assert_eq!(
            Language::from_path("ejs"),
            Language::EmbeddedTemplate,
            "EmbeddedTemplate should be mapped from .ejs extension"
        );
    }

    #[test]
    fn test_go_extensions() {
        assert_eq!(
            Language::from_path("go"),
            Language::Go,
            "Go should be mapped from .go extension"
        );
    }

    #[test]
    fn test_haskell_extensions() {
        assert_eq!(
            Language::from_path("hs"),
            Language::Haskell,
            "Haskell should be mapped from .hs extension"
        );
    }

    #[test]
    fn test_html_extensions() {
        assert_eq!(
            Language::from_path("html"),
            Language::Html,
            "Html should be mapped from .html extension"
        );
        assert_eq!(
            Language::from_path("htm"),
            Language::Html,
            "Html should be mapped from .htm extension"
        );
    }

    #[test]
    fn test_java_extensions() {
        assert_eq!(
            Language::from_path("java"),
            Language::Java,
            "Java should be mapped from .java extension"
        );
    }

    #[test]
    fn test_javascript_extensions() {
        assert_eq!(
            Language::from_path("js"),
            Language::JavaScript,
            "JavaScript should be mapped from .js extension"
        );
        assert_eq!(
            Language::from_path("mjs"),
            Language::JavaScript,
            "JavaScript should be mapped from .mjs extension"
        );
        assert_eq!(
            Language::from_path("cjs"),
            Language::JavaScript,
            "JavaScript should be mapped from .cjs extension"
        );
        assert_eq!(
            Language::from_path("jsx"),
            Language::JavaScript,
            "JavaScript should be mapped from .jsx extension"
        );
    }

    #[test]
    fn test_jsdoc_extensions() {
        assert_eq!(
            Language::from_path("jsdoc"),
            Language::Jsdoc,
            "Jsdoc should be mapped from .jsdoc extension"
        );
    }

    #[test]
    fn test_json_extensions() {
        assert_eq!(
            Language::from_path("json"),
            Language::Json,
            "Json should be mapped from .json extension"
        );
    }

    #[test]
    fn test_julia_extensions() {
        assert_eq!(
            Language::from_path("jl"),
            Language::Julia,
            "Julia should be mapped from .jl extension"
        );
    }

    #[test]
    fn test_ocaml_extensions() {
        assert_eq!(
            Language::from_path("ml"),
            Language::Ocaml,
            "Ocaml should be mapped from .ml extension"
        );
        assert_eq!(
            Language::from_path("mli"),
            Language::Ocaml,
            "Ocaml should be mapped from .mli header extension"
        );
    }

    #[test]
    fn test_php_extensions() {
        assert_eq!(
            Language::from_path("php"),
            Language::Php,
            "Php should be mapped from .php extension"
        );
    }

    #[test]
    fn test_python_extensions() {
        assert_eq!(
            Language::from_path("py"),
            Language::Python,
            "Python should be mapped from .py extension"
        );
        assert_eq!(
            Language::from_path("pyi"),
            Language::Python,
            "Python should be mapped from .pyi stub extension"
        );
    }

    #[test]
    fn test_regex_extensions() {
        assert_eq!(
            Language::from_path("regex"),
            Language::Regex,
            "Regex should be mapped from .regex extension"
        );
    }

    #[test]
    fn test_ruby_extensions() {
        assert_eq!(
            Language::from_path("rb"),
            Language::Ruby,
            "Ruby should be mapped from .rb extension"
        );
        assert_eq!(
            Language::from_path("rake"),
            Language::Ruby,
            "Ruby should be mapped from .rake extension"
        );
    }

    #[test]
    fn test_rust_extensions() {
        assert_eq!(
            Language::from_path("rs"),
            Language::Rust,
            "Rust should be mapped from .rs extension"
        );
    }

    #[test]
    fn test_scala_extensions() {
        assert_eq!(
            Language::from_path("scala"),
            Language::Scala,
            "Scala should be mapped from .scala extension"
        );
        assert_eq!(
            Language::from_path("sc"),
            Language::Scala,
            "Scala should be mapped from .sc extension"
        );
    }

    #[test]
    fn test_typescript_extensions() {
        assert_eq!(
            Language::from_path("ts"),
            Language::TypeScript,
            "TypeScript should be mapped from .ts extension"
        );
    }

    #[test]
    fn test_tsx_extensions() {
        assert_eq!(
            Language::from_path("tsx"),
            Language::Tsx,
            "Tsx should be mapped from .tsx extension"
        );
    }

    #[test]
    fn test_verilog_extensions() {
        assert_eq!(
            Language::from_path("v"),
            Language::Verilog,
            "Verilog should be mapped from .v extension"
        );
        assert_eq!(
            Language::from_path("vh"),
            Language::Verilog,
            "Verilog should be mapped from .vh header extension"
        );
    }

    #[test]
    fn test_from_extension() {
        assert_eq!(
            Language::from_extension(Path::new("test.rs")),
            Language::Rust
        );
        assert_eq!(
            Language::from_extension(Path::new("test.py")),
            Language::Python
        );
        assert_eq!(
            Language::from_extension(Path::new("test.ts")),
            Language::TypeScript
        );
        assert_eq!(
            Language::from_extension(Path::new("test.js")),
            Language::JavaScript
        );
    }

    #[test]
    fn test_as_str_returns_correct_strings() {
        assert_eq!(Language::Agda.as_str(), "agda");
        assert_eq!(Language::Bash.as_str(), "bash");
        assert_eq!(Language::C.as_str(), "c");
        assert_eq!(Language::Cpp.as_str(), "cpp");
        assert_eq!(Language::CSharp.as_str(), "csharp");
        assert_eq!(Language::Css.as_str(), "css");
        assert_eq!(Language::EmbeddedTemplate.as_str(), "embedded_template");
        assert_eq!(Language::Go.as_str(), "go");
        assert_eq!(Language::Haskell.as_str(), "haskell");
        assert_eq!(Language::Html.as_str(), "html");
        assert_eq!(Language::Java.as_str(), "java");
        assert_eq!(Language::JavaScript.as_str(), "javascript");
        assert_eq!(Language::Jsdoc.as_str(), "jsdoc");
        assert_eq!(Language::Json.as_str(), "json");
        assert_eq!(Language::Julia.as_str(), "julia");
        assert_eq!(Language::Ocaml.as_str(), "ocaml");
        assert_eq!(Language::Php.as_str(), "php");
        assert_eq!(Language::Python.as_str(), "python");
        assert_eq!(Language::Regex.as_str(), "regex");
        assert_eq!(Language::Ruby.as_str(), "ruby");
        assert_eq!(Language::Rust.as_str(), "rust");
        assert_eq!(Language::Scala.as_str(), "scala");
        assert_eq!(Language::TypeScript.as_str(), "typescript");
        assert_eq!(Language::Tsx.as_str(), "tsx");
        assert_eq!(Language::Verilog.as_str(), "verilog");
        assert_eq!(Language::Unknown.as_str(), "unknown");
    }
}

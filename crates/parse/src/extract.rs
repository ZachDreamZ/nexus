use core::types::Language;

pub fn extract_imports(content: &str, language: &Language) -> Vec<String> {
    match language {
        Language::Rust => extract_rust_imports(content),
        Language::Python => extract_python_imports(content),
        Language::JavaScript | Language::TypeScript => extract_js_imports(content),
        Language::Go => extract_go_imports(content),
        Language::Other(_) => Vec::new(),
    }
}

fn extract_rust_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let import = trimmed
                .trim_start_matches("use ")
                .trim_end_matches(';')
                .trim();
            if let Some(path) = import.split(" as ").next() {
                imports.push(path.trim().to_string());
            } else {
                imports.push(import.to_string());
            }
        } else if trimmed.starts_with("mod ") && !trimmed.contains(';') {
            continue;
        } else if trimmed.starts_with("pub use ") {
            let import = trimmed
                .trim_start_matches("pub use ")
                .trim_end_matches(';')
                .trim();
            if let Some(path) = import.split(" as ").next() {
                imports.push(path.trim().to_string());
            } else {
                imports.push(import.to_string());
            }
        }
    }
    imports
}

fn extract_python_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") {
            let rest = trimmed.trim_start_matches("import ");
            for part in rest.split(',') {
                let module = part.trim().split(" as ").next().unwrap_or(part).trim();
                imports.push(module.to_string());
            }
        } else if trimmed.starts_with("from ") {
            let rest = trimmed.trim_start_matches("from ");
            if let Some((module, _)) = rest.split_once(" import ") {
                imports.push(format!("{}::{}", module.trim(), rest.split(" import ").nth(1).unwrap_or("").split(',').next().unwrap_or("").trim()));
                imports.push(module.trim().to_string());
            }
        }
    }
    imports
}

fn extract_js_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") || trimmed.starts_with("const ") {
            if let Some(from_part) = trimmed.split("from ").nth(1) {
                if let Some(path) = extract_quoted_path(from_part) {
                    imports.push(path);
                }
            }
        }

        if trimmed.contains("require(") && !trimmed.starts_with("import ") {
            if let Some(require_part) = trimmed.split("require(").nth(1) {
                if let Some(path) = extract_quoted_path(require_part) {
                    imports.push(path);
                }
            }
        }
    }
    imports
}

fn extract_quoted_path(s: &str) -> Option<String> {
    let s = s.trim().trim_end_matches(';').trim();
    let quote = s.chars().next().filter(|&c| c == '\'' || c == '"')?;
    let rest = &s[1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn extract_go_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    let mut in_import_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("import (") {
            in_import_block = true;
            continue;
        }
        if in_import_block {
            if trimmed == ")" {
                in_import_block = false;
                continue;
            }
            let module = trimmed.trim_matches('"');
            if !module.is_empty() {
                imports.push(module.to_string());
            }
            continue;
        }
        if trimmed.starts_with("import ") && !trimmed.contains('(') {
            let module = trimmed
                .trim_start_matches("import ")
                .trim_matches('"')
                .trim_matches(';');
            imports.push(module.to_string());
        }
    }
    imports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_imports_simple() {
        let content = "use std::collections::HashMap;\nuse serde::Serialize;\n";
        let imports = extract_rust_imports(content);
        assert!(imports.contains(&"std::collections::HashMap".to_string()));
        assert!(imports.contains(&"serde::Serialize".to_string()));
    }

    #[test]
    fn test_extract_rust_imports_pub_use() {
        let content = "pub use crate::types::FileNode;\npub use self::graph::analyze;\n";
        let imports = extract_rust_imports(content);
        assert!(imports.contains(&"crate::types::FileNode".to_string()));
        assert!(imports.contains(&"self::graph::analyze".to_string()));
    }

    #[test]
    fn test_extract_rust_imports_nested() {
        let content = "use std::collections::HashMap;\nuse crate::foo;\n";
        let imports = extract_rust_imports(content);
        assert!(imports.contains(&"std::collections::HashMap".to_string()));
        assert!(imports.contains(&"crate::foo".to_string()));
    }

    #[test]
    fn test_extract_python_imports_simple() {
        let content = "import os\nimport sys\n";
        let imports = extract_python_imports(content);
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"sys".to_string()));
    }

    #[test]
    fn test_extract_python_from_import() {
        let content = "from collections import defaultdict\nfrom pathlib import Path\n";
        let imports = extract_python_imports(content);
        assert!(imports.contains(&"collections".to_string()));
    }

    #[test]
    fn test_extract_js_imports_esm() {
        let content = "import fs from 'fs';\nimport { parse } from './parser';\n";
        let imports = extract_js_imports(content);
        assert!(imports.contains(&"fs".to_string()));
        assert!(imports.contains(&"./parser".to_string()));
    }

    #[test]
    fn test_extract_js_imports_require() {
        let content = "const fs = require('fs');\n";
        let imports = extract_js_imports(content);
        assert!(imports.contains(&"fs".to_string()));
    }

    #[test]
    fn test_extract_go_imports_simple() {
        let content = "import \"fmt\"\nimport \"os\"\n";
        let imports = extract_go_imports(content);
        assert!(imports.contains(&"fmt".to_string()));
        assert!(imports.contains(&"os".to_string()));
    }

    #[test]
    fn test_extract_go_imports_grouped() {
        let content = "import (\n    \"fmt\"\n    \"os\"\n    \"strings\"\n)\n";
        let imports = extract_go_imports(content);
        assert!(imports.contains(&"fmt".to_string()));
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"strings".to_string()));
    }

    #[test]
    fn test_extract_rust_with_as() {
        let content = "use std::collections::HashMap as Map;\n";
        let imports = extract_rust_imports(content);
        assert!(imports.contains(&"std::collections::HashMap".to_string()));
    }

    #[test]
    fn test_extract_python_import_as() {
        let content = "import numpy as np\n";
        let imports = extract_python_imports(content);
        assert!(imports.contains(&"numpy".to_string()));
    }

    #[test]
    fn test_extract_js_imports_no_imports() {
        let content = "const x = 1;\nfunction foo() {}\n";
        let imports = extract_js_imports(content);
        assert!(imports.is_empty());
    }

    #[test]
    fn test_extract_go_no_imports() {
        let content = "package main\nfunc main() {}\n";
        let imports = extract_go_imports(content);
        assert!(imports.is_empty());
    }
}

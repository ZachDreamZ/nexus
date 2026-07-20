pub fn estimate_complexity(content: &str) -> f64 {
    let lines: Vec<&str> = content.lines().collect();
    let code_lines = count_code_lines(&lines);
    let control_flow = count_control_flow(&lines);
    let nesting = estimate_nesting(&lines);
    let token_variety = estimate_token_variety(content);

    let base = (code_lines as f64 * 0.3)
        + (control_flow as f64 * 1.5)
        + (nesting as f64 * 2.0)
        + (token_variety as f64 * 0.5);

    (base * 10.0).round() / 10.0
}

pub fn count_lines(content: &str) -> (usize, usize) {
    let total = content.lines().count();
    let code = count_code_lines(&content.lines().collect::<Vec<_>>());
    (total, code)
}

fn count_code_lines(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("#")
                && !trimmed.starts_with("/*")
                && !trimmed.starts_with("*")
                && !trimmed.starts_with("--")
                && !trimmed.starts_with("'''")
                && !trimmed.starts_with("\"\"\"")
        })
        .count()
}

fn count_control_flow(lines: &[&str]) -> usize {
    let keywords = [
        "if ", "else ", "for ", "while ", "match ", "switch ",
        "return ", "break ", "continue ", "try ", "catch ",
        "fn ", "def ", "function ", "class ", "impl ",
        "match", "where ",
    ];

    lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            keywords.iter().any(|kw| {
                trimmed.starts_with(kw) || trimmed.contains(&format!(" {kw}"))
            })
        })
        .count()
}

fn estimate_nesting(lines: &[&str]) -> usize {
    let mut max_depth = 0usize;
    let mut current = 0usize;

    for line in lines {
        let trimmed = line.trim_start();
        let leading = line.len() - trimmed.len();

        let depth = leading / 4;
        if depth > current {
            current = depth;
            max_depth = max_depth.max(current);
        } else if depth < current {
            current = depth;
        } else if trimmed.starts_with('}')
            || trimmed.starts_with(']')
            || trimmed.starts_with(')')
            || trimmed == "end"
        {
            current = current.saturating_sub(1);
        }

        if trimmed.ends_with('{')
            || trimmed.ends_with("do")
            || trimmed.ends_with(':') && !trimmed.starts_with('#')
        {
            current += 1;
            max_depth = max_depth.max(current);
        }
    }

    max_depth
}

fn estimate_token_variety(content: &str) -> usize {
    let mut tokens = std::collections::HashSet::new();
    for word in content.split_whitespace() {
        let clean = word
            .trim_start_matches(|c: char| c.is_ascii_punctuation())
            .trim_end_matches(|c: char| c.is_ascii_punctuation());
        if clean.len() > 1 {
            tokens.insert(clean.to_string());
        }
    }
    tokens.len().min(500)
}

pub fn detect_test_file(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    path_lower.contains("test")
        || path_lower.contains("spec")
        || path_lower.contains("_test")
        || path_lower.contains("_spec")
}

pub enum DependencyKind {
    ModuleLocal(String),
    External(String),
}

pub fn classify_dependency(import: &str, _language: &crate::types::Language) -> DependencyKind {
    let is_relative = import.starts_with('.') || import.starts_with("..") || import.starts_with("crate::") || import.starts_with("self::") || import.starts_with("super::");
    if is_relative {
        DependencyKind::ModuleLocal(import.to_string())
    } else {
        DependencyKind::External(import.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines_empty() {
        let (total, code) = count_lines("");
        assert_eq!(total, 0);
        assert_eq!(code, 0);
    }

    #[test]
    fn test_count_lines_code_only() {
        let (total, code) = count_lines("fn foo() {\n    let x = 1;\n    x + 1\n}");
        assert_eq!(total, 4);
        assert_eq!(code, 4);
    }

    #[test]
    fn test_count_lines_with_comments() {
        let content = "// comment\nfn foo() {\n    # python comment\n    let x = 1;\n}\n";
        let (total, code) = count_lines(content);
        assert_eq!(total, 5);
        assert_eq!(code, 3);
    }

    #[test]
    fn test_estimate_complexity_simple() {
        let c = estimate_complexity("fn foo() { let x = 1; x + 1 }");
        assert!(c > 0.0);
    }

    #[test]
    fn test_estimate_complexity_nested() {
        let code = "fn foo() {\n    if a {\n        for b in c {\n            while d {\n                // deep\n            }\n        }\n    }\n}";
        let c = estimate_complexity(code);
        let simple = estimate_complexity("fn foo() { let x = 1; }");
        assert!(c > simple, "nested should be more complex than simple");
    }

    #[test]
    fn test_nesting_depth() {
        let lines = vec![
            "fn foo() {",
            "    if a {",
            "        for b in c {",
            "            let x = 1;",
            "        }",
            "    }",
            "}",
        ];
        let depth = estimate_nesting(&lines);
        assert_eq!(depth, 3);
    }

    #[test]
    fn test_control_flow_count() {
        let lines = vec![
            "fn foo() {",
            "    if a {",
            "        for b in c {",
            "            return 1;",
            "        }",
            "    }",
            "}",
        ];
        let count = count_control_flow(&lines);
        assert!(count >= 3);
    }

    #[test]
    fn test_detect_test_file() {
        assert!(detect_test_file("src/test/mod.rs"));
        assert!(detect_test_file("src/lib/test.rs"));
        assert!(detect_test_file("src/foo_spec.ts"));
        assert!(!detect_test_file("src/main.rs"));
    }

    #[test]
    fn test_classify_dependency() {
        let rust = crate::types::Language::Rust;
        assert!(matches!(
            classify_dependency("crate::foo::bar", &rust),
            DependencyKind::ModuleLocal(_)
        ));
        assert!(matches!(
            classify_dependency("serde::Serialize", &rust),
            DependencyKind::External(_)
        ));
    }

    #[test]
    fn test_token_variety() {
        let content = "fn foo(a: i32, b: i32) -> i32 { a + b }";
        let variety = estimate_token_variety(content);
        assert!(variety > 0);
        assert!(variety <= 500);
    }

    #[test]
    fn test_count_code_lines_multiline_comments() {
        let content = "fn foo() {\n/* block comment */\nlet x = 1;\n}\n";
        let (total, code) = count_lines(content);
        assert_eq!(total, 4);
        assert_eq!(code, 3);
    }
}

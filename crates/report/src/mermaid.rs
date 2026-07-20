use core::graph::AnalysisResult;

pub fn format_mermaid(result: &AnalysisResult) -> String {
    let mut output = String::new();
    output.push_str("graph TD\n");
    output.push_str("  %% Auto-generated dependency graph by nexus\n\n");

    for node in result.graph.nodes.values() {
        let safe_id = safe_id(&node.path);
        let label = std::path::Path::new(&node.path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&node.path);
        let lang_emoji = match node.language {
            core::types::Language::Rust => "🦀",
            core::types::Language::Python => "🐍",
            core::types::Language::JavaScript => "📜",
            core::types::Language::TypeScript => "📘",
            core::types::Language::Go => "🔵",
            core::types::Language::Other(_) => "📄",
        };
        output.push_str(&format!("  {}[\"{}{}\"]\n", safe_id, lang_emoji, label));
    }

    output.push_str("\n");

    for (&from_id, to_set) in &result.graph.edges {
        for &to_id in to_set {
            let from_safe = result
                .graph
                .nodes
                .get(&from_id)
                .map(|n| safe_id(&n.path))
                .unwrap_or_default();
            let to_safe = result
                .graph
                .nodes
                .get(&to_id)
                .map(|n| safe_id(&n.path))
                .unwrap_or_default();
            if !from_safe.is_empty() && !to_safe.is_empty() {
                output.push_str(&format!("  {} --> {}\n", from_safe, to_safe));
            }
        }
    }

    output
}

fn safe_id(path: &str) -> String {
    let id = path
        .replace('\\', "_")
        .replace('/', "_")
        .replace('.', "_")
        .replace('-', "_")
        .replace(':', "_")
        .replace(' ', "_");
    format!("n{}", id)
}

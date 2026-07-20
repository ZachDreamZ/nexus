use core::graph::{AnalysisResult, ImpactReport};
use core::types::DependencyGraph;

pub fn format_report(result: &AnalysisResult) -> String {
    let mut output = String::new();

    output.push_str(&format!("\n  {} {}\n", "═══", "nexus — Dependency Analysis"));
    output.push_str(&format!("  {}\n\n", "━".repeat(40)));

    output.push_str(&format!("  {} {}\n", "📊", "Summary"));
    output.push_str(&format!("  {} Files:     {}\n", " ", result.stats.total_files));
    output.push_str(&format!("  {} Deps:      {} (avg {:.1})\n", " ", result.stats.total_dependencies, result.stats.avg_deps));
    output.push_str(&format!("  {} Lines:     {} ({} code)\n", " ", result.stats.total_lines, result.stats.total_code_lines));
    output.push_str(&format!("  {} Complexity: {:.1} avg\n", " ", result.stats.avg_complexity));
    output.push_str(&format!("  {} Tests:     {}\n\n", " ", result.stats.test_file_count));

    if !result.stats.language_breakdown.is_empty() {
        output.push_str(&format!("  {} {}\n", "🔤", "Languages"));
        let mut langs: Vec<_> = result.stats.language_breakdown.iter().collect();
        langs.sort_by(|a, b| b.1.cmp(a.1));
        for (lang, count) in langs {
            output.push_str(&format!("  {}   {:12} {}\n", " ", lang, count));
        }
        output.push_str("\n");
    }

    if result.stats.cycle_count > 0 {
        output.push_str(&format!("  {} {}\n", "🔄", "Circular Dependencies"));
        output.push_str(&format!("  {}   Found {} cycle(s)\n", " ", result.stats.cycle_count));
        for cycle in result.cycles.iter().take(3) {
            let names: Vec<&str> = cycle
                .iter()
                .filter_map(|id| result.graph.nodes.get(id))
                .map(|n| {
                    std::path::Path::new(&n.path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(&n.path)
                })
                .collect();
            output.push_str(&format!("  {}   {} → ... → {}\n", " ", names.first().unwrap_or(&""), names.last().unwrap_or(&"")));
        }
        if result.cycles.len() > 3 {
            output.push_str(&format!("  {}   ... and {} more\n", " ", result.cycles.len() - 3));
        }
        output.push_str("\n");
    }

    if !result.top_deps.is_empty() {
        output.push_str(&format!("  {} {}\n", "🔗", "Top Dependers (files with most deps)"));
        for (file_id, count) in &result.top_deps {
            if let Some(node) = result.graph.nodes.get(file_id) {
                let name = std::path::Path::new(&node.path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&node.path);
                output.push_str(&format!("  {}   {:4} deps  {}\n", " ", count, name));
            }
        }
        output.push_str("\n");
    }

    if !result.top_dependents.is_empty() {
        output.push_str(&format!("  {} {}\n", "🎯", "Top Depended (most imported files)"));
        for (file_id, count) in &result.top_dependents {
            if let Some(node) = result.graph.nodes.get(file_id) {
                let name = std::path::Path::new(&node.path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&node.path);
                output.push_str(&format!("  {}   {:4} imports  {}\n", " ", count, name));
            }
        }
        output.push_str("\n");
    }

    if result.stats.isolated_count > 0 {
        output.push_str(&format!("  {} {}\n", "📭", "Isolated Files (no deps, no dependents)"));
        output.push_str(&format!("  {}   {} files\n", " ", result.stats.isolated_count));
    }

    output
}

pub fn format_impact(report: &ImpactReport, graph: &DependencyGraph) -> String {
    let mut output = String::new();

    output.push_str(&format!("\n  {}  {}\n", "═══", "nexus — Impact Analysis"));
    output.push_str(&format!("  {}\n\n", "━".repeat(40)));

    output.push_str(&format!("  {} Risk Score: {}/10\n\n", "⚠️", report.risk_score));

    output.push_str(&format!("  {} Target Files:\n", "🎯"));
    for &id in &report.target_files {
        if let Some(node) = graph.nodes.get(&id) {
            output.push_str(&format!("  {}   {}\n", " ", node.path));
        }
    }
    output.push_str("\n");

    output.push_str(&format!("  {} Directly Affected: {} files\n", "📌", report.directly_affected.len()));
    for &id in &report.directly_affected {
        if let Some(node) = graph.nodes.get(&id) {
            output.push_str(&format!("  {}   {}\n", " ", node.path));
        }
    }
    output.push_str("\n");

    output.push_str(&format!("  {} Transitively Affected: {} files\n", "🔀", report.transitively_affected.len()));
    let extra = report.transitively_affected.len().saturating_sub(report.directly_affected.len());
    if extra > 0 {
        output.push_str(&format!("  {}   ({} beyond direct)\n", " ", extra));
    }
    output.push_str("\n");

    output.push_str(&format!("  {} Total Files Touched: {}\n", "📦", report.total_files_touched));
    output.push_str(&format!("  {} Dependencies Needed: {}\n", "📥", report.total_dependencies.len()));

    output
}

use core::graph::{AnalysisResult, ImpactReport};
use core::types::DependencyGraph;

pub fn format_json(result: &AnalysisResult) -> String {
    let mut parts = Vec::new();

    parts.push(format!("\"summary\": {{"));
    parts.push(format!("\"total_files\": {},", result.stats.total_files));
    parts.push(format!("\"total_dependencies\": {},", result.stats.total_dependencies));
    parts.push(format!("\"average_deps\": {:.2},", result.stats.avg_deps));
    parts.push(format!("\"total_lines\": {},", result.stats.total_lines));
    parts.push(format!("\"total_code_lines\": {},", result.stats.total_code_lines));
    parts.push(format!("\"average_complexity\": {:.2},", result.stats.avg_complexity));
    parts.push(format!("\"cycle_count\": {},", result.stats.cycle_count));
    parts.push(format!("\"isolated_files\": {},", result.stats.isolated_count));
    parts.push(format!("\"test_files\": {}", result.stats.test_file_count));
    parts.push(format!("}},"));

    if !result.stats.language_breakdown.is_empty() {
        parts.push(format!("\"languages\": {{"));
        let mut lang_parts = Vec::new();
        let mut langs: Vec<_> = result.stats.language_breakdown.iter().collect();
        langs.sort_by(|a, b| b.1.cmp(a.1));
        for (lang, count) in langs {
            lang_parts.push(format!("\"{lang}\": {count}"));
        }
        parts.push(lang_parts.join(",\n"));
        parts.push(format!("}},"));
    }

    parts.push(format!("\"files\": ["));
    let mut file_parts = Vec::new();
    let mut files: Vec<_> = result.graph.nodes.values().collect();
    files.sort_by(|a, b| a.path.cmp(&b.path));
    for node in files {
        let deps: Vec<String> = result
            .graph
            .edges
            .get(&node.id)
            .into_iter()
            .flat_map(|s| s.iter())
            .filter_map(|id| result.graph.nodes.get(id))
            .map(|n| format!("\"{}\"", n.path.replace('\\', "\\\\").replace('"', "\\\"")))
            .collect();

        file_parts.push(format!(
            "{{ \"path\": \"{}\", \"language\": \"{:?}\", \"lines\": {}, \"code_lines\": {}, \"complexity\": {:.1}, \"imports\": {}, \"has_test\": {}, \"dependencies\": [{}] }}",
            node.path.replace('\\', "\\\\").replace('"', "\\\""),
            node.language,
            node.lines,
            node.code_lines,
            node.complexity,
            node.imports.len(),
            node.has_test_file,
            deps.join(", ")
        ));
    }
    parts.push(file_parts.join(",\n"));
    parts.push(format!("],"));

    if !result.cycles.is_empty() {
        parts.push(format!("\"cycles\": ["));
        let cycle_parts: Vec<String> = result
            .cycles
            .iter()
            .map(|cycle| {
                let names: Vec<String> = cycle
                    .iter()
                    .filter_map(|id| result.graph.nodes.get(id))
                    .map(|n| format!("\"{}\"", n.path.replace('\\', "\\\\").replace('"', "\\\"")))
                    .collect();
                format!("[{}]", names.join(", "))
            })
            .collect();
        parts.push(cycle_parts.join(",\n"));
        parts.push(format!("]"));
    }

    format!("{{\n{}\n}}", parts.join("\n"))
}

pub fn format_impact_json(report: &ImpactReport, graph: &DependencyGraph) -> String {
    let mut parts = Vec::new();

    parts.push(format!("\"risk_score\": {},", report.risk_score));
    parts.push(format!("\"total_files_touched\": {},", report.total_files_touched));

    parts.push(format!("\"target_files\": ["));
    let targets: Vec<String> = report
        .target_files
        .iter()
        .filter_map(|id| graph.nodes.get(id))
        .map(|n| format!("\"{}\"", n.path))
        .collect();
    parts.push(targets.join(", "));
    parts.push(format!("],"));

    parts.push(format!("\"directly_affected\": {} ,", report.directly_affected.len()));
    parts.push(format!("\"transitively_affected\": {},", report.transitively_affected.len()));
    parts.push(format!("\"dependencies_needed\": {}", report.total_dependencies.len()));

    format!("{{\n{}\n}}", parts.join("\n"))
}

use std::collections::{HashMap, HashSet};
use crate::types::*;

pub struct AnalysisResult {
    pub graph: DependencyGraph,
    pub cycles: Vec<Vec<FileId>>,
    pub stats: GraphStats,
    pub top_deps: Vec<(FileId, usize)>,
    pub top_dependents: Vec<(FileId, usize)>,
}

#[derive(Debug, Clone)]
pub struct GraphStats {
    pub total_files: usize,
    pub total_dependencies: usize,
    pub max_deps: usize,
    pub max_dependents: usize,
    pub isolated_count: usize,
    pub cycle_count: usize,
    pub avg_deps: f64,
    pub avg_complexity: f64,
    pub total_lines: usize,
    pub total_code_lines: usize,
    pub language_breakdown: HashMap<String, usize>,
    pub test_file_count: usize,
}

pub fn analyze(graph: &DependencyGraph) -> AnalysisResult {
    let cycles = if !graph.nodes.is_empty() {
        graph.find_cycles()
    } else {
        Vec::new()
    };

    let mut deps_by_node: Vec<(FileId, usize)> = graph
        .nodes
        .keys()
        .map(|&id| {
            let count = graph.edges.get(&id).map_or(0, |s| s.len());
            (id, count)
        })
        .collect();
    deps_by_node.sort_by_key(|b| std::cmp::Reverse(b.1));

    let mut dependents_by_node: Vec<(FileId, usize)> = graph
        .nodes
        .keys()
        .map(|&id| {
            let count = graph.reverse_edges.get(&id).map_or(0, |s| s.len());
            (id, count)
        })
        .collect();
    dependents_by_node.sort_by_key(|b| std::cmp::Reverse(b.1));

    let max_deps = deps_by_node.first().map(|(_, c)| *c).unwrap_or(0);
    let max_dependents = dependents_by_node.first().map(|(_, c)| *c).unwrap_or(0);

    let total_deps: usize = graph
        .edges
        .values()
        .map(|s| s.len())
        .sum();
    let avg_deps = if graph.nodes.is_empty() {
        0.0
    } else {
        total_deps as f64 / graph.nodes.len() as f64
    };

    let total_lines: usize = graph.nodes.values().map(|n| n.lines).sum();
    let total_code_lines: usize = graph.nodes.values().map(|n| n.code_lines).sum();
    let avg_complexity = if graph.nodes.is_empty() {
        0.0
    } else {
        graph.nodes.values().map(|n| n.complexity).sum::<f64>() / graph.nodes.len() as f64
    };

    let mut language_breakdown: HashMap<String, usize> = HashMap::new();
    for node in graph.nodes.values() {
        let lang = format!("{:?}", node.language);
        *language_breakdown.entry(lang).or_insert(0) += 1;
    }

    let isolated_count = graph.isolated_nodes().len();
    let test_file_count = graph.nodes.values().filter(|n| n.has_test_file).count();

    AnalysisResult {
        stats: GraphStats {
            total_files: graph.nodes.len(),
            total_dependencies: total_deps,
            max_deps,
            max_dependents,
            isolated_count,
            cycle_count: cycles.len(),
            avg_deps,
            avg_complexity,
            total_lines,
            total_code_lines,
            language_breakdown,
            test_file_count,
        },
        cycles,
        top_deps: deps_by_node.into_iter().take(10).collect(),
        top_dependents: dependents_by_node.into_iter().take(10).collect(),
        graph: graph.clone(),
    }
}

pub fn impact_analysis(
    graph: &DependencyGraph,
    target_files: &[FileId],
) -> ImpactReport {
    let mut affected = HashSet::new();
    let mut total_deps = HashSet::new();

    for &target in target_files {
        let t_deps = graph.transitive_dependencies(target);
        let t_dependents = graph.transitive_dependents(target);
        affected.extend(&t_dependents);
        total_deps.extend(&t_deps);
    }

    let total_impact = affected.len() + total_deps.len() + target_files.len();
    let risk_score = calculate_risk_score(graph, &affected, &total_deps, target_files);

    ImpactReport {
        target_files: target_files.to_vec(),
            directly_affected: affected.iter().filter(|id| {
                target_files.iter().any(|t| {
                    graph
                        .reverse_edges
                        .get(t)
                        .is_some_and(|s| s.contains(id))
                })
            }).copied().collect(),
        transitively_affected: affected,
        total_dependencies: total_deps,
        total_files_touched: total_impact,
        risk_score,
    }
}

#[derive(Debug, Clone)]
pub struct ImpactReport {
    pub target_files: Vec<FileId>,
    pub directly_affected: HashSet<FileId>,
    pub transitively_affected: HashSet<FileId>,
    pub total_dependencies: HashSet<FileId>,
    pub total_files_touched: usize,
    pub risk_score: u8,
}

fn calculate_risk_score(
    graph: &DependencyGraph,
    affected: &HashSet<FileId>,
    deps: &HashSet<FileId>,
    targets: &[FileId],
) -> u8 {
    if graph.nodes.is_empty() {
        return 0;
    }

    let total_nodes = graph.nodes.len().max(1);
    let touch_ratio = (affected.len() + deps.len() + targets.len()) as f64 / total_nodes as f64;

    let avg_complexity: f64 = targets
        .iter()
        .filter_map(|id| graph.nodes.get(id))
        .map(|n| n.complexity)
        .sum::<f64>()
        .max(1.0);

    let has_cycles = graph.find_cycles().len() as f64;

    let mut score = (touch_ratio * 5.0
        + (avg_complexity / 20.0).min(3.0)
        + (has_cycles / 5.0).min(2.0))
        .round() as u8;

    score = score.min(10);
    score
}

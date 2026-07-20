use std::collections::{HashMap, HashSet};

pub type FileId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct FileNode {
    pub id: FileId,
    pub path: String,
    pub language: Language,
    pub lines: usize,
    pub code_lines: usize,
    pub imports: Vec<String>,
    pub has_test_file: bool,
    pub complexity: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Other(String),
}

impl Language {
    pub fn from_extension(path: &str) -> Self {
        if path.ends_with(".rs") {
            Language::Rust
        } else if path.ends_with(".py") {
            Language::Python
        } else if path.ends_with(".js") || path.ends_with(".jsx") {
            Language::JavaScript
        } else if path.ends_with(".ts") || path.ends_with(".tsx") {
            Language::TypeScript
        } else if path.ends_with(".go") {
            Language::Go
        } else {
            Language::Other(
                std::path::Path::new(path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    pub nodes: HashMap<FileId, FileNode>,
    pub edges: HashMap<FileId, HashSet<FileId>>,
    pub reverse_edges: HashMap<FileId, HashSet<FileId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph::default()
    }

    pub fn add_node(&mut self, node: FileNode) -> FileId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.edges.entry(id).or_default();
        self.reverse_edges.entry(id).or_default();
        id
    }

    pub fn add_dependency(&mut self, from: FileId, to: FileId) {
        self.edges.entry(from).or_default().insert(to);
        self.reverse_edges.entry(to).or_default().insert(from);
    }

    pub fn dependencies(&self, id: FileId) -> Vec<&FileNode> {
        self.edges
            .get(&id)
            .into_iter()
            .flat_map(|set| set.iter())
            .filter_map(|dep_id| self.nodes.get(dep_id))
            .collect()
    }

    pub fn dependents(&self, id: FileId) -> Vec<&FileNode> {
        self.reverse_edges
            .get(&id)
            .into_iter()
            .flat_map(|set| set.iter())
            .filter_map(|dep_id| self.nodes.get(dep_id))
            .collect()
    }

    pub fn transitive_dependencies(&self, id: FileId) -> HashSet<FileId> {
        let mut visited = HashSet::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }
            if let Some(deps) = self.edges.get(&current) {
                for dep in deps.iter() {
                    stack.push(*dep);
                }
            }
        }
        visited.remove(&id);
        visited
    }

    pub fn transitive_dependents(&self, id: FileId) -> HashSet<FileId> {
        let mut visited = HashSet::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }
            if let Some(deps) = self.reverse_edges.get(&current) {
                for dep in deps.iter() {
                    stack.push(*dep);
                }
            }
        }
        visited.remove(&id);
        visited
    }

    pub fn find_cycles(&self) -> Vec<Vec<FileId>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();
        let mut path = Vec::new();

        for &node_id in self.nodes.keys() {
            if !visited.contains(&node_id) {
                self.dfs_cycle(node_id, &mut visited, &mut in_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycle(
        &self,
        node: FileId,
        visited: &mut HashSet<FileId>,
        in_stack: &mut HashSet<FileId>,
        path: &mut Vec<FileId>,
        cycles: &mut Vec<Vec<FileId>>,
    ) {
        visited.insert(node);
        in_stack.insert(node);
        path.push(node);

        if let Some(deps) = self.edges.get(&node) {
            for &dep in deps.iter() {
                if !visited.contains(&dep) {
                    self.dfs_cycle(dep, visited, in_stack, path, cycles);
                } else if in_stack.contains(&dep) {
                    let cycle_start = path.iter().position(|&n| n == dep).unwrap();
                    let cycle: Vec<FileId> = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        path.pop();
        in_stack.remove(&node);
    }

    pub fn file_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|s| s.len()).sum()
    }

    pub fn isolated_nodes(&self) -> Vec<FileId> {
        self.nodes
            .keys()
            .filter(|&id| {
                let has_deps = self.edges.get(id).map_or(0, |s| s.len()) > 0;
                let has_dependents = self.reverse_edges.get(id).map_or(0, |s| s.len()) > 0;
                !has_deps && !has_dependents && !self.nodes.get(id).is_some_and(|n| n.has_test_file)
            })
            .copied()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct NexusConfig {
    pub paths: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub include_extensions: Vec<String>,
    pub max_depth: usize,
    pub detect_tests: bool,
    pub detect_circular: bool,
    pub format: OutputFormat,
}

impl Default for NexusConfig {
    fn default() -> Self {
        NexusConfig {
            paths: vec![".".to_string()],
            exclude_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "__pycache__".to_string(),
                ".venv".to_string(),
                "vendor".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            include_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "go".to_string(),
            ],
            max_depth: usize::MAX,
            detect_tests: true,
            detect_circular: true,
            format: OutputFormat::Terminal,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Terminal,
    Json,
    Mermaid,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "terminal" | "term" | "tty" => Ok(OutputFormat::Terminal),
            "json" => Ok(OutputFormat::Json),
            "mermaid" | "mmd" => Ok(OutputFormat::Mermaid),
            _ => Err(format!("Unknown format: {s}. Use: terminal, json, or mermaid")),
        }
    }
}

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use core::types::*;

pub fn walk_directory(config: &NexusConfig) -> Result<DependencyGraph, String> {
    let mut graph = DependencyGraph::new();
    let mut file_id = 0usize;
    let mut path_to_id: HashMap<String, FileId> = HashMap::new();

    let mut all_files: Vec<String> = Vec::new();
    for path_str in &config.paths {
        let path = Path::new(path_str);
        if path.is_file() {
            all_files.push(path_str.clone());
        } else if path.is_dir() {
            collect_files(path, config, &mut all_files, 0)?;
        }
    }

    for path_str in &all_files {
        if let Ok(content) = fs::read_to_string(path_str) {
            let language = Language::from_extension(path_str);
            let (total_lines, code_lines) = core::metrics::count_lines(&content);
            let complexity = core::metrics::estimate_complexity(&content);
            let imports = crate::extract_imports(&content, &language);
            let has_test = core::metrics::detect_test_file(path_str);

            let node = FileNode {
                id: file_id,
                path: path_str.clone(),
                language,
                lines: total_lines,
                code_lines,
                imports: imports.clone(),
                has_test_file: has_test,
                complexity,
            };
            path_to_id.insert(path_str.clone(), file_id);
            graph.add_node(node);
            file_id += 1;
        }
    }

    let mut edges_to_add: Vec<(FileId, FileId)> = Vec::new();
    for (path_str, id) in &path_to_id {
        if let Some(node) = graph.nodes.get(id) {
            for import in &node.imports {
                if let Some(resolved) = resolve_import(path_str, import) {
                    if let Some(&dep_id) = path_to_id.get(&resolved) {
                        edges_to_add.push((*id, dep_id));
                    }
                }
            }
        }
    }
    for (from, to) in edges_to_add {
        graph.add_dependency(from, to);
    }

    Ok(graph)
}

fn collect_files(
    dir: &Path,
    config: &NexusConfig,
    results: &mut Vec<String>,
    depth: usize,
) -> Result<(), String> {
    if depth > config.max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("Cannot read dir {:?}: {e}", dir))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Cannot read entry: {e}"))?;
        let path = entry.path();

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if config
            .exclude_patterns
            .iter()
            .any(|pat| file_name.contains(pat) || path.to_string_lossy().contains(pat))
        {
            continue;
        }

        if path.is_dir() {
            collect_files(&path, config, results, depth + 1)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            if config.include_extensions.is_empty()
                || config.include_extensions.contains(&ext)
            {
                results.push(path.to_string_lossy().to_string());
            }
        }
    }

    Ok(())
}

fn resolve_import(current_file: &str, import: &str) -> Option<String> {
    let current = Path::new(current_file);
    let parent = current.parent()?;

    if import.starts_with('.') || import.starts_with("..") {
        let resolved = parent.join(import);
        let candidates = [
            resolved.with_extension("rs"),
            resolved.with_extension("py"),
            resolved.with_extension("js"),
            resolved.with_extension("ts"),
            resolved.with_extension("jsx"),
            resolved.with_extension("tsx"),
            resolved.with_extension("go"),
        ];
        for c in &candidates {
            if c.exists() {
                return Some(c.to_string_lossy().to_string());
            }
        }
        if resolved.is_dir() {
            let mod_file = resolved.join("mod.rs");
            if mod_file.exists() {
                return Some(mod_file.to_string_lossy().to_string());
            }
            let init_file = resolved.join("__init__.py");
            if init_file.exists() {
                return Some(init_file.to_string_lossy().to_string());
            }
            let index_file = resolved.join("index.js");
            if index_file.exists() {
                return Some(index_file.to_string_lossy().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_files_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let mut files = Vec::new();
        let config = NexusConfig::default();
        let result = collect_files(tmp.path(), &config, &mut files, 0);
        assert!(result.is_ok());
        assert!(files.is_empty());
    }

    #[test]
    fn test_resolve_import_relative() {
        let tmp = tempfile::tempdir().unwrap();
        let test_file = tmp.path().join("main.rs");
        let mod_file = tmp.path().join("helper.rs");
        std::fs::write(&mod_file, "").unwrap();
        let resolved = resolve_import(
            test_file.to_str().unwrap(),
            "./helper",
        );
        assert!(resolved.is_some());
    }

    #[test]
    fn test_resolve_import_nonexistent() {
        let resolved = resolve_import("/tmp/main.rs", "./nonexistent");
        assert!(resolved.is_none());
    }
}

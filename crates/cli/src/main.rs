use core::graph::{analyze, impact_analysis};
use core::types::{FileId, NexusConfig};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args.first().map(|s| s.as_str()).unwrap_or("nexus");

    if args.len() < 2 {
        print_help(program);
        return;
    }

    let result = match args[1].as_str() {
        "analyze" | "a" => cmd_analyze(&args[2..]),
        "impact" | "i" => cmd_impact(&args[2..]),
        "cycles" | "c" => cmd_cycles(&args[2..]),
        "stats" | "s" => cmd_stats(&args[2..]),
        "isolated" => cmd_isolated(&args[2..]),
        "mermaid" | "mmd" => cmd_mermaid(&args[2..]),
        "json" | "j" => cmd_json(&args[2..]),
        "help" | "-h" | "--help" => {
            print_help(program);
            return;
        }
        "version" | "-V" | "--version" => {
            println!("nexus 0.1.0");
            return;
        }
        other => {
            eprintln!("❌ Unknown command: {other}");
            eprintln!("   Run '{program} help' for usage.");
            std::process::exit(1);
        }
    };

    match result {
        Ok(output) => println!("{output}"),
        Err(e) => {
            eprintln!("❌ Error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_help(program: &str) {
    println!("nexus — Codebase Dependency Analyzer");
    println!();
    println!("USAGE:");
    println!("  {program} <command> [options] [path]");
    println!();
    println!("COMMANDS:");
    println!("  analyze, a     Analyze dependencies (default format)");
    println!("  impact, i      Analyze impact of changing specific files");
    println!("  cycles, c      Find circular dependencies");
    println!("  stats, s       Show summary statistics only");
    println!("  isolated       Find files with no deps or dependents");
    println!("  mermaid, mmd   Output dependency graph as Mermaid diagram");
    println!("  json, j        Output complete analysis as JSON");
    println!("  help           Show this help");
    println!("  version        Show version");
    println!();
    println!("OPTIONS:");
    println!("  --exclude PAT   Exclude paths containing PAT (repeatable)");
    println!("  --depth N       Max directory depth");
    println!("  --ext EXT       File extension to include (repeatable)");
    println!();
    println!("EXAMPLES:");
    println!("  {program} analyze");
    println!("  {program} analyze src/");
    println!("  {program} impact src/main.rs");
    println!("  {program} cycles --exclude node_modules");
    println!("  {program} mermaid src/ > graph.mmd");
    println!("  {program} json src/ > analysis.json");
}

fn parse_config(args: &[String]) -> Result<NexusConfig, String> {
    let mut config = NexusConfig::default();
    let mut i = 0;
    let mut paths = Vec::new();

    while i < args.len() {
        match args[i].as_str() {
            "--exclude" if i + 1 < args.len() => {
                config.exclude_patterns.push(args[i + 1].clone());
                i += 2;
            }
            "--depth" if i + 1 < args.len() => {
                config.max_depth = args[i + 1]
                    .parse()
                    .map_err(|_| format!("Invalid depth: {}", args[i + 1]))?;
                i += 2;
            }
            "--ext" if i + 1 < args.len() => {
                config.include_extensions.push(args[i + 1].clone());
                i += 2;
            }
            flag if flag.starts_with('-') => {
                return Err(format!("Unknown option: {flag}"));
            }
            _ => {
                paths.push(args[i].clone());
                i += 1;
            }
        }
    }

    if !paths.is_empty() {
        config.paths = paths;
    }

    Ok(config)
}

fn build_graph(config: &NexusConfig) -> Result<core::types::DependencyGraph, String> {
    parse::walk_directory(config)
}

fn cmd_analyze(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let result = analyze(&graph);

    if graph.nodes.is_empty() {
        return Ok("  No source files found to analyze.".to_string());
    }

    Ok(report::terminal::format_report(&result))
}

fn cmd_impact(args: &[String]) -> Result<String, String> {
    if args.is_empty() || args[0].starts_with('-') {
        return Err("Usage: nexus impact <file1> [file2 ...]".to_string());
    }

    let target_names: Vec<String> = args
        .iter()
        .take_while(|a| !a.starts_with('-'))
        .cloned()
        .collect();

    let config = parse_config(args)?;
    let graph = build_graph(&config)?;

    let mut target_ids: Vec<FileId> = Vec::new();
    for name in &target_names {
        let found: Vec<FileId> = graph
            .nodes
            .iter()
            .filter(|(_, n)| n.path.contains(name.as_str()) || n.path == *name)
            .map(|(&id, _)| id)
            .collect();

        if found.is_empty() {
            return Err(format!("No file found matching: {name}"));
        }
        target_ids.extend(found);
    }

    let report = impact_analysis(&graph, &target_ids);
    Ok(report::terminal::format_impact(&report, &graph))
}

fn cmd_cycles(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let result = analyze(&graph);

    if result.cycles.is_empty() {
        return Ok("  No circular dependencies found.".to_string());
    }

    let mut output = format!("  Found {} circular dependenc(ies):\n\n", result.cycles.len());
    for (i, cycle) in result.cycles.iter().enumerate() {
        output.push_str(&format!("  Cycle #{}:\n", i + 1));
        for &id in cycle {
            if let Some(node) = graph.nodes.get(&id) {
                output.push_str(&format!("    {} {}\n", "→", node.path));
            }
        }
        output.push('\n');
    }

    Ok(output)
}

fn cmd_stats(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let result = analyze(&graph);

    if graph.nodes.is_empty() {
        return Ok("  No source files found.".to_string());
    }

    let mut output = format!("  Total files:    {}\n", result.stats.total_files);
    output.push_str(&format!("  Total deps:     {}\n", result.stats.total_dependencies));
    output.push_str(&format!("  Avg deps/file:  {:.2}\n", result.stats.avg_deps));
    output.push_str(&format!("  Total lines:    {} ({} code)\n", result.stats.total_lines, result.stats.total_code_lines));
    output.push_str(&format!("  Avg complexity: {:.2}\n", result.stats.avg_complexity));
    output.push_str(&format!("  Cycles:         {}\n", result.stats.cycle_count));
    output.push_str(&format!("  Isolated files:  {}\n", result.stats.isolated_count));
    output.push_str(&format!("  Test files:     {}", result.stats.test_file_count));

    Ok(output)
}

fn cmd_isolated(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let isolated = graph.isolated_nodes();

    if isolated.is_empty() {
        return Ok("  No isolated files found.".to_string());
    }

    let mut output = format!("  {} isolated file(s):\n\n", isolated.len());
    for id in isolated {
        if let Some(node) = graph.nodes.get(&id) {
            output.push_str(&format!("    📄 {}\n", node.path));
        }
    }

    Ok(output)
}

fn cmd_mermaid(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let result = analyze(&graph);
    Ok(report::mermaid::format_mermaid(&result))
}

fn cmd_json(args: &[String]) -> Result<String, String> {
    let config = parse_config(args)?;
    let graph = build_graph(&config)?;
    let result = analyze(&graph);
    Ok(report::json_format::format_json(&result))
}

//! Mycelium CLI — Static analysis tool for mapping codebase connections.

use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use mycelium_core::config::AnalysisConfig;
use mycelium_core::output::write_output;
use mycelium_core::pipeline;

#[derive(Parser)]
#[command(
    name = "mycelium-map",
    about = "Mycelium - Map the hidden network of connections in your codebase"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyse a source code repository and produce a structural map
    Analyze {
        /// Path to the repository to analyse
        path: PathBuf,

        /// Output JSON file path
        #[arg(short, long)]
        output: Option<String>,

        /// Comma-separated language filter
        #[arg(short, long)]
        languages: Option<String>,

        /// Louvain resolution parameter
        #[arg(long, default_value = "1.0")]
        resolution: f64,

        /// Maximum execution flows to detect
        #[arg(long, default_value = "75")]
        max_processes: usize,

        /// Maximum BFS trace depth
        #[arg(long, default_value = "10")]
        max_depth: usize,

        /// Additional glob patterns to exclude
        #[arg(long)]
        exclude: Vec<String>,

        /// Show per-phase timing breakdown
        #[arg(long)]
        verbose: bool,

        /// Suppress all output except errors
        #[arg(long)]
        quiet: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            path,
            output,
            languages,
            resolution,
            max_processes,
            max_depth,
            exclude,
            verbose,
            quiet,
        } => {
            let repo_path = path.canonicalize().unwrap_or(path);
            let repo_name = repo_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "repo".to_string());

            let output_path = output.unwrap_or_else(|| format!("{repo_name}.mycelium.json"));

            let lang_filter = languages.map(|l| {
                l.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            });

            let config = AnalysisConfig {
                repo_path: repo_path.to_string_lossy().to_string(),
                output_path: Some(output_path.clone()),
                languages: lang_filter,
                resolution,
                max_processes,
                max_depth,
                exclude_patterns: exclude,
                verbose,
                quiet,
                ..Default::default()
            };

            if quiet {
                run_quiet(&config, &output_path);
            } else {
                run_with_progress(&config, &output_path, verbose);
            }
        }
    }
}

fn run_quiet(config: &AnalysisConfig, output_path: &str) {
    match pipeline::run_pipeline(config, None) {
        Ok(result) => {
            if let Err(e) = write_output(&result, output_path) {
                eprintln!("Error writing output: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Analysis failed: {e}");
            std::process::exit(1);
        }
    }
}

fn run_with_progress(config: &AnalysisConfig, output_path: &str, verbose: bool) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message("Initialising...");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let progress: pipeline::ProgressCallback = {
        let pb = pb.clone();
        Box::new(move |_name, label| {
            pb.set_message(label.to_string());
        })
    };

    let start = Instant::now();
    let result = match pipeline::run_pipeline(config, Some(progress)) {
        Ok(r) => r,
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("Analysis failed: {e}");
            std::process::exit(1);
        }
    };
    pb.finish_and_clear();

    // Summary
    println!(
        "\n{}  Mycelium Analysis: {}",
        style("✓").green().bold(),
        style(
            std::path::Path::new(&config.repo_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        )
        .bold()
    );
    println!(
        "  {:<14} {}",
        "Files:",
        result.stats.get("files").unwrap_or(&serde_json::json!(0))
    );
    println!(
        "  {:<14} {}",
        "Symbols:",
        result.stats.get("symbols").unwrap_or(&serde_json::json!(0))
    );
    println!(
        "  {:<14} {}",
        "Calls:",
        result.stats.get("calls").unwrap_or(&serde_json::json!(0))
    );
    println!(
        "  {:<14} {}",
        "Communities:",
        result
            .stats
            .get("communities")
            .unwrap_or(&serde_json::json!(0))
    );
    println!(
        "  {:<14} {}",
        "Processes:",
        result
            .stats
            .get("processes")
            .unwrap_or(&serde_json::json!(0))
    );

    let duration = start.elapsed();
    println!(
        "  {:<14} {:.1}ms",
        "Duration:",
        duration.as_secs_f64() * 1000.0
    );

    if verbose {
        if let Some(serde_json::Value::Object(timings)) = result.metadata.get("phase_timings") {
            println!("\n  Phase Timings:");
            for (phase, ms) in timings {
                if let Some(val) = ms.as_f64() {
                    println!("    {:<14} {:.1}ms", phase, val * 1000.0);
                }
            }
        }
    }

    if let Err(e) = write_output(&result, output_path) {
        eprintln!("Error writing output: {e}");
        std::process::exit(1);
    }

    println!(
        "\n  {} {}",
        style("Output written to:").green(),
        output_path
    );
}

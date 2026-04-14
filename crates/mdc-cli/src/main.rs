use anyhow::Result;
use clap::{Parser, Subcommand};
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS};
use mdc_core::{Analyzer, Cleaner, CleanMode, Scanner, format_size};
use mdc_rules::builtins::macos_builtin_rules;


#[derive(Parser)]
#[command(name = "mdc")]
#[command(about = "macOS Deep Cleaner - scan and clean leftover files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan the system for cleanable files
    Scan {
        /// Output format: table | json
        #[arg(long, default_value = "table")]
        report: String,
        /// Show top N largest items
        #[arg(long, default_value = "20")]
        top: usize,
        /// Restrict scan to specific categories (comma-separated)
        #[arg(long)]
        category: Option<String>,
    },
    /// Clean (move to trash) the detected files
    Clean {
        /// Perform a dry-run without actually deleting anything
        #[arg(long)]
        dry_run: bool,
        /// Automatically confirm without prompting
        #[arg(long)]
        confirm: bool,
        /// Restrict cleaning to specific categories (comma-separated)
        #[arg(long)]
        category: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { report, top, category } => {
            let rules = filter_rules(category)?;
            let scanner = Scanner::new();
            let items = scanner.scan(&rules);
            let analyzer = Analyzer::new();
            let analysis = analyzer.analyze(&items, top);

            match report.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&analysis)?);
                }
                _ => print_table_report(&analysis),
            }
        }
        Commands::Clean { dry_run, confirm, category } => {
            let rules = filter_rules(category)?;
            let scanner = Scanner::new();
            let items = scanner.scan(&rules);

            if items.is_empty() {
                println!("No cleanable items found.");
                return Ok(());
            }

            let total_size: u64 = items.iter().filter_map(|i| i.size).sum();
            println!(
                "Found {} items totaling {}",
                items.len(),
                format_size(total_size)
            );

            let mode = if dry_run {
                CleanMode::DryRun
            } else {
                if !confirm {
                    println!("This will move the listed items to the trash.");
                    println!("Use --dry-run to preview, or --confirm to proceed.");
                    return Ok(());
                }
                CleanMode::MoveToTrash
            };

            let cleaner = Cleaner::new();
            let results = cleaner.clean(&items, mode)?;
            let success_count = results.iter().filter(|r| r.success).count();
            println!("Processed {}/{} items successfully.", success_count, results.len());
        }
    }

    Ok(())
}

fn filter_rules(category_filter: Option<String>) -> Result<Vec<mdc_rules::Rule>> {
    let all = macos_builtin_rules();
    if let Some(filter) = category_filter {
        let filters: Vec<String> = filter.split(',').map(|s| s.trim().to_lowercase()).collect();
        let filtered: Vec<_> = all
            .into_iter()
            .filter(|r| {
                let cat = format!("{}", r.category).to_lowercase();
                filters.iter().any(|f| cat.contains(f))
            })
            .collect();
        Ok(filtered)
    } else {
        Ok(all)
    }
}

fn print_table_report(report: &mdc_core::AnalysisReport) {
    println!("\n=== Summary ===");
    println!("Total items: {}", report.total_items);
    println!("Total size:  {}", format_size(report.total_size));

    println!("\n=== By Category ===");
    let mut cat_table = Table::new();
    cat_table
        .set_header(vec!["Category", "Items", "Size"])
        .apply_modifier(UTF8_ROUND_CORNERS);
    let mut categories: Vec<_> = report.by_category.iter().collect();
    categories.sort_by(|a, b| b.1.size.cmp(&a.1.size));
    for (cat, summary) in categories {
        cat_table.add_row(vec![
            format!("{}", cat),
            format!("{}", summary.count),
            format_size(summary.size),
        ]);
    }
    println!("{}", cat_table);

    println!("\n=== By Safety Level ===");
    let mut safety_table = Table::new();
    safety_table.set_header(vec!["Safety", "Size"]);
    for (safety, size) in &report.by_safety {
        safety_table.add_row(vec![format!("{}", safety), format_size(*size)]);
    }
    println!("{}", safety_table);

    println!("\n=== Top Items ===");
    let mut top_table = Table::new();
    top_table
        .set_header(vec!["Path", "Category", "Safety", "Size"])
        .apply_modifier(UTF8_ROUND_CORNERS);
    for item in &report.top_items {
        top_table.add_row(vec![
            item.path.display().to_string(),
            format!("{}", item.category),
            format!("{}", item.safety),
            format_size(item.size.unwrap_or(0)),
        ]);
    }
    println!("{}", top_table);
}

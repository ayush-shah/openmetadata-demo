# Phase 10: Complete OpenMetadata CLI Application 🚀

Welcome to the final phase! Build `omctl` - a production-ready command-line tool for OpenMetadata. This is where everything comes together!

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Build a complete CLI application
- ✅ Implement all OpenMetadata operations
- ✅ Create interactive and batch modes
- ✅ Add logging and observability
- ✅ Handle configuration properly
- ✅ Implement progress tracking
- ✅ Deploy a production-ready tool

## ⏱️ Estimated Time

14-21 days (4-5 hours per day)

## 🎯 Project: omctl - OpenMetadata Control CLI

A comprehensive command-line tool for managing OpenMetadata.

### Project Structure

```
omctl/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── config/
│   └── default.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── tables.rs
│   │   │   ├── databases.rs
│   │   │   ├── services.rs
│   │   │   ├── lineage.rs
│   │   │   ├── search.rs
│   │   │   ├── export.rs
│   │   │   ├── import.rs
│   │   │   └── repl.rs
│   │   └── output.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── formatting.rs
│   │   ├── progress.rs
│   │   └── validation.rs
│   └── error.rs
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   └── cli_tests.rs
│   └── fixtures/
│       └── test_data.json
├── docs/
│   ├── README.md
│   ├── COMMANDS.md
│   ├── EXAMPLES.md
│   └── CONTRIBUTING.md
└── scripts/
    ├── install.sh
    └── release.sh
```

### Cargo.toml

```toml
[package]
name = "omctl"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "Command-line tool for OpenMetadata"
license = "Apache-2.0"
repository = "https://github.com/yourusername/omctl"

[dependencies]
# CLI
clap = { version = "4.4", features = ["derive", "cargo"] }
dialoguer = "0.11"
indicatif = "0.17"
colored = "2.1"

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# HTTP & Serialization
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Config
config = "0.13"
directories = "5.0"

# Table formatting
tabled = "0.15"
comfy-table = "7.1"

# Date/Time
chrono = "0.4"

# Utils
uuid = { version = "1.6", features = ["v4"] }
regex = "1.10"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"

[[bin]]
name = "omctl"
path = "src/main.rs"
```

## 🏗️ Implementation

### 1. CLI Structure with Clap

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "omctl")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// OpenMetadata server URL
    #[arg(short = 'H', long, env = "OPENMETADATA_HOST")]
    host: Option<String>,

    /// Authentication token
    #[arg(short, long, env = "OPENMETADATA_TOKEN")]
    token: Option<String>,

    /// Output format: table, json, yaml
    #[arg(short, long, default_value = "table")]
    output: OutputFormat,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tables
    Table {
        #[command(subcommand)]
        action: TableCommands,
    },

    /// Manage databases
    Database {
        #[command(subcommand)]
        action: DatabaseCommands,
    },

    /// Manage services
    Service {
        #[command(subcommand)]
        action: ServiceCommands,
    },

    /// Work with lineage
    Lineage {
        #[command(subcommand)]
        action: LineageCommands,
    },

    /// Search metadata
    Search {
        /// Search query
        query: String,

        /// Entity type to search
        #[arg(short, long)]
        entity_type: Option<String>,

        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Export metadata
    Export {
        /// Entity type to export
        entity_type: String,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Export format: json, yaml
        #[arg(short, long, default_value = "json")]
        format: ExportFormat,
    },

    /// Import metadata
    Import {
        /// Input file
        input: PathBuf,

        /// Dry run (validate only)
        #[arg(long)]
        dry_run: bool,
    },

    /// Interactive REPL mode
    Repl,

    /// Initialize configuration
    Init,
}

#[derive(Subcommand)]
enum TableCommands {
    /// List all tables
    List {
        /// Database filter
        #[arg(short, long)]
        database: Option<String>,

        /// Schema filter
        #[arg(short, long)]
        schema: Option<String>,
    },

    /// Get table details
    Get {
        /// Fully qualified table name
        fqn: String,
    },

    /// Create a new table
    Create {
        /// Table definition file (JSON/YAML)
        file: PathBuf,
    },

    /// Update table
    Update {
        /// Fully qualified table name
        fqn: String,

        /// Update file (JSON/YAML)
        file: PathBuf,
    },

    /// Delete table
    Delete {
        /// Fully qualified table name
        fqn: String,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// Add tags to table
    Tag {
        /// Fully qualified table name
        fqn: String,

        /// Tags to add (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
}
```

### 2. Table Commands Implementation

```rust
pub async fn handle_table_commands(
    client: &OpenMetadataClient,
    cmd: TableCommands,
    output_format: OutputFormat,
) -> Result<()> {
    match cmd {
        TableCommands::List { database, schema } => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Fetching tables...");
            spinner.enable_steady_tick(Duration::from_millis(100));

            let tables = client.tables().list().await?;

            let filtered_tables: Vec<_> = tables
                .into_iter()
                .filter(|t| {
                    database.as_ref().map_or(true, |db| t.database == *db)
                        && schema.as_ref().map_or(true, |s| t.schema == *s)
                })
                .collect();

            spinner.finish_and_clear();

            match output_format {
                OutputFormat::Table => {
                    print_tables_table(&filtered_tables);
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&filtered_tables)?);
                }
                OutputFormat::Yaml => {
                    println!("{}", serde_yaml::to_string(&filtered_tables)?);
                }
            }

            println!(
                "\n{} Found {} tables",
                "✓".green().bold(),
                filtered_tables.len()
            );
        }

        TableCommands::Get { fqn } => {
            let table = client.tables().get(&fqn).await?;

            match output_format {
                OutputFormat::Table => print_table_details(&table),
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&table)?);
                }
                OutputFormat::Yaml => {
                    println!("{}", serde_yaml::to_string(&table)?);
                }
            }
        }

        TableCommands::Create { file } => {
            let content = std::fs::read_to_string(&file)?;
            let request: CreateTableRequest = if file.extension() == Some("yaml".as_ref()) {
                serde_yaml::from_str(&content)?
            } else {
                serde_json::from_str(&content)?
            };

            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Creating table...");
            spinner.enable_steady_tick(Duration::from_millis(100));

            let table = client.tables().create(request).await?;

            spinner.finish_and_clear();

            println!("{} Table created: {}", "✓".green().bold(), table.name);
            println!("  FQN: {}", table.fully_qualified_name);
            println!("  ID: {}", table.id);
        }

        TableCommands::Delete { fqn, yes } => {
            if !yes {
                let confirmation = dialoguer::Confirm::new()
                    .with_prompt(format!("Delete table '{}'?", fqn))
                    .default(false)
                    .interact()?;

                if !confirmation {
                    println!("Cancelled");
                    return Ok(());
                }
            }

            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Deleting table...");
            spinner.enable_steady_tick(Duration::from_millis(100));

            client.tables().delete(&fqn).await?;

            spinner.finish_and_clear();

            println!("{} Table deleted: {}", "✓".green().bold(), fqn);
        }

        TableCommands::Tag { fqn, tags } => {
            let pb = ProgressBar::new(tags.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40} {pos}/{len} {msg}")?
                    .progress_chars("=>-"),
            );

            for tag in tags {
                pb.set_message(format!("Adding tag: {}", tag));

                let tag_label = TagLabel {
                    tag_fqn: tag.clone(),
                    label_type: LabelType::Manual,
                    state: State::Confirmed,
                    source: TagSource::Classification,
                };

                client.tables().add_tag(&fqn, tag_label).await?;
                pb.inc(1);
            }

            pb.finish_with_message("Done");

            println!("{} Tags added to table: {}", "✓".green().bold(), fqn);
        }

        _ => {}
    }

    Ok(())
}
```

### 3. Output Formatting

```rust
use comfy_table::{Table, Row, Cell, Color, Attribute};

fn print_tables_table(tables: &[Table]) {
    let mut table = Table::new();

    table.set_header(vec![
        Cell::new("Name").fg(Color::Green).add_attribute(Attribute::Bold),
        Cell::new("Database").fg(Color::Blue),
        Cell::new("Schema").fg(Color::Blue),
        Cell::new("Columns").fg(Color::Yellow),
        Cell::new("Tags").fg(Color::Magenta),
    ]);

    for t in tables {
        table.add_row(vec![
            Cell::new(&t.name),
            Cell::new(&t.database),
            Cell::new(&t.schema),
            Cell::new(t.columns.len()),
            Cell::new(t.tags.len()),
        ]);
    }

    println!("{}", table);
}

fn print_table_details(table: &Table) {
    println!("{}", "Table Details".bold().underline());
    println!();
    println!("{:20} {}", "Name:", table.name.bold());
    println!("{:20} {}", "FQN:", table.fully_qualified_name);
    println!("{:20} {}", "Database:", table.database);
    println!("{:20} {}", "Schema:", table.schema);

    if let Some(desc) = &table.description {
        println!("{:20} {}", "Description:", desc);
    }

    println!();
    println!("{}", "Columns:".bold());

    let mut columns_table = Table::new();
    columns_table.set_header(vec!["Name", "Type", "Description"]);

    for col in &table.columns {
        columns_table.add_row(vec![
            Cell::new(&col.name),
            Cell::new(&col.data_type).fg(Color::Cyan),
            Cell::new(col.description.as_deref().unwrap_or("-")),
        ]);
    }

    println!("{}", columns_table);

    if !table.tags.is_empty() {
        println!();
        println!("{}", "Tags:".bold());
        for tag in &table.tags {
            println!("  • {}", tag.tag_fqn.cyan());
        }
    }
}
```

### 4. Interactive REPL Mode

```rust
use dialoguer::{Input, Select, theme::ColorfulTheme};

pub async fn run_repl(client: Arc<OpenMetadataClient>) -> Result<()> {
    println!("{}", "OpenMetadata Interactive Shell".bold().cyan());
    println!("Type 'help' for available commands, 'exit' to quit\n");

    let theme = ColorfulTheme::default();

    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("omctl>")
            .interact_text()?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }

            "help" => {
                print_repl_help();
            }

            "list tables" => {
                match client.tables().list().await {
                    Ok(tables) => print_tables_table(&tables),
                    Err(e) => eprintln!("{} {}", "Error:".red(), e),
                }
            }

            cmd if cmd.starts_with("get table ") => {
                let fqn = cmd.strip_prefix("get table ").unwrap().trim();
                match client.tables().get(fqn).await {
                    Ok(table) => print_table_details(&table),
                    Err(e) => eprintln!("{} {}", "Error:".red(), e),
                }
            }

            cmd if cmd.starts_with("search ") => {
                let query = cmd.strip_prefix("search ").unwrap().trim();
                match client.search().query(query).await {
                    Ok(results) => print_search_results(&results),
                    Err(e) => eprintln!("{} {}", "Error:".red(), e),
                }
            }

            _ => {
                eprintln!("{} Unknown command: '{}'", "Error:".red(), input);
                eprintln!("Type 'help' for available commands");
            }
        }

        println!();
    }

    Ok(())
}

fn print_repl_help() {
    println!("{}", "Available Commands:".bold());
    println!("  list tables              - List all tables");
    println!("  get table <fqn>          - Get table details");
    println!("  list databases           - List all databases");
    println!("  search <query>           - Search metadata");
    println!("  help                     - Show this help");
    println!("  exit                     - Exit the shell");
}
```

### 5. Export/Import Commands

```rust
pub async fn export_metadata(
    client: &OpenMetadataClient,
    entity_type: &str,
    output: &Path,
    format: ExportFormat,
) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("Exporting {}...", entity_type));
    pb.enable_steady_tick(Duration::from_millis(100));

    let data = match entity_type {
        "tables" => {
            let tables = client.tables().list_all().await?;
            serde_json::to_value(tables)?
        }
        "databases" => {
            let dbs = client.databases().list_all().await?;
            serde_json::to_value(dbs)?
        }
        "services" => {
            let services = client.services().list_all().await?;
            serde_json::to_value(services)?
        }
        _ => return Err(anyhow::anyhow!("Unknown entity type: {}", entity_type)),
    };

    pb.set_message("Writing to file...");

    let content = match format {
        ExportFormat::Json => serde_json::to_string_pretty(&data)?,
        ExportFormat::Yaml => serde_yaml::to_string(&data)?,
    };

    std::fs::write(output, content)?;

    pb.finish_and_clear();

    println!(
        "{} Exported {} to {}",
        "✓".green().bold(),
        entity_type,
        output.display()
    );

    Ok(())
}

pub async fn import_metadata(
    client: &OpenMetadataClient,
    input: &Path,
    dry_run: bool,
) -> Result<()> {
    let content = std::fs::read_to_string(input)?;

    let data: ImportData = if input.extension() == Some("yaml".as_ref()) {
        serde_yaml::from_str(&content)?
    } else {
        serde_json::from_str(&content)?
    };

    if dry_run {
        println!("{} Dry run mode - validating only", "ℹ".blue());
    }

    let pb = ProgressBar::new(data.total_items() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40} {pos}/{len} {msg}")?
            .progress_chars("=>-"),
    );

    let mut errors = Vec::new();

    // Import tables
    for table in data.tables {
        pb.set_message(format!("Importing table: {}", table.name));

        if !dry_run {
            if let Err(e) = client.tables().create(table).await {
                errors.push(format!("Failed to import table: {}", e));
            }
        }

        pb.inc(1);
    }

    // Import other entities...

    pb.finish_and_clear();

    if errors.is_empty() {
        println!(
            "{} Successfully imported {} items",
            "✓".green().bold(),
            data.total_items()
        );
    } else {
        println!(
            "{} Imported with {} errors:",
            "⚠".yellow().bold(),
            errors.len()
        );
        for error in errors {
            eprintln!("  • {}", error.red());
        }
    }

    Ok(())
}
```

### 6. Configuration Management

```rust
use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub host: String,
    pub token: String,
    pub timeout: u64,
    pub max_retries: u32,
    pub cache_enabled: bool,
    pub output_format: String,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = ProjectDirs::from("com", "openmetadata", "omctl")
            .ok_or_else(|| ConfigError::NotFound("config directory".to_string()))?;

        let config_path = config_dir.config_dir().join("config.toml");

        let s = Config::builder()
            .add_source(File::from(config_path).required(false))
            .add_source(config::Environment::with_prefix("OMCTL"))
            .build()?;

        s.try_deserialize()
    }

    pub fn init_config() -> Result<()> {
        let config_dir = ProjectDirs::from("com", "openmetadata", "omctl")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        let config_path = config_dir.config_dir().join("config.toml");

        if config_path.exists() {
            let overwrite = dialoguer::Confirm::new()
                .with_prompt("Configuration file already exists. Overwrite?")
                .default(false)
                .interact()?;

            if !overwrite {
                return Ok(());
            }
        }

        std::fs::create_dir_all(config_dir.config_dir())?;

        let default_config = Settings {
            host: "http://localhost:8585".to_string(),
            token: String::new(),
            timeout: 30,
            max_retries: 3,
            cache_enabled: true,
            output_format: "table".to_string(),
            log_level: "info".to_string(),
        };

        let config_content = toml::to_string_pretty(&default_config)?;
        std::fs::write(&config_path, config_content)?;

        println!(
            "{} Configuration file created at: {}",
            "✓".green().bold(),
            config_path.display()
        );

        Ok(())
    }
}
```

## 🎯 Complete Feature Set

### Core Features
- ✅ List, get, create, update, delete all entity types
- ✅ Add and manage tags
- ✅ Query and visualize lineage
- ✅ Search across all metadata
- ✅ Export/import functionality
- ✅ Interactive REPL mode

### User Experience
- ✅ Beautiful table formatting
- ✅ Progress bars for long operations
- ✅ Colored output
- ✅ Interactive prompts
- ✅ Multiple output formats (table, JSON, YAML)

### Configuration
- ✅ Config file support
- ✅ Environment variables
- ✅ Command-line overrides
- ✅ Init command to setup

### Quality
- ✅ Comprehensive error handling
- ✅ Logging and tracing
- ✅ Integration tests
- ✅ Documentation
- ✅ CI/CD ready

## ✅ Final Checklist

Congratulations! You've completed the Rust learning path. You should now be able to:

- [ ] Write idiomatic Rust code
- [ ] Understand ownership and borrowing
- [ ] Build async applications
- [ ] Design and implement SDKs
- [ ] Create CLI applications
- [ ] Work with HTTP APIs
- [ ] Handle errors gracefully
- [ ] Write comprehensive tests
- [ ] Build production-ready software
- [ ] Integrate with OpenMetadata

## 🎓 Next Steps

### Continue Learning
1. Contribute to Rust open-source projects
2. Build more complex applications
3. Explore WebAssembly with Rust
4. Learn unsafe Rust and FFI
5. Study Rust compiler internals

### OpenMetadata
1. Enhance the SDK with more features
2. Build web UI with Yew or Leptos
3. Create custom connectors in Rust
4. Build real-time monitoring tools
5. Contribute to OpenMetadata

### Share Your Journey
1. Write blog posts about your learning
2. Create tutorials for others
3. Contribute to Rust community
4. Mentor new Rustaceans
5. Build and share your projects

## 🏆 Congratulations!

You've mastered Rust from zero to advanced! You now have:

✅ Solid understanding of Rust fundamentals
✅ Professional coding skills
✅ Production-ready SDK development experience
✅ Complete CLI application portfolio
✅ OpenMetadata expertise

Keep building, keep learning, and enjoy your Rust journey! 🦀✨

---

**Questions or feedback?**
- Open an issue on GitHub
- Join Rust community forums
- Connect with OpenMetadata community
- Share your projects!

Happy coding! 🚀

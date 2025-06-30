use clap::{Parser, Subcommand};
use colored::*;
use std::process::exit;
use task_runner::{config::Config, executor::TaskExecutor, error::TaskRunnerError};

#[derive(Parser)]
#[command(
    name = "task-runner",
    about = "A fast and flexible CLI task runner for managing development workflows",
    version,
    long_about = "Task Runner helps you manage complex development workflows with parallel and sequential task execution, dependency management, and rich output formatting."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path (default: task-runner.json, task-runner.yaml, or task-runner.toml)
    #[arg(short, long)]
    config: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Environment to use for task execution
    #[arg(short, long)]
    env: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available tasks
    List {
        /// Show task details
        #[arg(short, long)]
        details: bool,
    },
    /// Run one or more tasks
    Run {
        /// Task names to run
        tasks: Vec<String>,
        
        /// Run tasks in parallel
        #[arg(short, long)]
        parallel: bool,
        
        /// Run tasks sequentially (default)
        #[arg(short, long)]
        sequential: bool,
        
        /// Continue execution even if some tasks fail
        #[arg(short, long)]
        continue_on_error: bool,
    },
    /// Show task information
    Info {
        /// Task name
        task: String,
    },
    /// Validate configuration file
    Validate,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    // Initialize colored output
    colored::control::set_override(true);
    
    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), TaskRunnerError> {
    let config = Config::load(cli.config.as_deref())?;
    let executor = TaskExecutor::new(config);
    
    match cli.command {
        Commands::List { details } => {
            executor.list_tasks(details).await?;
        }
        Commands::Run { 
            tasks, 
            parallel, 
            sequential, 
            continue_on_error 
        } => {
            if tasks.is_empty() {
                return Err(TaskRunnerError::NoTasksSpecified);
            }
            
            let execution_mode = if parallel {
                task_runner::ExecutionMode::Parallel
            } else if sequential {
                task_runner::ExecutionMode::Sequential
            } else {
                task_runner::ExecutionMode::Auto
            };
            
            executor.run_tasks(&tasks, execution_mode, continue_on_error).await?;
        }
        Commands::Info { task } => {
            executor.show_task_info(&task).await?;
        }
        Commands::Validate => {
            println!("{} Configuration file is valid!", "✓".green());
        }
    }
    
    Ok(())
} 
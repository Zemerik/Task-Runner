use crate::config::Config;
use crate::error::TaskRunnerError;
use crate::task::{ExecutionMode, Task};
use crate::utils::expand_env_vars;
use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Instant;
use tokio::time::{sleep, Duration};

pub struct TaskExecutor {
    config: Config,
}

impl TaskExecutor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    /// List all available tasks
    pub async fn list_tasks(&self, details: bool) -> Result<(), TaskRunnerError> {
        let tasks = self.config.get_visible_tasks();
        
        if tasks.is_empty() {
            println!("{} No tasks found in configuration", "⚠️".yellow());
            return Ok(());
        }
        
        println!("{} Available tasks:", "📋".blue());
        println!();
        
        for task_name in tasks {
            if let Some(task) = self.config.get_task(&task_name) {
                if details {
                    self.print_task_details(&task_name, task);
                } else {
                    println!("  {} {}", "•".green(), task_name.bold());
                    if let Some(desc) = &task.description {
                        println!("    {}", desc.dimmed());
                    }
                }
                println!();
            }
        }
        
        Ok(())
    }
    
    /// Show detailed information about a specific task
    pub async fn show_task_info(&self, task_name: &str) -> Result<(), TaskRunnerError> {
        let task = self.config.get_task(task_name)
            .ok_or_else(|| TaskRunnerError::TaskNotFound(task_name.to_string()))?;
        
        println!("{} Task: {}", "📋".blue(), task_name.bold());
        println!();
        
        self.print_task_details(task_name, task);
        
        Ok(())
    }
    
    /// Run tasks with specified execution mode
    pub async fn run_tasks(
        &self,
        task_names: &[String],
        execution_mode: ExecutionMode,
        continue_on_error: bool,
    ) -> Result<(), TaskRunnerError> {
        // Get execution order including dependencies
        let execution_order = self.config.get_execution_order(task_names)?;
        
        if execution_order.is_empty() {
            return Err(TaskRunnerError::NoTasksSpecified);
        }
        
        println!("{} Executing {} tasks...", "🚀".green(), execution_order.len());
        println!();
        
        let multi_progress = MultiProgress::new();
        let style = ProgressStyle::default_spinner()
            .template("{spinner:.green} {wide_msg}")
            .unwrap();
        
        let mut handles = Vec::new();
        let mut results = HashMap::new();
        
        match execution_mode {
            ExecutionMode::Parallel => {
                // Run all tasks in parallel
                for task_name in &execution_order {
                    let pb = multi_progress.add(ProgressBar::new_spinner());
                    pb.set_style(style.clone());
                    pb.set_message(format!("Running {}", task_name));
                    
                    let config = self.config.clone();
                    let task_name = task_name.clone();
                    
                    let handle = tokio::spawn(async move {
                        let result = Self::execute_single_task(&config, &task_name, &pb).await;
                        (task_name, result)
                    });
                    
                    handles.push(handle);
                }
                
                // Wait for all tasks to complete
                for handle in handles {
                    let (task_name, result) = handle.await.unwrap();
                    results.insert(task_name, result);
                }
            }
            ExecutionMode::Sequential => {
                // Run tasks sequentially
                for task_name in &execution_order {
                    let pb = multi_progress.add(ProgressBar::new_spinner());
                    pb.set_style(style.clone());
                    pb.set_message(format!("Running {}", task_name));
                    
                    let result = Self::execute_single_task(&self.config, task_name, &pb).await;
                    let is_err = result.is_err();
                    results.insert(task_name.clone(), result);
                    
                    if is_err {
                        if !continue_on_error {
                            break;
                        }
                    }
                }
            }
            ExecutionMode::Auto => {
                // Determine execution mode based on task configuration
                for task_name in &execution_order {
                    let task = self.config.get_task(task_name).unwrap();
                    let _task_mode = task.execution_mode();
                    
                    let pb = multi_progress.add(ProgressBar::new_spinner());
                    pb.set_style(style.clone());
                    pb.set_message(format!("Running {}", task_name));
                    
                    let result = Self::execute_single_task(&self.config, task_name, &pb).await;
                    let is_err = result.is_err();
                    results.insert(task_name.clone(), result);
                    
                    if is_err {
                        if !continue_on_error {
                            break;
                        }
                    }
                }
            }
        }
        
        multi_progress.clear().unwrap();
        
        // Print results
        self.print_execution_results(&results);
        
        // Check if any tasks failed
        let failed_tasks: Vec<_> = results.iter()
            .filter(|(_, result)| result.is_err())
            .map(|(name, _)| name)
            .collect();
        
        if !failed_tasks.is_empty() {
            return Err(TaskRunnerError::TaskExecutionFailed(
                format!("{} tasks failed: {:?}", failed_tasks.len(), failed_tasks)
            ));
        }
        
        Ok(())
    }
    
    /// Execute a single task
    async fn execute_single_task(
        config: &Config,
        task_name: &str,
        pb: &ProgressBar,
    ) -> Result<(), TaskRunnerError> {
        let task = config.get_task(task_name)
            .ok_or_else(|| TaskRunnerError::TaskNotFound(task_name.to_string()))?;
        
        let start_time = Instant::now();
        
        // Set up environment variables
        let mut env_vars: HashMap<String, String> = std::env::vars().collect();
        env_vars.extend(config.env.clone());
        env_vars.extend(task.env.clone());
        
        // Determine working directory: task-specific or default from config
        // Expand environment variables in working directory path
        let working_dir = task.working_dir.as_deref()
            .or_else(|| config.default_working_dir.as_deref())
            .map(|dir| expand_env_vars(dir, &env_vars));
        let working_dir = working_dir.as_deref();
        
        // Determine timeout: task-specific or default from config
        let timeout = task.timeout
            .or(config.default_timeout);
        
        // Execute commands
        let mut command_results = Vec::new();
        
        for (i, command) in task.commands.iter().enumerate() {
            // Expand environment variables in command string
            let expanded_command = expand_env_vars(command, &env_vars);
            pb.set_message(format!("{} [{}] {}", task_name, i + 1, &expanded_command));
            
            let result = Self::execute_command(
                &expanded_command, 
                &env_vars, 
                working_dir,
                timeout
            ).await;
            let is_err = result.is_err();
            command_results.push(result);
            
            if is_err {
                if !task.continue_on_error {
                    break;
                }
            }
        }
        
        let duration = start_time.elapsed();
        
        // Check if all commands succeeded
        let success = command_results.iter().all(|r| r.is_ok());
        
        if success {
            pb.finish_with_message(format!("{} {} completed in {:.2}s", 
                "✓".green(), task_name, duration.as_secs_f64()));
        } else {
            pb.finish_with_message(format!("{} {} failed in {:.2}s", 
                "✗".red(), task_name, duration.as_secs_f64()));
        }
        
        if success {
            Ok(())
        } else {
            Err(TaskRunnerError::TaskExecutionFailed(
                format!("Task '{}' failed", task_name)
            ))
        }
    }
    
    /// Execute a single command
    async fn execute_command(
        command: &str,
        env_vars: &HashMap<String, String>,
        working_dir: Option<&str>,
        timeout: Option<u64>,
    ) -> Result<(), TaskRunnerError> {
        let mut parts = command.split_whitespace();
        let program = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();
        
        let mut cmd = Command::new(program);
        cmd.args(&args);
        
        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        // Set working directory
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        
        // Set up output
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        
        // Execute command with optional timeout
        if let Some(timeout_secs) = timeout {
            // Use spawn and poll with timeout support
            let timeout_duration = Duration::from_secs(timeout_secs);
            let start = Instant::now();
            
            let mut child = cmd.spawn()
                .map_err(|e| TaskRunnerError::TaskExecutionFailed(e.to_string()))?;
            
            // Poll for completion with timeout
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            return Ok(());
                        } else {
                            return Err(TaskRunnerError::TaskExecutionFailed(
                                format!("Command failed with exit code: {}", status)
                            ));
                        }
                    }
                    Ok(None) => {
                        // Process still running - check timeout
                        if start.elapsed() >= timeout_duration {
                            // Timeout - try to kill the process
                            let _ = child.kill();
                            let _ = child.wait(); // Wait for kill to complete
                            return Err(TaskRunnerError::TaskExecutionFailed(
                                format!("Command timed out after {} seconds", timeout_secs)
                            ));
                        }
                        // Wait a bit before checking again
                        sleep(Duration::from_millis(100)).await;
                    }
                    Err(e) => {
                        return Err(TaskRunnerError::TaskExecutionFailed(e.to_string()));
                    }
                }
            }
        } else {
            // No timeout - execute normally
            let output = cmd.output()
                .map_err(|e| TaskRunnerError::TaskExecutionFailed(e.to_string()))?;
            
            if output.status.success() {
                Ok(())
            } else {
                Err(TaskRunnerError::TaskExecutionFailed(
                    format!("Command failed with exit code: {}", output.status)
                ))
            }
        }
    }
    
    /// Print task details
    fn print_task_details(&self, task_name: &str, task: &Task) {
        println!("  {} {}", "•".green(), task_name.bold());
        
        if let Some(desc) = &task.description {
            println!("    Description: {}", desc);
        }
        
        println!("    Commands:");
        for (i, cmd) in task.commands.iter().enumerate() {
            println!("      {}. {}", i + 1, cmd);
        }
        
        if !task.dependencies.is_empty() {
            println!("    Dependencies: {}", task.dependencies.join(", "));
        }
        
        if !task.env.is_empty() {
            println!("    Environment:");
            for (key, value) in &task.env {
                println!("      {}={}", key, value);
            }
        }
        
        if task.parallel {
            println!("    Execution: {}", "Parallel".blue());
        } else if task.sequential {
            println!("    Execution: {}", "Sequential".yellow());
        }
        
        // Show timeout: task-specific, or default from config, or none
        let timeout = task.timeout
            .or(self.config.default_timeout);
        if let Some(timeout_secs) = timeout {
            if task.timeout.is_some() {
                println!("    Timeout: {}s", timeout_secs);
            } else {
                println!("    Timeout: {}s (from default)", timeout_secs);
            }
        }
        
        // Show working directory: task-specific, or default from config, or none
        let working_dir = task.working_dir.as_deref()
            .or_else(|| self.config.default_working_dir.as_deref());
        if let Some(dir) = working_dir {
            if task.working_dir.is_some() {
                println!("    Working Directory: {}", dir);
            } else {
                println!("    Working Directory: {} (from default)", dir);
            }
        }
        
        if task.continue_on_error {
            println!("    Continue on error: {}", "Yes".yellow());
        }
    }
    
    /// Print execution results
    fn print_execution_results(&self, results: &HashMap<String, Result<(), TaskRunnerError>>) {
        println!();
        println!("{} Execution Results:", "📊".blue());
        println!();
        
        let mut success_count = 0;
        let mut failure_count = 0;
        
        for (task_name, result) in results {
            match result {
                Ok(_) => {
                    println!("  {} {}", "✓".green(), task_name);
                    success_count += 1;
                }
                Err(_) => {
                    println!("  {} {}", "✗".red(), task_name);
                    failure_count += 1;
                }
            }
        }
        
        println!();
        println!("  {} {} successful, {} failed", 
            "📈".blue(), success_count, failure_count);
    }
} 
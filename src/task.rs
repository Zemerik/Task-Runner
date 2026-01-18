use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task description
    pub description: Option<String>,
    
    /// Commands to execute
    pub commands: Vec<String>,
    
    /// Task dependencies (other task names)
    #[serde(default)]
    pub dependencies: Vec<String>,
    
    /// Environment variables for this task
    #[serde(default)]
    pub env: HashMap<String, String>,
    
    /// Whether to run commands in parallel
    #[serde(default)]
    pub parallel: bool,
    
    /// Whether to run commands sequentially
    #[serde(default)]
    pub sequential: bool,
    
    /// Working directory for task execution
    pub working_dir: Option<String>,
    
    /// Timeout in seconds (None = no timeout)
    pub timeout: Option<u64>,
    
    /// Whether to continue on error
    #[serde(default)]
    pub continue_on_error: bool,
    
    /// Task is hidden from list (for internal tasks)
    #[serde(default)]
    pub hidden: bool,
}

impl Task {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            description: None,
            commands,
            dependencies: Vec::new(),
            env: HashMap::new(),
            parallel: false,
            sequential: false,
            working_dir: None,
            timeout: None,
            continue_on_error: false,
            hidden: false,
        }
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
    
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }
    
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self.sequential = false;
        self
    }
    
    pub fn sequential(mut self) -> Self {
        self.sequential = true;
        self.parallel = false;
        self
    }
    
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub fn continue_on_error(mut self) -> Self {
        self.continue_on_error = true;
        self
    }
    
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }
    
    /// Get execution mode for this task
    pub fn execution_mode(&self) -> ExecutionMode {
        if self.parallel {
            ExecutionMode::Parallel
        } else if self.sequential {
            ExecutionMode::Sequential
        } else {
            ExecutionMode::Auto
        }
    }
    
    /// Validate task configuration
    pub fn validate(&self, name: &str) -> Result<(), String> {
        // Allow empty commands if task has dependencies (orchestrator tasks)
        if self.commands.is_empty() && self.dependencies.is_empty() {
            return Err(format!("Task '{}' has no commands and no dependencies", name));
        }
        
        if self.parallel && self.sequential {
            return Err(format!("Task '{}' cannot be both parallel and sequential", name));
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Automatically determine based on task configuration
    Auto,
    /// Run commands in parallel
    Parallel,
    /// Run commands sequentially
    Sequential,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let commands = vec!["echo hello".to_string()];
        let task = Task::new(commands.clone());
        
        assert_eq!(task.commands, commands);
        assert_eq!(task.description, None);
        assert_eq!(task.dependencies.len(), 0);
        assert_eq!(task.env.len(), 0);
        assert!(!task.parallel);
        assert!(!task.sequential);
        assert_eq!(task.working_dir, None);
        assert_eq!(task.timeout, None);
        assert!(!task.continue_on_error);
        assert!(!task.hidden);
    }

    #[test]
    fn test_task_builder_methods() {
        let task = Task::new(vec!["echo test".to_string()])
            .with_description("Test task".to_string())
            .with_dependencies(vec!["dep1".to_string(), "dep2".to_string()])
            .parallel()
            .with_timeout(30)
            .continue_on_error()
            .hidden();

        assert_eq!(task.description, Some("Test task".to_string()));
        assert_eq!(task.dependencies, vec!["dep1", "dep2"]);
        assert!(task.parallel);
        assert!(!task.sequential);
        assert_eq!(task.timeout, Some(30));
        assert!(task.continue_on_error);
        assert!(task.hidden);
    }

    #[test]
    fn test_execution_mode() {
        let mut task = Task::new(vec!["echo test".to_string()]);
        
        // Default should be Auto
        assert_eq!(task.execution_mode(), ExecutionMode::Auto);
        
        // Test parallel
        task.parallel = true;
        assert_eq!(task.execution_mode(), ExecutionMode::Parallel);
        
        // Test sequential
        task.parallel = false;
        task.sequential = true;
        assert_eq!(task.execution_mode(), ExecutionMode::Sequential);
    }

    #[test]
    fn test_task_validation() {
        // Valid task
        let task = Task::new(vec!["echo hello".to_string()]);
        assert!(task.validate("test").is_ok());
        
        // Task with no commands and no dependencies (invalid)
        let task = Task::new(vec![]);
        assert!(task.validate("test").is_err());
        
        // Task with no commands but has dependencies (orchestrator task - valid)
        let mut task = Task::new(vec![]);
        task.dependencies = vec!["dep1".to_string(), "dep2".to_string()];
        assert!(task.validate("test").is_ok());
        
        // Task with both parallel and sequential
        let mut task = Task::new(vec!["echo hello".to_string()]);
        task.parallel = true;
        task.sequential = true;
        assert!(task.validate("test").is_err());
    }

    #[test]
    fn test_task_serialization() {
        let task = Task::new(vec!["echo hello".to_string()])
            .with_description("Test task".to_string())
            .with_dependencies(vec!["dep1".to_string()])
            .parallel();

        // Test JSON serialization
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();
        
        assert_eq!(task.description, deserialized.description);
        assert_eq!(task.commands, deserialized.commands);
        assert_eq!(task.dependencies, deserialized.dependencies);
        assert_eq!(task.parallel, deserialized.parallel);
    }
} 
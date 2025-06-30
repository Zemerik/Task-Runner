use crate::error::TaskRunnerError;
use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Global environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,
    
    /// Task definitions
    pub tasks: HashMap<String, Task>,
    
    /// Default timeout for all tasks (in seconds)
    pub default_timeout: Option<u64>,
    
    /// Default working directory
    pub default_working_dir: Option<String>,
}

impl Config {
    /// Load configuration from file or search for default config files
    pub fn load(config_path: Option<&str>) -> Result<Self, TaskRunnerError> {
        if let Some(path) = config_path {
            Self::load_from_file(path)
        } else {
            Self::load_default()
        }
    }
    
    /// Load configuration from a specific file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaskRunnerError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|_| TaskRunnerError::ConfigNotFound)?;
        
        let config: Config = match path.extension().and_then(|s| s.to_str()) {
            Some("json") => serde_json::from_str(&content)
                .map_err(|e| TaskRunnerError::ConfigParseError(e.to_string()))?,
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content)
                .map_err(|e| TaskRunnerError::ConfigParseError(e.to_string()))?,
            Some("toml") => toml::from_str(&content)
                .map_err(|e| TaskRunnerError::ConfigParseError(e.to_string()))?,
            _ => return Err(TaskRunnerError::ConfigParseError(
                "Unsupported file format. Use .json, .yaml, .yml, or .toml".to_string()
            )),
        };
        
        config.validate()?;
        Ok(config)
    }
    
    /// Search for and load default configuration files
    pub fn load_default() -> Result<Self, TaskRunnerError> {
        let config_names = [
            "task-runner.json",
            "task-runner.yaml", 
            "task-runner.yml",
            "task-runner.toml",
        ];
        
        for name in &config_names {
            if let Ok(config) = Self::load_from_file(name) {
                return Ok(config);
            }
        }
        
        Err(TaskRunnerError::ConfigNotFound)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), TaskRunnerError> {
        // Validate each task
        for (name, task) in &self.tasks {
            task.validate(name)
                .map_err(|e| TaskRunnerError::ConfigParseError(e))?;
        }
        
        // Check for circular dependencies
        self.check_circular_dependencies()?;
        
        // Check that all dependencies exist
        self.check_dependencies()?;
        
        Ok(())
    }
    
    /// Check for circular dependencies using DFS
    fn check_circular_dependencies(&self) -> Result<(), TaskRunnerError> {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        for task_name in self.tasks.keys() {
            if !visited.contains(task_name) {
                if self.has_circular_dependency(task_name, &mut visited, &mut rec_stack) {
                    return Err(TaskRunnerError::CircularDependency(task_name.clone()));
                }
            }
        }
        
        Ok(())
    }
    
    fn has_circular_dependency(
        &self,
        task_name: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        visited.insert(task_name.to_string());
        rec_stack.insert(task_name.to_string());
        
        if let Some(task) = self.tasks.get(task_name) {
            for dep in &task.dependencies {
                if !visited.contains(dep) {
                    if self.has_circular_dependency(dep, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(task_name);
        false
    }
    
    /// Check that all dependencies exist
    fn check_dependencies(&self) -> Result<(), TaskRunnerError> {
        for (task_name, task) in &self.tasks {
            for dep in &task.dependencies {
                if !self.tasks.contains_key(dep) {
                    return Err(TaskRunnerError::DependencyNotFound(
                        dep.clone(),
                        task_name.clone(),
                    ));
                }
            }
        }
        Ok(())
    }
    
    /// Get task by name
    pub fn get_task(&self, name: &str) -> Option<&Task> {
        self.tasks.get(name)
    }
    
    /// Get all task names (excluding hidden ones)
    pub fn get_visible_tasks(&self) -> Vec<String> {
        self.tasks
            .iter()
            .filter(|(_, task)| !task.hidden)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get task dependencies in execution order
    pub fn get_execution_order(&self, task_names: &[String]) -> Result<Vec<String>, TaskRunnerError> {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        for task_name in task_names {
            self.add_task_with_dependencies(task_name, &mut order, &mut visited)?;
        }
        
        Ok(order)
    }
    
    fn add_task_with_dependencies(
        &self,
        task_name: &str,
        order: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), TaskRunnerError> {
        if visited.contains(task_name) {
            return Ok(());
        }
        
        if let Some(task) = self.tasks.get(task_name) {
            // Add dependencies first
            for dep in &task.dependencies {
                self.add_task_with_dependencies(dep, order, visited)?;
            }
            
            // Add this task
            order.push(task_name.to_string());
            visited.insert(task_name.to_string());
        } else {
            return Err(TaskRunnerError::TaskNotFound(task_name.to_string()));
        }
        
        Ok(())
    }
} 
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskRunnerError {
    #[error("Configuration file not found")]
    ConfigNotFound,
    
    #[error("Failed to parse configuration file: {0}")]
    ConfigParseError(String),
    
    #[error("Task '{0}' not found")]
    TaskNotFound(String),
    
    #[error("Circular dependency detected in task '{0}'")]
    CircularDependency(String),
    
    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),
    
    #[error("No tasks specified")]
    NoTasksSpecified,
    
    #[error("Dependency '{0}' not found for task '{1}'")]
    DependencyNotFound(String, String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
} 
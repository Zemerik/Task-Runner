use std::collections::HashMap;
use std::env;

/// Expand environment variables in a string
pub fn expand_env_vars(s: &str, env_vars: &HashMap<String, String>) -> String {
    let mut result = s.to_string();
    
    // Replace ${VAR} and $VAR patterns
    for (key, value) in env_vars {
        let var_pattern = format!("${{{}}}", key);
        result = result.replace(&var_pattern, value);
        
        let simple_pattern = format!("${}", key);
        result = result.replace(&simple_pattern, value);
    }
    
    result
}

/// Get the current working directory as a string
pub fn get_current_dir() -> Option<String> {
    env::current_dir().ok().and_then(|path| path.to_str().map(|s| s.to_string()))
}

/// Check if a command exists in PATH
pub fn command_exists(command: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let mut cmd_path = std::path::PathBuf::from(dir);
            cmd_path.push(command);
            if cmd_path.exists() && cmd_path.is_file() {
                return true;
            }
        }
    }
    false
}

/// Parse command string into program and arguments
pub fn parse_command(command: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return (String::new(), Vec::new());
    }
    
    let program = parts[0].to_string();
    let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
    
    (program, args)
}

/// Format duration for display
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if secs > 0 {
        format!("{}.{:03}s", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Validate task name (alphanumeric, hyphens, underscores only)
pub fn is_valid_task_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Get platform-specific shell command
pub fn get_shell_command() -> String {
    if cfg!(target_os = "windows") {
        "cmd".to_string()
    } else {
        "sh".to_string()
    }
}

/// Get platform-specific shell arguments
pub fn get_shell_args() -> Vec<String> {
    if cfg!(target_os = "windows") {
        vec!["/C".to_string()]
    } else {
        vec!["-c".to_string()]
    }
}

 
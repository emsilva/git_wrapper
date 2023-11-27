// src/main.rs

// Import necessary modules from the standard library and external crates
use serde::{Serialize, Deserialize}; // Importing both Serialize and Deserialize
use std::env;                         // To work with environment variables and arguments
use std::fs;                          // For file operations like reading files
use std::path::Path;                  // For working with file paths
use std::process::{Command, exit};    // For executing external commands and handling process exit
use dirs;

// Define a struct to represent the configuration file format
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default: String,                  // Default SSH key path
    directories: Vec<DirectoryConfig>, // List of directory-specific configurations
}

// Define a struct for each directory configuration
#[derive(Debug, Serialize, Deserialize)]
struct DirectoryConfig {
    path: String, // Directory path
    key: String,  // SSH key path for this directory
}

// Function to load and parse the configuration file
fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;       // Read the contents of the file
    let config = serde_yaml::from_str(&contents)?;  // Parse the YAML content into a Config struct
    Ok(config)                                      // Return the parsed configuration
}

// Function to determine the appropriate SSH key based on the current directory
fn determine_ssh_key(config: &Config) -> String {
    let current_dir = env::current_dir().unwrap();  // Get the current working directory
    for dir_config in &config.directories {
        if current_dir.starts_with(&dir_config.path) {  // Check if current directory matches any in the config
            return dir_config.key.clone();              // Return the corresponding SSH key
        }
    }
    config.default.clone()  // If no match is found, return the default SSH key
}

// The main function of the program
fn main() {
    // Construct the full path to the .git_ssh_ids file
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_path = home_dir.join(".git_ssh_ids");  // Join the filename with the home directory path

    let config = load_config(config_path)
        .expect("Failed to load config");

    // Determine the SSH key to use based on the current directory
    let ssh_key = determine_ssh_key(&config);

    // Collect the command-line arguments passed to the program
    let args: Vec<String> = env::args().collect();
    let git_command = &args[1..]; // Ignore the first argument (program name)

    // Execute the git command with the modified GIT_SSH_COMMAND environment variable
    let status = Command::new("git")
        .env("GIT_SSH_COMMAND", format!("ssh -i {} -o IdentitiesOnly=yes", ssh_key))  // Set the GIT_SSH_COMMAND
        .args(git_command)  // Pass the remaining arguments to git
        .status()           // Execute the command and get its status
        .expect("Failed to execute git");  // Handle any errors in executing git

    // Exit the program with the same exit code as the git command
    exit(status.code().unwrap_or_default());
}

#[test]
fn test_determine_ssh_key() {
    let config = Config {
        default: "default_key".to_string(),
        directories: vec![
            DirectoryConfig {
                path: "test/path".to_string(),
                key: "test_key".to_string(),
            },
        ],
    };

    // Simulate being in the "test/path" directory
    env::set_current_dir("test/path").unwrap();
    let key = determine_ssh_key(&config);
    assert_eq!(key, "test_key");

    // Test the default case
    env::set_current_dir("some/other/path").unwrap();
    let key = determine_ssh_key(&config);
    assert_eq!(key, "default_key");
}
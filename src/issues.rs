//! Issue management module for tracking and resolving issues
use crate::tomlrw::{Config, Issue};

/// List all issues in the configuration
pub fn list_issues(config: &Config) {
    if config.issues.is_empty() {
        println!("No issues tracked.");
        return;
    }

    println!("Tracked Issues:");
    println!("=============");
    for issue in &config.issues {
        println!("{}", issue);
    }
}

/// Add a new issue manually
pub fn add_issue(config: &mut Config, title: String, description: Option<String>, severity: Option<String>) {
    let issue = Issue {
        id: config.next_issue_id,
        title,
        description,
        severity: severity.unwrap_or_else(|| "Medium".to_string()),
        status: "Open".to_string(),
        created_by: "Manual".to_string(),
    };
    
    config.next_issue_id += 1;
    config.issues.push(issue);
    println!("Issue #{} added successfully.", config.next_issue_id - 1);
}

/// Resolve an issue by marking it as resolved
pub fn resolve_issue(config: &mut Config, id: u32) {
    for issue in &mut config.issues {
        if issue.id == id {
            issue.status = "Resolved".to_string();
            println!("Issue #{} marked as resolved.", id);
            return;
        }
    }
    println!("Issue #{} not found.", id);
}

/// Show detailed information about a specific issue
pub fn show_issue(config: &Config, id: u32) {
    for issue in &config.issues {
        if issue.id == id {
            println!("Issue #{}", issue.id);
            println!("Title: {}", issue.title);
            println!("Severity: {}", issue.severity);
            println!("Status: {}", issue.status);
            println!("Created by: {}", issue.created_by);
            if let Some(description) = &issue.description {
                println!("Description: {}", description);
            }
            return;
        }
    }
    println!("Issue #{} not found.", id);
}
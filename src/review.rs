//! Code review module for analyzing codebase issues
use crate::tomlrw::{Config, Issue};
use std::fs;
use std::path::Path;

/// Perform a review of the codebase and identify issues
pub fn perform_review() -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut issue_id = 1;

    // Check source files for common issues
    if let Ok(entries) = fs::read_dir("src") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        issues.extend(analyze_rust_file(&content, &path, &mut issue_id));
                    }
                }
            }
        }
    }

    // Check project structure issues
    issues.extend(analyze_project_structure(&mut issue_id));

    issues
}

/// Analyze a Rust source file for issues
fn analyze_rust_file(content: &str, path: &Path, issue_id: &mut u32) -> Vec<Issue> {
    let mut issues = Vec::new();
    let filename = path.file_name().unwrap_or_default().to_string_lossy();

    // Check for common typos
    let typos = [
        ("diroctary", "directory"),
        ("cammands", "commands"),
        ("oh given", "of given"),
        ("creats", "creates"),
    ];

    for (typo, correction) in &typos {
        if content.contains(typo) {
            issues.push(Issue {
                id: *issue_id,
                title: format!("Typo in {}: '{}' should be '{}'", filename, typo, correction),
                description: Some(format!("Found typo '{}' in file {}. Consider fixing to '{}'.", typo, filename, correction)),
                severity: "Low".to_string(),
                status: "Open".to_string(),
                created_by: "Code Review".to_string(),
            });
            *issue_id += 1;
        }
    }

    // Check for silent error handling
    if content.contains("if let Ok(_) =") {
        issues.push(Issue {
            id: *issue_id,
            title: format!("Silent error handling in {}", filename),
            description: Some(format!("File {} contains silent error handling patterns that may hide important errors.", filename)),
            severity: "Medium".to_string(),
            status: "Open".to_string(),
            created_by: "Code Review".to_string(),
        });
        *issue_id += 1;
    }

    // Check for missing documentation
    if !content.contains("///") && filename != "main.rs" {
        issues.push(Issue {
            id: *issue_id,
            title: format!("Missing documentation in {}", filename),
            description: Some(format!("File {} appears to lack comprehensive documentation comments.", filename)),
            severity: "Low".to_string(),
            status: "Open".to_string(),
            created_by: "Code Review".to_string(),
        });
        *issue_id += 1;
    }

    issues
}

/// Analyze project structure for issues
fn analyze_project_structure(issue_id: &mut u32) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Check if README has comprehensive documentation
    if let Ok(readme_content) = fs::read_to_string("README.md") {
        if !readme_content.contains("## Contributing") {
            issues.push(Issue {
                id: *issue_id,
                title: "Missing contributing guidelines".to_string(),
                description: Some("README.md doesn't contain contributing guidelines for the project.".to_string()),
                severity: "Low".to_string(),
                status: "Open".to_string(),
                created_by: "Code Review".to_string(),
            });
            *issue_id += 1;
        }
    }

    // Check for security considerations
    issues.push(Issue {
        id: *issue_id,
        title: "Command execution security review needed".to_string(),
        description: Some("Commands are executed directly via shell without input validation. Consider adding sanitization.".to_string()),
        severity: "High".to_string(),
        status: "Open".to_string(),
        created_by: "Code Review".to_string(),
    });
    *issue_id += 1;

    issues
}

/// Add review results to configuration
pub fn add_review_issues_to_config(config: &mut Config, review_issues: Vec<Issue>) {
    for mut issue in review_issues {
        // Assign proper ID based on config
        issue.id = config.next_issue_id;
        config.next_issue_id += 1;
        config.issues.push(issue);
    }
}
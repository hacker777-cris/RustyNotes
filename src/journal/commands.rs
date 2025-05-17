// src/journal/commands.rs
use std::{error::Error, fs::{self, File}, io::{self, Write}, path::PathBuf, process::Command};

use chrono::Local;
use colored::*;

use crate::{app::App, config::settings::{get_editor, get_journal_dir}};

pub fn open_editor(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        let mut file = File::create(path)?;
        let template = format!("# Journal Entry: {}\n\n", Local::now().format("%Y-%m-%d"));
        file.write_all(template.as_bytes())?;
    }
    
    let editor = get_editor()?;
    Command::new(editor)
        .arg(path)
        .status()
        .expect("Failed to open editor");
    
    Ok(())
}

pub fn create_new_entry(app: &mut App) -> Result<(), Box<dyn Error>> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let path = get_journal_dir().join(format!("{date}.md"));
    open_editor(&path)?;
    app.set_status("Created new entry for today");
    Ok(())
}

pub fn open_entry(date: &str) -> Result<(), Box<dyn Error>> {
    let path = get_journal_dir().join(format!("{date}.md"));
    open_editor(&path)?;
    Ok(())
}

pub fn delete_entry(date: String, app: &mut App) -> Result<(), Box<dyn Error>> {
    let path = get_journal_dir().join(format!("{date}.md"));
    if path.exists() {
        fs::remove_file(&path)?;
        app.set_status(&format!("Deleted entry for {}", date));
    }
    Ok(())
}

pub fn view_entry_cli(date: &str) -> Result<(), Box<dyn Error>> {
    let path = get_journal_dir().join(format!("{date}.md"));
    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        println!(
            "{}\n{}\n{}",
            format!("📖 Entry for {date}").bold().green(),
            "─".repeat(50),
            contents
        );
    } else {
        println!("{}", "No entry for this date.".red());
    }
    Ok(())
}

pub fn delete_entry_cli(date: &str) -> Result<(), Box<dyn Error>> {
    let path = get_journal_dir().join(format!("{date}.md"));
    if path.exists() {
        println!(
            "{}",
            format!("Are you sure you want to delete entry for {}? (y/N): ", date).yellow()
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "y" {
            fs::remove_file(&path)?;
            println!("{}", "✓ Entry deleted.".green());
        } else {
            println!("{}", "Deletion cancelled.".blue());
        }
    } else {
        println!("{}", "No entry to delete.".red());
    }
    Ok(())
}

pub fn display_entries_list() -> Result<(), Box<dyn Error>> {
    let journal_dir = get_journal_dir();
    println!("{}", "📅 Journal Entries".bold().blue());
    println!("{}", "─".repeat(50));
    
    let mut entries = Vec::new();
    for entry in fs::read_dir(&journal_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Only process .md files
        if path.extension().unwrap_or_default() == "md" {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    let date = name_str.replace(".md", "");
                    
                    // Get file metadata for sorting by date
                    if let Ok(metadata) = fs::metadata(&path) {
                        entries.push((date, metadata.len()));
                    }
                }
            }
        }
    }
    
    // Sort entries by date (newest first)
    entries.sort_by(|a, b| b.0.cmp(&a.0));
    
    if entries.is_empty() {
        println!("{}", "No journal entries found.".yellow());
    } else {
        // Find the longest date string for alignment
        let _max_len = entries.iter().map(|(date, _)| date.len()).max().unwrap_or(10);
        
        for (date, size) in &entries {
            let size_str = format!("({} bytes)", size);
            println!("• {} {}", date.bold(), size_str.bright_black());
        }
        
        println!("\n{} entries found", entries.len());
    }
    
    Ok(())
}

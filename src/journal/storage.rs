// src/journal/storage.rs
use std::{error::Error, fs::{self, File}, io::Read};

use crate::{app::App, config::settings::get_journal_dir, journal::entry::{JournalEntry, StatefulList}};

pub fn load_entries() -> Result<Vec<JournalEntry>, Box<dyn Error>> {
    let journal_dir = get_journal_dir();
    let mut entries = Vec::new();

    if let Ok(entries_iter) = fs::read_dir(&journal_dir) {
        for entry in entries_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                // Only process .md files
                if path.extension().unwrap_or_default() == "md" {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            let date = name_str.replace(".md", "");
                            
                            // Get file metadata
                            if let Ok(metadata) = fs::metadata(&path) {
                                let size = metadata.len();
                                
                                // Read content
                                let mut content = String::new();
                                if let Ok(mut file) = File::open(&path) {
                                    let _ = file.read_to_string(&mut content);
                                }
                                
                                entries.push(JournalEntry {
                                    date,
                                    size,
                                    content,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort entries by date (newest first)
    entries.sort_by(|a, b| b.date.cmp(&a.date));
    
    Ok(entries)
}

pub fn refresh_entries(app: &mut App) -> Result<(), Box<dyn Error>> {
    let entries = load_entries()?;
    app.entries_list = StatefulList::with_items(entries);
    Ok(())
}

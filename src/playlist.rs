// src/playlist.rs
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Playlist {
    pub items: Vec<String>,
    pub current_index: Option<usize>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: None,
        }
    }

    // Add a song to the list
    pub fn add(&mut self, path: String) {
        self.items.push(path);
        // If it's the first song, set it as current
        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
    }

    // Get the path of the current song
    pub fn get_current(&self) -> Option<&String> {
        self.current_index.and_then(|idx| self.items.get(idx))
    }

    // Move to the next song (returns the path if it exists)
    pub fn next(&mut self) -> Option<String> {
        if let Some(idx) = self.current_index {
            if idx + 1 < self.items.len() {
                self.current_index = Some(idx + 1);
                return self.get_current().cloned();
            }
        }
        None
    }

    // Move to the previous song
    pub fn previous(&mut self) -> Option<String> {
        if let Some(idx) = self.current_index {
            if idx > 0 {
                self.current_index = Some(idx - 1);
                return self.get_current().cloned();
            }
        }
        None
    }

    // Select a specific song by index
    pub fn select(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            self.current_index = Some(index);
            return self.get_current().cloned();
        }
        None
    }

    // Remove a song from the list
    pub fn remove(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            // Fix the current index if needed
            if let Some(curr) = self.current_index {
                if curr >= self.items.len() && !self.items.is_empty() {
                    self.current_index = Some(self.items.len() - 1);
                } else if self.items.is_empty() {
                    self.current_index = None;
                }
            }
        }
    }

    // Save playlist to an M3U file
    pub fn save_m3u(&self, file_path: &Path) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;
        writeln!(file, "#EXTM3U")?;
        for item in &self.items {
            writeln!(file, "{}", item)?;
        }
        Ok(())
    }

    // Load playlist from an M3U file
    pub fn load_m3u(file_path: &Path) -> std::io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut items = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() && !line.starts_with('#') {
                items.push(line);
            }
        }

        let current_index = if items.is_empty() { None } else { Some(0) };

        Ok(Self {
            items,
            current_index,
        })
    }
}

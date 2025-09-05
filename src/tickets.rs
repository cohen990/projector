use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::file_system::{self, StoredOnFileSystem};

#[derive(Serialize, Deserialize, Clone)]
pub struct TicketMetadata {
    pub number: usize,
    pub title: String,
}

impl TicketMetadata {
    pub fn new(number: usize, title: String) -> Self {
        Self { number, title }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub number: usize,
    pub title: String,
    pub description: String,
    pub comments: Vec<String>,
    pub status: TicketStatus,
}

impl Ticket {
    pub fn save_new(number: usize, title: String, description: String) -> Self {
        let ticket = Self {
            number,
            title,
            description,
            comments: Vec::new(),
            status: TicketStatus::Open,
        };
        file_system::save_to_new_file(&ticket);
        ticket
    }

    pub fn load(number: usize) -> Self {
        file_system::read_to_struct(&format!("{number}.yaml"))
    }

    pub fn save(&self) {
        file_system::update_file(self);
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TicketStatus {
    Open,
    Closed,
}

impl Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketStatus::Open => f.write_str("Open"),
            TicketStatus::Closed => f.write_str("Closed"),
        }
    }
}

impl StoredOnFileSystem for Ticket {
    fn get_file_name(&self) -> PathBuf {
        Path::new(&format!("{}.yaml", self.number)).to_path_buf()
    }
}

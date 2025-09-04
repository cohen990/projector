use serde::{Deserialize, Serialize};

use crate::file_system;

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
        file_system::save_to_new_file(&format!("{}.yaml", ticket.number), &ticket);
        ticket
    }

    pub fn load(number: usize) -> Self {
        file_system::read_to_struct(&format!("{number}.yaml"))
    }

    pub fn save(&self) {
        file_system::update_file(&format!("{}.yaml", self.number), self);
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TicketStatus {
    Open,
    Closed,
}

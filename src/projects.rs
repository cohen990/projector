use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    file_system::{self, StoredOnFileSystem},
    tickets::{TicketMetadata, TicketStatus},
};

const DEFAULT_FILE_NAME: &str = "project.yaml";

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub file_name: PathBuf,
    pub name: String,
    pub ticket_pointer: usize,
    pub open_tickets: Vec<TicketMetadata>,
    pub closed_tickets: Vec<TicketMetadata>,
}

impl Project {
    pub fn save_new(name: String) -> Self {
        let project = Self {
            file_name: Path::new(DEFAULT_FILE_NAME).to_owned(),
            name,
            ticket_pointer: 1,
            open_tickets: Vec::new(),
            closed_tickets: Vec::new(),
        };
        file_system::save_to_new_file(&project);
        project
    }

    pub fn load() -> Self {
        file_system::read_to_struct(DEFAULT_FILE_NAME)
    }

    pub fn save(&self) {
        file_system::update_file(self);
    }

    pub(crate) fn change_status(
        &mut self,
        number: usize,
        old_status: TicketStatus,
        status: TicketStatus,
    ) {
        let old_ticket = match old_status {
            TicketStatus::Open => {
                let (index, borrowed_ticket) = self
                    .open_tickets
                    .iter()
                    .enumerate()
                    .find(|(_, ticket)| ticket.number == number)
                    .unwrap();
                let ticket = borrowed_ticket.to_owned();
                self.open_tickets.remove(index);
                ticket
            }
            TicketStatus::Closed => {
                let (index, borrowed_ticket) = self
                    .closed_tickets
                    .iter()
                    .enumerate()
                    .find(|(_, ticket)| ticket.number == number)
                    .unwrap();
                let ticket = borrowed_ticket.to_owned();
                self.closed_tickets.remove(index);
                ticket
            }
        };

        match status {
            TicketStatus::Open => self.open_tickets.push(old_ticket),
            TicketStatus::Closed => self.closed_tickets.push(old_ticket),
        }

        self.save();
    }
}

impl StoredOnFileSystem for Project {
    fn get_file_name(&self) -> PathBuf {
        self.file_name.to_owned()
    }
}

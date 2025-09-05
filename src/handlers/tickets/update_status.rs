use crate::{
    file_system::StoredOnFileSystem,
    git, printer,
    projects::Project,
    tickets::{Ticket, TicketStatus},
};

pub fn handle(number: usize, status: String) {
    let mut ticket = Ticket::load(number);

    let mut project = Project::load();

    let old_status = ticket.status;

    if !status.is_empty() {
        if status.to_ascii_lowercase() == "open" {
            ticket.status = TicketStatus::Open;
            project.change_status(ticket.number, old_status, ticket.status)
        }
        if status.to_ascii_lowercase() == "closed" {
            ticket.status = TicketStatus::Closed;
            project.change_status(ticket.number, old_status, ticket.status)
        }
    }

    ticket.save();
    printer::print(&ticket);
    git::commit_and_push(
        &[
            project.get_file_name().as_path(),
            ticket.get_file_name().as_path(),
        ],
        &format!("Updated status for ticket: {number}. {old_status} -> {status}"),
    );
}

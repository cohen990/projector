use crate::{
    file_system::StoredOnFileSystem,
    git, printer,
    projects::Project,
    tickets::{Ticket, TicketMetadata},
};

pub fn handle(title: String, description: String) {
    let mut project = Project::load();

    let ticket = Ticket::save_new(project.ticket_pointer, title, description);

    printer::print(&ticket);

    let ticket_metadata = TicketMetadata::new(project.ticket_pointer, ticket.title.to_owned());
    project.open_tickets.push(ticket_metadata);
    project.ticket_pointer += 1;
    project.save();
    git::commit_and_push(
        &[
            ticket.get_file_name().as_path(),
            project.get_file_name().as_path(),
        ],
        &format!("Created new ticket: {} - {}", ticket.number, ticket.title),
    );
}

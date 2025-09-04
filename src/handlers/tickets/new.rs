use crate::{
    printer,
    projects::Project,
    tickets::{Ticket, TicketMetadata},
};

pub fn handle(title: String, description: String) {
    let mut project = Project::load();

    let ticket = Ticket::save_new(project.ticket_pointer, title, description);

    printer::print(&ticket);

    let ticket_metadata = TicketMetadata::new(project.ticket_pointer, ticket.title);
    project.open_tickets.push(ticket_metadata);
    project.ticket_pointer += 1;
    project.save();
}

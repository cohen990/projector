use crate::{file_system::StoredOnFileSystem, git, printer, tickets::Ticket};

pub fn handle(number: usize, comment: String) {
    let mut ticket = Ticket::load(number);

    ticket.comments.push(comment);

    ticket.save();

    printer::print(&ticket);
    git::commit_and_push(
        &[ticket.get_file_name().as_path()],
        &format!(
            "Added comment to ticket: {} - {}",
            ticket.number, ticket.title
        ),
    );
}

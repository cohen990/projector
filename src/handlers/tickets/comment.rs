use crate::{printer, tickets::Ticket};

pub fn handle(number: usize, comment: String) {
    let mut ticket = Ticket::load(number);

    ticket.comments.push(comment);

    ticket.save();

    printer::print(&ticket);
}

use crate::{printer, tickets::Ticket};

pub fn handle(number: usize) {
    let ticket = Ticket::load(number);

    printer::print(&ticket);
}

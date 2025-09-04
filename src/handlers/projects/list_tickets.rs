use crate::{printer, projects::Project, tickets::Ticket};

pub fn handle(detailed_list: bool) {
    let project = Project::load();

    let mut output: String = String::new();
    output.push_str("Open Tickets:\n");
    if detailed_list {
        for ticket in project.open_tickets {
            let ticket = Ticket::load(ticket.number);
            output.push_str(&format!(
                "{}: {} - {}\n",
                ticket.number, ticket.title, ticket.description
            ));
        }
    } else {
        for ticket in project.open_tickets {
            output.push_str(&format!("{}: {}\n", ticket.number, ticket.title));
        }
    }
    output.push_str("Closed Tickets:\n");
    if detailed_list {
        for ticket in project.closed_tickets {
            let ticket = Ticket::load(ticket.number);
            output.push_str(&format!(
                "{}: {} - {}\n",
                ticket.number, ticket.title, ticket.description
            ));
        }
    } else {
        for ticket in project.closed_tickets {
            output.push_str(&format!("{}: {}\n", ticket.number, ticket.title));
        }
    }
    printer::print_string(output);
}

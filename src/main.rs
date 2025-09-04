use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod file_system;
mod handlers;
mod printer;
mod projects;
mod test;
mod tickets;

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Init { project_name } => handlers::projects::init::handle(project_name),
        Commands::Ticket {
            number,
            comment,
            status,
            new,
            title,
            description,
        } => {
            if new {
                handlers::tickets::new::handle(title, description)
            } else if !comment.is_empty() {
                handlers::tickets::comment::handle(number, comment);
            } else if !status.is_empty() {
                handlers::tickets::update_status::handle(number, status);
            } else {
                handlers::tickets::print::handle(number)
            }
        }
        Commands::Project {
            list_tickets,
            wipe,
            detailed_list,
        } => {
            if list_tickets {
                handlers::projects::list_tickets::handle(detailed_list)
            } else if wipe {
                handlers::projects::wipe::handle();
            }
        }
    }
}

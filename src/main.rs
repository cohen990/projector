use std::{
    fs,
    io::{Write, stdin, stdout},
};

use clap::{Parser, Subcommand, command};
use serde::{Deserialize, Serialize};

mod test;

#[derive(Parser, Debug)]
#[command(name = "projector")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Clone, PartialEq, Subcommand, Debug)]
pub enum Commands {
    Init {
        project_name: String,
    },
    New {
        #[arg(long("title"), short('t'))]
        title: String,
        #[arg(long("description"), short('d'))]
        description: String,
    },
    Ticket {
        number: usize,
        #[arg(long("comment"), short('c'), default_value(""))]
        comment: String,
    },
    Project {
        #[arg(long("list-tickets"), short('l'), exclusive(true))]
        list_tickets: bool,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    ticket_pointer: usize,
    tickets: Vec<TicketMetadata>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ticket_pointer: 1,
            tickets: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TicketMetadata {
    number: usize,
    title: String,
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
}

impl Ticket {
    pub fn new(number: usize, title: String, description: String) -> Self {
        Self {
            number,
            title,
            description,
            comments: Vec::new(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Init { project_name } => {
            let _ = git2::Repository::init(std::env::current_dir().unwrap());
            let project_file = fs::File::create_new("project.yaml").unwrap();
            let project = Project::new(project_name);
            let _ = serde_yaml::to_writer(&project_file, &project);
        }
        Commands::New { title, description } => {
            let mut project: Project;
            {
                let project_file = fs::File::options().read(true).open("project.yaml").unwrap();
                project = serde_yaml::from_reader(&project_file).unwrap();
            }

            let ticket = Ticket::new(project.ticket_pointer, title, description);

            {
                let ticket_file =
                    fs::File::create_new(format!("{}.yaml", &project.ticket_pointer)).unwrap();

                let _ = serde_yaml::to_writer(&ticket_file, &ticket);
            }
            {
                let ticket_metadata = TicketMetadata::new(project.ticket_pointer, ticket.title);
                project.tickets.push(ticket_metadata);
                project.ticket_pointer += 1;
                let project_file = fs::File::options()
                    .write(true)
                    .truncate(true)
                    .open("project.yaml")
                    .unwrap();
                serde_yaml::to_writer(&project_file, &project).unwrap();
            }

            let _ = stdout().flush();
        }
        Commands::Ticket { number, comment } => {
            let mut ticket: Ticket;
            {
                let ticket_file = fs::File::options()
                    .read(true)
                    .open(format!("{}.yaml", number))
                    .unwrap();
                ticket = serde_yaml::from_reader(&ticket_file).unwrap();
            }

            if comment.is_empty() {
                let ticket_string = serde_yaml::to_string(&ticket).unwrap();
                let _ = stdout().write(ticket_string.as_bytes());
                let _ = stdout().flush();
            } else {
                ticket.comments.push(comment);

                {
                    let ticket_file = fs::File::options()
                        .write(true)
                        .truncate(true)
                        .open(format!("{}.yaml", number))
                        .unwrap();
                    let _ = serde_yaml::to_writer(&ticket_file, &ticket);
                }
            }
        }
        Commands::Project { list_tickets } => {
            let project_file = fs::File::options().read(true).open("project.yaml").unwrap();
            let project: Project = serde_yaml::from_reader(project_file).unwrap();

            let mut output: String = String::new();
            for ticket in project.tickets {
                output.push_str(&format!("{}: {}\n", ticket.number, ticket.title));
            }

            let _ = stdout().write(output.as_bytes());
            let _ = stdout().flush();
        }
    }
}

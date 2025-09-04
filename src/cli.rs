use clap::{ArgAction, Parser, Subcommand, command};

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
    Ticket {
        #[arg(conflicts_with("new"), default_value("0"))]
        number: usize,
        #[arg(
            conflicts_with("number"),
            action(ArgAction::SetTrue),
            default_value("false"),
            long
        )]
        new: bool,
        #[arg(long, short, default_value(""))]
        comment: String,
        #[arg(long, short, default_value(""))]
        status: String,
        #[arg(long, short, default_value(""), required_if_eq("new", "true"))]
        title: String,
        #[arg(long, short, default_value(""), required_if_eq("new", "true"))]
        description: String,
    },
    Project {
        #[arg(long("list-tickets"), short, conflicts_with("wipe"))]
        list_tickets: bool,
        #[arg(long, short, requires("list_tickets"), default_value("false"))]
        detailed_list: bool,
        #[arg(long, conflicts_with("list_tickets"))]
        wipe: bool,
    },
}

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(flatten)]
    pub verbosity: Verbosity,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    Edit {
        #[arg(short, long)]
        name: String,
    },
    List,
}

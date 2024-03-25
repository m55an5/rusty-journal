use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add { 
        /// The task description here.
        #[structopt()]
        task: String, 
    }, 
    /// remove any entry from the journal file by position.
    Done { 
        #[structopt()]
        position: usize,
    },
    /// List all the tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action,

    /// use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
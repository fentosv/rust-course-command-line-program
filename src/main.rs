use anyhow::anyhow;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    // Because journal_file is of type Option<PathBuf>, we need to extract the path to our journal file or emit a panic
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        // .expect("Failed to find journal file.")?; // Changed this for the next line
        .ok_or(anyhow!("Failed to find journal file."))?;
    // If the user hasn't provided a target journal file and find_default_journal_file can't find a suitable file, we cause the program to panic because it's impossible for it to do anything without a journal file.

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}

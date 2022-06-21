use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;

const NOTES_FILENAME: &str = "notes.txt";

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Test argument that does nothing
    #[clap(value_parser)]
    test: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// add a new sticky note
    Add {
        /// the note to add
        #[clap(value_parser)]
        note: String,
    },
    /// remove a sticky note
    Done {
        /// the index of the note to remove
        #[clap(value_parser)]
        index: u16,
    },
}

macro_rules! get_proj_dirs {
    () => {
        ProjectDirs::from("xyz", "moddedBear", "sticky").unwrap()
    };
}

// check for the existence of the notes file and create it if it doesn't exist
fn check_notes_file() {
    let proj_dirs = get_proj_dirs!();
    let data_dir = proj_dirs.data_dir();
    // create data directory if it doesn't exist
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir).unwrap();
    }
    let notes_file = data_dir.join(NOTES_FILENAME);
    if !notes_file.exists() {
        let mut file = File::create(notes_file).unwrap();
        file.write_all(b"").unwrap();
    }
}

// append the given line to the notes file
fn append_note(note: &String) {
    let proj_dirs = get_proj_dirs!();
    let data_dir = proj_dirs.data_dir();
    let notes_file = data_dir.join(NOTES_FILENAME);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(notes_file)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", note) {
        eprintln!("{}", e);
    }
}

fn command_add(note: &String) {
    append_note(note);
    println!("Note added!");
}

fn main() {
    let cli = Cli::parse();

    check_notes_file();

    if let Some(test) = cli.test.as_deref() {
        println!("Value for test: {}", test);
    }

    match &cli.command {
        Some(Commands::Add { note }) => command_add(note),
        Some(Commands::Done { index }) => println!("Removing note: {}", index),
        _ => println!("No command specified"),
    }
}

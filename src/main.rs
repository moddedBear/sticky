use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

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
    Remove {
        /// the index of the note to remove
        #[clap(value_parser)]
        index: u16,
    },
    /// mark a sticky note as done
    Done {
        /// the index of the note to mark as done
        #[clap(value_parser)]
        index: u16,
    },
    /// mark a sticky note as undone
    Undone {
        /// the index of the note to mark as undone
        #[clap(value_parser)]
        index: u16,
    },
    /// clear sticky notes marked as done
    Clear {
        /// clear all notes regardless of completion status
        #[clap(short, long, action)]
        all: bool,
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

fn remove_notes(indicies: &Vec<u16>) -> bool {
    let proj_dirs = get_proj_dirs!();
    let data_dir = proj_dirs.data_dir();
    let notes_file = data_dir.join(NOTES_FILENAME);
    let mut file = &OpenOptions::new()
        .read(true)
        .write(true)
        .open(notes_file)
        .unwrap();
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = buf.lines().map(|x| x.unwrap()).collect();
    for index in indicies {
        if index >= &(lines.len() as u16) {
            println!("Note index out of bounds!");
            return false;
        }
        lines.remove(*index as usize);
    }
    file.set_len(0).unwrap(); // clear file
    write!(file, "{}", lines.join("\n")).unwrap();
    return true;
}

fn command_add(note: &String) {
    append_note(note);
    println!("Note added!");
}

fn command_remove(index: &u16) {
    if *index == 0 {
        println!("Note index out of bounds!");
        return;
    }
    let adjusted_index = *index - 1;
    if remove_notes(&vec![adjusted_index]) {
        println!("Note removed!");
        return;
    };
    println!("Error removing note!");
}

fn main() {
    let cli = Cli::parse();

    check_notes_file();

    if let Some(test) = cli.test.as_deref() {
        println!("Value for test: {}", test);
    }

    match &cli.command {
        Some(Commands::Add { note }) => command_add(note),
        Some(Commands::Remove { index }) => command_remove(index),
        _ => println!("No command specified"),
    }
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;
use owo_colors::OwoColorize;

const NOTES_FILENAME: &str = "notes.txt";

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
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
    /// toggle the completion status of a note
    Done {
        /// the index of the note to mark as done/undone
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

fn remove_note(index: &u16) -> bool {
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
    if index >= &(lines.len() as u16) {
        println!("Note index out of bounds!");
        return false;
    }
    lines.remove(*index as usize);

    file.set_len(0).unwrap(); // clear file
    if lines.len() > 0 {
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        write!(file, "{}\n", lines.join("\n")).unwrap();
    }
    return true;
}

// returns 0 if marked undone, 1 if marked done, and -1 if error
fn mark_done(index: &u16) -> i8 {
    let marked_done: bool;
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

    if index >= &(lines.len() as u16) {
        println!("Note index out of bounds!");
        return -1;
    }
    let mut line = lines.remove(*index as usize);
    if line.starts_with('*') {
        // note is already done, mark as undone
        line.remove(0);
        marked_done = false;
    } else {
        line.insert(0, '*');
        marked_done = true;
    }
    lines.insert(*index as usize, line);

    file.set_len(0).unwrap(); // clear file
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    write!(file, "{}\n", lines.join("\n")).unwrap();
    if marked_done {
        return 1;
    }
    return 0;
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
    let adjusted_index = index - 1; // convert to 0-indexed
    if remove_note(&adjusted_index) {
        println!("Note removed!");
        return;
    };
    println!("Error removing note!");
}

fn command_done(index: &u16) {
    if *index == 0 {
        println!("Note index out of bounds!");
        return;
    }
    let adjusted_index = index - 1; // convert to 0-indexed
    match mark_done(&adjusted_index) {
        0 => println!("Note marked undone!"),
        1 => println!("Note marked done!"),
        _ => println!("Error toggling note completion!"),
    };
}

fn display_notes() {
    let proj_dirs = get_proj_dirs!();
    let data_dir = proj_dirs.data_dir();
    let notes_file = data_dir.join(NOTES_FILENAME);
    let file = &OpenOptions::new().read(true).open(notes_file).unwrap();
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|x| x.unwrap()).collect();
    if lines.len() == 0 {
        println!("{}\n", "No sticky notes!".bold().green());
        return;
    }
    println!("\t{}", "Sticky Notes".bold().yellow());
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with('*') {
            println!(
                "{} {}\t{}",
                (index + 1).green().bold(),
                "✅".bold().green(),
                line.strip_prefix('*').unwrap()
            );
        } else {
            println!("{}\t{}", (index + 1).yellow().bold(), line);
        }
    }
    print!("\n");
}

fn main() {
    let cli = Cli::parse();

    check_notes_file();

    match &cli.command {
        Some(Commands::Add { note }) => command_add(note),
        Some(Commands::Remove { index }) => command_remove(&index),
        Some(Commands::Done { index }) => command_done(&index),
        _ => display_notes(),
    }
}

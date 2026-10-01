use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use std::{
    env, error,
    io::{self, Write},
    process,
};

mod note;

#[derive(Parser)]
#[command(name = "ziton")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New,
    List,
    Edit { id: u8 },
    Delete { id: u8 },
}

fn read_input(label: &str) -> io::Result<String> {
    print!("{label}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_date(label: &str) -> io::Result<NaiveDate> {
    let today = Local::now().date_naive();

    loop {
        let input = read_input(label)?;
        match NaiveDate::parse_from_str(&input, "%d.%m.%Y") {
            Ok(date) if date >= today => return Ok(date),
            Ok(_) => {
                println!("Das Datum muss der heutige Tag sein oder in der Zukunft liegen!");
            }
            Err(_) => {
                println!("Ungültiges Datum!");
            }
        }
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    println!("Ziton - Version {}", env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();
    match cli.command {
        Command::New => {
            println!("Neue Notiz anlegen");

            let note_text = read_input("Text: ")?;
            let note_date = read_date("Datum (TT.MM.JJJJ): ")?;

            note::add_note(note_text, note_date)?;
        }
        Command::List => {
            println!("Notizen anzeigen");
            note::list_notes()?;
        }
        Command::Edit { id } => {
            println!("Notiz {} bearbeiten", id);

            let note_text = read_input("Neuer Text: ")?;
            let note_date = read_date("Neues Datum (TT.MM.JJJJ): ")?;

            note::update_note(id, note_text, note_date)?;
        }
        Command::Delete { id } => {
            note::delete_note(id)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fehler: {e}");
        process::exit(1);
    }
}

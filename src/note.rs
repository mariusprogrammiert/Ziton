use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{error, fs, path::Path};

const NOTES_PATH: &str = "notes.json";

#[derive(Serialize, Deserialize)]
struct Note {
    id: u8,
    text: String,
    created_at: NaiveDate,
    due_to: NaiveDate,
}

fn build_note(id: u8, text: String, created_at: NaiveDate, due_to: NaiveDate) -> Note {
    Note {
        id,
        text,
        created_at,
        due_to,
    }
}

fn load_file() -> Result<Vec<Note>, Box<dyn error::Error>> {
    if !Path::new(NOTES_PATH).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(NOTES_PATH)?;
    let notes = serde_json::from_str(&content)?;

    Ok(notes)
}

fn save_file(notes: &[Note]) -> Result<(), Box<dyn error::Error>> {
    let output = serde_json::to_string(notes)?;
    fs::write(NOTES_PATH, output)?;

    Ok(())
}

pub fn add_note(text: String, due_to: NaiveDate) -> Result<(), Box<dyn error::Error>> {
    let mut notes = load_file()?;
    let next_id = notes
        .iter()
        .map(|note| note.id)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or("Keine weitere ID verfügbar!")?;

    let note = build_note(next_id, text, Local::now().date_naive(), due_to);
    notes.push(note);
    save_file(&notes)?;
    println!("Notiz {} wurde erstellt.", next_id);

    Ok(())
}

pub fn list_notes() -> Result<(), Box<dyn error::Error>> {
    let notes = load_file()?;
    if notes.is_empty() {
        println!("Keine Notizen vorhanden!");
        return Ok(());
    }

    println!();
    for note in notes {
        println!("{:=<40}", "");
        println!(
            "{}: {}\nErstellt:   {}\nFällig bis: {}",
            note.id,
            note.text,
            note.created_at.format("%d.%m.%Y"),
            note.due_to.format("%d.%m.%Y"),
        );
    }

    println!("{:=<40}", "");

    Ok(())
}

pub fn update_note(id: u8, text: String, due_to: NaiveDate) -> Result<(), Box<dyn error::Error>> {
    let mut notes = load_file()?;
    let note = notes
        .iter_mut()
        .find(|note| note.id == id)
        .ok_or_else(|| format!("Keine Notiz mit der ID {} gefunden!", id))?;

    note.text = text;
    note.due_to = due_to;
    save_file(&notes)?;
    println!("Notiz {} wurde bearbeitet.", id);

    Ok(())
}

pub fn delete_note(id: u8) -> Result<(), Box<dyn error::Error>> {
    let mut notes = load_file()?;
    let old_len = notes.len();

    notes.retain(|note| note.id != id);
    if notes.len() == old_len {
        println!("Keine Notiz mit der ID {} gefunden!", id);
        return Ok(());
    }

    save_file(&notes)?;
    println!("Notiz {} wurde gelöscht.", id);

    Ok(())
}


use std::{fs::OpenOptions, io::{self, Write}};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;
use uuid::Uuid;

use crate::planit::input_utils::{get_note_input, get_note_type_input};

use super::{data::{fetch_month_notes, get_contents_of_month_notes_file, write_to_month_notes_file}, display::display_month_notes};

pub fn add_month_note(current_date: DateTime<Local>) {
    println!("");
    println!("A new note it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let note_type = get_note_type_input();
    let note = get_note_input();
    let note_id = String::from(Uuid::new_v4());

    let note_month = format!("{}-{}", current_date.month(), current_date.year());

    let modified_date_time = chrono::Local::now().timestamp().to_string();

    let is_complete = "false";

    let new_note = format!("{}--{}--{}--{}--{}--{}\n", note_id, note_type, note, note_month, is_complete, modified_date_time);

    let file_name = format!("{}-{}-MonthNotes.txt", current_date.month(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(file_name);

    // Add a new note
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(new_note.as_bytes())
        .expect("write failed");

    println!("\n");
    println!("Your note has been added! Time to party!");
    println!("\n");
}

pub fn delete_month_note(current_date: DateTime<Local>) {
    let month_notes_file_contents = get_contents_of_month_notes_file(current_date);

    let month_notes = fetch_month_notes(current_date);

    display_month_notes(current_date, month_notes, true);

    let note_id: String;
    loop {
        print!("Enter the note_id (Grab from the print out above): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_id_raw = String::new();
        io::stdin().read_line(&mut note_id_raw).expect("Unable to read note");

        let note_id_raw_format = String::from(note_id_raw.trim());
        if note_id_raw_format.len() > 0 {
            note_id = note_id_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a note without text bro.")
        }
    }

    // Find the line in the month_notes_file_contents
    let mut original_line = "";
    for line in month_notes_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            break;
        }
    }

    let updated_week_file_contents = month_notes_file_contents.replace(original_line, "");
    
    if updated_week_file_contents.len() == 0 {
        return;
    }

    write_to_month_notes_file(current_date, updated_week_file_contents);
}
use std::{fs::OpenOptions, io::{self, Write}};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;
use uuid::Uuid;

use crate::planit::input_utils::{get_note_input, get_note_month, get_note_type_input};

use super::{data::{fetch_year_notes, get_contents_of_year_notes_file, write_to_year_notes_file}, display::display_year_notes};

pub fn add_new_year_note(current_date: DateTime<Local>) {
    println!("");
    println!("A new note it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let note_id = String::from(Uuid::new_v4());
    let note_month = get_note_month();
    let note_type = get_note_type_input();
    let note = get_note_input();
    let note_year = current_date.year();
    let modified_date_time = chrono::Local::now().timestamp().to_string();
    let is_complete = "false";

    let note_text = format!("{}--{}--{}--{}--{}--{}--{}\n", note_id, note_type, note, note_year, is_complete, note_month, modified_date_time);

    let current_year_note_file_name = format!("{}-YearNotes.txt", current_date.year());
    let year_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join(current_year_note_file_name);

    // Add a new note
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(year_file_path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(note_text.as_bytes())
        .expect("write failed");

    println!("\n");
    println!("Your note has been added! Time to party!");
    println!("\n");
}

pub fn mark_year_note_complete(current_date: DateTime<Local>) {
    let year_file_contents = get_contents_of_year_notes_file(current_date);

    let year_notes = fetch_year_notes(current_date);

    display_year_notes(year_notes, true, current_date);

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

    let mut updated_year_file_contents = String::from("");
    for line in year_file_contents.lines() {
        if line.contains(&note_id) {
            let mut updated_line_vector: Vec<&str> = line.split("--").collect();
            updated_line_vector[4] = "true";

            let updated_line_string = updated_line_vector.join("--");

            updated_year_file_contents = year_file_contents.replace(line, &updated_line_string);
        }
    }

    if updated_year_file_contents.len() == 0 {
        return;
    }

    write_to_year_notes_file(current_date, updated_year_file_contents);
}

pub fn edit_year_note(current_date: DateTime<Local>) {
    let year_file_contents = get_contents_of_year_notes_file(current_date);

    let year_notes = fetch_year_notes(current_date);

    display_year_notes(year_notes, true, current_date);

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

    let mut updated_line_vector: Vec<&str> = vec![];
    let mut original_line = "";
    // Find the line in the week_file_contents
    for line in year_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            updated_line_vector = line.split("--").collect();
        }
    }

    let note_month = get_note_month();
    let note_type = get_note_type_input();
    let note = get_note_input();

    updated_line_vector[5] = &note_month;
    updated_line_vector[1] = &note_type;
    updated_line_vector[2] = &note;

    let updated_line_string = updated_line_vector.join("--");

    let updated_year_file_contents = year_file_contents.replace(original_line, &updated_line_string);
    
    if updated_year_file_contents.len() == 0 {
        return;
    }

    write_to_year_notes_file(current_date, updated_year_file_contents);
}

pub fn delete_year_note(current_date: DateTime<Local>) {
    let year_file_contents = get_contents_of_year_notes_file(current_date);

    let year_notes = fetch_year_notes(current_date);

    display_year_notes(year_notes, true, current_date);

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

    // Find the line in the week_file_contents
    let mut original_line = "";
    for line in year_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            break;
        }
    }

    let updated_year_file_contents = year_file_contents.replace(original_line, "");
    
    if updated_year_file_contents.len() == 0 {
        return;
    }

    write_to_year_notes_file(current_date, updated_year_file_contents);
}

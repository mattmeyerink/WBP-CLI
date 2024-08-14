use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use dirs::home_dir;
use uuid::Uuid;

use crate::planit::input_utils::get_day_of_week_input;
use crate::planit::input_utils::get_note_input;
use crate::planit::input_utils::get_note_type_input;

use super::data::fetch_week_notes;
use super::data::get_contents_of_week_notes_file;
use super::display::display_week_notes;
use super::data::write_to_week_notes_file;

pub fn add_new_week_note(current_date: DateTime<Local>) {
    println!("");
    println!("A new note it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let day_of_week = get_day_of_week_input();
    let note_type = get_note_type_input();
    let note = get_note_input();
    let note_id = String::from(Uuid::new_v4());

    let note_date_object = current_date + Duration::days(day_of_week.parse::<i64>().unwrap());
    let date = format!("{}-{}-{}", note_date_object.month(), note_date_object.day(), note_date_object.year());

    let modified_date_time = chrono::Local::now().timestamp().to_string();

    let is_complete = "false";

    let new_note = format!("{}--{}--{}--{}--{}--{}--{}\n", note_id, date, day_of_week, note_type, is_complete, note, modified_date_time);

    let file_name = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("week-notes").join(file_name);

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

pub fn mark_week_note_completed(current_date: DateTime<Local>) {
    let week_file_contents = get_contents_of_week_notes_file(current_date);

    let week_notes = fetch_week_notes(current_date);

    display_week_notes(week_notes, true, current_date);

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

    let mut updated_week_file_contents = String::from("");
    for line in week_file_contents.lines() {
        if line.contains(&note_id) {
            let mut updated_line_vector: Vec<&str> = line.split("--").collect();
            updated_line_vector[4] = "true";

            let updated_line_string = updated_line_vector.join("--");

            updated_week_file_contents = week_file_contents.replace(line, &updated_line_string);
        }
    }

    if updated_week_file_contents.len() == 0 {
        return;
    }

    write_to_week_notes_file(current_date, updated_week_file_contents);
}

pub fn edit_note(current_date: DateTime<Local>) {
    let week_file_contents = get_contents_of_week_notes_file(current_date);

    let week_notes = fetch_week_notes(current_date);

    display_week_notes(week_notes, true, current_date);

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
    for line in week_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            updated_line_vector = line.split("--").collect();
        }
    }

    let day_of_week = get_day_of_week_input();
    let note_type = get_note_type_input();
    let note = get_note_input();

    updated_line_vector[2] = &day_of_week;
    updated_line_vector[3] = &note_type;
    updated_line_vector[5] = &note;

    let updated_line_string = updated_line_vector.join("--");

    let updated_week_file_contents = week_file_contents.replace(original_line, &updated_line_string);
    
    if updated_week_file_contents.len() == 0 {
        return;
    }

    write_to_week_notes_file(current_date, updated_week_file_contents);
}


pub fn delete_week_note(current_date: DateTime<Local>) {
    let week_file_contents = get_contents_of_week_notes_file(current_date);

    let week_notes = fetch_week_notes(current_date);

    display_week_notes(week_notes, true, current_date);

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
    for line in week_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            break;
        }
    }

    let updated_week_file_contents = week_file_contents.replace(original_line, "");
    
    if updated_week_file_contents.len() == 0 {
        return;
    }

    write_to_week_notes_file(current_date, updated_week_file_contents);
}

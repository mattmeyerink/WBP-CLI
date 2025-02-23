
use std::{fs::OpenOptions, io::{self, Write}};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;
use uuid::Uuid;

use crate::planit::input_utils::{get_highlight_day_input, get_highlight_input, get_note_input, get_note_type_input};

use super::{data::{fetch_month_highlights, fetch_month_notes, get_contents_of_month_highlights_file, get_contents_of_month_notes_file, write_to_month_highlights_file, write_to_month_notes_file}, display::{display_month_highlights, display_month_notes}};

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
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-lists").join(file_name);

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

pub fn add_month_highlight(current_date: DateTime<Local>) {
    println!("");
    println!("A new highlight it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let highlight_day = get_highlight_day_input(current_date);
    let highlight = get_highlight_input();

    let new_highlight = format!("{}--{}\n", highlight_day, highlight);

    // Determine if there is already a highlight saved for that day
    let month_highlight_file_contents = get_contents_of_month_highlights_file(current_date);
    let mut day_highlight_to_overwrite = "";
    for highlight_line in month_highlight_file_contents.lines() {
        let current_highlight_day = highlight_line.split("--").next().unwrap();
        if String::from(current_highlight_day) == highlight_day {
            day_highlight_to_overwrite = highlight_line;
            break;
        }
    }

    // If there is not a highlight saved for that day append the new highlight. Else overwrite the old highlight.
    if day_highlight_to_overwrite == "" {
        let current_month_highlights_file_name = format!("{}-{}-MonthHighlights.txt", current_date.month(), current_date.year());
        let month_highlights_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-highlights").join(current_month_highlights_file_name);

        let mut data_file = OpenOptions::new()
            .append(true)
            .open(month_highlights_file_path)
            .expect("cannot open file");

        // Write to a file
        data_file
            .write(new_highlight.as_bytes())
            .expect("write failed");
    } else {
        let updated_month_highlight_file_contents = month_highlight_file_contents.replace(day_highlight_to_overwrite, &new_highlight);

        write_to_month_highlights_file(current_date, updated_month_highlight_file_contents);
    }

    println!("\n");
    println!("Your highlight has been added! Time to party!");
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

pub fn delete_month_highlight(current_date: DateTime<Local>) {
    let month_highlight_file_contents = get_contents_of_month_highlights_file(current_date);

    let month_highlights = fetch_month_highlights(current_date);

    display_month_highlights(current_date, month_highlights);

    let highlight_day = get_highlight_day_input(current_date);

    let mut line_to_delete = "";
    for highlight_line in month_highlight_file_contents.lines() {
        let current_highlight_day = highlight_line.split("--").next().unwrap();
        if String::from(current_highlight_day) == highlight_day {
            line_to_delete = highlight_line;
            break;
        }
    }

    let updated_month_highlight_file_contents = month_highlight_file_contents.replace(line_to_delete, "");

    write_to_month_highlights_file(current_date, updated_month_highlight_file_contents);
}

pub fn mark_month_note_complete(current_date: DateTime<Local>) {
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

    let mut updated_month_notes_file_contents = String::from("");
    for line in month_notes_file_contents.lines() {
        if line.contains(&note_id) {
            let mut updated_line_vector: Vec<&str> = line.split("--").collect();
            updated_line_vector[4] = "true";

            let updated_line_string = updated_line_vector.join("--");

            updated_month_notes_file_contents = month_notes_file_contents.replace(line, &updated_line_string);
        }
    }

    if updated_month_notes_file_contents.len() == 0 {
        return;
    }

    write_to_month_notes_file(current_date, updated_month_notes_file_contents);
}

pub fn edit_month_note(current_date: DateTime<Local>) {
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

    let mut updated_line_vector: Vec<&str> = vec![];
    let mut original_line = "";
    // Find the line in the week_file_contents
    for line in month_notes_file_contents.lines() {
        if line.contains(&note_id) {
            original_line = line;
            updated_line_vector = line.split("--").collect();
        }
    }

    let note_type = get_note_type_input();
    let note = get_note_input();

    updated_line_vector[1] = &note_type;
    updated_line_vector[2] = &note;

    let updated_line_string = updated_line_vector.join("--");

    let updated_month_notes_file_contents = month_notes_file_contents.replace(original_line, &updated_line_string);
    
    if updated_month_notes_file_contents.len() == 0 {
        return;
    }

    write_to_month_notes_file(current_date, updated_month_notes_file_contents);
}

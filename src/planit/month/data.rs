use std::{fs::{self, OpenOptions}, io::Write};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;

pub struct MonthNote {
    pub note_id: String,
    pub note_type: String,
    pub note: String,
    pub note_month: String,
    pub is_complete: String,
    pub modified_date_time: String
}

pub fn get_contents_of_month_notes_file(current_date: DateTime<Local>) -> String {
    let current_month_note_file_name = format!("{}-{}-MonthNotes.txt", current_date.month(), current_date.year());

    // Attempt to pull the text file that has this month's notes
    let month_notes_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_month_note_file_name);
    if !month_notes_file_path.exists() {
        std::fs::File::create_new(&month_notes_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(month_notes_file_path).expect("Should have been able to read the file");

    return contents;
}

pub fn fetch_month_notes(current_date: DateTime<Local>) -> Vec<MonthNote> {
    let contents = get_contents_of_month_notes_file(current_date);

    let mut month_notes: Vec<MonthNote>  = Vec::new();

    for line in contents.lines() {
        let raw_month_note_array: Vec<&str> = line.split("--").collect();

        if raw_month_note_array.len() < 6 {
            continue;
        }
        
        let month_note = MonthNote {
            note_id: String::from(raw_month_note_array[0]),
            note_type: String::from(raw_month_note_array[1]),
            note: String::from(raw_month_note_array[2]),
            note_month: String::from(raw_month_note_array[3]),
            is_complete: String::from(raw_month_note_array[4]),
            modified_date_time: String::from(raw_month_note_array[5])
        };

        month_notes.push(month_note);
    }

    return month_notes;
}

pub fn write_to_month_notes_file(current_date: DateTime<Local>, updated_file_contents: String) {
    let file_name = format!("{}-{}-MonthNotes.txt", current_date.month(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(file_name);

    // Add a new note
    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(updated_file_contents.as_bytes())
        .expect("write failed");
}

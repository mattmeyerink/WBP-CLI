use std::{fs::{self, OpenOptions}, io::Write};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;

use crate::planit::utils;

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
    let month_notes_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-lists").join(current_month_note_file_name);
    if !month_notes_file_path.exists() {
        std::fs::File::create_new(&month_notes_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(month_notes_file_path).expect("Should have been able to read the file");

    return contents;
}

pub fn fetch_month_notes(current_date: DateTime<Local>) -> Vec<MonthNote> {
    let contents = get_contents_of_month_notes_file(current_date);

    let mut month_notes: Vec<MonthNote> = Vec::new();

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
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-lists").join(file_name);

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

pub fn write_to_month_highlights_file(current_date: DateTime<Local>, updated_file_contents: String) {
    let file_name = format!("{}-{}-MonthHighlights.txt", current_date.month(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-highlights").join(file_name);

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

pub fn get_contents_of_month_highlights_file(current_date: DateTime<Local>) -> String {
    let current_month_highlights_file_name = format!("{}-{}-MonthHighlights.txt", current_date.month(), current_date.year());

    // Attempt to pull the text file that has this month's notes
    let month_highlights_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("month-notes").join("month-highlights").join(current_month_highlights_file_name);
    if !month_highlights_file_path.exists() {
        std::fs::File::create_new(&month_highlights_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(month_highlights_file_path).expect("Should have been able to read the file");

    return contents;
}

pub fn fetch_month_highlights(current_date: DateTime<Local>) -> Vec<String> {
    let contents = get_contents_of_month_highlights_file(current_date);

    let days_in_current_month = usize::try_from(utils::get_number_of_days_in_month(current_date)).unwrap();
    let mut month_highlights: Vec<String>  = vec![String::new(); days_in_current_month];

    for line in contents.lines() {
        let raw_month_highlight_array: Vec<&str> = line.split("--").collect();

        if raw_month_highlight_array.len() < 2 {
            continue;
        }

        // Days are saved unzeroed but they need to be zeroed for indexing
        let highlight_day_raw_int = raw_month_highlight_array[0].parse::<u32>().unwrap() - 1;
        let highlight_day = usize::try_from(highlight_day_raw_int).unwrap();
        let highlight_text = String::from(raw_month_highlight_array[1]);

        month_highlights[highlight_day] = highlight_text;
    }

    return month_highlights;
}

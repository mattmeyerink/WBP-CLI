use std::{collections::HashMap, fs::{self, OpenOptions}, io::Write};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;

pub struct YearNote {
    pub note_id: String,
    pub note_type: String,
    pub note: String,
    pub note_year: String,
    pub is_complete: String,
    pub note_month: String,
    pub modified_date_time: String
}

pub fn get_contents_of_year_notes_file(current_date: DateTime<Local>) -> String {
    let current_year_note_file_name = format!("{}-YearNotes.txt", current_date.year());

    // Attempt to pull the text file that has this weeks notes
    let year_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_year_note_file_name);
    if !year_file_path.exists() {
        std::fs::File::create_new(&year_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(year_file_path).expect("Should have been able to read the file");

    return contents;
}

pub fn fetch_year_notes(current_date: DateTime<Local>) -> HashMap<String, Vec<YearNote>> {
    let year_note_file_contents = get_contents_of_year_notes_file(current_date);

    let mut year_notes: HashMap<String, Vec<YearNote>>  = HashMap::from([
        (String::from("1"), Vec::new()),
        (String::from("2"), Vec::new()),
        (String::from("3"), Vec::new()),
        (String::from("4"), Vec::new()),
        (String::from("5"), Vec::new()),
        (String::from("6"), Vec::new()),
        (String::from("7"), Vec::new()),
        (String::from("8"), Vec::new()),
        (String::from("9"), Vec::new()),
        (String::from("10"), Vec::new()),
        (String::from("11"), Vec::new()),
        (String::from("12"), Vec::new())
    ]);

    for line in year_note_file_contents.lines() {
        let year_note_array: Vec<&str> = line.split("--").collect();

        if year_note_array.len() < 7 {
            continue;
        }

        let year_note = YearNote {
            note_id: String::from(year_note_array[0]),
            note_type: String::from(year_note_array[1]),
            note: String::from(year_note_array[2]),
            note_year: String::from(year_note_array[3]),
            is_complete: String::from(year_note_array[4]),
            note_month: String::from(year_note_array[5]),
            modified_date_time: String::from(year_note_array[6]),
        };

        year_notes.get_mut(&String::from(year_note_array[5])).unwrap().push(year_note);
    }

    return year_notes;
}

pub fn write_to_year_notes_file(current_date: DateTime<Local>, updated_file_contents: String) {
    let current_year_note_file_name = format!("{}-YearNotes.txt", current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_year_note_file_name);

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

use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use dirs::home_dir;

pub struct WeekNote {
    pub note_id: String,
    pub date: String,
    pub day_of_week: String,
    pub note_type: String,
    pub is_complete: String,
    pub note: String,
    pub modified_date_time: String
}

pub fn get_contents_of_week_notes_file(current_date: DateTime<Local>) -> String {
    let current_week_monday_date_string = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());

    // Attempt to pull the text file that has this weeks notes
    let week_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("week-notes").join(current_week_monday_date_string);
    if !week_file_path.exists() {
        std::fs::File::create_new(&week_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(week_file_path).expect("Should have been able to read the file");

    return contents;
}

pub fn fetch_week_notes(current_date: DateTime<Local>) -> HashMap<String, Vec<WeekNote>> {
    let contents = get_contents_of_week_notes_file(current_date);

    let mut week_notes: HashMap<String, Vec<WeekNote>>  = HashMap::from([
        (String::from("0"), Vec::new()),
        (String::from("1"), Vec::new()),
        (String::from("2"), Vec::new()),
        (String::from("3"), Vec::new()),
        (String::from("4"), Vec::new()),
        (String::from("5"), Vec::new()),
        (String::from("6"), Vec::new())
    ]);

    for line in contents.lines() {
        let week_note_array: Vec<&str> = line.split("--").collect();

        if week_note_array.len() < 7 {
            continue;
        }
        
        let week_note = WeekNote {
            note_id: String::from(week_note_array[0]),
            date: String::from(week_note_array[1]),
            day_of_week: String::from(week_note_array[2]),
            note_type: String::from(week_note_array[3]),
            is_complete: String::from(week_note_array[4]),
            note: String::from(week_note_array[5]),
            modified_date_time: String::from(week_note_array[6])
        };

        week_notes.get_mut(&String::from(week_note_array[2])).unwrap().push(week_note);
    }

    return week_notes;
}

pub fn write_to_week_notes_file(current_date: DateTime<Local>, updated_file_contents: String) {
    let file_name = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_date.year().to_string()).join("week-notes").join(file_name);

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

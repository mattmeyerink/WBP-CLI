use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use dirs::home_dir;
use uuid::Uuid;

use super::data::fetch_week_notes;
use super::data::get_contents_of_week_notes_file;
use super::display::display_week_notes;
use super::data::write_to_week_notes_file;

pub fn add_new_week_note(current_date: DateTime<Local>) {
    println!("");
    println!("A new note it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let day_of_week;
    let day_of_week_to_weekday: HashMap<String, String> = HashMap::from([
        (String::from("monday"), String::from("0")),
        (String::from("tuesday"), String::from("1")),
        (String::from("wednesday"), String::from("2")),
        (String::from("thursday"), String::from("3")),
        (String::from("friday"), String::from("4")),
        (String::from("saturday"), String::from("5")),
        (String::from("sunday"), String::from("6"))
    ]);
    loop {
        let mut day_of_week_string = String::new();
        print!("Day of the week: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut day_of_week_string).expect("Unable to read date");

        if day_of_week_to_weekday.contains_key(day_of_week_string.trim()) {
            day_of_week = day_of_week_to_weekday.get(day_of_week_string.trim()).unwrap().to_string();
            break;
        } else {
            println!("Not a valid day of the week. Try again dude!")
        }
    }
    
    let valid_note_types = vec![String::from("task"), String::from("event"), String::from("note")];
    let note_type;
    loop {
        print!("Type of note (Task/Event/Note): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_type_raw = String::new();
        io::stdin().read_line(&mut note_type_raw).expect("Unable to read note type");

        let note_type_formatted = note_type_raw.trim().to_lowercase();
        if valid_note_types.contains(&note_type_formatted) {
            note_type = note_type_formatted;
            break;
        } else {
            println!("Not a valid type dudeeee. How bout we try that again.");
        }
    }
    
    let note;
    loop {
        print!("Enter your note: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_raw = String::new();
        io::stdin().read_line(&mut note_raw).expect("Unable to read note");

        let note_raw_format = String::from(note_raw.trim());
        if note_raw_format.len() > 0 {
            note = note_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a note without text bro.")
        }
    }
    
    let note_id = String::from(Uuid::new_v4());

    let note_date_object = current_date + Duration::days(day_of_week.parse::<i64>().unwrap());
    let date = format!("{}-{}-{}", note_date_object.month(), note_date_object.day(), note_date_object.year());

    // TODO -> This needs to be calculated
    let modified_date_time = chrono::Local::now().timestamp().to_string();

    let is_complete = "false";

    let new_note = format!("{}--{}--{}--{}--{}--{}--{}\n", note_id, date, day_of_week, note_type, is_complete, note, modified_date_time);

    let file_name = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());
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

    let mut line_day_off_week = String::new();
    let note_type = String::new();
    let note = String::new();
    // Find the line in the week_file_contents
    for line in week_file_contents.lines() {
        if line.contains(&note_id) {
            let mut updated_line_vector: Vec<&str> = line.split("--").collect();
            
            line_day_off_week = String::from(updated_line_vector[2]);

            // Yank all of the goodies out of this vector and put them in the default holders
        }
    }

    // ask the user to re-input everything about the note defaulting to the current values

    // Calculate the updated value of the note

    // Replace the original value of the note in the week_file_contents with the new value

    // Overwrite the original file with the new week file contents
}

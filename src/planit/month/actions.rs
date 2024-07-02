use std::{fs::OpenOptions, io::Write};

use chrono::{DateTime, Datelike, Local};
use dirs::home_dir;
use uuid::Uuid;

use crate::planit::input_utils::{get_note_input, get_note_type_input};

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
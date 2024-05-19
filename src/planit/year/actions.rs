use chrono::{DateTime, Datelike, Local};
use uuid::Uuid;

use crate::planit::input_utils::{get_note_input, get_note_month, get_note_type_input};

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

    let note_text = format!("{}--{}--{}--{}--{}--{}--{}", note_id, note_month, note_type, note, note_year, modified_date_time, is_complete);

    println!("{}", note_text);
}

pub fn mark_year_note_complete() {
    println!("This will be the mark year note complete note action.");
    println!("");
    println!("Come back when its done fool!");
}

pub fn edit_year_note() {
    println!("This will be the edit a note action.");
    println!("");
    println!("Come back when its done fool!");
}

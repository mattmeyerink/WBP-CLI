use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local};

use super::data::YearNote;

pub fn print_year_header(current_date: DateTime<Local>) {
    println!("");
    println!("{} Year View", current_date.year());
    println!("");
}

pub fn display_year_notes(year_notes: HashMap<String, Vec<YearNote>>, display_id: bool, current_date: DateTime<Local>) {
    print_year_header(current_date);

    for month_to_display_num in 0..12 {
        let normalized_month_num = month_to_display_num + 1;
        let month_notes = year_notes.get(&normalized_month_num.to_string());
        if month_notes.is_some() {
            display_year_month_notes(month_notes.unwrap(), display_id, month_to_display_num);
        }
    }
}

fn display_year_month_notes(month_notes: &Vec<YearNote>, display_id: bool, month_to_display: usize) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("{}", month_names.get(month_to_display).unwrap());
    println!("--------------");

    // TODO -> Sort the notes by type and by completion status
    for month_note in month_notes {
        let note_symbol;
        if month_note.note_type == "task" && month_note.is_complete == "true" {
            note_symbol = "x";
        } else if month_note.note_type == "task" && month_note.is_complete == "false" {
            note_symbol = "o";
        } else if month_note.note_type == "event" {
            note_symbol = "*";
        } else {
            note_symbol = "-";
        }
        if display_id {
            println!("{} {} ({})", note_symbol, month_note.note, month_note.note_id);
        } else {
            println!("{} {}", note_symbol, month_note.note);
        }
    }
    println!("");
}

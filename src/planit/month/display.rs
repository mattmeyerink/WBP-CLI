use chrono::{DateTime, Datelike, Local};

use super::data::MonthNote;

pub fn display_month_notes(current_date: DateTime<Local>, month_notes: Vec<MonthNote>, display_id: bool) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let safe_current_month = usize::try_from(current_date.month0()).unwrap();
    println!("");
    println!("{} {} Month Notes", month_names.get(safe_current_month).unwrap(), current_date.year());
    println!("---------------------------------------------");

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

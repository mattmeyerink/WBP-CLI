use std::collections::HashMap;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;

use super::data::WeekNote;

pub fn display_days_notes(week_notes: &Vec<WeekNote>, section_title: String, display_id: bool) {
    println!("{}", section_title);
    println!("--------------");
    for week_note in week_notes {
        let note_symbol;
        if week_note.note_type == "task" && week_note.is_complete == "true" {
            note_symbol = "x";
        } else if week_note.note_type == "task" && week_note.is_complete == "false" {
            note_symbol = "o";
        } else if week_note.note_type == "event" {
            note_symbol = "*";
        } else {
            note_symbol = "-";
        }
        if display_id {
            println!("{} {} ({})", note_symbol, week_note.note, week_note.note_id);
        } else {
            println!("{} {}", note_symbol, week_note.note);
        }
        
    }
    println!("");
}

pub fn display_week_notes(week_notes: HashMap<String, Vec<WeekNote>>, display_id: bool, current_date: DateTime<Local>) {
    // Display the week's notes
    println!("");
    println!("*****************************************************************************");
    println!("*****************************************************************************");
    println!("");
    println!("Week View - Week of {}-{}-{}", current_date.month(), current_date.day(), current_date.year());
    println!("");
    
    display_days_notes(week_notes.get(&String::from("0")).unwrap(), String::from("Monday"), display_id);

    display_days_notes(week_notes.get(&String::from("1")).unwrap(), String::from("Tuesday"), display_id);

    display_days_notes(week_notes.get(&String::from("2")).unwrap(), String::from("Wednesday"), display_id);

    display_days_notes(week_notes.get(&String::from("3")).unwrap(), String::from("Thursday"), display_id);

    display_days_notes(week_notes.get(&String::from("4")).unwrap(), String::from("Friday"), display_id);

    display_days_notes(week_notes.get(&String::from("5")).unwrap(), String::from("Saturday"), display_id);

    display_days_notes(week_notes.get(&String::from("6")).unwrap(), String::from("Sunday"), display_id);

    println!("*****************************************************************************");
    println!("*****************************************************************************");
    println!("");
}


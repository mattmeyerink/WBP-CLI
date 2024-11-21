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

    let event_symbol = "*";
    let finished_task_symbol = "x";
    let unfinished_task_symbol = "o";
    let note_symbol = "-";

    let mut events: Vec<&YearNote> = vec![];
    let mut finished_tasks: Vec<&YearNote> = vec![];
    let mut unfinished_tasks: Vec<&YearNote> = vec![]; 
    let mut notes: Vec<&YearNote> = vec![];

    for month_note in month_notes {
        if month_note.note_type == "task" && month_note.is_complete == "true" {
            finished_tasks.push(month_note)
        } else if month_note.note_type == "task" && month_note.is_complete == "false" {
            unfinished_tasks.push(month_note)
        } else if month_note.note_type == "event" {
            events.push(month_note)
        } else {
            notes.push(month_note)
        }
    }

    let tasks_present = finished_tasks.len() > 0 || unfinished_tasks.len() > 0;
    let print_space_after_events = events.len() > 0 && (tasks_present || notes.len() > 0);
    let print_space_after_tasks = tasks_present && notes.len() > 0;

    display_year_month_note_section(&event_symbol, display_id, events);

    // Give some extra visual space if we have some after some events tasks incoming
    if print_space_after_events {
        println!("");
    }

    display_year_month_note_section(&finished_task_symbol, display_id, finished_tasks);
    display_year_month_note_section(&unfinished_task_symbol, display_id, unfinished_tasks);

    // Give some extra visual space if we have some notes incoming
    if print_space_after_tasks {
        println!("");
    }

    display_year_month_note_section(&note_symbol, display_id, notes);
    println!("");
}

fn display_year_month_note_section(note_symbol: &str, display_id: bool, section_notes: Vec<&YearNote>) {
    for section_note in section_notes {
        if display_id {
            println!("{} {} ({})", note_symbol, section_note.note, section_note.note_id);
        } else {
            println!("{} {}", note_symbol, section_note.note);
        }
    }
}

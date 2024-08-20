use std::collections::HashMap;
use chrono::Duration;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;

use super::data::WeekNote;

fn display_week_note_section(note_symbol: &str, display_id: bool, week_notes: Vec<&WeekNote>) {
    for week_note in week_notes {
        if display_id {
            println!("{} {} ({})", note_symbol, week_note.note, week_note.note_id);
        } else {
            println!("{} {}", note_symbol, week_note.note);
        }
    }
}

pub fn display_days_notes(week_notes: &Vec<WeekNote>, section_title: String, display_id: bool) {
    println!("{}", section_title);
    println!("--------------");

    let event_symbol = "*";
    let finished_task_symbol = "x";
    let unfinished_task_symbol = "o";
    let note_symbol = "-";

    let mut events: Vec<&WeekNote> = vec![];
    let mut finished_tasks: Vec<&WeekNote> = vec![];
    let mut unfinished_tasks: Vec<&WeekNote> = vec![]; 
    let mut notes: Vec<&WeekNote> = vec![];

    for week_note in week_notes {
        if week_note.note_type == "task" && week_note.is_complete == "true" {
            finished_tasks.push(week_note)
        } else if week_note.note_type == "task" && week_note.is_complete == "false" {
            unfinished_tasks.push(week_note)
        } else if week_note.note_type == "event" {
            events.push(week_note)
        } else {
            notes.push(week_note)
        }
    }

    let tasks_present = finished_tasks.len() > 0 || unfinished_tasks.len() > 0;
    let print_space_after_events = events.len() > 0 && (tasks_present || notes.len() > 0);
    let print_space_after_tasks = tasks_present && notes.len() > 0;

    display_week_note_section(&event_symbol, display_id, events);

    // Give some extra visual space if we have some after some events tasks incoming
    if print_space_after_events {
        println!("");
    }

    display_week_note_section(&finished_task_symbol, display_id, finished_tasks);
    display_week_note_section(&unfinished_task_symbol, display_id, unfinished_tasks);

    // Give some extra visual space if we have some notes incoming
    if print_space_after_tasks {
        println!("");
    }

    display_week_note_section(&note_symbol, display_id, notes);
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
    
    let monday_title = format!("Monday {}-{}-{}", current_date.month(), current_date.day(), current_date.year());
    display_days_notes(week_notes.get(&String::from("0")).unwrap(), String::from(monday_title), display_id);

    let tuesday_date = current_date + Duration::days(1);
    let tuesday_title = format!("Tuesday {}-{}-{}", tuesday_date.month(), tuesday_date.day(), tuesday_date.year());
    display_days_notes(week_notes.get(&String::from("1")).unwrap(), tuesday_title, display_id);

    let wednesday_date = current_date + Duration::days(2);
    let wednesday_title = format!("Wednesday {}-{}-{}", wednesday_date.month(), wednesday_date.day(), wednesday_date.year());
    display_days_notes(week_notes.get(&String::from("2")).unwrap(), wednesday_title, display_id);

    let thursday_date = current_date + Duration::days(3);
    let thursday_title = format!("Thursday {}-{}-{}", thursday_date.month(), thursday_date.day(), thursday_date.year());
    display_days_notes(week_notes.get(&String::from("3")).unwrap(), thursday_title, display_id);

    let friday_date = current_date + Duration::days(4);
    let friday_title = format!("Friday {}-{}-{}", friday_date.month(), friday_date.day(), friday_date.year());
    display_days_notes(week_notes.get(&String::from("4")).unwrap(), friday_title, display_id);

    let saturday_date = current_date + Duration::days(5);
    let saturday_title = format!("Saturday {}-{}-{}", saturday_date.month(), saturday_date.day(), saturday_date.year());
    display_days_notes(week_notes.get(&String::from("5")).unwrap(), saturday_title, display_id);

    let sunday_date = current_date + Duration::days(6);
    let sunday_title = format!("Sunday {}-{}-{}", sunday_date.month(), sunday_date.day(), sunday_date.year());
    display_days_notes(week_notes.get(&String::from("6")).unwrap(), sunday_title, display_id);

    println!("*****************************************************************************");
    println!("*****************************************************************************");
    println!("");
}


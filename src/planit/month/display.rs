use chrono::{DateTime, Datelike, Local, TimeZone};

use super::data::MonthNote;

pub fn display_month_notes(current_date: DateTime<Local>, month_notes: Vec<MonthNote>, display_id: bool) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let safe_current_month = usize::try_from(current_date.month0()).unwrap();
    println!("");
    println!("{} {} Month Notes", month_names.get(safe_current_month).unwrap(), current_date.year());
    println!("---------------------------------------------");

    let event_symbol = "*";
    let finished_task_symbol = "x";
    let unfinished_task_symbol = "o";
    let note_symbol = "-";

    let mut events: Vec<MonthNote> = vec![];
    let mut finished_tasks: Vec<MonthNote> = vec![];
    let mut unfinished_tasks: Vec<MonthNote> = vec![]; 
    let mut notes: Vec<MonthNote> = vec![];

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

    display_month_note_section(&event_symbol, display_id, events);

    // Give some extra visual space if we have some after some events tasks incoming
    if print_space_after_events {
        println!("");
    }

    display_month_note_section(&finished_task_symbol, display_id, finished_tasks);
    display_month_note_section(&unfinished_task_symbol, display_id, unfinished_tasks);

    // Give some extra visual space if we have some notes incoming
    if print_space_after_tasks {
        println!("");
    }

    display_month_note_section(&note_symbol, display_id, notes);
    println!("");
}

fn display_month_note_section(note_symbol: &str, display_id: bool, month_notes: Vec<MonthNote>) {
    for month_note in month_notes {
        if display_id {
            println!("{} {} ({})", note_symbol, month_note.note, month_note.note_id);
        } else {
            println!("{} {}", note_symbol, month_note.note);
        }
    }
}

pub fn display_month_highlights(current_date: DateTime<Local>, month_highlights: Vec<String>) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let safe_current_month = usize::try_from(current_date.month0()).unwrap();
    println!("");
    println!("{} {} Month Highlights", month_names.get(safe_current_month).unwrap(), current_date.year());
    println!("---------------------------------------------");

    for day_of_month_zeroed in 0..month_highlights.len() {
        let month_highlight = month_highlights.get(day_of_month_zeroed).unwrap();

        let day_object = Local.with_ymd_and_hms(current_date.year(), current_date.month(), (day_of_month_zeroed + 1).try_into().unwrap(), 0, 0, 0).unwrap();
        let mut formatted_day_string = format!("{} {}", day_object.weekday(), &(day_of_month_zeroed + 1).to_string());

        let todays_date = chrono::Local::now();
        if format!("{}", todays_date.format("%Y-%m-%d")) == format!("{}", day_object.format("%Y-%m-%d")) {
            formatted_day_string = format!("* {}", formatted_day_string);
        }

        if month_highlight.len() > 0 {
            println!("{} - {}", formatted_day_string, month_highlight);
        } else {
            println!("{}", formatted_day_string);
        }
    }

    println!("");
}

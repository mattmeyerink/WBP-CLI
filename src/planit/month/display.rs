use chrono::{DateTime, Datelike, Local, TimeZone};

use super::data::MonthNote;

pub fn display_month_notes(current_date: DateTime<Local>, month_notes: Vec<MonthNote>, display_id: bool) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let safe_current_month = usize::try_from(current_date.month0()).unwrap();
    println!("");
    println!("{} {} Month Notes", month_names.get(safe_current_month).unwrap(), current_date.year());
    println!("---------------------------------------------");

    // TODO -> Sort the notes by type and whether they are completed
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

pub fn display_month_highlights(current_date: DateTime<Local>, month_highlights: Vec<String>) {
    let month_names = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let safe_current_month = usize::try_from(current_date.month0()).unwrap();
    println!("");
    println!("{} {} Month Highlights", month_names.get(safe_current_month).unwrap(), current_date.year());
    println!("---------------------------------------------");

    // TODO -> add abbreviation for the day of the week next to each listed day.
    // Also should add an astrisk if the day is today.

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

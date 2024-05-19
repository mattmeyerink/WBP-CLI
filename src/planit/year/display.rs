use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local};

use super::data::YearNote;

pub fn print_year_header(current_date: DateTime<Local>) {
    println!("");
    println!("{} Year View", current_date.year());
    println!("");
}

pub fn display_year_notes(year_notes: HashMap<String, Vec<YearNote>>, display_id: bool, current_date: DateTime<Local>) {

}

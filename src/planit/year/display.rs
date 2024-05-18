use chrono::{DateTime, Datelike, Local};

pub fn print_year_header(current_date: DateTime<Local>) {
    println!("");
    println!("{} Year View", current_date.year());
    println!("");
}
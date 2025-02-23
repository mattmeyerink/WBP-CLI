use std::{collections::HashMap, io::{self, Write}};

use chrono::{DateTime, Local};

use super::utils;

pub fn get_day_of_week_input() -> String {
    let day_of_week;
    let day_of_week_to_weekday: HashMap<String, String> = HashMap::from([
        (String::from("monday"), String::from("0")),
        (String::from("tuesday"), String::from("1")),
        (String::from("wednesday"), String::from("2")),
        (String::from("thursday"), String::from("3")),
        (String::from("friday"), String::from("4")),
        (String::from("saturday"), String::from("5")),
        (String::from("sunday"), String::from("6"))
    ]);
    loop {
        let mut day_of_week_string = String::new();
        print!("Day of the week: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut day_of_week_string).expect("Unable to read date");

        if day_of_week_to_weekday.contains_key(day_of_week_string.trim()) {
            day_of_week = day_of_week_to_weekday.get(day_of_week_string.trim()).unwrap().to_string();
            break;
        } else {
            println!("Not a valid day of the week. Try again dude!")
        }
    }

    return day_of_week;
}

pub fn get_note_type_input() -> String {
    let valid_note_types = vec![String::from("task"), String::from("event"), String::from("note")];
    let note_type;
    loop {
        print!("Type of note (Task/Event/Note): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_type_raw = String::new();
        io::stdin().read_line(&mut note_type_raw).expect("Unable to read note type");

        let note_type_formatted = note_type_raw.trim().to_lowercase();
        if valid_note_types.contains(&note_type_formatted) {
            note_type = note_type_formatted;
            break;
        } else {
            println!("Not a valid type dudeeee. How bout we try that again.");
        }
    }

    return note_type;
}

pub fn get_note_input() -> String {
    let note;
    loop {
        print!("Enter your note: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_raw = String::new();
        io::stdin().read_line(&mut note_raw).expect("Unable to read note");

        let note_raw_format = String::from(note_raw.trim());
        if note_raw_format.len() > 0 {
            note = note_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a note without text bro.")
        }
    }

    return note;
}

pub fn get_note_month() -> String {
    let valid_months = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"];
    let note_month;

    loop {
        print!("Enter note month (ex 1 = Jan): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_month_raw = String::new();
        io::stdin().read_line(&mut note_month_raw).expect("Unable to read note");

        if valid_months.contains(&note_month_raw.trim()) {
            note_month = note_month_raw;
            break;
        } else {
            println!("Can you actually put in a valid note buster?");
        }
    }

    return note_month.trim().to_string();
}

pub fn get_highlight_input() -> String {
    let highlight;
    loop {
        print!("Enter your highlight: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut highlight_raw = String::new();
        io::stdin().read_line(&mut highlight_raw).expect("Unable to read highlight");

        let highlight_raw_format = String::from(highlight_raw.trim());
        if highlight_raw_format.len() > 0 {
            highlight = highlight_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a highlight without text bro.")
        }
    }

    return highlight;
}

pub fn get_highlight_day_input(current_date: DateTime<Local>) -> String {
    let highlight_day;
    let mut valid_day_strings: Vec<String> = Vec::new();
    let number_of_days_in_month = utils::get_number_of_days_in_month(current_date);
    for day_in_month in 0..number_of_days_in_month {
        valid_day_strings.push((day_in_month + 1).to_string());
    }

    loop {
        print!("Enter highlight day: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut highlight_day_raw = String::new();
        io::stdin().read_line(&mut highlight_day_raw).expect("Unable to read day");

        if valid_day_strings.contains(&String::from(highlight_day_raw.trim())) {
            highlight_day = highlight_day_raw;
            break;
        } else {
            println!("Can you actually put in a valid day buster?");
        }
    }

    return highlight_day.trim().to_string();
}

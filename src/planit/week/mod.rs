use std::{fs, io};
use std::io::Write;
use chrono::{Datelike, Duration};
use dirs::home_dir;

mod utils;
mod data;
mod display;
mod actions;

pub fn week_view() {
    let mut current_date = utils::get_current_week_monday();

    loop {
        // Create the outer dir for the current year if it doesn't already exist
        let current_year = current_date.year().to_string();
        let current_wbp_plan_it_year_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_year.clone());
        if !current_wbp_plan_it_year_data_path.exists() {
            fs::create_dir(&current_wbp_plan_it_year_data_path).expect("Unable to make current year outer dir");
        }

        // Create the dir for the week notes within the current year directory if it doesn't already exist
        let current_wbp_plan_it_week_notes_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_year).join("week-notes");
        if !current_wbp_plan_it_week_notes_data_path.exists() {
            fs::create_dir(&current_wbp_plan_it_week_notes_data_path).expect("Unable to make current week notes dir");
        }

        let week_notes = data::fetch_week_notes(current_date);

        display::display_week_notes(week_notes, false, current_date);

        println!("Actions you can take.");
        println!("[1]: Add new note.");
        println!("[2]: Mark a task complete.");
        println!("[3]: Edit a current note.");
        println!("[4]: Delete a current note.");
        println!("[5]: Previous week.");
        println!("[6]: Next week.");
        println!("[7]: Quit week view");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            actions::add_new_week_note(current_date);
        } else if action.trim() == "2" {
            actions::mark_week_note_completed(current_date);
        } else if action.trim() == "3" {
            actions::edit_note(current_date);
        } else if action.trim() == "4" {
            actions::delete_week_note(current_date);
        } else if action.trim() == "5" {
            current_date = current_date - Duration::days(7);
        } else if action.trim() == "6" {
            current_date = current_date + Duration::days(7);
        } else if action.trim() == "7" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

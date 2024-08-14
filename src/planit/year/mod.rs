use std::{fs, io::{self, Write}};

use chrono::{Datelike, Months};
use dirs::home_dir;

mod actions;
mod display;
mod data;

pub fn year_view() {
    let mut current_date = chrono::Local::now();

    loop {
        // Create the outer dir for the current year if it doesn't already exist
        let current_year = current_date.year().to_string();
        let current_wbp_plan_it_year_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_year.clone());
        if !current_wbp_plan_it_year_data_path.exists() {
            fs::create_dir(&current_wbp_plan_it_year_data_path).expect("Unable to make current year outer dir");
        }
        
        let year_notes = data::fetch_year_notes(current_date);

        display::display_year_notes(year_notes, false, current_date);
        
        println!("Actions you can take.");
        println!("[1]: Add new note.");
        println!("[2]: Mark a task complete.");
        println!("[3]: Edit a current note.");
        println!("[4]: Delete a current note.");
        println!("[5]: Previous year.");
        println!("[6]: Next year.");
        println!("[7]: Quit year view");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            actions::add_new_year_note(current_date);
        } else if action.trim() == "2" {
            actions::mark_year_note_complete(current_date);
        } else if action.trim() == "3" {
            actions::edit_year_note(current_date);
        } else if action.trim() == "4" {
            actions::delete_year_note(current_date);
        } else if action.trim() == "5" {
            current_date = current_date.checked_sub_months(Months::new(12)).unwrap();
        } else if action.trim() == "6" {
            current_date = current_date.checked_add_months(Months::new(12)).unwrap();
        } else if action.trim() == "7" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

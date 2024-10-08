use std::io::{self, Write};

use actions::Actions;
use chrono::Duration;
use utils::Utils;

mod utils;
mod actions;
mod data;
mod display;
mod input_utils;

pub fn milesonmiles() {
    println!("Welcome to Miles on Miles!");

    println!("");
    println!("");

    let mut current_date = Utils::get_current_week_monday();

    loop {
        Utils::create_miles_on_miles_directory_structure(current_date);

        println!("Actions you can take.");
        println!("[1]: Log a run.");
        println!("[2]: Plan training.");
        println!("[3]: Analyze running.");
        println!("[5]: Quit miles on miles.");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            Actions::log_run();
        } else if action.trim() == "2" {
            Actions::plan_training();
        } else if action.trim() == "3" {
            // This will contain week, month, and year views with both the training and performed runs
            println!("Oopsies. Haven't done this one yet. This will be the analyze running section");
        } else if action.trim() == "4" {
            current_date = current_date + Duration::days(7);
        } else if action.trim() == "5" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

use std::io::{self, Write};

use chrono::Duration;

mod utils;
mod actions;
mod data;
mod display;

pub fn milesonmiles() {
    println!("Welcome to Miles on Miles!");

    println!("");
    println!("");

    let mut current_date = utils::get_current_week_monday();

    loop {
        utils::create_miles_on_miles_directory_structure(current_date);

        // This is where we are going to print out data about the runs for this week.

        println!("Actions you can take.");
        println!("[1]: Log a run.");
        println!("[2]: View run details.");
        println!("[3]: Next week.");
        println!("[4]: Previous week.");
        println!("[5]: Quit miles on miles.");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            println!("Oopsies. Haven't done this one yet. This will be the log a run action");
        } else if action.trim() == "2" {
            println!("Oopsies. Haven't done this one yet. This will be the ");
        } else if action.trim() == "3" {
            current_date = current_date - Duration::days(7);
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

use std::io::{self, Write};

use chrono::Duration;
use data::{Run, WeekPlan};
use utils::Utils;

mod utils;
mod data;
mod input_utils;

pub fn milesonmiles() {
    println!("");
    println!("Welcome to Miles on Miles!");

    let mut current_date = Utils::get_current_week_monday();

    loop {
        Utils::create_miles_on_miles_directory_structure(current_date);

        let current_week_plan = WeekPlan::read_week_plan(current_date);
        println!("");
        println!("Week of {}", current_week_plan.date);
        println!("");
        current_week_plan.print_week_plan();
        println!("");

        println!("Actions you can take.");
        println!("[1]: Log a run.");
        println!("[2]: Create plan for current week");
        println!("[3]: Add a race");
        println!("[4]: Analyze running.");
        println!("[5]: Previous week");
        println!("[6]: Next week");
        println!("[7]: Quit miles on miles.");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            Run::log_run();
        } else if action.trim() == "2" {
            WeekPlan::create_week_plan(current_date);
        } else if action.trim() == "3" {
            // This will be the option to add a race.
            println!("Oopsies. Haven't done this one yet. This will be to add a race.");
        } else if action.trim() == "4" {
            // This will contain week, month, and year views with both the training and performed runs
            println!("Oopsies. Haven't done this one yet. This will be the analyze running section");
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

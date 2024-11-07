use std::io::{self, Write};

use chrono::Duration;

use crate::milesonmiles::{data::{Run, WeekPlan}, input_utils::InputUtils, utils::Utils};

pub struct Actions;

impl Actions {
    pub(crate) fn log_run() {
        println!("A run has been done! Many congrats!");
        println!("Enter the following information and it will be done!");
    
        let mut run;
    
        loop {
            let date = InputUtils::get_run_date_input();
            let distance = InputUtils::get_run_distance_input();
            let time = InputUtils::get_run_time_input();
            let is_workout = InputUtils::get_is_run_workout_input();
            let is_race = InputUtils::get_is_run_race_input();
            let description = InputUtils::get_run_description_input();
    
            run = Run {
                date,
                distance,
                time,
                is_workout,
                is_race,
                description
            };
    
            println!("");
            println!("Does this look right?");
            println!("Date: {}", run.date);
            println!("Distance: {}", run.distance);
            println!("Time: {}", run.time);
            println!("Is workout: {}", run.is_workout);
            println!("Is race: {}", run.is_race);
            println!("Description: {}", run.description);
    
            let is_input_confirmed = InputUtils::get_input_confirmation(String::from("Keep on keeping on?"));
    
            if is_input_confirmed {
                println!("Awesome we will add it then!");
                break;
            } else {
                println!("Lets run that back and try again.");
            }
        }
    
        run.save_run();
    }

    pub(crate) fn plan_training() {
        println!("");
        println!("Planning here we go!");
        println!("");

        let mut current_date = Utils::get_current_week_monday();

        loop {
            // Print out the training plan for the current week.
            // Print out how long until each of the races in the next 4 months.

            println!("Actions you can take.");
            println!("[1]: Create plan for this week");
            println!("[2]: Add a race");
            println!("[3]: View next week's plan");
            println!("[4]: View last week's plan");
            println!("[5]: Return to main menu");

            println!("");

            print!("Which action do you want to take: ");
            io::stdout().flush().expect("Darn toilet got stuck again");

            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Unable to read action");

            if action.trim() == "1" {
                WeekPlan::create_week_plan(current_date);
            } else if action.trim() == "2" {
                println!("This will be the option to add a race to the race file. That whole concept needs to be fleshed out");
            } else if action.trim() == "3" {
                current_date = current_date + Duration::days(1);
            } else if action.trim() == "4" {
                current_date = current_date - Duration::days(1);
            } else if action.trim() == "5" {
                break;
            } else {
                println!("\n");
                println!("Boooo thats not a correct action");
                println!("\n");
            }
        }
    }
}

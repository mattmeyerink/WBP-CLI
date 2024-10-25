use chrono::{DateTime, Datelike, Local, NaiveDate};
use dirs::home_dir;

use crate::milesonmiles::input_utils::InputUtils;

use super::utils::Utils;

pub struct Run {
    pub(crate) date: String,
    pub(crate) distance: f64,
    pub(crate) time: String,
    pub(crate) is_workout: bool,
    pub(crate) is_race: bool,
    pub(crate) description: String
}

impl Run {
    pub(crate) fn get_file_name(&self) -> String {
        let date_object = self.get_date_object();
        
        let monday_date = Utils::get_current_week_monday_from_date(date_object);

        return format!("{}-{}-{}-WeekLog.txt", monday_date.month(), monday_date.day(), monday_date.year());
    }

    pub(crate) fn get_date_object(&self) -> NaiveDate {
        return NaiveDate::parse_from_str(self.date.as_str().trim(), Utils::get_miles_on_miles_date_string_format().as_str()).unwrap();
    }

    pub(crate) fn save_run(&self) {
        let filename = self.get_file_name();
        let run_year = self.get_date_object();
        Utils::create_weekly_run_log_file(&filename, run_year.year().to_string());

        let new_run_string = format!("{}--{}--{}--{}--{}--{}\n", self.date, self.distance, self.time, self.is_workout, self.is_race, self.description);
        let run_log_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(run_year.year().to_string()).join(filename);

        Utils::write_to_file(run_log_file_path, new_run_string);
    }
}

pub struct WeekPlan {
    pub(crate) date: String,
    pub(crate) runs: Vec<Run>
}

impl WeekPlan {
    pub(crate) fn create_week_plan(current_date: DateTime<Local>) {
        // Store the date string. This will be the first line in the file.
        // Create a loop getting information for each day.
        // After each day maybe output a little summary of distances/isworkout of runs added so far.
        // Have first thing entered be a mileage total for the week. 
        // At each input show remaining miles to do to hit the goal.

        let date_string = format!("{}-{}-{}", current_date.month(), current_date.day(), current_date.year());
        println!("Planning the training for week of {}", date_string);

        let mut runs: Vec<Run> = vec![];
        let goal_weekly_mileage = InputUtils::get_goal_weekly_mileage();
        let current_weekly_mileage = 0.0;

        println!("Lets get to planning!");

        loop {
            // Print out the current state of the week plan.

            // This is the loop to gather runs until the user says the plan is complete
            let remaining_miles_to_plan = goal_weekly_mileage - current_weekly_mileage;
            println!("You have {} miles left to plan", remaining_miles_to_plan);

            let add_another_run = InputUtils::get_input_confirmation(String::from("Do you want to add another run?"));
            if !add_another_run {
                break;
            }
    
            loop {
                let day_of_week = InputUtils::get_day_of_week();
                // Get the date using the current date and the input day of the week.
                // let date = InputUtils::get_run_date_input();

                let distance = InputUtils::get_run_distance_input();
                let is_workout = InputUtils::get_is_run_workout_input();
                let is_race = InputUtils::get_is_run_race_input();
                let description = InputUtils::get_run_description_input();

        
                let run = Run {
                    date: String::from("THIS IS A PLACEHOLDER"),
                    distance,
                    time: String::from("0"),
                    is_workout,
                    is_race,
                    description
                };
        
                println!("");
                println!("Does this look right?");
                println!("Date: {}", run.date);
                println!("Distance: {}", run.distance);
                println!("Is workout: {}", run.is_workout);
                println!("Is race: {}", run.is_race);
                println!("Description: {}", run.description);
        
                let is_input_confirmed = InputUtils::get_input_confirmation(String::from("Keep on keeping on?"));
        
                if is_input_confirmed {
                    runs.push(run);
                    println!("Awesome on to the next day!");
                    break;
                } else {
                    println!("Lets run that back and try again.");
                }
            }
        }
    }
}

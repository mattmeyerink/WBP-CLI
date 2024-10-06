use core::f64;
use std::io::{self, Write};
use chrono::{Datelike, Local, NaiveDate};

use crate::milesonmiles::utils::Utils;

pub struct InputUtils;

impl InputUtils {
    pub(crate) fn get_run_date_input() -> String {
        let date;
    
        loop {
            let mut date_raw_string = String::new();
            print!("Date of the run (Ex -> 10/31/1981): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut date_raw_string).expect("Unable to read date");
    
            let is_date_valid = NaiveDate::parse_from_str(date_raw_string.as_str().trim(), &Utils::get_miles_on_miles_date_string_format().as_str()).is_ok();

            if !is_date_valid {
                println!("Not a valid date dude. Try again in mm/dd/yyyy format!");
                continue;
            }

            let run_date = NaiveDate::parse_from_str(date_raw_string.as_str().trim(), &Utils::get_miles_on_miles_date_string_format().as_str()).unwrap();

            if run_date.year() < 1900 {
                println!("I don't think you were running before 1900. Lets try that again.");
                continue;
            }

            let current_date = Local::now().date_naive();
            let is_run_date_in_future = current_date.signed_duration_since(run_date).num_days() <= -1;
            if is_run_date_in_future {
                println!("LIAR! That date is in the future and couldn't have happened yet! Try again.");
                continue;
            }

            date = String::from(date_raw_string.trim());
            break;
        }
    
        return date;
    }

    pub(crate) fn get_run_distance_input() -> f64 {
        let run_distance;
    
        loop {
            let mut run_distance_raw_string = String::new();
            print!("Distance of run in miles (Ex -> 10.2): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut run_distance_raw_string).expect("Unable to read date");
    
            if run_distance_raw_string.trim().parse::<f64>().is_ok() {
                run_distance = run_distance_raw_string.trim().parse::<f64>().unwrap();
                break;
            } else {
                println!("Not a valid distance dude. Try again!");
            }
        }
    
        return run_distance;
    }

    pub(crate) fn get_run_time_input() -> String {
        let run_time;
    
        loop {
            let mut run_time_raw_string = String::new();
            print!("Time of run (hh:mm:ss Ex -> 1:32:12): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut run_time_raw_string).expect("Unable to read time");
    
            let is_run_time_valid;
    
            let run_time_raw_parts: Vec<&str> = run_time_raw_string.trim().split(":").collect();
            if run_time_raw_parts.len() == 3 {
                let is_hours_valid = run_time_raw_parts[0].parse::<u32>().is_ok();
                let is_minutes_valid = run_time_raw_parts[1].parse::<u32>().is_ok_and(|x| x < 60);
                let is_seconds_valid = run_time_raw_parts[2].parse::<u32>().is_ok_and(|x| x < 60);
    
                is_run_time_valid = is_hours_valid && is_minutes_valid && is_seconds_valid;
            } else {
                is_run_time_valid = false;
            }
    
            if is_run_time_valid {
                run_time = String::from(run_time_raw_string.trim());
                break;
            } else {
                println!("Not a valid run time dude. Try again!");
            }
        }
    
        return run_time;
    }

    pub(crate) fn get_run_description_input() -> String {
        let mut run_description = String::new();
    
        print!("Description of run: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut run_description).expect("Unable to read description");
    
        return run_description;
    }

    pub(crate) fn get_is_run_workout_input() -> bool {
        let is_run_workout;
    
        loop {
            let mut is_run_workout_raw = String::new();
            print!("Was this a workout session? (Yes/No): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut is_run_workout_raw).expect("Unable to read date");
    
            if is_run_workout_raw.trim().to_lowercase() == "yes" {
                is_run_workout = true;
                break;
            } else if is_run_workout_raw.trim().to_lowercase() == "no" {
                is_run_workout = false;
                break;
            } else {
                println!("It's a yes or no question dingis!");
            }
        }
    
        return is_run_workout;
    }

    pub(crate) fn get_is_run_race_input() -> bool {
        let is_run_race;
    
        loop {
            let mut is_run_race_raw = String::new();
            print!("Was this a race? (Yes/No): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut is_run_race_raw).expect("Unable to read date");
    
            if is_run_race_raw.trim().to_lowercase() == "yes" {
                is_run_race = true;
                break;
            } else if is_run_race_raw.trim().to_lowercase() == "no" {
                is_run_race = false;
                break;
            } else {
                println!("It's a yes or no question dingis!");
            }
        }
    
        return is_run_race;
    }

    pub(crate) fn get_input_confirmation() -> bool {
        let is_input_confirmed;
    
        loop {
            let mut is_input_confirmed_raw = String::new();
            print!("Keep on keeping on? (Yes/No): ");
            io::stdout().flush().expect("Darn toilet got stuck again");
            io::stdin().read_line(&mut is_input_confirmed_raw).expect("Unable to read date");
    
            if is_input_confirmed_raw.trim().to_lowercase() == "yes" {
                is_input_confirmed = true;
                break;
            } else if is_input_confirmed_raw.trim().to_lowercase() == "no" {
                is_input_confirmed = false;
                break;
            } else {
                println!("It's a yes or no question dingis!");
            }
        }
    
        return is_input_confirmed;
    }
}

use core::f64;
use std::io::{self, Write};
use chrono::NaiveDate;

use crate::milesonmiles::utils::Utils;

pub fn get_run_date_input() -> String {
    let date;

    loop {
        let mut date_raw_string = String::new();
        print!("Date of the run (Ex -> 10/31/1981): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut date_raw_string).expect("Unable to read date");

        // TODO -> validate the date is not after today. Can't log a future run
        if NaiveDate::parse_from_str(date_raw_string.as_str().trim(), &Utils::get_miles_on_miles_date_string_format().as_str()).is_ok() {
            date = date_raw_string;
            break;
        } else {
            println!("Not a valid date dude. Try again in mm/dd/yyyy format!");
        }
    }

    return date;
}

pub fn get_run_distance_input() -> f64 {
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

pub fn get_run_time_input() -> String {
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
            run_time = run_time_raw_string;
            break;
        } else {
            println!("Not a valid run time dude. Try again!");
        }
    }

    return run_time;
}

pub fn get_run_description_input() -> String {
    let mut run_description = String::new();

    print!("Description of run: ");
    io::stdout().flush().expect("Darn toilet got stuck again");
    io::stdin().read_line(&mut run_description).expect("Unable to read description");

    return run_description;
}

pub fn get_is_run_workout_input() -> bool {
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

pub fn get_is_run_race_input() -> bool {
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

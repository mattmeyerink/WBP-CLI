use core::f64;
use std::io::{self, Write};
use chrono::NaiveDate;

const MILES_ON_MILES_DATE_STRING_FORMAT: &str = "m/d/Y";

pub fn get_run_date_input() -> String {
    let date;

    loop {
        let mut date_raw_string = String::new();
        print!("Date of the run (Ex -> 10/31/1981): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut date_raw_string).expect("Unable to read date");

        if NaiveDate::parse_from_str(MILES_ON_MILES_DATE_STRING_FORMAT, date_raw_string.as_str().trim()).is_ok() {
            date = date_raw_string;
            break;
        } else {
            println!("Not a valid date dude. Try again in mm/dd/yyyy format!");
        }
    }

    return date;
}

pub fn get_run_distance_input() -> String {
    let run_distance: String;

    loop {
        let mut run_distance_raw_string = String::new();
        print!("Distance of run in miles (Ex -> 10.2): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut run_distance_raw_string).expect("Unable to read date");

        if run_distance_raw_string.trim().parse::<f64>().is_ok() {
            run_distance = run_distance_raw_string;
            break;
        } else {
            println!("Not a valid distance dude. Try again!");
        }
    }

    return run_distance;
}

pub fn get_run_time_input() -> String {
    let run_time: String;

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

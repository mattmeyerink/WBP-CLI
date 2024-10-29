use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Weekday;
use dirs::home_dir;

pub struct Utils;

impl Utils {
    pub(crate) fn get_current_week_monday_from_date(date: NaiveDate) -> NaiveDate {
        let mut monday_date = date;
        loop {
            if monday_date.weekday() == Weekday::Mon {
                break;
            } else {
                monday_date = monday_date - Duration::days(1);
            }
        }

        return monday_date;
    }

    pub(crate) fn get_current_week_monday() -> DateTime<Local> {
        let mut current_date = chrono::Local::now();
        loop {
            if current_date.weekday() == Weekday::Mon {
                break;
            } else {
                current_date = current_date - Duration::days(1);
            }
        }
    
        return current_date;
    }

    pub(crate) fn get_date_string_from_weekday(current_week_monday: DateTime<Local>, weekday: Weekday) -> String {
        let mut current_date = current_week_monday.clone();

        loop {
            if current_date.weekday() == weekday {
                break;
            } else {
                current_date = current_date + Duration::days(1);
            }
        }

        return format!("{}/{}/{}", current_date.month(), current_date.day(), current_date.year());
    }

    pub(crate) fn create_miles_on_miles_directory_structure(current_date: DateTime<Local>) {
        // Make the milesonmiles data folder if it doesn't already exist
        let wbp_miles_on_miles_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles");
        if !wbp_miles_on_miles_data_path.exists() {
            fs::create_dir(&wbp_miles_on_miles_data_path).expect("Unable to make plan it dir");
        }

        // Create the outer dir for the current year if it doesn't already exist
        let current_year = current_date.year().to_string();
        let current_wbp_miles_on_miles_year_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(current_year.clone());
        if !current_wbp_miles_on_miles_year_data_path.exists() {
            fs::create_dir(&current_wbp_miles_on_miles_year_data_path).expect("Unable to make current year outer dir");
        }
    }

    pub(crate) fn create_weekly_run_log_file(filename: &String, year: String) {
        let current_wbp_miles_on_miles_year_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year);
        if !current_wbp_miles_on_miles_year_data_path.exists() {
            fs::create_dir(&current_wbp_miles_on_miles_year_data_path).expect("Unable to make current year outer dir");
        }

        let current_wbp_miles_on_miles_run_log_dir_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year).join("log");
        if !current_wbp_miles_on_miles_run_log_dir_path.exists() {
            fs::create_dir(&current_wbp_miles_on_miles_run_log_dir_path).expect("Unable to make current year run log dir");
        }

        let run_log_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year).join("log").join(filename);
        if !run_log_file_path.exists() {
            std::fs::File::create_new(&run_log_file_path).expect("There was an error making the needed file");
        }
    }

    pub(crate) fn create_week_plan_file(filename: &String, year: String) {
        let current_wbp_miles_on_miles_year_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year);
        if !current_wbp_miles_on_miles_year_data_path.exists() {
            fs::create_dir(&current_wbp_miles_on_miles_year_data_path).expect("Unable to make current year outer dir");
        }

        let current_wbp_miles_on_miles_week_plan_dir_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year).join("plan");
        if !current_wbp_miles_on_miles_week_plan_dir_path.exists() {
            fs::create_dir(&current_wbp_miles_on_miles_week_plan_dir_path).expect("Unable to make current year run log dir");
        }

        let week_plan_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(&year).join("plan").join(filename);
        if !week_plan_file_path.exists() {
            std::fs::File::create_new(&week_plan_file_path).expect("There was an error making the needed file");
        }
    }

    pub(crate) fn get_miles_on_miles_date_string_format() -> String {
        return String::from("%m/%d/%Y");
    }

    pub(crate) fn write_to_file(file_path: PathBuf, line_to_write: String, append: bool) {
        let mut data_file = OpenOptions::new()
            .append(append)
            .open(file_path)
            .expect("cannot open file");

        // Write to a file
        data_file
            .write(line_to_write.as_bytes())
            .expect("write failed");

        println!("\n");
        println!("Your run has been added! Time to party!");
        println!("\n");
    }
}

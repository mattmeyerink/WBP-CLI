use std::fs;

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

    pub(crate) fn get_miles_on_miles_date_string_format() -> String {
        return String::from("%m/%d/%Y");
    }
}

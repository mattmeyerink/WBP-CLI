use chrono::{Datelike, NaiveDate};
use dirs::home_dir;

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
    pub(crate) fn create_week_plan() {
        // Store the date string. This will be the first line in the file.
        // Create a loop getting information for each day.
        // After each day maybe output a little summary of distances/isworkout of runs added so far.
        // Have first thing entered be a mileage total for the week. 
        // At each input show remaining miles to do to hit the goal.
    }
}

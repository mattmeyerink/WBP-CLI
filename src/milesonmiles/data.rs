use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Datelike, Local, NaiveDate, Weekday};
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

    pub(crate) fn get_file_name(&self) -> String {
        let date_object = self.get_date_object();
        
        let monday_date = Utils::get_current_week_monday_from_date(date_object);

        return format!("{}-{}-{}-WeekLog.txt", monday_date.month(), monday_date.day(), monday_date.year());
    }

    pub(crate) fn get_date_object(&self) -> NaiveDate {
        return NaiveDate::parse_from_str(self.date.as_str().trim(), Utils::get_miles_on_miles_date_string_format().as_str()).unwrap();
    }

    pub(crate) fn generate_run_string(&self) -> String {
        return format!("{}--{}--{}--{}--{}--{}\n", self.date, self.distance, self.time, self.is_workout, self.is_race, self.description);
    }

    pub(crate) fn save_run(&self) {
        let filename = self.get_file_name();
        let run_year = self.get_date_object();
        Utils::create_weekly_run_log_file(&filename, run_year.year().to_string());

        let new_run_string = self.generate_run_string();
        let run_log_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(run_year.year().to_string()).join("log").join(filename);

        Utils::append_to_file(run_log_file_path, new_run_string);
    }

    pub(crate) fn read_run_from_string(run_string: &str) -> Run {
        let run_parts: Vec<&str> = run_string.split("--").collect();

        if run_parts.len() < 6 {
            let empty_run = Run {
                date: String::new(),
                distance: 0.0,
                time: String::new(),
                is_workout: false,
                is_race: false,
                description: String::new()
            };

            return empty_run;
        }

        let run = Run {
            date: String::from(run_parts[0]),
            distance: run_parts[1].trim().parse::<f64>().unwrap(),
            time: String::from(run_parts[2]),
            is_workout: run_parts[3] == "true",
            is_race: run_parts[4] == "true",
            description: String::from(run_parts[5])
        };

        return run;
    }
}

pub struct WeekPlan {
    pub(crate) date: String,
    pub(crate) runs: Vec<Run>
}

impl WeekPlan {
    pub(crate) fn create_week_plan(current_date: DateTime<Local>) {
        let mut week_plan = WeekPlan {
            date: format!("{}/{}/{}", current_date.month(), current_date.day(), current_date.year()),
            runs: vec![]
        };

        println!("");
        println!("Planning the training for week of {}", week_plan.date);
        println!("");
        
        let goal_weekly_mileage = InputUtils::get_goal_weekly_mileage();
        let mut current_weekly_mileage = 0.0;

        println!("");
        println!("Lets get to planning!");

        loop {
            let remaining_miles_to_plan = goal_weekly_mileage - current_weekly_mileage;

            println!("");
            println!("You have {} miles left to plan", remaining_miles_to_plan);
            println!("");

            week_plan.print_week_plan();
            println!("");

            let add_another_run = InputUtils::get_input_confirmation(String::from("Do you want to add another run?"));
            println!("");

            if !add_another_run {
                break;
            }
    
            loop {
                let day_of_week = InputUtils::get_day_of_week();
                let date = Utils::get_date_string_from_weekday(current_date.clone(), day_of_week);
                let distance = InputUtils::get_run_distance_input();
                let is_workout = InputUtils::get_is_run_workout_input();
                let is_race = InputUtils::get_is_run_race_input();
                let description = InputUtils::get_run_description_input();

        
                let run = Run {
                    date,
                    distance,
                    time: String::from("0"),
                    is_workout,
                    is_race,
                    description
                };
        
                println!("");
                println!("Does this look right?");
                println!("");
                println!("Date: {}", run.date);
                println!("Distance: {}", run.distance);
                println!("Is workout: {}", run.is_workout);
                println!("Is race: {}", run.is_race);
                println!("Description: {}", run.description);
        
                let is_input_confirmed = InputUtils::get_input_confirmation(String::from("Does this run look good?"));
        
                if is_input_confirmed {
                    week_plan.runs.push(run);
                    current_weekly_mileage += distance;
                    println!("Awesome on to the next day!");
                    break;
                } else {
                    println!("Lets run that back and try again.");
                }
            }
        }

        week_plan.save_week_plan();
    }

    pub(crate) fn read_week_plan(current_date: DateTime<Local>) -> WeekPlan{
        let mut week_plan = WeekPlan {
            date: format!("{}/{}/{}", current_date.month(), current_date.day(), current_date.year()),
            runs: vec![]
        };

        let file_path = week_plan.get_week_plan_file_path();
        let week_plan_file_contents = Utils::read_from_file(file_path);

        for line in week_plan_file_contents.lines() {
            let run = Run::read_run_from_string(line);

            // The line wasn't a valid run if we couldn't pull a date off of it
            if run.date.len() == 0 {
                continue;
            }

            week_plan.runs.push(run);
        }

        return week_plan;
    }

    pub(crate) fn get_date_object(&self) -> NaiveDate {
        return NaiveDate::parse_from_str(self.date.as_str().trim(), Utils::get_miles_on_miles_date_string_format().as_str()).unwrap();
    }

    fn save_week_plan(&self) {
        let week_date_object = self.get_date_object();
        let filename = self.get_file_name();
        Utils::create_week_plan_file(&filename, String::from(week_date_object.year().to_string()));

        // Create the first line that has the date of the first monday of the plan
        let mut week_plan_file_string = format!("{}\n", self.date.clone());

        for run in &self.runs {
            week_plan_file_string = format!("{}{}", week_plan_file_string, run.generate_run_string());
        }

        let week_plan_file_path = self.get_week_plan_file_path();
        
        Utils::write_to_file(week_plan_file_path, week_plan_file_string);
    }

    fn get_file_name(&self) -> String {
        return format!("{}-WeekPlan.txt", self.date.replace("/", "-"));
    }

    fn get_week_plan_file_path(&self) -> PathBuf {
        let file_name = self.get_file_name();
        let week_date_object = self.get_date_object();
        return home_dir().unwrap().join("Documents").join("wbp-data").join("miles-on-miles").join(week_date_object.year().to_string()).join("plan").join(file_name);
    }   

    fn calculate_total_weekly_mileage(&self) -> f64 {
        let mut total_mileage = 0.0;
        for run in &self.runs {
            total_mileage += run.distance;
        }

        return total_mileage;
    }

    pub(crate) fn print_week_plan(&self) {
        let weekday_iterable = vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun
        ];

        let mut runs_by_week_day: HashMap<Weekday, Vec<&Run>> = HashMap::from([
            (Weekday::Mon, vec![]),
            (Weekday::Tue, vec![]),
            (Weekday::Wed, vec![]),
            (Weekday::Thu, vec![]),
            (Weekday::Fri, vec![]),
            (Weekday::Sat, vec![]),
            (Weekday::Sun, vec![])
        ]);

        let weekday_to_display_text: HashMap<Weekday, String> = HashMap::from([
            (Weekday::Mon, String::from("Monday")),
            (Weekday::Tue, String::from("Tuesday")),
            (Weekday::Wed, String::from("Wednesday")),
            (Weekday::Thu, String::from("Thursday")),
            (Weekday::Fri, String::from("Friday")),
            (Weekday::Sat, String::from("Saturday")),
            (Weekday::Sun, String::from("Sunday"))
        ]);

        for run in &self.runs {
            let run_date_object = run.get_date_object();

            runs_by_week_day.get_mut(&run_date_object.weekday()).unwrap().push(run);
        }

        for weekday in weekday_iterable {
            let weekday_runs = runs_by_week_day.get(&weekday).unwrap();

            let mut weekday_run_strings: Vec<String> = vec![];
            for run in weekday_runs {

                let descriptor;
                if run.is_race {
                    descriptor = "R";
                } else if run.is_workout {
                    descriptor = "W";
                } else {
                    descriptor = "";
                }

                let run_string = format!("{} {}", run.distance, descriptor);
                weekday_run_strings.push(run_string);
            }

            let weekday_runs_formatted;
            if weekday_run_strings.len() == 0 {
                weekday_runs_formatted = String::from("REST DAY");
            } else {
                weekday_runs_formatted = weekday_run_strings.join(",");
            }

            let weekday_string = weekday_to_display_text.get(&weekday).unwrap();

            println!("{}: {}", weekday_string, weekday_runs_formatted);
        }
        
        let total_weekly_mileage = &self.calculate_total_weekly_mileage();
        println!("--------------------");
        println!("Total: {}", total_weekly_mileage);
    }
}

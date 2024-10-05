use chrono::Datelike;

use crate::milesonmiles::{data::Run, input_utils::InputUtils, utils::Utils};

pub fn log_run() {
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

        println!("Does this look right?");
        println!("Date: {}", run.date);
        println!("Distance: {}", run.distance);
        println!("Time: {}", run.time);
        println!("Is workout: {}", run.is_workout);
        println!("Is race: {}", run.is_race);
        println!("Description: {}", run.description);

        let is_input_confirmed = InputUtils::get_input_confirmation();

        if is_input_confirmed {
            println!("Awesome we will add it then!");
            break;
        } else {
            println!("Lets run that back and try again.");
        }
    }

    let filename = run.get_file_name();
    let run_year = run.get_date_object();
    Utils::create_weekly_run_log_file(filename, run_year.year().to_string());

    // This needs to work a little differently than writing a note in planit
    // The user could enter ANY date for a run, not just a date during the current week in view.
    // Should that be the case or should we limit users to dates in their current view?
    // I think allowing flexibility is better. As long as we have confirmation 

    // Get a current date object using the run date.

    // Pass that durrent date object to a get monday from date function.

    // Use that monday to determine the file name to save this run to.

    // Create the file if it doesn't already exist.

    // Create the new line to store this run in the file.
}

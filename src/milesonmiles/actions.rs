use crate::milesonmiles::{data::Run, input_utils};

pub fn log_run() {
    println!("A run has been done! Many congrats!");
    println!("Enter the following information and it will be done!");

    let date = input_utils::get_run_date_input();
    let distance = input_utils::get_run_distance_input();
    let time = input_utils::get_run_time_input();
    let is_workout = input_utils::get_is_run_workout_input();
    let is_race = input_utils::get_is_run_race_input();
    let description = input_utils::get_run_description_input();

    let run = Run {
        date,
        distance,
        time,
        is_workout,
        is_race,
        description
    };

    let filename = run.get_file_name();

    // Add a confirmation step that prints out the info to be stored

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

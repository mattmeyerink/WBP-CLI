use crate::milesonmiles::input_utils;

pub fn log_run() {
    println!("A run has been done! Many congrats!");
    println!("Enter the following information and it will be done!");

    let run_date = input_utils::get_run_date_input();
    let run_distance = input_utils::get_run_distance_input();
    let run_time = input_utils::get_run_time_input();
    let is_run_workout = input_utils::get_is_run_workout_input();
    let is_run_race = input_utils::get_is_run_race_input();
    let run_description = input_utils::get_run_description_input();
}

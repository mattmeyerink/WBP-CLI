use crate::milesonmiles::{data::Run, input_utils::InputUtils};

pub struct Actions;

impl Actions {
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
}

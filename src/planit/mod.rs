use std::fs;
use std::io;
use std::io::Write;
use dirs::home_dir;

mod week;
mod year;

pub fn planit() {
    loop {
        // Make the planit data folder if it doesn't already exist
        let wbp_plan_it_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it");
        if !wbp_plan_it_data_path.exists() {
            fs::create_dir(&wbp_plan_it_data_path).expect("Unable to make plan it dir");
        }

        // TODO -> Maybe add a step to create a year directory

        println!("");
        println!("Welcome to PlanIt!");
        println!("");

        print!("Which view do you want to take? (Week, Month, Year) or quit to exit: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut view = String::new();
        io::stdin().read_line(&mut view).expect("Unable to read view");

        if view.trim().to_lowercase() == "week" {
            week::week_view();
        } else if view.trim().to_lowercase() == "month" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if view.trim().to_lowercase() == "year" {
            year::year_view();
        } else if view.trim().to_lowercase() == "quit" {
            break;
        } else {
            println!("\n");
            println!("That is not a supported view");
            println!("\n");
        }
    }
}

use std::io;
use std::fs;
use dirs::home_dir;
use std::io::Write;

mod planit;

fn main() {
    println!("Welcome to the Wizard Boy Productions CLI tool!");
    println!("Enjoy the Magic!!!");
    println!("");

    // Make the wbp-data folder if it doesn't already exist
    let wbp_data_path = home_dir().unwrap().join("Documents").join("wbp-data");
    if !wbp_data_path.exists() {
        fs::create_dir(&wbp_data_path).expect("Unable to create needed data directory");
    }

    print!("Enter an app (planit): ");
    io::stdout().flush().expect("Darn toilet got stuck again");

    let mut app = String::new();

    io::stdin().read_line(&mut app)
        .expect("Failed to read app");

    if app.trim() == "planit" {
        planit::planit();
    } else {
        println!("That is not a supported app");
    }
}

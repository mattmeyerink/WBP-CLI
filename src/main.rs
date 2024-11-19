use std::io;
use std::fs;
use dirs::home_dir;
use std::io::Write;

mod planit;
mod milesonmiles;

fn main() {
    println!("");
    println!("Welcome to the Wizard Boy Productions CLI tool!");
    println!("Enjoy the Magic!!!");
    println!("");

    println!("
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣼⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⡍⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠃⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⢀⣤⣾⠿⠛⠛⠛⠿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⠿⠟⠛⠛⠿⢷⣦⡀⠀⠀⠀⠀
        ⠀⠀⢠⣿⢟⡄⠀⠀⠀⠀⠀⠀⠹⣷⡄⠀⠀⠀⠀⠀⢀⣾⠟⡁⠀⠀⠀⠀⠀⠀⠙⢿⣆⠀⠀⠀
        ⠀⠀⣿⠇⣾⠁⠀⠀⠀⠀⠀⠀⠀⢹⣷⣾⠿⠛⠻⣷⣾⡟⣸⡇⠀⠀⠀⠀⠀⠀⠀⠈⣿⡄⠀⠀
        ⠿⠿⣿⡀⣿⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠁⠀⠀⠀⠈⢿⡇⢿⡄⠀⠀⠀⠀⠀⠀⠀⠀⣿⡿⠿⠂
        ⠀⠀⢻⣧⠹⣧⡀⠀⠀⠀⠀⠀⢀⣾⡏⠀⠀⠀⠀⠀⠸⣿⡘⢷⣄⠀⠀⠀⠀⠀⠀⣰⡿⠀⠀⠀
        ⠀⠀⠀⠻⣷⣬⣛⠁⠀⠀⣀⣤⣾⠏⠀⠀⠀⠀⠀⠀⠀⠙⢿⣦⣝⡃⠀⠀⢀⣠⣾⠟⠁⠀⠀⠀
        ⠀⠀⠀⠀⠈⠙⠛⠿⠿⠿⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⠿⠛⠛⠁⠀⠀
    ");

    println!("");


    // Make the wbp-data folder if it doesn't already exist
    let wbp_data_path = home_dir().unwrap().join("Documents").join("wbp-data");
    if !wbp_data_path.exists() {
        fs::create_dir(&wbp_data_path).expect("Unable to create needed data directory");
    }

    loop {
        print!("Enter an app (planit/milesonmiles) or quit: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut app = String::new();

        io::stdin().read_line(&mut app)
            .expect("Failed to read app");

        if app.trim() == "planit" {
            planit::planit();
        } else if app.trim() == "milesonmiles" {
            milesonmiles::milesonmiles();
        } else if app.trim() == "quit" {
            break;
        } else {
            println!("That is not a supported app");
        }
    }

    println!("");
    println!("Thanks for stopping by! Please come again soon!");
    println!("
        ⠀⠀⠀⠀⠀⠀⠀⣠⣠⣶⣿⣷⣿⣿⣿⣷⣷⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⣤⣾⣿⢿⣻⡽⣞⣳⡽⠚⠉⠉⠙⠛⢿⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⣼⣿⣿⢻⣟⣧⢿⣻⢿⠀⠀⠀⠀⠀⠀⠀⠻⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⢀⣾⣿⡿⠞⠛⠚⠫⣟⡿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣧⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⣼⣿⡟⠀⠀⠀⠀⠀⠈⢻⡽⣆⠀⠀⣴⣷⡄⠀⠀⠀⠘⣿⡆⠀⠀⣀⣠⣤⡄
        ⠀⠀⣿⣿⠁⠀⠀⠀⠀⠀⠀⠈⣿⠿⢷⡀⠘⠛⠃⠀⠀⠀⠀⣿⣅⣴⡶⠟⠋⢹⣿
        ⠀⠀⢻⣿⡀⠀⠀⠀⣾⣿⡆⠀⢿⣴⣴⡇⠀⠀⠀⠀⠀⠀⢠⡟⠋⠁⠀⠀⠀⢸⣿
        ⠀⠀⠈⢿⣇⠀⠀⠀⠀⠉⠁⠀⠀⠉⠉⠀⠀⠀⠀⠀⠀⢀⡾⠁⠀⠀⠀⠀⠀⣾⡏
        ⠀⠀⠀⠈⢿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⢸⠁⠀⠀⠀⠀⠀⣼⡟⠀
        ⠀⠀⠀⠀⠀⣹⣿⣶⣤⣀⡀⠀⠀⠀⠀⠀⣀⠀⠀⠂⠁⠀⠐⢧⡀⠀⢀⣾⠟⠀⠀
        ⠀⠀⢀⣰⣾⠟⠉⠀⠀⠉⠉⠀⠐⠂⠀⠁⠁⠀⠀⠀⠀⠀⠀⠈⢿⣶⡟⠋⠀⠀⠀
        ⣠⣶⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⡆⠀⠀⠀⠀
        ⢻⣧⣄⠀⠀⠀⢰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀
        ⠀⠉⠛⠿⣷⣶⣾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣤⣾⣿⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣷⣦⡀⠀⢀⣀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣷⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠻⢿⣿⣿⣿⣿⠿⠿⠿⠿⠿⠿⠿⠿⣿⣿⣿⠿⠟⠁⠀
    ");
    println!("");
    
}

use std::io;

fn main() {
    println!("Welcome to the Wizard Boy Productions CLI tool!");
    println!("Enjoy the Magic!!!");
    println!("");
    println!("Select an app (planit)");

    let mut app = String::new();

    io::stdin().read_line(&mut app)
        .expect("Failed to read app");

    if app.trim() == "planit" {
        println!("Which view do you want to take? (Week, Month, Year)");

        let mut view = String::new();
        io::stdin().read_line(&mut view).expect("Unable to read view");

        if view.trim() == "Week" {
            // Attempt to pull the text file that has this weeks notes
            
            // Display the week's notes
            println!("Week View");
            println!("Monday");
            println!("--------------");
            println!("Tuesday");
            println!("--------------");
            println!("Wednesday");
            println!("--------------");
            println!("Thursday");
            println!("--------------");
            println!("Friday");
            println!("--------------");
            println!("Saturday");
            println!("--------------");
            println!("Sunday");
            println!("--------------");

            println!("");
            println!("");

            println!("What would you like to do?");
            println!("[1]: Edit a current note.");
            println!("[2]: Add new note.");
            println!("[3]: Next week.");
            println!("[4]: Previous week.");

            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Unable to read action");

            if action.trim() == "1" {
                // Edit a current note.
            } else if action.trim() == "2" {
                // Add a new note
            } else if action.trim() == "3" {
                // Move forward to the next week
            } else if action.trim() == "4" {
                // Move backward to the previous week
            } else {
                println!("Boooo thats not a correct action");
            }
        } else if view.trim() == "Month" {
            
        } else if view.trim() == "Year" {

        } else {
            println!("That is not a supported view");
        }
    } else {
        println!("That is not a supported app");
    }
}

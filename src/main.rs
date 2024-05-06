use std::io;
use std::fs;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Welcome to the Wizard Boy Productions CLI tool!");
    println!("Enjoy the Magic!!!");
    println!("");

    // Make the wbp-data folder if it doesn't already exist
    let wbp_data_path = home_dir().unwrap().join("Documents").join("wbp-data");
    if !wbp_data_path.exists() {
        fs::create_dir(&wbp_data_path).expect("Unable to create needed data directory");
    }

    println!("Select an app (planit)");

    let mut app = String::new();

    io::stdin().read_line(&mut app)
        .expect("Failed to read app");

    if app.trim() == "planit" {
        // Make the planit data folder if it doesn't already exist
        let wbp_plan_it_data_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it");
        if !wbp_plan_it_data_path.exists() {
            fs::create_dir(&wbp_plan_it_data_path).expect("Unable to make plan it dir");
        }

        // TODO -> Maybe add a step to create a year directory

        println!("Which view do you want to take? (Week, Month, Year)");

        let mut view = String::new();
        io::stdin().read_line(&mut view).expect("Unable to read view");

        if view.trim() == "Week" {
            // Attempt to pull the text file that has this weeks notes
            let week_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join("test.txt");
            if !week_file_path.exists() {
                // TODO -> Create the file once I can look up how to do that
            }
    
            let contents = fs::read_to_string(week_file_path).expect("Should have been able to read the file");

            for line in contents.lines() {
                println!("{}", line);
            }

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
                // Actually just have them put in the day of the week and associate that with 
                // The right date
                println!("Enter the day of the week to add it to");
                let mut day_of_week = String::new();
                io::stdin().read_line(&mut day_of_week).expect("Unable to read date");

                println!("Enter the type of note (task/event/note)");
                let mut note_type = String::new();
                io::stdin().read_line(&mut note_type).expect("Unable to read note type");

                println!("Enter your note");
                let mut note = String::new();
                io::stdin().read_line(&mut note).expect("Unable to read note");

                // TODO -> This needs to be a unique id
                let note_id = "UNIQUE_ID";

                // TODO -> Need to do this so we actully know which date it is
                let date = "SomeDate";

                // TODO -> This needs to be calculated
                let modified_date_time = "SomeDateTimeNumbers";

                let is_complete = "false";

                let new_note = format!("{}-{}-{}-{}-{}-{}-{}\n", note_id, date, day_of_week.trim(), note_type.trim(), is_complete, note.trim(), modified_date_time);

                let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join("test.txt");

                // Add a new note
                let mut data_file = OpenOptions::new()
                    .append(true)
                    .open(file_path)
                    .expect("cannot open file");

                // Write to a file
                data_file
                    .write(new_note.as_bytes())
                    .expect("write failed");

                println!("Appended content to a file");
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

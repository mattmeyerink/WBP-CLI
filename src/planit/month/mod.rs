use std::io::{self, Write};

mod utils;
mod display;
mod data;
mod actions;

pub fn month_view() {
    loop {
        print!("Which view do you want to take? (Highlight/List) or quit to exit: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut type_of_month_view = String::new();
        io::stdin().read_line(&mut type_of_month_view).expect("Unable to read view");

        if type_of_month_view.trim().to_lowercase() == "highlight" {
            month_highlight_view();
        } else if type_of_month_view.trim().to_lowercase() == "list" {
            month_list_view();
        } else if type_of_month_view.trim().to_lowercase() == "quit" {
            break;
        } else {
            println!("\n");
            println!("That is not a supported view");
            println!("\n");
        }
    }
}

fn month_highlight_view() {
    // let mut current_date = chrono::Local::now();
}

fn month_list_view() {
    let current_date = chrono::Local::now();

    loop {
        let month_notes = data::fetch_month_notes(current_date);

        display::display_month_notes(current_date, month_notes, false);
        // Fetch the current state of the month list notes

        // Display those notes

        // Display the actions they can take on these notes
        
        // Switch over which action to take.
    }
}

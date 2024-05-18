use std::io::{self, Write};

use chrono::Duration;

use self::display::print_year_header;

mod actions;
mod display;

pub fn year_view() {
    let mut current_date = chrono::Local::now();

    loop {
        print_year_header(current_date);
        
        println!("Actions you can take.");
        println!("[1]: Add new note.");
        println!("[2]: Mark a task complete.");
        println!("[3]: Edit a current note.");
        println!("[4]: Next year.");
        println!("[5]: Previous year.");
        println!("[6]: Quit year view");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            actions::add_new_year_note();
        } else if action.trim() == "2" {
            actions::mark_year_note_complete();
        } else if action.trim() == "3" {
            actions::edit_year_note();
        } else if action.trim() == "4" {
            current_date = current_date + Duration::days(365);
        } else if action.trim() == "5" {
            current_date = current_date - Duration::days(365);
        } else if action.trim() == "6" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

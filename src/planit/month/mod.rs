use std::io::{self, Write};

use chrono::Months;

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
    let mut current_date = chrono::Local::now();

    loop {
        let month_highlights = data::fetch_month_highlights(current_date);

        display::display_month_highlights(current_date, month_highlights);

        println!("Actions you can take.");
        println!("[1]: Add a highlight.");
        println!("[2]: Delete a highlight.");
        println!("[3]: Previous month.");
        println!("[4]: Next month.");
        println!("[5]: Quit month highlight view.");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            actions::add_month_highlight(current_date);
        } else if action.trim() == "2" {
            actions::delete_month_highlight(current_date);
        } else if action.trim() == "3" {
            current_date = current_date.checked_sub_months(Months::new(1)).unwrap();
        } else if action.trim() == "4" {
            current_date = current_date.checked_add_months(Months::new(1)).unwrap();
        } else if action.trim() == "5" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

fn month_list_view() {
    let mut current_date = chrono::Local::now();

    loop {
        let month_notes = data::fetch_month_notes(current_date);

        display::display_month_notes(current_date, month_notes, false);
        
        println!("Actions you can take.");
        println!("[1]: Add new note.");
        println!("[2]: Mark a task complete.");
        println!("[3]: Edit a current note.");
        println!("[4]: Delete a current note.");
        println!("[5]: Previous month.");
        println!("[6]: Next month.");
        println!("[7]: Quit month note list view");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            actions::add_month_note(current_date);
        } else if action.trim() == "2" {
            actions::mark_month_note_complete(current_date);
        } else if action.trim() == "3" {
            actions::edit_month_note(current_date);
        } else if action.trim() == "4" {
            actions::delete_month_note(current_date);
        } else if action.trim() == "5" {
            current_date = current_date.checked_sub_months(Months::new(1)).unwrap();
        } else if action.trim() == "6" {
            current_date = current_date.checked_add_months(Months::new(1)).unwrap();
        } else if action.trim() == "7" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

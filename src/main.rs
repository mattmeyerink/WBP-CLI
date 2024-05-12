use std::io;
use std::fs;
use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Weekday;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Datelike;
use std::collections::HashMap;
use uuid::Uuid;

struct WeekNote {
    note_id: String,
    date: String,
    day_of_week: String,
    note_type: String,
    is_complete: String,
    note: String,
    modified_date_time: String
}

fn display_days_notes(week_notes: &Vec<WeekNote>, section_title: String, display_id: bool) {
    println!("{}", section_title);
    println!("--------------");
    for week_note in week_notes {
        let note_symbol;
        if week_note.note_type == "task" && week_note.is_complete == "true" {
            note_symbol = "x";
        } else if week_note.note_type == "task" && week_note.is_complete == "false" {
            note_symbol = "o";
        } else if week_note.note_type == "event" {
            note_symbol = "*";
        } else {
            note_symbol = "-";
        }
        if display_id {
            println!("{} {} ({})", note_symbol, week_note.note, week_note.note_id);
        } else {
            println!("{} {}", note_symbol, week_note.note);
        }
        
    }
    println!("");
}

fn get_current_week_monday() -> DateTime<Local> {
    let mut current_date = chrono::Local::now();
    loop {
        if current_date.weekday() == Weekday::Mon {
            break;
        } else {
            current_date = current_date - Duration::days(1);
        }
    }

    return current_date;
}

fn get_contents_of_week_notes_file(current_date: DateTime<Local>) -> String {
    let current_week_monday_date_string = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());

    // Attempt to pull the text file that has this weeks notes
    let week_file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(current_week_monday_date_string);
    if !week_file_path.exists() {
        std::fs::File::create_new(&week_file_path).expect("There was an error making the needed file");
    }

    let contents = fs::read_to_string(week_file_path).expect("Should have been able to read the file");

    return contents;
}

fn fetch_week_notes(current_date: DateTime<Local>) -> HashMap<String, Vec<WeekNote>> {
    let contents = get_contents_of_week_notes_file(current_date);

    let mut week_notes: HashMap<String, Vec<WeekNote>>  = HashMap::from([
        (String::from("0"), Vec::new()),
        (String::from("1"), Vec::new()),
        (String::from("2"), Vec::new()),
        (String::from("3"), Vec::new()),
        (String::from("4"), Vec::new()),
        (String::from("5"), Vec::new()),
        (String::from("6"), Vec::new())
    ]);

    for line in contents.lines() {
        let week_note_array: Vec<&str> = line.split("--").collect();

        if week_note_array.len() < 7 {
            continue;
        }
        
        let week_note = WeekNote {
            note_id: String::from(week_note_array[0]),
            date: String::from(week_note_array[1]),
            day_of_week: String::from(week_note_array[2]),
            note_type: String::from(week_note_array[3]),
            is_complete: String::from(week_note_array[4]),
            note: String::from(week_note_array[5]),
            modified_date_time: String::from(week_note_array[6])
        };

        week_notes.get_mut(&String::from(week_note_array[2])).unwrap().push(week_note);
    }

    return week_notes;
}

fn display_week_notes(week_notes: HashMap<String, Vec<WeekNote>>, display_id: bool) {
    // Display the week's notes
    println!("");
    println!("Week View");
    println!("");
    
    display_days_notes(week_notes.get(&String::from("0")).unwrap(), String::from("Monday"), display_id);

    display_days_notes(week_notes.get(&String::from("1")).unwrap(), String::from("Tuesday"), display_id);

    display_days_notes(week_notes.get(&String::from("2")).unwrap(), String::from("Wednesday"), display_id);

    display_days_notes(week_notes.get(&String::from("3")).unwrap(), String::from("Thursday"), display_id);

    display_days_notes(week_notes.get(&String::from("4")).unwrap(), String::from("Friday"), display_id);

    display_days_notes(week_notes.get(&String::from("5")).unwrap(), String::from("Saturday"), display_id);

    display_days_notes(week_notes.get(&String::from("6")).unwrap(), String::from("Sunday"), display_id);
}

fn write_to_week_notes_file(current_date: DateTime<Local>, updated_file_contents: String) {
    let file_name = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(file_name);

    // Add a new note
    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(updated_file_contents.as_bytes())
        .expect("write failed");
}

fn add_new_week_note(current_date: DateTime<Local>) {
    println!("");
    println!("A new note it is good sir or madam!");
    println!("Fill out the following information and it will be done!");
    println!("");

    let day_of_week;
    let day_of_week_to_weekday: HashMap<String, String> = HashMap::from([
        (String::from("monday"), String::from("0")),
        (String::from("tuesday"), String::from("1")),
        (String::from("wednesday"), String::from("2")),
        (String::from("thursday"), String::from("3")),
        (String::from("friday"), String::from("4")),
        (String::from("saturday"), String::from("5")),
        (String::from("sunday"), String::from("6"))
    ]);
    loop {
        let mut day_of_week_string = String::new();
        print!("Day of the week: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        io::stdin().read_line(&mut day_of_week_string).expect("Unable to read date");

        if day_of_week_to_weekday.contains_key(day_of_week_string.trim()) {
            day_of_week = day_of_week_to_weekday.get(day_of_week_string.trim()).unwrap().to_string();
            break;
        } else {
            println!("Not a valid day of the week. Try again dude!")
        }
    }
    
    let valid_note_types = vec![String::from("task"), String::from("event"), String::from("note")];
    let note_type;
    loop {
        print!("Type of note (Task/Event/Note): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_type_raw = String::new();
        io::stdin().read_line(&mut note_type_raw).expect("Unable to read note type");

        let note_type_formatted = note_type_raw.trim().to_lowercase();
        if valid_note_types.contains(&note_type_formatted) {
            note_type = note_type_formatted;
            break;
        } else {
            println!("Not a valid type dudeeee. How bout we try that again.");
        }
    }
    
    let note;
    loop {
        print!("Enter your note: ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_raw = String::new();
        io::stdin().read_line(&mut note_raw).expect("Unable to read note");

        let note_raw_format = String::from(note_raw.trim());
        if note_raw_format.len() > 0 {
            note = note_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a note without text bro.")
        }
    }
    
    // TODO -> This needs to be a unique id
    let note_id = String::from(Uuid::new_v4());

    // TODO -> Need to do this so we actully know which date it is

    // Need to grab the date object for the day of the week the user is using 
    // To add to the note
    let note_date_object = current_date + Duration::days(day_of_week.parse::<i64>().unwrap());
    let date = format!("{}-{}-{}", note_date_object.month(), note_date_object.day(), note_date_object.year());

    // TODO -> This needs to be calculated
    let modified_date_time = chrono::Local::now().timestamp().to_string();

    let is_complete = "false";

    let new_note = format!("{}--{}--{}--{}--{}--{}--{}\n", note_id, date, day_of_week, note_type, is_complete, note, modified_date_time);

    let file_name = format!("{}-{}-{}-WeekNotes.txt", current_date.month(), current_date.day(), current_date.year());
    let file_path = home_dir().unwrap().join("Documents").join("wbp-data").join("plan-it").join(file_name);

    // Add a new note
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write(new_note.as_bytes())
        .expect("write failed");

    println!("\n");
    println!("Your note has been added! Time to party!");
    println!("\n");
}

fn mark_week_note_completed(current_date: DateTime<Local>) {
    let week_file_contents = get_contents_of_week_notes_file(current_date);

    let week_notes = fetch_week_notes(current_date);

    display_week_notes(week_notes, true);

    let note_id: String;
    loop {
        print!("Enter the note_id (Grab from the print out above): ");
        io::stdout().flush().expect("Darn toilet got stuck again");
        let mut note_id_raw = String::new();
        io::stdin().read_line(&mut note_id_raw).expect("Unable to read note");

        let note_id_raw_format = String::from(note_id_raw.trim());
        if note_id_raw_format.len() > 0 {
            note_id = note_id_raw_format;
            break;
        } else {
            println!("It's going to be real confusing for future you if you make a note without text bro.")
        }
    }

    let mut updated_week_file_contents = String::from("");
    for line in week_file_contents.lines() {
        if line.contains(&note_id) {
            // Now we know that this line is the one we want to replace.
            println!("{:?}", line);
            let mut updated_line_vector: Vec<&str> = line.split("--").collect();
            updated_line_vector[4] = "true";

            let updated_line_string = updated_line_vector.join("--");

            updated_week_file_contents = week_file_contents.replace(line, &updated_line_string);
        }
    }

    if updated_week_file_contents.len() == 0 {
        return;
    }

    write_to_week_notes_file(current_date, updated_week_file_contents);
}

fn week_view() {
    loop {
        let current_date = get_current_week_monday();

        let week_notes = fetch_week_notes(current_date);

        display_week_notes(week_notes, false);

        println!("Actions you can take.");
        println!("[1]: Add new note.");
        println!("[2]: Mark a task complete.");
        println!("[3]: Edit a current note.");
        println!("[4]: Next week.");
        println!("[5]: Previous week.");
        println!("[6]: Quit week view");

        println!("");

        print!("Which action do you want to take: ");
        io::stdout().flush().expect("Darn toilet got stuck again");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Unable to read action");

        if action.trim() == "1" {
            add_new_week_note(current_date);
        } else if action.trim() == "2" {
            mark_week_note_completed(current_date);
        } else if action.trim() == "3" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if action.trim() == "4" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if action.trim() == "5" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if action.trim() == "6" {
            break;
        } else {
            println!("\n");
            println!("Boooo thats not a correct action");
            println!("\n");
        }
    }
}

fn planit() {
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
            week_view();
        } else if view.trim() == "Month" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if view.trim() == "Year" {
            println!("\n");
            println!("This bad boy isn't implemented quite yet. Gonna need to try again");
            println!("\n");
        } else if view.trim() == "quit" {
            break;
        } else {
            println!("\n");
            println!("That is not a supported view");
            println!("\n");
        }
    }
}

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
        planit();
    } else {
        println!("That is not a supported app");
    }
}

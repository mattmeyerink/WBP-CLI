use chrono::{DateTime, Datelike, Days, Local};

pub fn get_number_of_days_in_month(current_date: DateTime<Local>) -> u32 {
    let mut local_current_date_copy = current_date.clone();
    let current_month = current_date.month();
    let mut number_of_days_in_month = 28;
    
    loop {
        if local_current_date_copy.month() != current_month {
            break;
        }

        number_of_days_in_month = local_current_date_copy.day();
        local_current_date_copy = local_current_date_copy.checked_add_days(Days::new(1)).unwrap();
    }

    return number_of_days_in_month;
}
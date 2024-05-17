use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use chrono::Weekday;

pub fn get_current_week_monday() -> DateTime<Local> {
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

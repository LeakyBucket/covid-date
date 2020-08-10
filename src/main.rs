use chrono::prelude::*;

fn main() {
    let first_march = Local.ymd(2020, 3, 1);
    let today = Local::today();
    let days = today.signed_duration_since(first_march).num_days();
    let marches = days / 31;
    let mut date = days % 31;

    if date == 0 {
        date = 31;
    }

    println!("It is the {} of the {} March of the year of COVID", format_count(date), format_count(marches));
}

fn format_count(count: i64) -> String {
    if count > 10 && count < 20 {
        format!("{}th", count)
    } else {
        match count % 10 {
            1 => format!("{}st", count),
            2 => format!("{}nd", count),
            3 => format!("{}rd", count),
            _ => format!("{}th", count)
        }
    }
}

use chrono::{Timelike, Utc};
use wordclock::WordClock;

fn main() {
    let clock = WordClock::new("ch-bern".to_string());
    let now = Utc::now();
    for (letter, highlight, end_of_row) in
        clock.show_time_iterator(now.hour() as usize, now.minute() as usize)
    {
        if highlight {
            print!("{}", letter);
        } else {
            print!(".");
        }
        if end_of_row {
            println!("");
        }
    }
}

use pancurses::*;
use std::time::Duration;
use chrono::{Utc, Timelike};
use std::thread::sleep;

use wordclock::WordClock;

use std::env;

trait Extract: Default {
    /// Replace self with default and returns the initial value.
    fn extract(&mut self) -> Self;
}

impl<T: Default> Extract for T {
    fn extract(&mut self) -> Self {
        std::mem::replace(self, T::default())
    }
}

pub fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("Language dialect is {}", args[1]);
    } else {
        args.push("ch-bern".to_string());
    }

    let clock = WordClock::new(args[1].extract());

    let window = initscr();
    window.nodelay(true);
    start_color();
    use_default_colors();
    cbreak();
    noecho();
    curs_set(0);

    window.refresh();

    // Create a clock window
    let width = 22;
    let height = 11;

    let y_offset = (window.get_max_y() - height) / 2;
    let x_offset = (window.get_max_x() - width) / 2;

    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);

    let mut x : i32;
    let mut y : i32;

    while window.getch().is_none() {

        let now = Utc::now();
        y = y_offset;
        x = x_offset;
        for (letter, highlight, end_of_row) in clock.show_time_iterator(now.hour() as usize, now.minute() as usize)
        {
            // highlighted letters make the time appear in readable words
            // as they are spoken
            if highlight {
                // window.attrset(Attribute::Reverse);
                window.attrset(COLOR_PAIR(1));
            } else {
                window.attrset(Attribute::Normal);
                window.attrset(COLOR_PAIR(2));
            }

            window.mvaddstr(y, x, letter );

            x += 2;
            // end of row flag is set for every last element of the row
            if end_of_row {
                x = x_offset;
                y += 1;
            }
        }
        window.attrset(Attribute::Normal);
        window.refresh();

        sleep(Duration::new(1, 0));  // sleep 1 minute

    }
    endwin();
}

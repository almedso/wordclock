//! #  Example
//!
//! ```ignore
//!
//! use std::time::{Duration, SystemTime};
//! use chrono::{DateTime, Utc};
//! use std::thread::sleep;
//!
//! fn main()
//! {
//!     clock = WorldClock::new("Bern", "CH");
//!     let mut display = IcedWordClockDisplay();
//!     display.init();
//!     loop {
//!         sleep(Duration::new(60, 0));  // sleep 1 minute
//!         let now = let now: DateTime<Utc> = Utc::now();
//!         let mut display_iterator = clock.show_time_iterator(now);
//!         display_iterator.show_time(display);
//!     }
//! }
//! ```
//!

pub struct WordClock {
    text: [&'static str; MAX_COLUMNS * MAX_ROWS],
    map_fn: fn(ClockWord) -> (usize, usize),
}

impl WordClock {
    /// Create a word clock in respective language and dialect
    ///
    /// Note:
    ///   So far only swiss german / Bern dialect is supported
    pub fn new(_language: &str, _dialect: &str) -> Self {
        WordClock {
            text: CH_BERN_GRID,
            map_fn: map_swiss_bern,
        }
    }

    /// Create an iterator to display the time in words
    ///
    /// The time is organized in words arranged into a#
    /// two-dimensional array of letters, where the time
    /// is readable from top to button, left to right
    /// the iterations is over
    ///
    /// - letter: The letter to display
    /// - highlight flag: if the letter belongs to current time
    /// - end_of_row: if the next letter (if any) should start a new row
    pub fn show_time_iterator<'a>(&'a self, hour: usize, minute: usize) -> WordClockIterator<'a> {
        WordClockIterator {
            index: 0,
            hour,
            minute,
            word_clock: self,
        }
    }
}

pub struct WordClockIterator<'a> {
    index: usize,
    hour: usize,
    minute: usize,
    word_clock: &'a WordClock,
}

impl<'a> WordClockIterator<'a> {
    fn higlight_character(&self) -> bool {
        for word in map_time_to_clock_words(self.hour, self.minute) {
            if let Some(x) = word {
                let (start, length) = (self.word_clock.map_fn)(x);
                if start <= self.index && start + length > self.index {
                    return true;
                }
            }
        }
        false
    }
}

impl<'a> Iterator for WordClockIterator<'a> {
    type Item = (&'a str, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= MAX_COLUMNS * MAX_ROWS {
            return None;
        }

        let end_of_row = (self.index % MAX_COLUMNS) == (MAX_COLUMNS - 1);
        let highlight = self.higlight_character();
        let character = self.word_clock.text[self.index];
        self.index += 1;
        Some((character, highlight, end_of_row))
    }
}

/// 1-dimensional representation of an 11x11 array representing a WordClock
///
/// 1st 10 rows are used for the word, 11th row is to show minutes
///

pub const MAX_COLUMNS: usize = 11;
pub const MAX_ROWS: usize = 11;

pub const CH_BERN_GRID: [&str; MAX_COLUMNS * MAX_ROWS] = [
    "E", "S", "K", "I", "S", "C", "H", "A", "F", "Ü", "F", "V", "I", "E", "R", "T", "U", "B", "F",
    "Z", "Ä", "Ä", "Z", "W", "Ä", "N", "Z", "G", "S", "I", "V", "O", "R", "A", "B", "O", "H", "A",
    "U", "B", "I", "E", "P", "M", "E", "I", "S", "Z", "W", "O", "I", "S", "D", "R", "Ü", "V", "I",
    "E", "R", "F", "Ü", "N", "F", "I", "Q", "T", "S", "E", "C", "H", "S", "I", "S", "I", "B", "N",
    "I", "A", "C", "H", "T", "I", "N", "Ü", "N", "I", "E", "L", "Z", "Ä", "N", "I", "E", "R", "B",
    "E", "U", "F", "I", "Z", "W", "Ö", "U", "F", "I", "A", "M", "U", "H", "R", " ", " ", " ", "*",
    "*", "*", "*", " ", " ", " ", " ",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockWord {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    FullClock,
    Half,
    FiveMinutes,
    TenMinutes,
    To,
    Past,
    It,
    Is,
    Quarter,
    Twenty,
    OneMinute,
    TwoMinutes,
    ThreeMinutes,
    FourMinutes,
}

/// Map clock word to it"s the Position and length in the Field
/// in the switzer deutsch language
fn map_swiss_bern(clock_word: ClockWord) -> (usize, usize) {
    match clock_word {
        ClockWord::Zero => (0, 0),
        ClockWord::One => (4 * 11 + 0, 3),
        ClockWord::Two => (4 * 11 + 3, 4),
        ClockWord::Three => (4 * 11 + 8, 3),
        ClockWord::Four => (5 * 11 + 0, 4),
        ClockWord::Five => (5 * 11 + 4, 5),
        ClockWord::Six => (6 * 11 + 0, 6),
        ClockWord::Seven => (6 * 11 + 6, 5),
        ClockWord::Eight => (7 * 11 + 0, 5),
        ClockWord::Nine => (7 * 11 + 5, 4),
        ClockWord::Ten => (8 * 11 + 0, 4),
        ClockWord::Eleven => (8 * 11 + 7, 4),
        ClockWord::Twelve => (9 * 11 + 0, 6),
        ClockWord::FullClock => (9 * 11 + 8, 3),
        ClockWord::Half => (3 * 11 + 3, 5),
        ClockWord::FiveMinutes => (0 * 11 + 8, 3),
        ClockWord::TenMinutes => (1 * 11 + 8, 3),
        ClockWord::Quarter => (1 * 11 + 0, 6),
        ClockWord::Twenty => (2 * 11 + 0, 6),
        ClockWord::To => (2 * 11 + 8, 3),
        ClockWord::Past => (3 * 11 + 0, 2),
        ClockWord::It => (0 * 11 + 0, 2),
        ClockWord::Is => (0 * 11 + 3, 4),
        ClockWord::OneMinute => (10 * 11 + 3, 1),
        ClockWord::TwoMinutes => (10 * 11 + 3, 2),
        ClockWord::ThreeMinutes => (10 * 11 + 3, 3),
        ClockWord::FourMinutes => (10 * 11 + 3, 4),
    }
}

fn map_time_to_clock_words(hour: usize, minute: usize) -> [Option<ClockWord>; 5] {
    let mut clock_words: [Option<ClockWord>; 5] = [None; 5];

    let mut index: usize = 0;
    let mut hour = hour;

    // handle the minutes below 5
    let modus_five = minute % 5;
    match modus_five {
        0 => (), // ignore
        1 => {
            clock_words[index] = Some(ClockWord::OneMinute);
            index += 1;
        }
        2 => {
            clock_words[index] = Some(ClockWord::TwoMinutes);
            index += 1;
        }
        3 => {
            clock_words[index] = Some(ClockWord::ThreeMinutes);
            index += 1;
        }
        4 => {
            clock_words[index] = Some(ClockWord::FourMinutes);
            index += 1;
        }
        _ => {
            panic!("Cannot happen");
        }
    }

    // handle the minutes
    match minute - modus_five {
        0 => {
            clock_words[index] = Some(ClockWord::It);
            index += 1;
            clock_words[index] = Some(ClockWord::Is);
            index += 1;
            clock_words[index] = Some(ClockWord::FullClock);
            index += 1;
        }
        5 => {
            clock_words[index] = Some(ClockWord::FiveMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::Past);
            index += 1;
        }
        10 => {
            clock_words[index] = Some(ClockWord::TenMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::Past);
            index += 1;
        }
        15 => {
            clock_words[index] = Some(ClockWord::Quarter);
            index += 1;
            clock_words[index] = Some(ClockWord::Past);
            index += 1;
        }
        20 => {
            clock_words[index] = Some(ClockWord::Twenty);
            index += 1;
            clock_words[index] = Some(ClockWord::Past);
            index += 1;
        }
        25 => {
            clock_words[index] = Some(ClockWord::FiveMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::To);
            index += 1;
            clock_words[index] = Some(ClockWord::Half);
            index += 1;
            hour += 1;
        }
        30 => {
            clock_words[index] = Some(ClockWord::It);
            index += 1;
            clock_words[index] = Some(ClockWord::Is);
            index += 1;
            clock_words[index] = Some(ClockWord::Half);
            index += 1;
            hour += 1;
        }
        35 => {
            clock_words[index] = Some(ClockWord::FiveMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::Past);
            index += 1;
            clock_words[index] = Some(ClockWord::Half);
            index += 1;
            hour += 1;
        }
        40 => {
            clock_words[index] = Some(ClockWord::Twenty);
            index += 1;
            clock_words[index] = Some(ClockWord::To);
            index += 1;
            hour += 1;
        }
        45 => {
            clock_words[index] = Some(ClockWord::Quarter);
            index += 1;
            clock_words[index] = Some(ClockWord::To);
            index += 1;
            hour += 1;
        }
        50 => {
            clock_words[index] = Some(ClockWord::TenMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::To);
            index += 1;
            hour += 1;
        }
        55 => {
            clock_words[index] = Some(ClockWord::FiveMinutes);
            index += 1;
            clock_words[index] = Some(ClockWord::To);
            index += 1;
            hour += 1;
        }
        _ => {
            panic!("Cannot happen");
        }
    }

    // handle the hour
    hour %= 12;
    clock_words[index] = match hour {
        0 => Some(ClockWord::Twelve),
        1 => Some(ClockWord::One),
        2 => Some(ClockWord::Two),
        3 => Some(ClockWord::Three),
        4 => Some(ClockWord::Four),
        5 => Some(ClockWord::Five),
        6 => Some(ClockWord::Six),
        7 => Some(ClockWord::Seven),
        8 => Some(ClockWord::Eight),
        9 => Some(ClockWord::Nine),
        10 => Some(ClockWord::Ten),
        11 => Some(ClockWord::Eleven),
        _ => panic!("Cannot happen"),
    };
    clock_words
}

#[cfg(test)]
mod tests;

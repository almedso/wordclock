// #[allow(unused_imports)]
use super::*;

fn extract_string_from_token_en_uk(clock_word: ClockWord) -> String {
    let mut result = String::new();
    let (start, end) = map_en_uk(clock_word);
    for index in start..start + end {
        result.push_str(EN_UK_GRID[index]);
    }
    result
}

#[test]
fn test_en_uk_map_and_array() {
    assert_eq!(
        "MIDNIGHT".to_string(),
        extract_string_from_token_en_uk(ClockWord::Zero)
    );
    assert_eq!(
        "ONE".to_string(),
        extract_string_from_token_en_uk(ClockWord::One)
    );
    assert_eq!(
        "TWO".to_string(),
        extract_string_from_token_en_uk(ClockWord::Two)
    );
    assert_eq!(
        "THREE".to_string(),
        extract_string_from_token_en_uk(ClockWord::Three)
    );
    assert_eq!(
        "FOUR".to_string(),
        extract_string_from_token_en_uk(ClockWord::Four)
    );
    assert_eq!(
        "FIVE".to_string(),
        extract_string_from_token_en_uk(ClockWord::Five)
    );
    assert_eq!(
        "SIX".to_string(),
        extract_string_from_token_en_uk(ClockWord::Six)
    );
    assert_eq!(
        "SEVEN".to_string(),
        extract_string_from_token_en_uk(ClockWord::Seven)
    );
    assert_eq!(
        "EIGHT".to_string(),
        extract_string_from_token_en_uk(ClockWord::Eight)
    );
    assert_eq!(
        "NINE".to_string(),
        extract_string_from_token_en_uk(ClockWord::Nine)
    );
    assert_eq!(
        "TEN".to_string(),
        extract_string_from_token_en_uk(ClockWord::Ten)
    );
    assert_eq!(
        "ELEVEN".to_string(),
        extract_string_from_token_en_uk(ClockWord::Eleven)
    );
    assert_eq!(
        "NOON".to_string(),
        extract_string_from_token_en_uk(ClockWord::Twelve)
    );

    assert_eq!(
        "FIVE".to_string(),
        extract_string_from_token_en_uk(ClockWord::FiveMinutes)
    );
    assert_eq!(
        "TEN".to_string(),
        extract_string_from_token_en_uk(ClockWord::TenMinutes)
    );
    assert_eq!(
        "TWENTY".to_string(),
        extract_string_from_token_en_uk(ClockWord::Twenty)
    );
    assert_eq!(
        "QUARTER".to_string(),
        extract_string_from_token_en_uk(ClockWord::Quarter)
    );
    assert_eq!(
        "HALF".to_string(),
        extract_string_from_token_en_uk(ClockWord::Half)
    );
    assert_eq!(
        "OCLOCK".to_string(),
        extract_string_from_token_en_uk(ClockWord::FullClock)
    );
    assert_eq!(
        "".to_string(),
        extract_string_from_token_en_uk(ClockWord::Minutes)
    );

    assert_eq!(
        "IT".to_string(),
        extract_string_from_token_en_uk(ClockWord::It)
    );
    assert_eq!(
        "S".to_string(),
        extract_string_from_token_en_uk(ClockWord::Is)
    );

    assert_eq!(
        "TIL".to_string(),
        extract_string_from_token_en_uk(ClockWord::To)
    );
    assert_eq!(
        "PAST".to_string(),
        extract_string_from_token_en_uk(ClockWord::Past)
    );

    assert_eq!(
        "*".to_string(),
        extract_string_from_token_en_uk(ClockWord::OneMinute)
    );
    assert_eq!(
        "**".to_string(),
        extract_string_from_token_en_uk(ClockWord::TwoMinutes)
    );
    assert_eq!(
        "***".to_string(),
        extract_string_from_token_en_uk(ClockWord::ThreeMinutes)
    );
    assert_eq!(
        "****".to_string(),
        extract_string_from_token_en_uk(ClockWord::FourMinutes)
    );
}

fn extract_string_from_token_ch_bern(clock_word: ClockWord) -> String {
    let mut result = String::new();
    let (start, end) = map_swiss_bern(clock_word);
    for index in start..start + end {
        result.push_str(CH_BERN_GRID[index]);
    }
    result
}

#[test]
fn test_ch_bern_map_and_array() {
    assert_eq!(
        "ZWÖUFI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Zero)
    );
    assert_eq!(
        "EIS".to_string(),
        extract_string_from_token_ch_bern(ClockWord::One)
    );
    assert_eq!(
        "ZWOI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Two)
    );
    assert_eq!(
        "DRÜ".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Three)
    );
    assert_eq!(
        "VIER".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Four)
    );
    assert_eq!(
        "FÜNFI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Five)
    );
    assert_eq!(
        "SECHSI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Six)
    );
    assert_eq!(
        "SIBNI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Seven)
    );
    assert_eq!(
        "ACHTI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Eight)
    );
    assert_eq!(
        "NÜNI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Nine)
    );
    assert_eq!(
        "ZÄNI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Ten)
    );
    assert_eq!(
        "EUFI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Eleven)
    );
    assert_eq!(
        "ZWÖUFI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Twelve)
    );

    assert_eq!(
        "FÜF".to_string(),
        extract_string_from_token_ch_bern(ClockWord::FiveMinutes)
    );
    assert_eq!(
        "ZÄÄ".to_string(),
        extract_string_from_token_ch_bern(ClockWord::TenMinutes)
    );
    assert_eq!(
        "ZWÄNZG".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Twenty)
    );
    assert_eq!(
        "VIERTU".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Quarter)
    );
    assert_eq!(
        "HAUBI".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Half)
    );
    assert_eq!(
        "UHR".to_string(),
        extract_string_from_token_ch_bern(ClockWord::FullClock)
    );
    assert_eq!(
        "".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Minutes)
    );

    assert_eq!(
        "ES".to_string(),
        extract_string_from_token_ch_bern(ClockWord::It)
    );
    assert_eq!(
        "ISCH".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Is)
    );

    assert_eq!(
        "VOR".to_string(),
        extract_string_from_token_ch_bern(ClockWord::To)
    );
    assert_eq!(
        "AB".to_string(),
        extract_string_from_token_ch_bern(ClockWord::Past)
    );

    assert_eq!(
        "*".to_string(),
        extract_string_from_token_ch_bern(ClockWord::OneMinute)
    );
    assert_eq!(
        "**".to_string(),
        extract_string_from_token_ch_bern(ClockWord::TwoMinutes)
    );
    assert_eq!(
        "***".to_string(),
        extract_string_from_token_ch_bern(ClockWord::ThreeMinutes)
    );
    assert_eq!(
        "****".to_string(),
        extract_string_from_token_ch_bern(ClockWord::FourMinutes)
    );
}

fn extract_string_from_token_de_de(clock_word: ClockWord) -> String {
    let mut result = String::new();
    let (start, end) = map_de_de(clock_word);
    for index in start..start + end {
        result.push_str(DE_DE_GRID[index]);
    }
    result
}

#[test]
fn test_de_de_map_and_array() {
    assert_eq!(
        "ZWÖLF".to_string(),
        extract_string_from_token_de_de(ClockWord::Zero)
    );
    assert_eq!(
        "EINS".to_string(),
        extract_string_from_token_de_de(ClockWord::One)
    );
    assert_eq!(
        "ZWEI".to_string(),
        extract_string_from_token_de_de(ClockWord::Two)
    );
    assert_eq!(
        "DREI".to_string(),
        extract_string_from_token_de_de(ClockWord::Three)
    );
    assert_eq!(
        "VIER".to_string(),
        extract_string_from_token_de_de(ClockWord::Four)
    );
    assert_eq!(
        "FÜNF".to_string(),
        extract_string_from_token_de_de(ClockWord::Five)
    );
    assert_eq!(
        "SECHS".to_string(),
        extract_string_from_token_de_de(ClockWord::Six)
    );
    assert_eq!(
        "SIEBEN".to_string(),
        extract_string_from_token_de_de(ClockWord::Seven)
    );
    assert_eq!(
        "ACHT".to_string(),
        extract_string_from_token_de_de(ClockWord::Eight)
    );
    assert_eq!(
        "NEUN".to_string(),
        extract_string_from_token_de_de(ClockWord::Nine)
    );
    assert_eq!(
        "ZEHN".to_string(),
        extract_string_from_token_de_de(ClockWord::Ten)
    );
    assert_eq!(
        "ELF".to_string(),
        extract_string_from_token_de_de(ClockWord::Eleven)
    );
    assert_eq!(
        "ZWÖLF".to_string(),
        extract_string_from_token_de_de(ClockWord::Twelve)
    );

    assert_eq!(
        "FÜNF".to_string(),
        extract_string_from_token_de_de(ClockWord::FiveMinutes)
    );
    assert_eq!(
        "ZEHN".to_string(),
        extract_string_from_token_de_de(ClockWord::TenMinutes)
    );
    assert_eq!(
        "ZWANZIG".to_string(),
        extract_string_from_token_de_de(ClockWord::Twenty)
    );
    assert_eq!(
        "VIERTEL".to_string(),
        extract_string_from_token_de_de(ClockWord::Quarter)
    );
    assert_eq!(
        "HALB".to_string(),
        extract_string_from_token_de_de(ClockWord::Half)
    );
    assert_eq!(
        "UHR".to_string(),
        extract_string_from_token_de_de(ClockWord::FullClock)
    );
    assert_eq!(
        "".to_string(),
        extract_string_from_token_de_de(ClockWord::Minutes)
    );

    assert_eq!(
        "ES".to_string(),
        extract_string_from_token_de_de(ClockWord::It)
    );
    assert_eq!(
        "IST".to_string(),
        extract_string_from_token_de_de(ClockWord::Is)
    );

    assert_eq!(
        "VOR".to_string(),
        extract_string_from_token_de_de(ClockWord::To)
    );
    assert_eq!(
        "NACH".to_string(),
        extract_string_from_token_de_de(ClockWord::Past)
    );

    assert_eq!(
        "*".to_string(),
        extract_string_from_token_de_de(ClockWord::OneMinute)
    );
    assert_eq!(
        "**".to_string(),
        extract_string_from_token_de_de(ClockWord::TwoMinutes)
    );
    assert_eq!(
        "***".to_string(),
        extract_string_from_token_de_de(ClockWord::ThreeMinutes)
    );
    assert_eq!(
        "****".to_string(),
        extract_string_from_token_de_de(ClockWord::FourMinutes)
    );
}

#[test]
fn test_map_time_to_clock_words_half_mode_remainder_minutes() {
    let expected = [
        None,
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 10);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::OneMinute),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 11);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::TwoMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 12);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::ThreeMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 13);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::FourMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 14);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_mode_full_clock() {
    let expected = [
        None,
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::FullClock),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 0);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::FullClock),
        Some(ClockWord::Zero),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(0, 0);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::FullClock),
        Some(ClockWord::Twelve),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(12, 0);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::FullClock),
        Some(ClockWord::Zero),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(24, 0);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_mode_all_quarters() {
    let expected = [
        None,
        Some(ClockWord::Quarter),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 15);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::Quarter),
        Some(ClockWord::To),
        Some(ClockWord::Five),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 45);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_mode_5_10_20_min() {
    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 5);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 10);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::Twenty),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 20);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_mode_40_50_55_min() {
    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::To),
        Some(ClockWord::Five),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 55);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::TenMinutes),
        Some(ClockWord::To),
        Some(ClockWord::Five),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 50);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::Twenty),
        Some(ClockWord::To),
        Some(ClockWord::Five),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 40);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_mode_half() {
    let expected = [
        None,
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::Half),
        Some(ClockWord::Five),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 30);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::To),
        Some(ClockWord::Half),
        Some(ClockWord::Five),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 25);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Half),
        Some(ClockWord::Five),
        None,
    ];
    let received = map_time_to_clock_words_half_mode(4, 35);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_past_mode_half() {
    // fails right know since the map function is not language/ dialect dependent
    let expected = [
        None,
        Some(ClockWord::Half),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words_half_past_mode(4, 30);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::Twenty),
        Some(ClockWord::Minutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
    ];
    let received = map_time_to_clock_words_half_past_mode(4, 25);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        None,
        Some(ClockWord::FiveMinutes),
        Some(ClockWord::Twenty),
        Some(ClockWord::Minutes),
        Some(ClockWord::To),
        Some(ClockWord::Five),
    ];
    let received = map_time_to_clock_words_half_past_mode(4, 35);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_wordclock_iterator() {
    let clock = WordClock::new("ch-bern".to_string());
    let mut display_iterator = clock.show_time_iterator(1, 0);
    // 1st row
    assert_eq!(Some(("E", true, false)), display_iterator.next());
    assert_eq!(Some(("S", true, false)), display_iterator.next());
    assert_eq!(Some(("K", false, false)), display_iterator.next());
    assert_eq!(Some(("I", true, false)), display_iterator.next());
    assert_eq!(Some(("S", true, false)), display_iterator.next());
    assert_eq!(Some(("C", true, false)), display_iterator.next());
    assert_eq!(Some(("H", true, false)), display_iterator.next());
    assert_eq!(Some(("A", false, false)), display_iterator.next());
    assert_eq!(Some(("F", false, false)), display_iterator.next());
    assert_eq!(Some(("Ü", false, false)), display_iterator.next());
    assert_eq!(Some(("F", false, true)), display_iterator.next());
    // 2nd row
    assert_eq!(Some(("V", false, false)), display_iterator.next());
    assert_eq!(Some(("I", false, false)), display_iterator.next());
    assert_eq!(Some(("E", false, false)), display_iterator.next());
    assert_eq!(Some(("R", false, false)), display_iterator.next());
}

// #[allow(unused_imports)]
use super::*;

#[test]
fn test_map_swiss_bern() {
    assert_eq!((0, 0), map_swiss_bern(ClockWord::Zero));
    assert_eq!((36, 5), map_swiss_bern(ClockWord::Half));
}

#[test]
fn test_map_time_to_clock_words_remainder_minutes() {
    let expected = [
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words(4, 10);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::OneMinute),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words(4, 11);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::TwoMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words(4, 12);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::ThreeMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words(4, 13);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::FourMinutes),
        Some(ClockWord::TenMinutes),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words(4, 14);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_all_quarters() {
    let expected = [
        Some(ClockWord::Quarter),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words(4, 15);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::FullClock),
        Some(ClockWord::Four),
        None,
    ];
    let received = map_time_to_clock_words(4, 0);
    assert!(expected.iter().eq(received.iter()));

    let expected = [
        Some(ClockWord::Quarter),
        Some(ClockWord::To),
        Some(ClockWord::Five),
        None,
        None,
    ];
    let received = map_time_to_clock_words(4, 45);
    assert!(expected.iter().eq(received.iter()));
}

#[test]

fn test_map_time_to_clock_words_half_some_german_way() {
    let expected = [
        Some(ClockWord::It),
        Some(ClockWord::Is),
        Some(ClockWord::Half),
        Some(ClockWord::Five),
        None,
    ];
    let received = map_time_to_clock_words(4, 30);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_map_time_to_clock_words_half_english_way() {
    // fails right know since the map function is not language/ dialect dependent
    let expected = [
        Some(ClockWord::Half),
        Some(ClockWord::Past),
        Some(ClockWord::Four),
        None,
        None,
    ];
    let received = map_time_to_clock_words(4, 30);
    assert!(expected.iter().eq(received.iter()));
}

#[test]
fn test_wordclock_iterator() {
    let clock = WordClock::new("Bern", "CH");
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

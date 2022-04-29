use regex::Regex;

#[test]
fn test_regex() {
    // match string with all numbers
    // + means 1 or more
    assert!(Regex::new(r"^\d+$").unwrap().is_match("12345"));
    // match with date
    assert!(Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match("2019-01-01"));
    // * means 0 or more
    assert!(Regex::new(r"^zram\d*$").unwrap().is_match("zram"));
}

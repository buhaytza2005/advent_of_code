use advent_of_code::year_2015::day_5::{test_string, test_string_part_2};

#[test]
fn day_5_part_1() {
    let nice_1 = "ugknbfddgicrmopn";
    let naughty_1 = "jchzalrnumimnmhp";

    let result_2 = test_string(naughty_1);
    assert_eq!(result_2, 0);

    let result_1 = test_string(nice_1);
    assert_eq!(result_1, 1);
}
#[test]
fn day_5_part_1_naughty() {
    let naughty_1 = "haegwjzuvuyypxyu";

    let result_2 = test_string(naughty_1);
    assert_eq!(result_2, 0, "did not identify the naughty letter group");

    let naughty_1 = "jchzalrnumimnmhp";

    let result_2 = test_string(naughty_1);
    assert_eq!(
        result_2, 0,
        "did not identify that double letter is missing"
    );
    let naughty_1 = "dvszwmarrgswjxmb";

    let result_2 = test_string(naughty_1);
    assert_eq!(result_2, 0, "did not identify that there is just one vowel");
}

#[test]
fn day_5_part_2() {
    let tests = vec![
        ("aaa", 0, "overlapping pairs"),
        ("aabcdefgaa", 1, "this should match"),
        ("xxyxx", 1, "pairs with place in mid "),
        ("uurcxstgmygtbstg", 0, "just one pair"),
    ];

    for (t, expected, reason) in tests {
        let r = test_string_part_2(t);

        assert_eq!(r, expected, "{}", reason.to_string());
    }
}

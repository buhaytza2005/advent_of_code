use std::collections::HashSet;

use advent_of_code::year_2015::day_3::House;

#[test]
fn should_get_2_houses() {
    let input = ">";

    let houses = House::move_in_direction(input);

    assert_eq!(houses.len(), 2);
}
#[test]
fn should_get_2_houses_in_circle() {
    let input = "^>v<";

    let houses = House::move_in_direction(input);

    assert_eq!(houses.len(), 4);
}
#[test]
fn should_get_2_houses_after_bouncing() {
    let input = "^v^v^v^v^v";

    let houses = House::move_in_direction(input);

    assert_eq!(houses.len(), 2);
}

#[test]
fn robo_santa_tests_should_pass() {
    let input = "^v";

    let houses = House::santa_and_robot_move_in_direction(input);

    assert_eq!(houses.len(), 3);
}

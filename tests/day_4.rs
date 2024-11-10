use advent_of_code::year_2015::day_4::hash_generator;

#[test]
fn day_4_md5_generators() {
    let key = "abcdef";
    let goal = 609043;

    let smallest_number = hash_generator(&key);

    assert_eq!(smallest_number, goal);
}

#[test]
fn generator_logic() {
    let key = "abcdef";
    let goal = 609043;
    println!("{:x}", md5::compute(format!("{}{}", key, goal)));
}

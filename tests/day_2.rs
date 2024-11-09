use advent_of_code::year_2015::day_2::Dimensions;

#[test]
fn get_smallest_number() {
    //setup
    let mut d = Dimensions {
        length: 2,
        width: 3,
        height: 4,
        areas: Vec::new(),
    };
    //act
    d.calculate_surface();
    let smallest_value = d.get_smallest_value();

    //assert
    assert_eq!(smallest_value, 6);
}
#[test]
fn surface_is_calculated_right() {
    //setup
    let mut d = Dimensions {
        length: 2,
        width: 3,
        height: 4,
        areas: Vec::new(),
    };
    //act
    let surface = d.calculate_surface();
    assert_eq!(surface, 58);

    //act2
    let perimeter = d.get_smallest_perimeter();
    assert_eq!(perimeter, 10);
    let bow = d.get_bow();

    assert_eq!(bow, 24);
    let ribbon = d.get_ribbon_requirement();
    println!("{} {} ", perimeter, bow);
    assert_eq!(ribbon, 34);
}

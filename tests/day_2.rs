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
    assert_eq!(surface, 58)
}

use std::str::FromStr;

use advent_of_code::year_2015::day_6::{Action, Grid, Point};

#[test]
fn day_6_part_1() {
    let input_1 = "turn on 0,0 through 999,999";

    let res = Action::from_str(input_1).unwrap();

    assert_eq!(
        res,
        Action::On(Point { x: 0, y: 0 }, Point { x: 999, y: 999 })
    );
    let mut grid = Grid::new();
    let input_2 = "toggle 0,0 through 999,0\ntoggle 0,0 through 999,0";
    grid.get_actions(input_2);

    assert_eq!(grid.actions.len(), 2);
    let lit = grid.get_count_of_switched_on();

    assert_eq!(lit, 1000000);
}

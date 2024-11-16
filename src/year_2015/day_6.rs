use std::{str::FromStr, usize};

use anyhow::anyhow;
#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Grid {
    pub actions: Vec<Action>,
    pub grid: Vec<Vec<bool>>,
    pub brightness_grid: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        let mut brightness_grid: Vec<Vec<u32>> = Vec::new();
        for i in 0..1000 {
            grid.push(vec![]);
            brightness_grid.push(vec![]);
            for j in 0..1000 {
                grid[i].push(false);
                brightness_grid[i].push(0);
            }
        }
        Grid {
            actions: Vec::new(),
            grid,
            brightness_grid,
        }
    }
    pub fn get_actions(&mut self, input: impl Into<String>) -> &mut Self {
        let input = input.into();
        let total = input
            .split("\n")
            .into_iter()
            .filter_map(|s| Action::from_str(s).ok())
            .collect::<Vec<_>>();
        println!("There are {} actions", total.len());
        self.actions = total;
        self
    }
    pub fn get_count_of_switched_on(&mut self) -> usize {
        for action in self.actions.as_slice() {
            match action {
                Action::On(p1, p2) => {
                    for x in p1.x..p2.x + 1 {
                        for y in p1.y..p2.y + 1 {
                            self.grid[x as usize][y as usize] = true;
                        }
                    }
                }
                Action::Off(p1, p2) => {
                    for x in p1.x..p2.x + 1 {
                        for y in p1.y..p2.y + 1 {
                            self.grid[x as usize][y as usize] = false;
                        }
                    }
                }

                Action::Toggle(p1, p2) => {
                    for x in p1.x..p2.x + 1 {
                        for y in p1.y..p2.y + 1 {
                            self.grid[x as usize][y as usize] = !self.grid[x as usize][y as usize];
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                if self.grid[i][j] {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn get_brightness(&mut self) -> usize {
        for action in self.actions.as_slice() {
            match action {
                Action::On(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            self.brightness_grid[x as usize][y as usize] += 1;
                        }
                    }
                }
                Action::Off(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            self.brightness_grid[x as usize][y as usize] =
                                self.brightness_grid[x as usize][y as usize].saturating_sub(1);
                        }
                    }
                }
                Action::Toggle(p1, p2) => {
                    for x in p1.x..=p2.x {
                        for y in p1.y..=p2.y {
                            self.brightness_grid[x as usize][y as usize] += 2;
                        }
                    }
                }
            }
        }

        let total = self
            .brightness_grid
            .iter()
            .fold(0, |a, b| a + b.iter().sum::<u32>());
        total as usize
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    On(Point, Point),
    Off(Point, Point),
    Toggle(Point, Point),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        //let input_1 = "turn on 0,0 through 999,999";
        let parts = input.split(' ').collect::<Vec<_>>();
        if input == "" {
            return Err(anyhow!("Skip this"));
        }
        if parts.len() == 0 {
            return Err(anyhow!("Skip this"));
        }
        if input.starts_with("turn on") {
            Ok(Action::On(Point::parse(parts[2])?, Point::parse(parts[4])?))
        } else if input.starts_with("turn off") {
            Ok(Action::Off(
                Point::parse(parts[2])?,
                Point::parse(parts[4])?,
            ))
        } else if input.starts_with("toggle") {
            Ok(Action::Toggle(
                Point::parse(parts[1])?,
                Point::parse(parts[3])?,
            ))
        } else {
            println!("{:#?}", input);
            panic!("invalid input");
        }
    }
}

impl Point {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let parts = input.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("could not parse"));
        } else {
            return Ok(Point {
                x: parts[0].parse::<u32>()?,
                y: parts[1].parse::<u32>()?,
            });
        }
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let mut grid = Grid::new();
    grid.get_actions(input);

    let lit = grid.get_count_of_switched_on();
    println!("Lenght 1: {}", grid.grid.len());
    println!("Lenght 2: {}", grid.grid[0].len());
    println!("Lenght 2: {}", grid.grid[0][999]);
    Ok(lit.to_string())
}
pub fn part_2(input: &str) -> anyhow::Result<String> {
    let mut grid = Grid::new();
    grid.get_actions(input);

    let lit = grid.get_brightness();
    Ok(lit.to_string())
}

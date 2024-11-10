use std::{char, collections::HashSet};

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct House {
    pub x: i32,
    pub y: i32,
}

impl House {
    pub fn move_in_direction(directions: &str) -> HashSet<House> {
        let moves = directions.chars().collect::<Vec<char>>();
        let house = House::default();
        let mut houses = HashSet::new();
        houses.insert(house);

        let mut h = House::default();
        for c in moves {
            match c {
                '>' => h.x += 1,
                '<' => h.x -= 1,
                '^' => h.y += 1,
                'v' => h.y -= 1,
                _ => {}
            };

            houses.insert(h.clone());
        }
        houses
    }
    pub fn santa_and_robot_move_in_direction(directions: &str) -> HashSet<House> {
        let moves = directions.chars().collect::<Vec<char>>();
        let house = House::default();
        let mut houses = HashSet::new();
        //insert the 0,0 house
        houses.insert(house);

        let mut santa_houses = HashSet::new();
        let mut robo_houses = HashSet::new();

        let mut h = House::default();
        let mut santa_house = House::default();
        let mut robo_house = House::default();
        //split characters for santa and robo
        for (pos, c) in moves.iter().enumerate() {
            if pos % 2 != 0 {
                println!("{:#?}", h);
                robo_houses.insert(h.clone());
            } else {
                santa_houses.insert(h.clone());
            }

            match c {
                '>' => h.x += 1,
                '<' => h.x -= 1,
                '^' => h.y += 1,
                'v' => h.y -= 1,
                _ => {}
            };
        }
        println!("houses has {:#?}", houses);
        println!("robo has {:#?}", robo_houses);
        println!("santa has {:#?}", santa_houses);

        houses.extend(robo_houses);
        houses.extend(santa_houses);
        houses
    }
}
pub fn part_1(input: String) -> anyhow::Result<String> {
    let houses = House::move_in_direction(&input);
    Ok(houses.len().to_string())
}

pub fn part_2(input: String) -> anyhow::Result<String> {
    let houses = House::santa_and_robot_move_in_direction(&input);
    Ok(houses.len().to_string())
}

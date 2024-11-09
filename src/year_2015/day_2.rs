#![allow(unused)]
pub struct Dimensions {
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub areas: Vec<u32>,
}

impl Dimensions {
    pub fn from_input(line: &str) -> Vec<Self> {
        let lines: Vec<String> = line
            .split("\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let dimensions: Vec<Dimensions> = lines
            .iter()
            .map(|line| {
                let parts = line.split("x").collect::<Vec<_>>();
                Dimensions {
                    length: parts[0].parse().unwrap(),
                    width: parts[1].parse().unwrap(),
                    height: parts[2].parse().unwrap(),
                    areas: Vec::new(),
                }
            })
            .collect();
        dimensions
    }

    pub fn calculate_surface(&mut self) -> u32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;
        self.areas.push(area_1);
        self.areas.push(area_2);
        self.areas.push(area_3);
        2 * area_1 + 2 * area_2 + 2 * area_3 + self.get_smallest_value()
    }

    pub fn get_smallest_value(&self) -> u32 {
        let smallest = self.areas.iter().min();
        match smallest {
            Some(n) => *n,
            None => 0,
        }
    }
}
pub fn part_1(input: String) -> anyhow::Result<String> {
    let mut dimensions = Dimensions::from_input(&input);
    let totals: Vec<u32> = dimensions
        .iter_mut()
        .map(|d| d.calculate_surface())
        .collect();
    let total = totals.into_iter().reduce(|acc, e| acc + e).unwrap();
    Ok(total.to_string())
}

pub fn part_2(input: String) -> anyhow::Result<String> {
    Ok("".to_string())
}

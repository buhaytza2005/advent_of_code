use std::fs::{self, File};

use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

use crate::year_2015;

pub enum Day {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

pub struct App {
    pub years: Vec<u32>,
    pub days: Vec<u32>,
}

impl App {
    pub fn new() -> Self {
        App {
            days: vec![1, 2, 3, 4, 5, 6],
            years: vec![2015],
        }
    }
    pub async fn run_last(&self) -> anyhow::Result<()> {
        let day = self.days.last().unwrap();
        match day {
            1 => {
                let input = get_content(&self.years.last().unwrap(), &self.days.last().unwrap())
                    .await
                    .expect("should have input");
                let p1 = year_2015::day_1::part_1(input.clone()).expect("should process");
                let p2 = year_2015::day_1::part_2(input).expect("should process");
                println!("part 1: {}", p1);
                println!("part 2: {}", p2);
                "".to_string()
            }

            2 => {
                let input = get_content(&self.years.last().unwrap(), &self.days.last().unwrap())
                    .await
                    .expect("should have input");
                let p1 = year_2015::day_2::part_1(input.clone()).expect("should process");
                println!("part 1: {}", p1);
                let p2 = year_2015::day_2::part_2(input).expect("should process");
                println!("part 2: {}", p2);
                "".to_string()
            }
            3 => {
                let input = get_content(&self.years.last().unwrap(), &self.days.last().unwrap())
                    .await
                    .expect("should have input");
                let p1 = year_2015::day_3::part_1(input.clone()).expect("should process");
                println!("part 1: {}", p1);

                let p2 = year_2015::day_3::part_2(input).expect("should process");
                println!("part 2: {}", p2);

                "".to_string()
            }
            4 => {
                let p1 = year_2015::day_4::part_1("bgvyzdsv").expect("should process");
                println!("part 1: {}", p1);

                let p2 = year_2015::day_4::part_2("bgvyzdsv").expect("should process");
                println!("part 2: {}", p2);

                "".to_string()
            }
            5 => {
                let input = get_content(&self.years.last().unwrap(), &self.days.last().unwrap())
                    .await
                    .expect("should have input");
                let p1 = year_2015::day_5::part_1(&input.clone()).expect("should process");
                println!("part 1: {}", p1);

                let p2 = year_2015::day_5::part_2(&input.clone()).expect("should process");
                println!("part 2: {}", p2);
                "".to_string()
            }
            6 => {
                let input = get_content(&self.years.last().unwrap(), &self.days.last().unwrap())
                    .await
                    .expect("should have input");
                let p1 = year_2015::day_6::part_1(&input.clone()).expect("should process");
                println!("part 1: {}", p1);
                let p2 = year_2015::day_6::part_2(&input.clone()).expect("should process");
                println!("part 2: {}", p2);

                "".to_string()
            }
            _ => "".to_string(),
        };
        Ok(())
    }

    pub async fn get_input(&self, year: &u32, day: &u32) -> String {
        // check if the file already exists locally
        let mut current_path = std::env::current_dir().expect("Should get the current directory");
        current_path.set_file_name(format!("advent_of_code/inputs/{}/{}.txt", year, day));
        let content = fs::read_to_string(current_path.clone());

        let payload = match content {
            Err(_) => {
                let input = get_content(year, day)
                    .await
                    .expect("should have content")
                    .to_string();
                File::create(&current_path).expect("should work");

                fs::write(current_path, input.clone()).expect("should write to file");

                input
            }
            Ok(input) => input,
        };
        return payload;
    }
}

pub async fn get_content(year: &u32, day: &u32) -> anyhow::Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut headers = HeaderMap::new();
    let token = fs::read_to_string("token.cache").expect("could not read token");
    let formatted_token = format!("session={token}");
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&formatted_token.trim()).unwrap(),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let response = client.get(&url).send().await.expect("");

    let input = response.text().await.expect("could not get the input");

    return Ok(input);
}

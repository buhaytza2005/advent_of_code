use std::env;

pub struct App {
    pub years: Vec<u32>,
    pub days: Vec<u32>,
}

impl App {
    pub fn new() -> Self {
        App {
            days: vec![1],
            years: vec![2015],
        }
    }
    pub fn run() -> anyhow::Result<()> {
        Ok(())
    }
    pub async fn get_input(day: u32, year: u32) -> String {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let client = reqwest::Client::new();
        let input = reqwest::get(&url)
            .await
            .expect("")
            .text()
            .await
            .expect("could not get the input");
        let inp2 = client.get(url);
        // https://docs.rs/reqwest/latest/reqwest/
        // need to add username and password to inp2 as send params
        println!("{}", input);
        println!("{:?}", inp2);
        return String::new();
    }
}

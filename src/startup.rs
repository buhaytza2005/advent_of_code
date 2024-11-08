use std::fs::{self, File};

use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

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
    let session = response
        .headers()
        .get(COOKIE)
        .expect("should have a different sessionn");
    fs::write("token.cache", session).expect("should save token");

    let input = response.text().await.expect("could not get the input");

    return Ok(input);
}

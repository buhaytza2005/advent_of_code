use reqwest;
use std::env;
// build the link to get the input

pub async fn get_input() -> String {
    let path = env::current_dir().expect("yes").display().to_string();
    let pieces = path.split("/").collect::<Vec<_>>();

    let year: String = pieces[pieces.len() - 2].to_string();
    let temp_day: String = pieces[pieces.len() - 1].to_string();
    let day = temp_day.split("_").collect::<Vec<_>>();
    let day: i32 = day[day.len() - 1].parse::<i32>().unwrap();

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

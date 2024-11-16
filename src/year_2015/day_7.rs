/* use anyhow::anyhow;
use std::str::FromStr;

pub struct Challenge {}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Numeric,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
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
} */

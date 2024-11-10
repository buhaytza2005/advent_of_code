pub fn part_1(input: String) -> anyhow::Result<String> {
    let mut floor = 0;

    for c in input.split("").collect::<Vec<_>>().iter() {
        let movement = match *c {
            "(" => 1,
            ")" => -1,
            _ => 0,
        };
        floor += movement;
    }

    Ok(floor.to_string())
}
pub fn part_2(input: String) -> anyhow::Result<String> {
    let mut floor = 0;
    for (pos, c) in input.split("").collect::<Vec<_>>().iter().enumerate() {
        let movement = match *c {
            "(" => 1,
            ")" => -1,
            _ => 0,
        };
        floor += movement;
        if floor == -1 {
            return Ok(pos.to_string());
        }
    }
    Ok("".to_string())
}

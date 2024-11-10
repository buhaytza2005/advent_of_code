pub fn part_1(input: &str) -> anyhow::Result<String> {
    Ok(hash_generator(input).to_string())
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    for i in 0.. {
        let test_case = format!("{}{}", input, i);
        let hash = md5::compute(test_case.as_bytes());
        let stringed_hash = format!("{:x}", hash);
        let test_value = &stringed_hash[..6];

        if test_value == "000000" {
            return Ok(i.to_string());
        };
    }
    return Ok("Not found".to_string());
}

pub fn hash_generator(key: &str) -> i32 {
    for i in 0.. {
        let test_case = format!("{}{}", key, i);
        let hash = md5::compute(test_case.as_bytes());
        let stringed_hash = format!("{:x}", hash);
        let test_value = &stringed_hash[..5];

        if test_value == "00000" {
            return i;
        };
    }
    return 0;
}

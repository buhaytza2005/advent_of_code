use std::iter::zip;

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let total = input
        .split("\n")
        .into_iter()
        .filter(|s| test_string(s) == 1)
        .collect::<Vec<_>>();
    Ok(total.len().to_string())
}
pub fn part_2(input: &str) -> anyhow::Result<String> {
    let total = input
        .split("\n")
        .into_iter()
        .filter(|s| test_string_part_2(s) == 1)
        .collect::<Vec<_>>();
    Ok(total.len().to_string())
}

pub fn test_string(input: &str) -> u32 {
    let naughty = vec!["ab", "cd", "pq", "xy"];

    for e in naughty {
        if input.contains(e) {
            return 0;
        }
    }

    let mut has_two_consecutive_chars = false;
    for two_chars in zip(input.chars(), input.chars().skip(1)) {
        if two_chars.0 == two_chars.1 {
            has_two_consecutive_chars = true;
            break;
        }
    }
    if !has_two_consecutive_chars {
        return 0;
    }

    let vowels = input
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .collect::<Vec<_>>()
        .len();
    if vowels < 3 {
        return 0;
    }
    1
}

pub fn test_string_part_2(input: &str) -> u32 {
    let mut has_two_pairs = false;
    for (pos, two_chars) in zip(input.chars(), input.chars().skip(1)).enumerate() {
        // now see if they are anywhere else
        let string_to_look_for = format!("{}{}", two_chars.0, two_chars.1);
        if let Some(f) = input.rfind(&string_to_look_for) {
            if f > pos + 1 {
                has_two_pairs = true;
            }
        }
    }
    if !has_two_pairs {
        return 0;
    }
    let mut has_two_consecutive_chars = false;
    for two_chars in zip(input.chars(), input.chars().skip(2)) {
        if two_chars.0 == two_chars.1 {
            has_two_consecutive_chars = true;
            break;
        }
    }
    if !has_two_consecutive_chars {
        return 0;
    }

    1
}

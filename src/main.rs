use std::{env, str::Chars, string};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let arg_str = args.into_iter().nth(1).unwrap();
    println!("{:?}", pascalize(&arg_str));
}

fn pascalize(s: &str) -> Vec<String> {
    let mut runes: Vec<Vec<char>> = Vec::new();
    let mut case = 0;
    let mut lastcase = 0;
    for c in s.chars() {
        print!("{:?} is {:?}\n", c, c.is_uppercase());
        if c.is_uppercase() {
            case = 1;
        } else if c.is_lowercase() {
            case = 2;
        } else if c.is_numeric() {
            case = 3;
        } else {
            case = 4;
        }
        if lastcase == case {
            let last = runes.last_mut().unwrap();
            last.push(c);
        } else {
            runes.push(vec![c]);
        }
        lastcase = case;
    }
    println!("{:?}", runes);
    for i in 0..(runes.len() - 1) {
        if char::is_uppercase(runes[i][0]) && char::is_lowercase(runes[i + 1][0]) {
            let tmp = runes[i][runes[i].len() - 1];
            runes[i + 1].insert(0, tmp);
            runes[i].pop();
        }
    }
    let mut entries: Vec<String> = Vec::new();
    for s in &runes {
        if !s.is_empty() {
            let entry = s.iter().collect::<String>();
            entries.push(entry);
        }
    }
    println!("{:?}", entries);
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_runes() {
        let input = "HelloWorld";
        let expected_output = vec!["Hello", "World"];
        assert_eq!(pascalize(input), expected_output);

        // let input = "HelloWorldThisIsRust";
        // let expected_output = vec!["H", "ello", "W", "orld", "T", "his", "I", "s", "R", "ust"];
        // assert_eq!(pascalize(input), expected_output);

        // let input = "ThisIsALongStringWithoutAnyUppercase";
        // let expected_output = vec!["ThisIsALongStringWithoutAnyUppercase"];
        // assert_eq!(pascalize(input), expected_output);
    }
}

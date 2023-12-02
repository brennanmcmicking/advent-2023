use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    let words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let parts = contents.split("\n");
    for part in parts {
        if part.len() > 0 {
            let mut first_pos: i32 = (part.len() + 1) as i32;
            let mut last_pos: i32 = -1;
            let mut first_digit = "";
            let mut last_digit = "";
            for word in words.keys() {
                if part.contains(word) {
                    for (pos, s: String) in part.match_indices(word).collect() {
                        if pos < first_pos {
                            first_pos = pos;
                            first_digit = word;
                        }
                        if pos > last_pos {
                            last_pos = pos;
                            last_digit = word;
                        }
                    }
                }
            }

            println!(
                "{} -> {}{} -> {}",
                part,
                first_digit,
                last_digit,
                words[first_digit] * 10 + words[last_digit]
            );
            sum += words[first_digit] * 10 + words[last_digit];
            println!("sum: {}", sum);
        }
    }
    // for c in contents.chars() {
    //     if c.eq_ignore_ascii_case(&'\n') {
    //         sum = sum + first * 10 + last;
    //         first = -1;
    //     }
    //     if c.is_digit(10) {
    //         last = c.to_digit(10).unwrap() as i32;
    //         if first == -1 {
    //             first = last;
    //         }
    //     }
    // }

    println!("sum: {}", sum);
    Ok(())
}

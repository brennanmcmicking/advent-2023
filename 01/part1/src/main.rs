use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut first: i32 = -1;
    let mut last: i32 = -1;
    let mut sum = 0;

    for c in contents.chars() {
        if c.eq_ignore_ascii_case(&'\n') {
            sum = sum + first * 10 + last;
            first = -1;
        }
        if c.is_digit(10) {
            last = c.to_digit(10).unwrap() as i32;
            if first == -1 {
                first = last;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}

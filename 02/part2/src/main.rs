use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    // let game_regex = Regex::new(r"^\s*Game\s+(\d+)\s*$").unwrap();
    let red_regex = Regex::new(r"\D*(\d+)\s+red.*").unwrap();
    let green_regex = Regex::new(r"\D*(\d+)\s+green.*").unwrap();
    let blue_regex = Regex::new(r"\D*(\d+)\s+blue.*").unwrap();

    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut sum: i32 = 0;

    let lines = contents.split("\n");
    for line in lines {
        let spl: Vec<&str> = line.split(":").collect();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        // println!("game number is {}", &caps[1]);

        let pulls = spl[1].split(";");
        for pull in pulls {
            let num_red: i32;
            let num_green: i32;
            let num_blue: i32;
            let red_capture = red_regex.captures(pull);
            if red_capture.is_some() {
                num_red = red_capture
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
            } else {
                num_red = 0;
            }

            let green_capture = green_regex.captures(pull);
            if green_capture.is_some() {
                num_green = green_capture
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
            } else {
                num_green = 0;
            }

            let blue_capture = blue_regex.captures(pull);
            if blue_capture.is_some() {
                num_blue = blue_capture
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
            } else {
                num_blue = 0;
            }

            // println!("pull: {}", pull);
            println!("pull: {},{},{}", num_red, num_green, num_blue);
            if num_red > min_red {
                min_red = num_red;
            }
            if num_green > min_green {
                min_green = num_green;
            }
            if num_blue > min_blue {
                min_blue = num_blue;
            }
            println!("mins: {},{},{}", min_red, min_green, min_blue);
        }

        sum += min_red * min_green * min_blue;

        println!("sum: {}", sum);
    }
}

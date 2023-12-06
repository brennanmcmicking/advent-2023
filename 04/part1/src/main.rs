use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    // let re = Regex::new(
    //     r"Card\s+(\d{1,2}):\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+\|\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})\s+(\d{1,2})",
    // ).unwrap();
    let re = Regex::new(r"Card\s+(\d{1,3}):\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+\|\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    let base: i32 = 2;
    for line in contents.split("\n").collect::<Vec<&str>>() {
        println!("{}", line);
        let cap = re.captures(line).unwrap();
        let mut winning_scores = HashSet::new();
        let mut scores = HashSet::new();
        for i in 2..12 {
            winning_scores.insert(cap.get(i).unwrap().as_str());
        }
        for i in 12..37 {
            match cap.get(i) {
                Some(x) => {
                    // println!("{}", x.as_str());
                    scores.insert(x.as_str());
                }
                None => {
                    println!("get({}) was None", i);
                }
            };
        }
        let result = winning_scores.intersection(&scores).collect::<Vec<&&str>>();
        if result.len() > 0 {
            sum += base.pow((result.len() - 1).try_into().unwrap());
        }
    }

    println!("{}", sum);
}

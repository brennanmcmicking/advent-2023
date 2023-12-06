use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use regex::Regex;

fn main() {
    let mut remaining: VecDeque<i32> = VecDeque::new();
    let mut result_of: HashMap<i32, Vec<i32>> = HashMap::new();

    let re = Regex::new(r"Card\s+(\d{1,3}):\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+\|\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.split("\n").collect::<Vec<&str>>() {
        println!("{}", line);
        let cap = re.captures(line).unwrap();
        let game_num = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
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
        let result: Vec<&&str> = winning_scores.intersection(&scores).collect();
        let mut a: Vec<i32> = Vec::new();
        for i in 1..(result.len() + 1) {
            a.push(game_num + i as i32);
        }
        result_of.insert(game_num, a);
        remaining.push_back(game_num);
    }

    println!("remaining:");
    remaining
        .clone()
        .into_iter()
        .for_each(|n| println!("{}", n));

    let mut n = 0;
    while remaining.len() > 0 {
        // println!("remaining.len(): {}", remaining.len());
        let current = remaining.pop_front().unwrap();
        // println!("current: {}", current);
        // println!("front: {}", remaining.front().unwrap());
        result_of
            .get(&current)
            .unwrap()
            .into_iter()
            .for_each(|f| remaining.push_back(*f));

        n += 1;
    }

    println!("result: {}", n);
}

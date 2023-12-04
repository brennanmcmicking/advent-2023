use regex::Regex;
use std::fs;

fn is_symbol(c: char) -> bool {
    // println!("{} != .: {}", c, c != '.' && !c.is_ascii_digit());
    return c != '.' && !c.is_ascii_digit();
}

fn main() {
    let re = Regex::new(r"\D*(\d+)\D*").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();

    let mat = contents.split("\n").collect::<Vec<&str>>();
    let mut line_num = 0;
    let mut sum = 0;
    for line in mat.clone() {
        // println!("{}", line.contains("\n"));
        for cap in re.captures_iter(line) {
            cap.iter()
                .enumerate()
                .skip(1)
                .filter(|t| t.1.is_some())
                .for_each(|t| {
                    let idx = t.1.unwrap().start();
                    let n = t.1.unwrap().as_str();
                    // println!("{}: {}", t.0, num);
                    // println!("{}: {}", idx, n);
                    let mut found = false;
                    if idx > 0 {
                        let i = idx - 1;
                        if is_symbol(line.chars().nth(i).unwrap()) {
                            found = true;
                        }

                        if line_num > 0 {
                            if is_symbol(mat[line_num - 1].chars().nth(i).unwrap()) {
                                found = true;
                            }
                        }

                        if line_num + 1 < mat.len() {
                            if is_symbol(mat[line_num + 1].chars().nth(i).unwrap()) {
                                found = true;
                            }
                        }
                    }

                    let end = idx + n.len();
                    // println!("idx: {}, n.len(): {}, end: {}", idx, n.len(), end);
                    if end < line.len() {
                        if is_symbol(line.chars().nth(end).unwrap()) {
                            found = true;
                        }

                        if line_num > 0 {
                            if is_symbol(mat[line_num - 1].chars().nth(end).unwrap()) {
                                found = true;
                            }
                        }

                        if line_num + 1 < mat.len() {
                            if is_symbol(mat[line_num + 1].chars().nth(end).unwrap()) {
                                found = true;
                            }
                        }
                    }

                    for i in 0..n.len() {
                        // println!("i: {}", i);
                        if line_num > 0 {
                            if is_symbol(mat[line_num - 1].chars().nth(idx + i).unwrap()) {
                                found = true;
                            }
                        }

                        if line_num + 1 < mat.len() {
                            if is_symbol(mat[line_num + 1].chars().nth(idx + i).unwrap()) {
                                found = true;
                            }
                        }
                    }

                    println!("{} found: {}", n, found);
                    if found {
                        let parsed = n.parse::<i32>().unwrap();
                        // println!("parsed: {}", parsed);
                        sum += parsed;
                    }
                });
        }

        line_num += 1;
    }

    println!("sum: {}", sum);
}

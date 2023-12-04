use regex::Regex;
use std::fs;

#[derive(Copy, Clone)]
struct Gear {
    count: i32,
    ratio: i32,
}

fn is_gear(c: char) -> bool {
    // println!("{} != .: {}", c, c != '.' && !c.is_ascii_digit());
    return c == '*';
}

macro_rules! GRID_SIZE {
    () => {
        140
    };
}

fn check_and_iterate(
    state: &mut [[Gear; GRID_SIZE!()]; GRID_SIZE!()],
    x: usize,
    y: usize,
    c: char,
    n: i32,
) {
    println!("c: {}", c);
    if (is_gear(c)) {
        println!("gear found");
        let mut s = state[x][y];
        s.count += 1;
        s.ratio *= n;
        state[x][y] = s;
    }
}

fn main() {
    let mut state = [[Gear { count: 0, ratio: 1 }; GRID_SIZE!()]; GRID_SIZE!()];
    let re = Regex::new(r"\D*(\d+)\D*").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mat = contents.split("\n").collect::<Vec<&str>>();
    let mut line_num = 0;
    for line in mat.clone() {
        // println!("{}", line.contains("\n"));
        println!("{}", line_num);
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
                    if idx > 0 {
                        x = idx - 1;
                        y = line_num;
                        check_and_iterate(
                            &mut state,
                            x,
                            y,
                            mat[y].chars().nth(x).unwrap(),
                            n.parse::<i32>().unwrap(),
                        );

                        if line_num > 0 {
                            y = line_num - 1;
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }

                        if line_num + 1 < mat.len() {
                            y = line_num + 1;
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }
                    }

                    x = idx + n.len();
                    // println!("idx: {}, n.len(): {}, end: {}", idx, n.len(), end);
                    if x < line.len() {
                        y = line_num;
                        check_and_iterate(
                            &mut state,
                            x,
                            y,
                            mat[y].chars().nth(x).unwrap(),
                            n.parse::<i32>().unwrap(),
                        );

                        if line_num > 0 {
                            y = line_num - 1;
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }

                        if line_num + 1 < mat.len() {
                            y = line_num + 1;
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }
                    }

                    for i in 0..n.len() {
                        x = idx + i;
                        y = line_num;
                        println!("n: {} x: {}, y: {}", n, x, y);
                        check_and_iterate(
                            &mut state,
                            x,
                            y,
                            mat[y].chars().nth(x).unwrap(),
                            n.parse::<i32>().unwrap(),
                        );

                        if line_num > 0 {
                            y = line_num - 1;
                            println!("n: {} x: {}, y: {}", n, x, y);
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }

                        if line_num + 1 < mat.len() {
                            y = line_num + 1;
                            println!("n: {} x: {}, y: {}", n, x, y);
                            check_and_iterate(
                                &mut state,
                                x,
                                y,
                                mat[y].chars().nth(x).unwrap(),
                                n.parse::<i32>().unwrap(),
                            );
                        }
                    }
                });
        }

        line_num += 1;
    }

    let mut sum = 0;
    for i in 0..GRID_SIZE!() {
        for j in 0..GRID_SIZE!() {
            let g = state[i][j];
            println!("{}, {}: {}, {}", i, j, g.count, g.ratio);
            if g.count == 2 {
                // println!("({},{}): {}", i, j, g.ratio);
                sum += g.ratio;
            }
        }
    }

    println!("sum: {}", sum);
}

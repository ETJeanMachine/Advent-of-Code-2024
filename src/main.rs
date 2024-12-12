use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 1: {}", day1());
    println!("Day 2: {}", day2());
}

fn day1() -> i32 {
    let mut left = vec![];
    let mut right = HashMap::new();
    if let Ok(lines) = read_lines("./input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            // Splitting the list into two separate strings
            let mut split = line.split_whitespace();
            if let (Some(s1), Some(s2)) = (split.next(), split.next()) {
                let n = s1.parse::<i32>().unwrap();
                let m = s2.parse::<i32>().unwrap();
                left.push(n);
                if let Some(i) = right.get_mut(&m) {
                    *i += 1;
                } else {
                    right.insert(m, 1);
                }
            }
        }
    }
    let mut sum = 0;
    for n in left {
        sum += n * right.get(&n).unwrap_or(&0);
    }
    sum
}

fn day2() -> i32 {
    let mut reports = vec![];
    if let Ok(lines) = read_lines("./input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let split = line.split_whitespace();
            reports.push(vec![]);
            for s in split {
                let n = s.parse::<i32>().unwrap();
                reports.last_mut().unwrap().push(n);
            }
        }
    }
    let mut safe_count = reports.len() as i32;
    for v in reports {
        let slopes: Vec<i32> = v.windows(2).map(|n| n[1] - n[0]).collect();
        let sign = slopes.iter().sum::<i32>().signum();
        let valid = |n: i32| -> bool { n.signum() == sign && (1..=3).contains(&n.abs()) };
        let problem_idx = slopes
            .iter()
            .enumerate()
            .find(|(_, &n)| !valid(n))
            .map(|(i, _)| i);
        if let Some(i) = problem_idx {
            let mut shift_right = slopes.clone();
            let right_step = shift_right.remove(i);
            if i < shift_right.len() {
                shift_right[i] += right_step;
            }
            let mut shift_left = slopes.clone();
            let left_step = shift_left.remove(i);
            if i > 0 {
                shift_left[i - 1] += left_step;
            }
            let right_problem = shift_right.iter().find(|&&n| !valid(n));
            let left_problem = shift_left.iter().find(|&&n| !valid(n));
            if right_problem.is_some() && left_problem.is_some() {
                safe_count -= 1;
            }
        }
    }
    safe_count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

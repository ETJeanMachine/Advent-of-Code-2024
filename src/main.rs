use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 1: {}", day1())
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 1: {}", day1())
}

fn day1() -> i32 {
    let mut list1 = vec![];
    let mut list2 = vec![];
    if let Ok(lines) = read_lines("./input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            // Splitting the list into two separate strings
            let mut split = line.split_whitespace();
            if let (Some(s1), Some(s2)) = (split.next(), split.next()) {
                let n = s1.parse::<i32>().unwrap();
                let m = s2.parse::<i32>().unwrap();
                list1.push(n);
                list2.push(m);
            }
        }
    }
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
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

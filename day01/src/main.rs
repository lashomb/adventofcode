use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let binding = fs::read_to_string(file_path);
    let binding = binding.expect("REASON");
    let contents = binding.lines();
    fn part1(contents: std::str::Lines) -> i32 {
        let mut total = 0;
        for line in contents {
            let nums = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch as u8 - b'0')
                .collect::<Vec<_>>();
            let first = nums.iter().nth(0).unwrap();
            let last = nums.iter().last().unwrap();
            total += (*first as i32) * 10 + *last as i32
        }
        return total
    }
    println!("{:?}", part1(contents.clone()));

    fn part2(contents: std::str::Lines) -> i32 {
        let numz = ["one", "1",
            "two", "2",
            "three", "3",
            "four", "4",
            "five", "5",
            "six", "6",
            "seven", "7",
            "eight", "8",
            "nine", "9"];
        let mut total = 0;
        for line in contents {
            let mut first = None;
            'out: for i in 0..line.len() {
                for (index, num) in numz.iter().enumerate() {
                    if i + num.len() > line.len() {
                        continue;
                    }
                    if line[i..i + num.len()] == **num {
                        first = Some(1+ index / 2);
                        break 'out;
                    }
                }
            }
            let Some(first) = first else { panic!("invalid input"); };
            let mut last = None;
            'out: for i in (0..line.len()).rev() {
                for (index, num) in numz.iter().enumerate() {
                    if i + num.len() > line.len() {
                        continue;
                    }
                    if line[i..i + num.len()] == **num {
                        last = Some(1+ index / 2);
                        break 'out;
                    }
                }
            }
            let Some(last) = last else { panic!("invalid input"); };

            total += 10 * first as i32 + last as i32;
        }
        return total
    }
    println!("{:?}", part2(contents.clone()));
}
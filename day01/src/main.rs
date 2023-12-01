use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let binding = fs::read_to_string(file_path)
        .unwrap();

    let contents = binding.lines();
    let mut total: u32 = 0;

    for c in contents {
        let mut only_numbers: Vec<char> = Vec::new();
        let mut number_concat: String = Default::default();
        for ch in c.chars() {
            if ch.is_numeric() {
                only_numbers.append(&mut vec!(ch));
            }
        }
        let first = only_numbers.first().unwrap().to_string();
        let last = only_numbers.last().unwrap().to_string();
        number_concat.push_str(&first);
        number_concat.push_str(&last);
        println!("{:?}", number_concat);
        total = total + number_concat.parse::<u32>().unwrap();
    }
    println!("{:?}", &total);
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut number = 50;
    let mut counter = 0;

    let f = File::open("inputs/day01.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, steps) = line.split_at(1);
        let steps = steps.parse::<i32>().unwrap();

        match dir {
            "R" => {
                number += steps;
                number = number.rem_euclid(100);
            },
            "L" => {
                number -= steps;
                number = number.rem_euclid(100);

            },
            _ => panic!()
        }
        if number == 0 {
            counter += 1;
        }
    }
    println!("Counter is: {counter}")
}

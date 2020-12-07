use std::io::BufRead;
mod days;
use days::*;


fn main() {

	let day: i32 = std::io::stdin()
    .lock()
    .lines()
    .next()
    .expect("stdin should be available")
    .expect("couldn't read from stdin")
    .trim()
    .parse()
    .expect("input was not an integer");

	match day {
		1 => {
			println!("Day01!\n");
			day01::day01();
		},
		2 => {
			println!("Day02!\n");
			day02::day02();
        },
        3 => {
            println!("Day03!\n");
            day03::day03();
        },
        4 => {
            println!("Day04!\n");
            day04::day04();
        },
        5 => {
            println!("Day05!\n");
            day05::day05();
        },
        
        6 => {
            println!("Day06!\n");
            day06::day06();
        },

        7 => {
            println!("Day07!\n");
            day07::day07();
            
        }
        _ => println!("Ain't special"),
    }

}


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
            
        },
        8 => {
            println!("Day08!\n");
            day08::day08();
            
        },
        9 => {
            println!("Day09!\n");
            day09::day09();
        },
        
        10 =>{
            println!("Day10!\n");
            day10::p1();
            day10::p2();

        },

        11 => {
            println!("Day11!\n");
            day11_part_a::day11_part_a();
            day11_part_b::day11_part_b();

            
        },
        12 => {
            println!("Day12!\n");
            day12::day12();
        },
        13 => {
            println!("Day13!\n");
            day13::day13();
        },
        14 => {
            println!("Day14!\n");
            day14::day14();
        },
        15 => {
            println!("Day15!\n");
            day15::day15();
        },
        16 => {
            println!("Day16!\n");
            day16::day16();
        },
        17 => {
            println!("Day17!\n");
            day17::day17();
        },
        18 => {
            println!("Day18!\n");
            day18::day18();
        },
        19 => {
            println!("Day19!\n");
        },
        20 => {
            println!("Day20\n");
        },
        21 => {
            println!("Day21!\n");
        },
        22 => {
            println!("Day22!\n");
        },
        23 => {
            println!("Day23!\n");
        },
        24 => {
            println!("Day24!\n");
        },

        _ => println!("Ain't special"),
    }

}


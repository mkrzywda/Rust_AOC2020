use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn day02() {
    let file = File::open("in/day02").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    //println!("Part 1: {}",p1(vec));
    println!("Part 2: {}",p2(vec));

}

fn p1(vec: Vec<String>) -> u32{
    let mut correct_passwords = 0;
    for password_str in vec {
        let mut iter = password_str.split_whitespace();
        let numbers: String = iter.next().unwrap().to_string();
        let letter: char = iter.next().unwrap().chars().next().unwrap();
        let password: String = iter.next().unwrap().to_string();
        let v: Vec<&str> = numbers.split('-').collect();
        let first: u32 = v[0].parse::<u32>().unwrap();
        let second: u32 = v[1].parse::<u32>().unwrap();
        let mut count = 0;
        for c in password.chars() {
            if c == letter {
                count = count + 1;
            }
        }
        if count >= first && count <= second {
            correct_passwords += 1;
        }
    }
    correct_passwords
}

fn p2(vec: Vec<String>) -> u32 {
    let mut correct_passwords = 0;
    for password_str in vec {
        let mut iter = password_str.split_whitespace();
        let numbers: String = iter.next().unwrap().to_string();
        let letter: char = iter.next().unwrap().chars().next().unwrap();
        let password: String = iter.next().unwrap().to_string();
        let v: Vec<&str> = numbers.split('-').collect();
        let first: usize = v[0].parse::<usize>().unwrap();
        let second: usize = v[1].parse::<usize>().unwrap();
        let mut found_once = false;
        let mut found_twice = false;
        for (i, c) in password.chars().enumerate() {
            if (i+1) == first {
                if c == letter {
                    found_once = true;
                }
            }
            else if (i+1) == second {
                if found_once {
                    if c == letter {
                        found_twice = true;
                    }
                } else {
                    if c == letter {
                        found_once = true;
                    }
                }
            }
        }
        if found_once && !found_twice {
            correct_passwords += 1;
        }
    }
    correct_passwords
}
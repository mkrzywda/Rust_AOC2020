use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day03() {
    let file = File::open("in/day03").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    //println!("Part 1: {}",p1(vec));
    println!("Part 2: {}",p2(vec));

}

fn p1(vec: Vec<String>) -> u32{
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < vec.len() {
        if vec[y].chars().nth(x%vec[0].len()).unwrap() == '#' {
            count += 1;
        }
        x += 3;
        y += 1;
    }
    count
}

fn p2(vec: Vec<String>) -> u128{
    let mut answer = 1;
    let my_list = [(1,1), (3,1), (5,1), (7,1), (1, 2)];
    for (i,j) in my_list.iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut count: u128 = 0;
        
        while y < vec.len() {
            if vec[y].chars().nth(x%vec[0].len()).unwrap() == '#' {
                count += 1;
            }
            x += i;
            y += j;
        }
        answer *= count;
    }
    answer
}
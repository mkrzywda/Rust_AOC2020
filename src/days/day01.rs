use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn day01() {
    let file = File::open("in/day01").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    let file_b = File::open("in/day01_b").unwrap();
    let mut vec_b: Vec<String> = Vec::new();
    for line in BufReader::new(file_b).lines() {
        vec_b.push(line.unwrap());
    }

    println!("Part 1: {}",p1(vec));
    println!("Part 2: {}",p2(vec_b));

}

fn p1(vec: Vec<String>) -> u32 {
    let mut count = 0;

    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!{"Part01"};
                count = i*j;
                println!("2020= {} × {}, so res = {}", i, j, i*j);
            }
        }
    }
    count
}

fn p2(vec: Vec<String>) -> u32 {
    let mut count = 0;
    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == 2020 {
                    println!{"Part02"};
                    count = i*j*k;
                    println!("2020 = {} × {} × {}, so res = {}", i, j, k,i*j*k);
                }
            }
        }
    }
    count
}
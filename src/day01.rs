use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn day01() {
    p1();
    p2();
}

fn p1() {
    let file = File::open("in/day01").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!{"Part01"};
                println!("2020= {} × {}, so res = {}", i, j, i*j);
            }
        }
    }
}

fn p2() {
    let file = File::open("in/day01").unwrap();
    let mut vec: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    let v : Vec<u32> = vec.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k == 2020 {
                    println!{"Part02"};
                    println!("2020 = {} × {} × {}, so res = {}", i, j, k,i*j*k);
                }
            }
        }
    }
}
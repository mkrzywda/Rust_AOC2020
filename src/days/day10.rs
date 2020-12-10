use std::fs;

pub fn p1() {
    let file_string: String = fs::read_to_string("in/day10").unwrap();
    let mut lines: Vec<u64> = file_string
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.to_string().trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    lines.sort();

    let mut three_difference = 1;
    let mut one_difference = 0;
    let mut current_voltage = 0;
    for num in lines {
        let current_difference = num - current_voltage;
        match current_difference {
            1 => {
                one_difference += 1;
                current_voltage = num;
            }
            3 => {
                three_difference += 1;
                current_voltage = num;
            }
            _ => {
                current_voltage = num;
            }
        }
    }
    println!("{}", three_difference * one_difference);
}

pub fn p2() {
    let file_string: String = fs::read_to_string("in/day10").unwrap();
    let mut lines: Vec<u64> = file_string
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.to_string().trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    lines.sort();
    lines.push(lines[(lines.len() - 1)] + 3);
    let mut vec: Vec<u64> = vec![0; (lines[(lines.len() - 1)] + 4) as usize];
    vec[3] = 1;
    for i in 0..(lines[(lines.len() - 1)] + 1) as usize {
        if lines.iter().any(|&j| i == j as usize) {
            vec[i + 3] = vec[i + 2] + vec[i + 1] + vec[i];
        }
    }

    for (i, val) in vec.iter().enumerate() {
    }
    println!("{}", vec[vec.len() - 1]);
}
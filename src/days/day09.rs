use std::cmp::Ordering;
use std::fs;

struct Day09;

impl Day09 {
    fn part1(data: &[usize]) -> usize {
        find_invalid_number(&data, 25).unwrap()
    }

    fn part2(data: &[usize], target: usize) -> usize {
        find_enc_weakness(data, target)
    }
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(str::parse).flatten().collect()
}

fn find_invalid_number(data: &[usize], wlen: usize) -> Option<usize> {
    'outer: for w in data.windows(wlen + 1) {
        let target = w[wlen];

        for i in 0..wlen {
            for j in 0..wlen {
                if w[i] + w[j] == target && i != j {
                    continue 'outer;
                }
            }
        }
        return Some(target);
    }
    None
}

fn find_enc_weakness(data: &[usize], target: usize) -> usize {
    let mut lower = 0;
    let mut upper = 1;

    loop {
        match data[lower..upper].iter().sum::<usize>().cmp(&target) {
            Ordering::Less => upper += 1,
            Ordering::Equal => break,
            Ordering::Greater => lower += 1,
        }
    }

    let (min, max) = data[lower..upper]
        .iter()
        .fold((usize::MAX, 0), |(min, max), &v| (v.min(min), v.max(max)));

    min + max
}
pub fn day09() {
    let input = fs::read_to_string("./in/day09").expect("File not found!");
    let data = parse(&input);

    let invalid_number = Day09::part1(&data);
    println!("p1: {}", invalid_number);
    println!("p2: {}", Day09::part2(&data, invalid_number));
}
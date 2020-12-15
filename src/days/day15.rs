use std::io::{BufReader, BufRead};
use std::fs::File;
const ITERS: usize = 30_000_000;


pub fn day15() {
    let f = BufReader::new(File::open("in/day15").unwrap());
    let (mut mem, last_step) = get_initial_numbers(f);
    let ini = last_step + 2;

    let p1 = get_val(&mut mem, ini, 2020, 0);
    let p2 = get_val(&mut mem, 2021, ITERS, p1);
    
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

///////////////////////////////////////////////////////////////////////////////

fn get_val(mem: &mut Vec<usize>, initial_turn: usize, final_turn: usize, initial_val: usize) -> usize {
    let mut last_num = initial_val;

    for i in initial_turn..=final_turn {
        let last_seen = mem[last_num];
        mem[last_num] = i-1;
        last_num = match last_seen {
            0 => 0,
            x => i - x - 1,
        };
    }

    return last_num;
}

fn get_initial_numbers(mut f: BufReader<File>) -> (Vec<usize>, usize) {
    let mut s = String::new();
    let _ = f.read_line(&mut s);
    let split: Vec<&str> = s.split(',').collect();

    let mut res = vec![0; ITERS];

    split.iter()
         .enumerate()
         .for_each(|(i, spl)| {
             let val = spl.parse::<usize>().unwrap();
             res[val] = i + 1; 
         });

    return (res, split.len());
}                                                   
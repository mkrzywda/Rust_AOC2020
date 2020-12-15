use std::io::{BufReader, BufRead, Error};
use std::fs::File;use std::str::Chars;
use std::collections::HashMap;

enum Instruction {
    Mask {ones: u64, zeros: u64, floating: u64},
    Mem {addr: u64, val: u64},
}

use Instruction::*;

pub fn day14() {
    let f = BufReader::new(File::open("in/day14").unwrap());
    let instructions = process_input(f);

    let (p1, p2) = get_sols(&instructions);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);}

fn get_sols(ls: &Vec<Instruction>) -> (u64, u64) {
    let mut memory1 = HashMap::new();
    let mut memory2 = HashMap::new();

    let mut mask_ones = 0;
    let mut mask_zeros = 0;
    let mut mask_floating = 0;

    for instr in ls {
        match instr {
            Mask{ones, zeros, floating} => {
                mask_ones = *ones;
                mask_zeros = *zeros;
                mask_floating = *floating;
            },
            Mem{addr, val} => {
                let val_masked = *val & mask_zeros | mask_ones;
                let addr_masked = *addr | mask_ones;
                memory1.insert(*addr, val_masked);

                for addr_alt in get_alt_addrs(addr_masked, mask_floating) {
                    memory2.insert(addr_alt, *val);
                }
            }
        }
    }

    return (memory1.values().sum(), memory2.values().sum());
}

fn get_alt_addrs(addr: u64, floating: u64) -> Vec<u64> {
    let mut res = Vec::new();

    for i in 0..(2 as u64).pow(floating.count_ones()) {
        let mut k = i;
        let mut mask_ones = 0;
        let mut mask_zeros = u64::MAX;
        let mut float_bits = floating;
        let mut pos = 0;

        while float_bits != 0 {
            if float_bits & 1 != 0 {
                if k & 1 == 1 {
                    mask_ones |= 1 << pos;
                } else {
                    mask_zeros &= !(1 << pos);
                }

                k >>= 1;
            }
            
            float_bits >>= 1;
            pos += 1;
        }

        res.push(addr & mask_zeros | mask_ones);
    }

    return res;
}

fn process_input(f: BufReader<File>) -> Vec<Instruction> {
    f.lines()
     .map(line_to_instr)
     .collect()
}

fn line_to_instr(line: Result<String, Error>) -> Instruction {
    let line = line.unwrap();
    let mut chars = line.chars();

    chars.next();
    let ch = chars.next().unwrap();

    if ch == 'a' {
        for _ in 0..5 { chars.next(); }
        return get_mask(&mut chars);
    } else {
        for _ in 0..2 { chars.next(); }
        return get_mem(&mut chars);

    }
}

fn get_mem(chars: &mut Chars) -> Instruction {
    let mut addr: u64 = 0;
    let mut val: u64 = 0;
    let mut ch;

    while {ch = chars.next().unwrap(); ch != ']'} {
        let x = ch.to_digit(10).unwrap() as u64;
        addr = addr * 10 + x;
    }

    // Skip 3
    for _ in 0..3 { chars.next(); }

    while let Some(ch) = chars.next() {
        let x = ch.to_digit(10).unwrap() as u64;
        val = val * 10 + x;
    }

    return Mem{addr, val};
}

fn get_mask(chars: &mut Chars) -> Instruction {
    let mut ones: u64 = 0;
    let mut zeros: u64 = u64::MAX;
    let mut floating: u64 = 0;
    let mut pos = 36;

    while let Some(ch) = chars.next() {
        pos -= 1;
        match ch {
            'X' => floating |= 1 << pos,
            '1' => ones |= 1 << pos,
            '0' => zeros &= !(1 << pos),
             _  => panic!("wtf"),
        }
    }

    return Mask{ones, zeros, floating};
}

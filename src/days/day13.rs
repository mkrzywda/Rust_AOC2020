use std::io::{BufReader, BufRead};
use std::fs::File;
use itertools::izip;
use modinverse::modinverse;

pub fn day13() {
    let time = Instant::now();

    let f = BufReader::new(File::open("in/day13").unwrap());
    let (tstamp, buses, indices) = process_input(f);

    let mut min_wait = (i64::MAX, 0);

    for bus in buses.iter() {
        let wait = bus - (tstamp % bus);
        if wait < min_wait.0 {
            min_wait = (wait, *bus);
        }
    }

    let rems: Vec<i64> = izip!(&indices, &buses)
                              .map(|(i, bus)| (bus - i).rem_euclid(*bus))
                              .collect();
    
    let p1 = min_wait.0 * min_wait.1;
    let p2 = chinese_rem_theorem(&rems, &buses);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}


fn chinese_rem_theorem(rems: &Vec<i64>, mods: &Vec<i64>) -> i64 {
    let prod: i64 = mods.iter().product();
    let prods_i: Vec<i64> = mods.iter().map(|x| prod / x).collect();
    let invs_i: Vec<i64> = izip!(&prods_i, mods).map(|(ni, m)| modinverse(*ni, *m).unwrap()).collect();
    let sum: i64 = izip!(rems, &prods_i, &invs_i).map(|(r, n, x)| r * n * x).sum();
    return sum % prod;
}

fn process_input(f: BufReader<File>) -> (i64, Vec<i64>, Vec<i64>) {
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let tstamp = lines[0].parse().unwrap();
    let spl: Vec<&str> = lines[1].split(",").collect(); 
    let n_elems = spl.len();

    let mut buses = Vec::with_capacity(n_elems);
    let mut indices = Vec::with_capacity(n_elems);

    for (i, elem) in spl.iter().enumerate() {
        if *elem != "x" {
            indices.push(i as i64);
            buses.push(elem.parse().unwrap());
        }
    }

    return (tstamp, buses, indices);
}
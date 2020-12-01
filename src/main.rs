use std::io::BufRead;
mod day01;
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
        _ => println!("Ain't special"),
    }

}


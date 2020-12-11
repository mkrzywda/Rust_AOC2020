use std::{cmp::max, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

pub fn day11_part_b() -> Result<(), Box<dyn Error>> {
    let file = File::open("in/day11")?;
    let mut characters: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut result = characters.clone();

    loop {
        let mut changed_seat = false;

        for i in 0..characters.len() {
            for j in 0..characters[i].len() {
                let character = characters[i][j];

                if character == 'L' {
                    if get_num_occupied_seats(i, j, &characters) == 0 {
                        result[i][j] = '#';
                        changed_seat = true;
                    } else {
                        result[i][j] = 'L';
                    }
                } else if character == '#' {
                    if get_num_occupied_seats(i, j, &characters) >= 5 {
                        result[i][j] = 'L';
                        changed_seat = true;
                    } else {
                        result[i][j] = '#';
                    }
                } else {
                    result[i][j] = '.';
                }
            }
        }
        characters = result.clone();
        if !changed_seat {
            break;
        }
    }

    let num_occupied_seats = characters
        .iter()
        .flatten()
        .fold(0, |accumulator, character| {
            if *character == '#' {
                accumulator + 1
            } else {
                accumulator
            }
        });

    println!("{}", num_occupied_seats);

    Ok(())
}

fn get_num_occupied_seats(i: usize, j: usize, characters: &[Vec<char>]) -> i32 {
    let mut num_occupied_seats = 0;

    let directions = vec![
        (-1, 0),  // Top
        (1, 0),   // Bottom
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Top left
        (-1, 1),  // Top right
        (1, -1),  // Bottom left
        (1, 1),   // Bottom right
    ];

    let max = max(characters.len(), characters[0].len());

    for (k, l) in directions {
        for factor in 1..max as isize {
            let row_index = i as isize + k * factor;
            let column_index = j as isize + l * factor;

            if row_index < 0
                || row_index >= characters.len() as isize
                || column_index < 0
                || column_index >= characters[row_index as usize].len() as isize
            {
                break;
            }

            if characters[row_index as usize][column_index as usize] == '#' {
                num_occupied_seats += 1;
                break;
            } else if characters[row_index as usize][column_index as usize] == 'L' {
                break;
            }
        }
    }

    num_occupied_seats
}
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::result::Result;

fn read_input<R: Read>(f: R) -> Vec<String> {
    let file_reader = BufReader::new(f);
    file_reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> Result<(), Error>  {
    let file = File::open("src/day3/input.txt")?;
    let input = read_input(file);
    let mut first_row = true;
    let mut current_index :usize = 0;
    let mut number_of_trees = 0;
    for s in input {
        let chars : Vec<_> = s.chars().collect();
        if first_row {
            current_index +=  3;
            first_row = false;
            continue;
        } else {
            if current_index > chars.len() - 1 {
                current_index = current_index % chars.len();
            }
            if chars[current_index] == '#' {
                number_of_trees += 1
            }
            current_index +=  3;
        }

    }
    println!("{:?} trees", number_of_trees);
    Ok(())
}
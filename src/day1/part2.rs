use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::result::Result;

fn read_input<R: Read>(f: R) -> Vec<isize> {
    let file_reader = BufReader::new(f);
    let mut vector: Vec<isize> = vec![];

    for line in file_reader.lines() {
        match line {
            Err(err)   => panic!("{:?}", err),
            Ok(string) => match string.trim().parse::<isize>() {
                Err(_err)   => panic!("Not a number!"),
                Ok(number) => vector.push(number)
            }
        }
    }
    vector
}

fn main() -> Result<(), Error>  {
    let file = File::open("src/day1/input.txt")?;
    let mut numbers = read_input(file);
    numbers.sort();
    println!("{:?}", numbers);
    for (i, first_item) in numbers.iter().enumerate() {
        let remaining_sum = 2020 - first_item;
        let mut is_triplet_found = false;
        for (j, second_item) in numbers.iter().enumerate() {
            let number_to_search = remaining_sum - second_item;
            let index = numbers.binary_search(&number_to_search);
            if index.is_ok() && index.unwrap() != i && index.unwrap() != j {
                is_triplet_found = true;
                println!("{:?} {:?} {:?}", first_item, second_item, number_to_search);
                println!("Product of the 3 numbers is {:?}", first_item * second_item * number_to_search);
                break;
            }
        }
        if is_triplet_found {
            break;
        }
    }
    Ok(())
}


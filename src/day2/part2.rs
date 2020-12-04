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
    let file = File::open("src/day2/input.txt")?;
    let input = read_input(file);
    let mut valid_passwords_count = 0;
    for s in input {
        let mut components = s.split_ascii_whitespace();
        let input = (components.next(), components.next(), components.next());
        let mut parsed_range = "";
        let mut parsed_letter = "";
        let mut parsed_password = "";
        match input {
            (Some(range_to_parse), Some(letter_to_parse), Some(password)) => {
                parsed_range = range_to_parse;
                parsed_letter = letter_to_parse;
                parsed_password = password
            },
            _ => println!("err"),
        }
        let range_vec: Vec<&str> = parsed_range.split("-").collect();
        let letter_position_1 = range_vec[0].parse::<usize>().unwrap();
        let letter_position_2 = range_vec[1].parse::<usize>().unwrap();
        let letter_vec: Vec<&str> = parsed_letter.split(":").collect();
        let letter_to_search = letter_vec[0].parse::<char>().unwrap();
        let letters_at_policy_positions = (parsed_password.chars().nth(letter_position_1 - 1), parsed_password.chars().nth(letter_position_2 - 1));
        match letters_at_policy_positions {
            (Some(position_1_letter), Some(position_2_letter)) => {
                if position_1_letter != position_2_letter && (position_1_letter == letter_to_search || position_2_letter == letter_to_search) {
                    valid_passwords_count = valid_passwords_count + 1;
                }
            }
            _ => println!("err"),
        }
    }
    println!("{:?}", valid_passwords_count);
    Ok(())

}
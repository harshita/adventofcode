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
        let min_appearance_count = range_vec[0].parse::<usize>().unwrap();
        let max_appearance_count = range_vec[1].parse::<usize>().unwrap();
        let letter_vec: Vec<&str> = parsed_letter.split(":").collect();
        let letter_to_search = letter_vec[0];
        let count = parsed_password.matches(letter_to_search).count();

        if count >= min_appearance_count && count <= max_appearance_count {
            valid_passwords_count = valid_passwords_count + 1;
        }
    }
    println!("{:?}", valid_passwords_count);
    Ok(())

}
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
    let input_arr_tuple = [(1, 1), (3, 1), (5,1), (7,1), (1,2)];
    let mut product_of_tree_count :i64 = 1;
    for input_tuple in input_arr_tuple.iter()
    {
        let num_of_trees = get_number_of_trees_on_slope(&input, input_tuple.0, input_tuple.1);
        product_of_tree_count *= num_of_trees as i64;
    }
    println!("Product of tree count is {:?}", product_of_tree_count);
    Ok(())
}

fn get_number_of_trees_on_slope(input:&Vec<String>, right: usize, down: usize)  -> i32 {
    let mut first_row = true;
    let mut current_index :usize = 0;
    let mut number_of_trees = 0;
    let mut current_down = 1;
    for s in input {
        let chars : Vec<_> = s.chars().collect();
        if first_row {
            current_index +=  right;
            first_row = false;
            continue;
        } else {
            if current_down == down {
                if current_index > chars.len() - 1 {
                    current_index = current_index % chars.len();
                }
                if chars[current_index] == '#' {
                    number_of_trees += 1
                }
                current_index += right;
                current_down = 1;
            } else {
                current_down += 1;
                continue;
            }
        }

    }
    println!("{:?} trees for right {:?} and down {:?}", number_of_trees, right, down);
    return number_of_trees;
}
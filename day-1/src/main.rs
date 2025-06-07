use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let (mut col1, mut col2) = return_vecs("input.txt").unwrap();
    col1.sort();
    col2.sort();

    let mut final_vec: Vec<i32> = Vec::new();

    let mut final_output: i32 = 0;

    for i in 0..col1.len() {
        final_vec.push(i32::abs(col1[i] - col2[i]));
    }

    for i in final_vec.iter() {
        final_output += i;
    }

    println!("Part 1: {}", final_output);
    println!("Part 2: {}", part_two(col1, col2));
}

fn return_vecs(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path = Path::new(path);
    let file = File::open(&path)?;

    let reader = BufReader::new(file);

    let mut column1_data: Vec<i32> = Vec::new();
    let mut column2_data: Vec<i32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            match parts[0].parse::<i32>() {
                Ok(num1) => column1_data.push(num1),
                Err(e) => {
                    eprintln!(
                        "Warning: could not parse the first number '{}' on line '{}': {}",
                        parts[0], line, e
                    );
                    continue;
                }
            }
            match parts[1].parse::<i32>() {
                Ok(num2) => column2_data.push(num2),
                Err(e) => {
                    eprintln!(
                        "Warning: could not parse the second number '{}' on line '{}': {}",
                        parts[1], line, e
                    );
                    continue;
                }
            }
        }
    }

    println!("Successfully read {} pairs of numbers.", column1_data.len());

    Ok((column1_data, column2_data))
}

fn part_two(col1: Vec<i32>, col2: Vec<i32>) -> i32 {
    let mut final_vec: Vec<i32> = Vec::new();

    let mut init: i32 = 0;

    for i in col1.iter() {
        let mut x = 0;

        for j in col2.iter() {
            if i == j {
                x += 1;
            }
        }
        final_vec.push(i * x);
    }

    for i in final_vec.iter() {
        init += i;
    }

    init
}

// Entire program is written inside just one file just for ease of reviewing. Or.. I hope so ;)

use std::env;
use std::fs::read_to_string;

fn main() {
    let filename = read_file_name();
    let data = read_file(&filename);

    if data.is_empty() {
        panic!("File is empty");
    }

    let statistics = statistics(&data);

    println!("Min: {}", statistics.min);
    println!("Max: {}", statistics.max);
    println!("Average: {}", statistics.avg);
    println!(
        "Increasing sequence: {:?}",
        &data[statistics.increasing_sequence.0..=statistics.increasing_sequence.1]
    );
    println!(
        "Decreasing sequence: {:?}",
        &data[statistics.decreasing_sequence.0..=statistics.decreasing_sequence.1]
    );

    // You can call the median method somewhere here without having to clone the data vector inside the statistics function
    // P.S. The median function takes ownership of the passed vector
    println!("Median: {}", statistics.median);
}

struct Statistics {
    min: i32,
    max: i32,
    avg: f64,
    increasing_sequence: (usize, usize),
    decreasing_sequence: (usize, usize),
    median: f64,
}

fn statistics(data: &[i32]) -> Statistics {
    let mut s = Statistics {
        min: data[0],
        max: data[0],
        avg: 0.0,
        increasing_sequence: (0, 0),
        decreasing_sequence: (0, 0),
        median: 0.0,
    };

    let mut current_increasing_sequence: (usize, usize) = (0, 0);
    let mut current_decreasing_sequence: (usize, usize) = (0, 0);

    for (i, x) in data.iter().enumerate() {
        if *x < s.min {
            s.min = *x;
        }
        if *x > s.max {
            s.max = *x;
        }
        s.avg += *x as f64 / data.len() as f64;

        if x > &data[current_increasing_sequence.1] {
            current_increasing_sequence.1 = i;
        } else {
            current_increasing_sequence = (i, i);
        }

        if is_sequence_shorter(s.increasing_sequence, current_increasing_sequence) {
            s.increasing_sequence
                .clone_from(&current_increasing_sequence);
        }

        if x < &data[current_decreasing_sequence.1] {
            current_decreasing_sequence.1 = i;
        } else {
            current_decreasing_sequence = (i, i);
        }

        if is_sequence_shorter(s.decreasing_sequence, current_decreasing_sequence) {
            s.decreasing_sequence
                .clone_from(&current_decreasing_sequence);
        }
    }

    s.median = median(data.to_vec());

    s
}

fn is_sequence_shorter(a: (usize, usize), b: (usize, usize)) -> bool {
    a.1 - a.0 < b.1 - b.0
}

fn median(mut data: Vec<i32>) -> f64 {
    data.sort();

    if data.len() % 2 == 0 {
        (data[data.len() / 2] as f64 + data[data.len() / 2 - 1] as f64) / 2.0
    } else {
        data[data.len() / 2] as f64
    }
}

fn read_file_name() -> String {
    env::args().nth(1).expect("Please provide a file path")
}

fn read_file(filename: &str) -> Vec<i32> {
    read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .map(|line| line.parse::<i32>().expect("Error parsing line: {line}"))
        .collect()
}

use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let file_paths = ["Data/data1.txt","Data/data2.txt","Data/data3.txt","Data/data4.txt","Data/data5.txt","Data/data6.txt","Data/data7.txt","Data/dataWhale8.txt","Data/data9.txt","Data/data10.txt"];

    println!("Sequential Reading");
    match sequential_reading(&file_paths) {
        Ok(word_count) => println!("Unique words (sequential): {:?}", word_count.len()),
        Err(e) => eprintln!("Error in sequential reading: {:?}", e),
    };

    println!("\n\nParallel Reading");
    match parallel_reading(&file_paths) {
        Ok(word_count) => println!("Unique words (parallel): {:?}", word_count.len()),
        Err(e) => eprintln!("Error in parallel reading: {:?}", e),
    };

    println!("\n\nPipelined Reading");
    match pipelined_reading(&file_paths) {
        Ok(word_count) => println!("Unique words (pipelined): {:?}", word_count.len()),
        Err(e) => eprintln!("Error in pipelined reading: {:?}", e),
    };
}

fn sequential_reading(file_path:&[&str]) -> io::Result<HashMap<String, i32>> {
    let start = Instant::now();
    let mut word_count:HashMap<String,i32> = HashMap::new();
    for (i,_value) in file_path.into_iter().enumerate(){
        let file = File::open(file_path[i])?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?; // Handle possible error when reading a line
            let line_words:Vec<&str> = line.split_ascii_whitespace().collect();
            for word in line_words {
                *word_count.entry(word.to_string()).or_insert(0) += 1;
            }
        }
    }
    println!("###################################################################################");
    println!("Duration: {:?}",start.elapsed());
    Ok(word_count)
}

fn parallel_reading(file_path:&[&str]) -> io::Result<HashMap<String, i32>> {
    let mut global_word_count: HashMap<String, i32>  = HashMap::new();        
    let start = Instant::now();
    let word_counts: Result<Vec<_>, _> = file_path.par_iter().map(|&file_name| {
        count_words(file_name)
    }).collect();

    let word_counts = word_counts?;

    for word_count in word_counts {
        for (word, count) in word_count {
            *global_word_count.entry(word).or_insert(0) += count;
        }
    }
    println!("###################################################################################");
    println!("Duration: {:?}",start.elapsed());
    Ok(global_word_count)
}
fn count_words(file_path: &str) -> io::Result<HashMap<String, i32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    Ok(word_count)
}
fn pipelined_reading(file_path:&[&str]) -> io::Result<HashMap<String, i32>> {
    let global_word_count: HashMap<String, i32>  = HashMap::new();
    let start = Instant::now();
    file_path.par_iter().for_each(|file_name| {
        
        let file = File::open(file_name).unwrap();
        let _reader = BufReader::new(file);
    });
    println!("###################################################################################");
    println!("Duration:{:?}",start.elapsed());
    Ok(global_word_count)
}


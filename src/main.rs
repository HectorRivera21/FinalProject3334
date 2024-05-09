use rayon::prelude::*;
use std::fs::File;
#[allow(unused_imports)]use std::env;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
#[allow(unused_imports)]use std::sync::{Arc, Mutex,MutexGuard};
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
    for (_i, file_name) in file_path.into_iter().enumerate(){

        let file = File::open(file_name).expect("counld not open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {

            let line = line.expect("could not read line"); // Handle possible error when reading a line
            for word in line.split_ascii_whitespace() {

                *word_count.entry(word.to_string().to_lowercase()).or_insert(0) += 1;

            }
        }
    }
    let duration = start.elapsed();
    println!("###################################################################################");
    println!("Duration: {:?}",duration);
    Ok(word_count)
}
fn parallel_reading(file_paths: &[&str]) -> io::Result<HashMap<String, i32>> {
    let start = Instant::now();
    let mut global_word_count: HashMap<String, i32> = HashMap::new();
    println!("Parallel processing duration: {:?}", start.elapsed());
    println!("###################################################################################");
    Ok(global_word_count)
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
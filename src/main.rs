use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // let file_path = ["Data/data7.txt"];
    let file_path = ["Data/data1.txt","Data/data2.txt","Data/data3.txt","Data/data4.txt","Data/data5.txt","Data/data6.txt","Data/data7.txt","Data/dataWhale8.txt","Data/data9.txt","Data/data10.txt"];
    println!("Sequencial Reading");
    sequencial_reading(&file_path).unwrap();

    println!("Parallel Reading");
    parallel_reading(&file_path).unwrap();
    
    println!("piplined Reading");
    pipelined_reading(&file_path).unwrap();
}

fn sequencial_reading(file_path:&[&str]) -> io::Result<()> {
    for (i,value) in file_path.into_iter().enumerate(){
        let mut word_count:HashMap<String,i32> = HashMap::new();
        println!("###################################################################################");
        println!("In file {}", file_path[i]);
        let start = Instant::now();

        let file = File::open(file_path[i])?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?; // Handle possible error when reading a line
            let line_words:Vec<&str> = line.split_ascii_whitespace().collect();
            for word in line_words {
                *word_count.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        let duration = start.elapsed();
        println!("TextFile:{} in {:?}",value,duration);
        println!("Word count: {:?}", word_count.len());
    }
    Ok(())
}

fn parallel_reading(file_path:&[&str]) -> io::Result<()> {
    Ok(())
}

fn pipelined_reading(file_path:&[&str]) -> io::Result<()> {
    Ok(())
}
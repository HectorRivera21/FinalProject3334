use std::fs::File;
#[allow(unused_imports)]use std::env;
use std::io::{self, BufRead, BufReader};
use std::sync::{mpsc, Arc, Mutex};
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
    let mut vec_file:Vec<String> = Vec::new();
    let file_paths = ["Data/data1.txt","Data/data2.txt","Data/data3.txt","Data/data4.txt","Data/data5.txt","Data/data6.txt","Data/data7.txt","Data/dataWhale8.txt","Data/data9.txt","Data/data10.txt"];
    for file in file_paths {
        vec_file.push(file.to_string());
    }

    println!("\n\nPipelined Reading");
    match pipelined_reading(vec_file) {
        Ok(word_count) => println!("Unique words (pipelined): {:?}", word_count.lock().unwrap().len()),
        Err(e) => eprintln!("Error in pipelined reading: {:?}", e),
    };
}

fn sequential_reading(file_path:&[&str]) -> io::Result<HashMap<String, u32>> {
    // Start the timer
    let start = Instant::now();
    
    // Initialize the word count HashMap
    let mut word_count: HashMap<String, u32> = HashMap::new();
    
    // Iterate over each file path
    for (_i, file_name) in file_path.into_iter().enumerate(){
        
        // Open the file
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        
        // Read line by line
        for line in reader.lines() {
            
            // Handle possible error when reading a line
            let line = line?;
            
            // Split the line into words and update the word count
            for word in line.split_ascii_whitespace() {
                *word_count.entry(word.to_string().to_lowercase()).or_insert(0) += 1;
            }
        }
    }
    
    // Calculate the elapsed time
    let duration = start.elapsed();
    
    // Print the duration
    println!("###################################################################################");
    println!("Duration: {:?}",duration);
    
    // Return the word count HashMap
    Ok(word_count)
}

fn parallel_reading(file_paths: &[&str]) -> io::Result<HashMap<String, u32>> {
    // Import rayon for parallel processing
    use rayon::prelude::*;

    // Start the timer
    let start = Instant::now();

    // Initialize the global word count HashMap
    let mut global_word_count: HashMap<String, u32> = HashMap::new();

    // Process each file in parallel and collect the word counts
    let word_count: Vec<_> = file_paths.par_iter()
        .map(|filename| count_words_in_file(filename))
        .collect();

    // Aggregate word counts from each file into the global word count HashMap
    for wc in word_count {
        for (word, count) in wc? {
            *global_word_count.entry(word).or_insert(0) += count;
        }
    }

    // Print the duration of parallel processing
    println!("Parallel processing duration: {:?}", start.elapsed());
    println!("###################################################################################");

    // Return the global word count HashMap
    Ok(global_word_count)
}

fn count_words_in_file(file_path: &str) -> io::Result<HashMap<String, u32>> {
    let mut word_count = HashMap::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Read lines and count words
    for line in reader.lines() {
        let line = line?;
        let words = line.split_whitespace();

        for word in words {
            *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }

    Ok(word_count)
}

fn pipelined_reading(file_path: Vec<String>) -> io::Result<Mutex<HashMap<String, u32>>> {
    use std::thread;

    // Initialize a shared global word count using Arc and Mutex
    let global_word_count: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let start = Instant::now();

    // Create channels for communication between threads
    let (file_send, word_counter_receiver) = mpsc::channel();
    let (word_counter_sender, receiver_combine) = mpsc::channel::<HashMap<String, u32>>();

    // Spawn a thread for reading files
    let reader_thread = thread::spawn(move || {
        reader_actor(file_path, file_send);
    });

    // Spawn a thread for counting words
    let word_counter_thread = thread::spawn({
        let word_counter_sender = word_counter_sender.clone();
        move || counter_actor(word_counter_receiver, word_counter_sender)
    });

    // Spawn a thread for combining word counts
    let combine_thread = thread::spawn({
        let final_result = Arc::clone(&global_word_count);
        move || combine_actor(receiver_combine, final_result)
    });

    // Wait for all threads to finish
    reader_thread.join().unwrap();
    word_counter_thread.join().unwrap();
    combine_thread.join().unwrap();

    // Print duration and return the result
    println!("###################################################################################");
    println!("Duration: {:?}", start.elapsed());
    println!("Duration:{:?}",start.elapsed());
    let result = Arc::try_unwrap(global_word_count).unwrap();
    Ok(result)
}
fn reader_actor(file_paths: Vec<String>, sender: mpsc::Sender<(String, String)>) {
// Iterate over each file path
    for file_path in file_paths {
        let file: File = File::open(&file_path).unwrap(); // Open the file for reading
        let reader = io::BufReader::new(file); // Create a buffered reader
        let mut content = String::new(); // Initialize a string to store file content

        // Read each line in the file
        for line in reader.lines() {
            content.push_str(&line.unwrap()); // Append the line to the content string
            content.push('\n'); // Add a newline after each line
        }

        sender.send((file_path.to_string(), content)).unwrap(); // Send the content to the next stage
    }

    // Signal that reading is complete
    sender.send(("done".to_string(), "".to_string())).unwrap();
}
fn counter_actor(receiver: mpsc::Receiver<(String, String)>, sender: mpsc::Sender<HashMap<String, u32>>) {
// Receive file_name and content from the receiver channel
    while let Ok((file_name, content)) = receiver.recv() {
        // Break the loop if the file_name is "done"
        if file_name == "done".to_string() {
            break;
        }
        // Initialize a HashMap to store word counts
        let mut word_count: HashMap<String, u32> = HashMap::new();
        // Split the content into individual words
        let words = content.split_whitespace();
        
        // Count the frequency of each word
        for word in words {
            *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        // Send the word count HashMap to the sender channel
        sender.send(word_count).unwrap();
    }

}

fn combine_actor ( receiver: mpsc::Receiver<HashMap<String, u32>>, final_result: Arc<Mutex<HashMap<String, u32>>>) {
    loop {
        let word_count = receiver.recv().unwrap(); // Receive word count HashMap
        let mut total_word = final_result.lock().unwrap(); // Lock the final combined word count HashMap
        if word_count.is_empty() {
            break;
        }
        for (word, count) in word_count {
            *total_word.entry(word).or_insert(0) += count; // Combine the word counts
        }
    }
}

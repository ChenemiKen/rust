// Read text from a file.
// Count words, lines, characters.
// Identify most frequent word.
// Display stats to the user.

use std::fs;
use std::collections::HashMap;

fn main() {
    println!("A4. Text Analyzer CLI");
    
    let poem = fs::read_to_string("poem.txt")
        .expect("should have read the file");

    // println!("With text: \n{poem}");

    println!("Word count: {}", countWords(&poem));
    println!("Line count: {}", countLines(&poem));
    println!("Character count: {}", (poem.len()));
    println!("Most Frequent word: {}", mostFrequentWord(&poem));
}

fn countWords(s: &str) -> u32{
    let bytes = s.trim().as_bytes();
    let mut count = 0;
    for &item in bytes.iter(){
        if item == b' ' || item == b'\n' {
            count+=1;
        }
    }

    count
}

fn countLines(s: &str) -> u32{
    let bytes = s.trim().as_bytes();
    let mut count = 0;
    for &item in bytes.iter(){
        if item == b'\n' {
            count+=1;
        }
    }

    count
}

fn mostFrequentWord(s: &str) -> &str{
    let words = s.split(' ');
    let mut map = HashMap::new();
    for &word in words {
        *map.entry(word).or_insert(0) += 1
    }

    let mut max = 0;
    let mut word = "";
    for (key, val) in map.iter() {
        if val > max {
            max = val;
            word = key;
        }
    }
    
    word
}

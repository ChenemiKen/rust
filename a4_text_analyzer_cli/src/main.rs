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

    println!("Word count: {}", count_words(&poem));
    println!("Line count: {}", count_lines(&poem));
    println!("Character count: {}", (poem.len()));
    println!("Most Frequent word: {}", most_frequent_word(&poem));
}

fn count_words(s: &str) -> u32{
    let bytes = s.trim().as_bytes();
    let mut count = 0;
    for &item in bytes.iter(){
        if item == b' ' || item == b'\n' {
            count+=1;
        }
    }

    count
}

fn count_lines(s: &str) -> u32{
    let bytes = s.trim().as_bytes();
    let mut count = 0;
    for &item in bytes.iter(){
        if item == b'\n' {
            count+=1;
        }
    }

    count
}

fn most_frequent_word(s: &str) -> &str{
    let words = s.split_whitespace();
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word).or_insert(0) += 1
    }

    println!("{:?}", map);

    let mut max = 0;
    let mut word = "";
    for (key, val) in map.iter() {
        if *val > max {
            max = *val;
            word = key;
        }
    }
    
    word
}

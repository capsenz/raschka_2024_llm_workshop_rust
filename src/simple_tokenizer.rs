use std::fs::File;
// use std::io::prelude::*;
use std::io::Read;

use tokenizer::Tokenizer;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("data/the-verdict.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut tokenizer = Tokenizer::new();
    tokenizer.load_vocab(&contents);
    let text = r#""It's the last he painted, you know," 
Mrs. Gisburn said with pardonable pride."#;
    let encoded_text = tokenizer.encode(&text);
    for id in &encoded_text {
        println!("{}", id);
    }
    let decoded_text = tokenizer.decode(&encoded_text);
    println!("Decoded text: {}", decoded_text);
    Ok(())
}
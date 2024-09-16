use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use std::fs::File;
// use std::io::prelude::*;
use std::io::Read;

pub struct Tokenizer {
    vocab: HashMap<String, usize>,
    id_to_token: HashMap<usize, String> 
}

impl Tokenizer {
    pub fn new () -> Self {
        Tokenizer {
            vocab: HashMap::new(),
            id_to_token: HashMap::new()
        }
    }

    pub fn load_vocab (&mut self, text: &str) {
        let re = Regex::new(r#"([,.:;?!"()']|--|\s)"#).unwrap();
        let preprocessed: Vec<&str> = re.split(text).collect();
        let mut all_words:Vec<String> = preprocessed
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        all_words.sort();

        for (i, item) in all_words.iter().enumerate() {
            self.vocab.insert(item.clone(), i);
            self.id_to_token.insert(i, item.clone());
        }
    }
    
    pub fn encode (&self, text: &str) -> Vec<usize>{
        let re = Regex::new(r#"([,.:;?!"()']|--|\s)"#).unwrap();
        let preprocessed: Vec<&str> = re.split(text).collect();
        let mut ids: Vec<usize> = Vec::new();
        for &word in &preprocessed {
            let search_word = word.trim();
            if let Some(&id) = self.vocab.get(search_word) {
                ids.push(id);
            }
        }
        ids
    }
    // this does not work correctly yet as the punctuation is missing. i did not have the motivation yet to figure out why.
    pub fn decode (self, ids: &[usize]) -> String {
        let mut words: Vec<String> = Vec::new();
        for &id in ids {
            if let Some(word) = self.id_to_token.get(&id) {
                words.push(word.clone());
            }
        }
        let text = words.join(" ");
        let re = Regex::new(r#"\s+([,.?!\"()\'])"#).unwrap();
        let final_text = re.replace_all(&text, "$1").to_string();
        final_text
    }
}

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
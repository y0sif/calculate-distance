use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::BufReader;
use std::io::{self, Read};
//best score
//average execution time (ms) = 159
//Max execution time (ms) = 446

fn hash_word(word: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    word.hash(&mut hasher);
    hasher.finish()
}

fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let file = File::open(file_path)?;
    let mut buffer = Vec::new();

    let mut reader = BufReader::new(file);
    // Read the file as binary
    reader.read_to_end(&mut buffer)?;

    // Convert the bytes to UTF-8 string
    let doc_string = String::from_utf8_lossy(&buffer).to_lowercase();

    Ok(doc_string)
}

pub fn calculate_distance(document1: &str, document2: &str) -> f64 {
    let mut d0: f64 = 0.0;
    let mut d1: f64 = 0.0;
    let mut d2: f64 = 0.0;

    let doc1string = read_file_to_string(document1).unwrap_or_default();
    let doc2string = read_file_to_string(document2).unwrap_or_default();

    let doc1hashmap = split_string_to_dictionary(&doc1string);
    let doc2hashmap = split_string_to_dictionary(&doc2string);

    let (smaller_map, larger_map) = if doc1hashmap.len() > doc2hashmap.len() {
        (&doc2hashmap, &doc1hashmap)
    } else {
        (&doc1hashmap, &doc2hashmap)
    };

    for (key, value1) in smaller_map {
        if let Some(value2) = larger_map.get(key) {
            d0 += value1 * value2;
        }
    }

    for value in doc1hashmap.values() {
        d1 += value * value;
    }

    for value in doc2hashmap.values() {
        d2 += value * value;
    }

    f64::acos(d0 / f64::sqrt(d1 * d2)) * (180.0 / PI)
}

fn split_string_to_dictionary(doc_string: &str) -> HashMap<u64, f64> {
    let mut document_hashmap: HashMap<u64, f64> = HashMap::with_capacity(doc_string.len() / 2);

    let mut string = String::new();
    for i in doc_string.chars() {
        if i.is_alphanumeric() {
            string.push(i);
        } else if !string.is_empty() {
            let hashed = hash_word(&string);
            *document_hashmap.entry(hashed).or_insert(0.0) += 1.0;
            string.clear();
        }
    }

    if !string.is_empty() {
        let hashed = hash_word(&string);
        *document_hashmap.entry(hashed).or_insert(0.0) += 1.0;
    }

    document_hashmap
}

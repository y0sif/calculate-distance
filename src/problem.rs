use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

//best score:
//average execution time (ms) = 123
//Max execution time (ms) = 292

fn hash_word(word: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    word.hash(&mut hasher);
    hasher.finish()
}

fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    // Read the file as binary
    file.read_to_end(&mut buffer)?;

    // Convert the bytes to UTF-8 string
    let doc_string = String::from_utf8_lossy(&buffer).to_lowercase();

    Ok(doc_string)
}

pub fn calculate_distance(document1: &str, document2: &str) -> f64 {
    // Use Arc to share ownership between threads
    let document1_path = Arc::new(document1.to_string());
    let document2_path = Arc::new(document2.to_string());

    // Create handles for the threads
    let handle1 = thread::spawn({
        let document_path = Arc::clone(&document1_path);
        move || read_file_to_string(&document_path).unwrap_or_default()
    });

    let handle2 = thread::spawn({
        let document_path = Arc::clone(&document2_path);
        move || read_file_to_string(&document_path).unwrap_or_default()
    });

    // Wait for the threads to finish and get the results
    let doc1string = handle1.join().unwrap();
    let doc2string = handle2.join().unwrap();

    let doc1string_join = Arc::new(doc1string);
    let doc2string_join = Arc::new(doc2string);

    let handle1 = thread::spawn({
        let docstring = Arc::clone(&doc1string_join);
        move || split_string_to_dictionary(&docstring)
    });

    let handle2 = thread::spawn({
        let docstring = Arc::clone(&doc2string_join);
        move || split_string_to_dictionary(&docstring)
    });

    let doc1hashmap = handle1.join().unwrap();
    let doc2hashmap = handle2.join().unwrap();

    // The rest of the function remains unchanged
    let (smaller_map, larger_map) = if doc1hashmap.len() > doc2hashmap.len() {
        (&doc2hashmap, &doc1hashmap)
    } else {
        (&doc1hashmap, &doc2hashmap)
    };

    let d0: f64 = smaller_map
        .par_iter()
        .map(|(key, value1)| {
            if let Some(value2) = larger_map.get(key) {
                value1 * value2
            } else {
                0.0
            }
        })
        .sum();

    let d1: f64 = doc1hashmap.par_iter().map(|(_, value)| value * value).sum();
    let d2: f64 = doc2hashmap.par_iter().map(|(_, value)| value * value).sum();

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

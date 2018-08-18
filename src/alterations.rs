/*
    File meant to contain common data pre-processing utilities
*/

use std::vec::Vec;
use std::collections::{HashMap, BTreeSet};
use std::rc::Rc;


pub fn split_n_hot_encode(raw_texts: Vec<String>, sep: String, cutoff: usize) -> (Vec<String>, Vec<Vec<u8>>) {
    /*
    Given an array of strings of size (n_samples,) will return a one-hot encoded matrix for each sample
    indicating if the string/word was present
    */

    // Make mutable reference counter out of mutuble raw_texts
    // this way, we can pass .clone()s to functions without actually making a full copy
    // and pass ownership to parse_into_key_word_counts() without worrying about lifetimes there.
    let raw_texts: Rc<Vec<String>> = Rc::new(raw_texts);

    // Get a hashmap of keyword counts based from raw_texts and the sep value
    let string_counts: HashMap<String, usize> = parse_into_key_word_counts(raw_texts.clone(), sep);

    // Remove keys whose value is below cutoff vvalue, returns immediately if cutoff < 1
    let string_counts: HashMap<String, usize> = prune_keys(string_counts, cutoff);
    let key_words: Vec<String> = string_counts.keys().cloned().collect();

    // Create one-hot matrix
    let matrix: Vec<Vec<u8>> = produce_onehot(&key_words, &raw_texts);

    // Define the array of strings, which match the matrix dim 1
    let array_of_strings: Vec<String> = string_counts.keys().cloned().collect();
    (array_of_strings, matrix)
}

fn parse_into_key_word_counts(raw_texts: Rc<Vec<String>>, sep: String) -> HashMap<String, usize> {
    /*
    Split raw_texts by sep and return a hashmap of counts of the words
    */

    // The mapping of unique strings to how many times they occur
    let mut string_counts: HashMap<String, usize> = HashMap::new();

    // Iterate over raw texts and for each of those raw_text instances
    // split on sep and insert it as a key while updating counter
    for raw_text in raw_texts.iter() {
        for word in raw_text.split(&sep).collect::<Vec<&str>>().iter() {
            let count = string_counts.entry(word.trim().to_string()).or_insert(0);
            *count += 1;
        }
    }

    // Return back the hashmap of word counts
    string_counts
}

fn produce_onehot(key_words: &Vec<String>, raw_texts: &Vec<String>) -> Vec<Vec<u8>> {
    /*
    Given an array of raw texts and an array of words to look for, return one-hot matrix
    indicating if word at each index of key_words occurs in raw_text array

    Parameters
    ----------
    key_words: Array of keywords to concern oneself about in searching for
    raw_text:  Array of raw text strings to search keywords for

    Returns
    -------
    2d array where each occurrence of raw_text has a vector matching key_words length and order
    and consists of binary indicators if the key_word was present in the instance of raw_text
    */

    // Define the main matrix which will contain sub matrices comprised of 0/1 values
    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(raw_texts.len());

    // This portion could be done parallel by doing each raw text by itself and then collecting
    // all resulting one-hot vectors
    for raw_text in raw_texts.iter() {

        // Define new empty submatrix for this row of raw_text
        let mut submatrix: Vec<u8> = Vec::with_capacity(key_words.len());

        // Iterate over the keywords checking each, and adding the 1 or 0
        for key_word in key_words.iter() {
            if raw_text.contains(key_word) {
                submatrix.push(1);
            } else {
                submatrix.push(0);
            }
        }

        // Push the finished submatrix into the final matrix
        matrix.push(submatrix);
    }
    matrix
}

fn prune_keys(mut string_counts: HashMap<String, usize>, cutoff: usize) -> HashMap<String, usize> {
    /*

    Handles the removal of keys from a Hashmap given a cutoff value
    If the key's value is less than that value, the key is removed from the HashMap

    Parameters
    ----------
    string_counts:  HashMap consisting of string keys and a count of that string's occurrences.
    cutoff:         Value which the key's value must be over in order to keep.

    Returns
    -------
    HashMap which has keys removed whose values were below the cutoff
    */

    // If cutoff is 0, then there is no point other than to return original mapping
    if cutoff < 1 {
        return string_counts
    }

    // Define keys to remove set in this scope
    let mut keys_to_remove: BTreeSet<String> = BTreeSet::new();

    // Iterate over strings and their occurrence counts, adding to keys_to_remove as needed
    for string in string_counts.keys() {
        let count = string_counts.get(string);
        if let Some(ct) = count {
            if ct < &cutoff {
                keys_to_remove.insert(string.clone());
            }
        }
    }

    // Remove all keys placed into keys_to_remove
    for key in keys_to_remove.iter() {
        string_counts.remove(key);
    }

    // Return the pruned string counts HashMap
    string_counts
}

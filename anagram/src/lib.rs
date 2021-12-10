use std::collections::{HashMap, HashSet};



pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let lower_word = word.to_lowercase();

    //original word to hash
    let word_hash = to_hash(lower_word.clone());

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for ana in possible_anagrams.iter() {
        
        let lower_ana = ana.clone().to_lowercase();
        if lower_word == lower_ana {
            continue;
        }

        if ana.len() != word.len() {
            continue;
        }

        let a_hash = to_hash(lower_ana);
        let mut possible = true;
        for i in word_hash.iter() {

            let (key, value) = i;
            let entry = a_hash.get(key);
            match entry {
                Some(count) => {
                    if count < value {
                        possible = false;
                    }
                },
                None => possible = false,
            }
        }

        if possible {
            anagrams.insert(ana);
        }
    }

    anagrams
}

pub fn to_hash<'a>(word: String) -> HashMap<char, i32> {

    let char_vec: Vec<char> = word.chars().collect();
    let mut hash: HashMap<char, i32>  = HashMap::new();
    for c in char_vec.iter() {
        let entry = hash.get(c);
        match entry {
            Some(count) => {
                hash.insert(*c, *count + 1);
            }
            None => {
                hash.insert(*c, 1);
            }
        }
    }
    hash
}
// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    let mut mag_map = HashMap::<&&str, u32>::new();
    let mut note_map = HashMap::<&&str, u32>::new();

    for item in magazine.iter() {
        let entry = mag_map.get(item);
        match entry {
            Some(count) => {
                mag_map.insert(item, *count + 1);
            }
            None => {
                mag_map.insert(item, 1);
            }
        }
    }

    for item in note.iter() {
        let entry = note_map.get(item);
        match entry {
            Some(count) => {
                note_map.insert(item, *count + 1);
            }
            None => {
                note_map.insert(item, 1);
            }
        }
    }

    for key_pair in note_map.iter() {

        let (word, needed_count) = key_pair;
        let entry = mag_map.get(word);
        match entry {
            Some(count) => {
                if count < needed_count {
                    return  false;
                }
            }
            None => {
                return false;
            }
        }

    }

    true
}

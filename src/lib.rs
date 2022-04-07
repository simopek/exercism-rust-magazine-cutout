// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();

    for word in magazine {
        magazine_words
            .entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for note_word in note {
        magazine_words
            .entry(note_word)
            .and_modify(|e| *e -= 1)
            .or_insert(-1);
    }

    magazine_words.values().all(|&x| x >= 0)
}

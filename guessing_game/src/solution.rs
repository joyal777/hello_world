use std::time::{SystemTime, UNIX_EPOCH};
use crate::master::Master; // <-- Uses the Master component

pub struct Solution;

impl Solution {
    pub fn find_secret_word(mut words: Vec<String>, master: &Master) {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;

        for attempt in 1..=10 {
            if words.is_empty() { break; }

            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let guess_idx = seed % words.len();
            let guess_word = words[guess_idx].clone();
            
            let matches = master.guess(guess_word.clone());
            println!("Attempt {}: Guessed '{}', Matches = {}", attempt, guess_word, matches);

            if matches == 6 {
                println!("🎉 Success! The secret word was '{}'!", guess_word);
                return;
            }

            words = words
                .into_iter()
                .filter(|w| w != &guess_word && Self::get_matches(w, &guess_word) == matches)
                .collect();
        }
    }

    fn get_matches(w1: &str, w2: &str) -> i32 {
        w1.bytes().zip(w2.bytes()).filter(|(b1, b2)| b1 == b2).count() as i32
    }
}
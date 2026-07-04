use std::time::{SystemTime, UNIX_EPOCH};

// =====================================================================
// 1. THE MASTER INTERFACE (Simulating LeetCode's backend)
// =====================================================================
struct Master {
    secret: String,
}

impl Master {
    // This simulates the Master API. It compares your guess to the hidden secret.
    fn guess(&self, word: String) -> i32 {
        word.bytes()
            .zip(self.secret.bytes())
            .filter(|(b1, b2)| b1 == b2)
            .count() as i32
    }
}

// =====================================================================
// 2. YOUR LEETCODE SOLUTION
// =====================================================================
struct Solution;

impl Solution {
    pub fn find_secret_word(mut words: Vec<String>, master: &Master) {
        // Pseudo-random number generator helper to avoid deterministic traps
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;

        for attempt in 1..=10 {
            if words.is_empty() {
                break;
            }

            // Pick a random index from the remaining candidate words
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let guess_idx = seed % words.len();
            let guess_word = words[guess_idx].clone();
            
            // Get the number of matches from the Master API
            let matches = master.guess(guess_word.clone());
            println!("Attempt {}: Guessed '{}', Matches = {}", attempt, guess_word, matches);

            if matches == 6 {
                println!("🎉 Success! The secret word was '{}'!", guess_word);
                return;
            }

            // Filter the candidate list: keep words that share exactly `matches` characters
            words = words
                .into_iter()
                .filter(|w| w != &guess_word && Self::get_matches(w, &guess_word) == matches)
                .collect();
        }
        println!("❌ Failed to find the secret word within 10 attempts.");
    }

    // Helper function to count exact matches between two 6-letter strings
    fn get_matches(w1: &str, w2: &str) -> i32 {
        w1.bytes()
            .zip(w2.bytes())
            .filter(|(b1, b2)| b1 == b2)
            .count() as i32
    }
}

// =====================================================================
// 3. THE MAIN FUNCTION (The driver that kicks off the program)
// =====================================================================
fn main() {
    // A list of 6-letter words
    let word_list = vec![
        String::from("ccbazz"),
        String::from("eiowzz"),
        String::from("abchef"),
        String::from("acckzz"), // Let's make this one our secret word!
        String::from("lbwzzf"),
        String::from("mzoxif"),
    ];

    // Initialize the master with the secret word hidden inside it
    let master_instance = Master {
        secret: String::from("acckzz"),
    };

    println!("--- Starting Game ---");
    
    // Call the LeetCode solution function
    Solution::find_secret_word(word_list, &master_instance);
}
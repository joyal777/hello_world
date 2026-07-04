// 1. Tell Rust to look for master.rs and solution.rs files
mod master;
mod solution;

// 2. Bring them into scope so we can use them easily
use master::Master;
use solution::Solution;

fn main() {
    let word_list = vec![
        String::from("ccbazz"),
        String::from("eiowzz"),
        String::from("abchef"),
        String::from("acckzz"), // The secret
        String::from("lbwzzf"),
    ];

    let master_instance = Master {
        secret: String::from("acckzz"),
    };

    println!("--- Starting Component-Based Game ---");
    
    // Run the solution
    Solution::find_secret_word(word_list, &master_instance);
}
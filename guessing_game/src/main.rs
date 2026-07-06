// 1. Tell Rust to look for master.rs and solution.rs files
mod master;
mod solution;

// 2. Bring them into scope so we can use them easily
use master::Master;
use solution::Solution;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {


    let rect1 = Rectangle {
        width: 123,
        height: 6743,
    };

    println!("The area of the rectagle struct is {rect1:#?}  and the rectangle is {} square pixels.", area(&rect1));


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
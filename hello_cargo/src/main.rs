fn main() {
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
    x = 7;
    println!("{x}");
    const CONST_SHOULD_NAMED_LIKE_THIS: u32 = 60*60*3;
    println!("{CONST_SHOULD_NAMED_LIKE_THIS}");

    let x = 5;
        let x = x + 1;
        {
            let x = x * 40;
            println!("The value of x in the inner scope is: {x}");
            if x >= 100 {
                let x = x * 2;
                println!("The value of x in the inner inner scope is: {x}");
            }
        }
        println!("The value of x is: {x}");

        let spaces = "   ";
        let spaces =  spaces.len();
        println!("The value of spaces is: {spaces}");

        let guess: f64 = "4562.9".parse().expect("Not a number!");
        println!("The value of guess is: {guess}");
}

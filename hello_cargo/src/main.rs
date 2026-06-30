fn main() {
    // let mut x = 5;
    // println!("{x}");
    // x = 6;
    // println!("{x}");
    // x = 7;
    // println!("{x}");
    // const CONST_SHOULD_NAMED_LIKE_THIS: u32 = 60*60*3;
    // println!("{CONST_SHOULD_NAMED_LIKE_THIS}");

    // let x = 5;
    //     let x = x + 1;
    //     {
    //         let x = x * 40;
    //         println!("The value of x in the inner scope is: {x}");
    //         if x >= 100 {
    //             let x = x * 2;
    //             println!("The value of x in the inner inner scope is: {x}");
    //         }
    //     }
    //     println!("The value of x is: {x}");

    //     let spaces = "   ";
    //     let spaces =  spaces.len();
    //     println!("The value of spaces is: {spaces}");

    //     let guess: f64 = "4562.9".parse().expect("Not a number!");
    //     println!("The value of guess is: {guess}");

        let sum = 5 + 10;
        let difference = 95.5 - 4.3;

        let product = 4 * 30;

        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;

        let remainder = 43 % 5;
        println!("The value of sum is: {sum}");
        println!("The value of difference is: {difference}");
        println!("The value of product is: {product}");
        println!("The value of quotient is: {quotient}");
        println!("The value of truncated is: {truncated}");
        println!("The value of remainder is: {remainder}");

        let cb = [10,20,30,40,50,60,70];
        let mut m = 0;

        while m < 5 {
            println!("the value is: {}", cb[m]);
            
            m += 1;
        }


        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");

        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String

        println!{"{}", s};

        let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

        let s2 = String::from("hello");    // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

        let st = String::from("hello");  // st comes into scope

        takes_ownership(st);             // st's value moves into the function...
                                        // ... and so is no longer valid here

        let xb = 5;                      // xb comes into scope

        makes_copy(xb);                  // Because i32 implements the Copy trait,
                                        // xb does NOT move into the function,
                                        // so it's okay to use xb afterward.
}
fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(c_string: String) -> String {
    // c_string comes into
    // scope

    c_string  // c_string is returned and moves out to the calling function
}

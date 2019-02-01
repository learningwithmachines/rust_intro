extern crate rand; // let rust know that we will using external dependencies in this section.

// Since we don't know what traits rand has and what to use, we can use cargo doc --open command to
// let cargo build documentation locally for all our defined dependencies and open it in browser

use std::io;   // rust brings in limited number of types by default,
use rand::Rng; // for extra, we have to bring in our own types
               // into scope explicitly using "use"

use std::cmp::Ordering; // another enum like std::io::stdin.read_line.expect => Result
                        // here, the enumerated variants are Less, Greater and Equal

fn main() {
    println!("take a guess!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter a number ..");

        let mut guess = String::new();
        // let is used to create variables mut makes it mutable, :: indicates that new
        // is an associated function of type String

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        //  returns an instance of std::io::stdin &mut guess is our argument passed to read_line
        // '&' indicates the argument passed, is a reference, using references allows us to be efficient
        // avoiding copying data, and promotes re-use, mut keeps it mutable.

        // std::io::stdin().read_line() returns a value, of type, io::Result, which itself is a
        // fixed length "enumeration", the variants here are ('Ok','Err'), with each enumeration also
        // having either the result, or additional information in case of no result.

        // the io::Result instance used here has an expect method that can be called, and will either
        // have an 'Err' or 'Ok', depending on whatever was returned by the read_line method, for 'Ok',
        // the expect takes on the return value within 'Ok', and for 'Err',
        // it will take on the associate Error message.

        // Not using the expect method will still result in a successful compilation, albeit
        // with a warning from the compiler stating that a result value is not being used,
        // indicating that program isnt handling a possible error

        println!("Your guess was: {}", guess); // {} is for formatting, a la python

        // match is an expression, and is made up of arms, arms consists of a pattern and a piece of
        // code that should be run if the value given to the match expression fits that arm's pattern.

        // since the inferred type for guess is of type string, we need to cast it into a common type,
        // such that both secret_num and guess can be evaluated by ::cmp::Ordering, as Ordering needs
        // it's operands to be share a common numeric type.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("An Integer was expected, please try again!");
                continue;
            }
        };
        // shadowing to reuse the guess variable, trim is method on our String instance, guess
        // and eliminates whitespaces, as /n is appended during read_line, from the ENTER key.
        // parse method then parses the string into some numeric type, here it's u32, and has an
        // associated Result type, in case a non-numeric input is given.

        // in the event when parse is not able to turn the string into a number, it will cause
        // match to go to second arm's pattern, Err(_) where, "_" is a catchall, which means Err
        // will always match, no matter what, should the Ok arm fail to match due to parse failing
        // to convert string into a number.


        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_num);
}

// for the dependency part where we import the rand lib
// by defining it in the cargo.toml file's dependencies section.
// https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
// we have specified our version requirements to be open by using "0"

// Cargo only updates the build with whatever tiny change there is to the src/main.rs file.
// If our dependencies haven’t changed Cargo will know that, and will reuse what it can
// by rebuilding our part of the code, that has actually changed, this information is
// kept in the cargo lock file, so as to be able to keep track of dependency drift
// and ensure reproducibility in builds, the lock can be bypassed by using the "cargo update"
// command, by default cargo will stick closely to the semver specifed, which in our case is loose
// so it will update to whatever version is the latest, which has the potential to break our code.



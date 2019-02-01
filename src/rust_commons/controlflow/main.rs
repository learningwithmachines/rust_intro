// control flow

use std::io;

fn main() {

    println!("Enter a number ..");

    let mynum: i64 = loop {

        let mut mynum = String::new();

        io::stdin().read_line( &mut mynum)
            .expect("failed to read line");

        let mynum = match mynum.trim().parse() {
            Ok(mynum) => mynum,
            Err(_) => {
                println!("Number could not be cast into a numeric type, try again!");
                continue;
            }
        };

        if check_nonzero(mynum) {
            println!("Number is valid, exiting ...");
            break mynum;
        } else {
            println!("Number is not valid, try again!");
        }

        // if expression leads to evaluation of it's arms, much like the match expression, and
        // on success, leads to associated block's execution. else is optional, and can be used
        // in case we want to handle some code execution for the if block failing. the condition
        // evaluated must always result in a bool type.

        // optionally, variables can be returned via the break expression, and be used together
        // with a looped expression to assign values to variables, provided a variable is being
        // returned by the loop body.
    };

    let mut counter  = 0;
    let mut sumall: i128 = 0;
    let mut tempsum: i128  = 0;
    let arr_null = [1,2,3,4,5,6,7,8,9];
    println!("counting up to the chosen number ..");

    if mynum < 0 {
        while counter >= mynum {
            println!("currently at {}..", counter);
            for element in arr_null.iter() {
                tempsum += element * counter as i128;
            }
            sumall += tempsum;
            tempsum = 0;
            counter -= 1;
        }
    } else {
        while counter <= mynum {
            println!("currently at {}..", counter);
            for element in arr_null.iter() {
                tempsum += element * counter as i128;
            }
            sumall += tempsum;
            tempsum = 0;
            counter += 1;
        }
    }

    println!("the totally useless summed up value was calculated upto, {}", sumall);
    println!("all done, exiting...");


}

fn check_nonzero(num: i64) -> bool {

    // it's also possible to have nested and/or multiple conditions with if, else and "else if"

    if num > 0 {
        if num % 2 == 0 {
            println!("Even and Positive");
        } else {
            println!("Odd and Positive");
        }
        return true
    } else if num < 0 {
        if num % 2 == 0 {
            println!("Even and Negative");
        } else {
            println!("Odd and Negative");
        }
        return true
    } else {
        println!("Number is Zero");
        return false
    }
}

// it's also possible to use if's inside a let statement as if is a expression that results in a value
// being returned, unlike a statement. in cases where multiple value are being evaluated for
// assignment, the types must be consistent, i.e. and if else used in a let for value assignment cannot
// optionally assign different / incompatible types, as rust needs to know types at compile time,
// and not at run time.


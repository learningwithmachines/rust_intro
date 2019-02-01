// variables and mutability

fn main() {

    //VARIABLES

    // in rust variables are immutable, by default
    // this is done to ensure safety and concurrency

    let x  = 5;
    println!("the value of x is {}, and x is immutable", x);
    // x is now an immutable variable bound to the value 5
    // attempts to change its value will lead to errors.

    let mut y = 5;
    println!("the original value of y is {}, and y is mutable", y);

    y = 65;
    println!("the value of y is now {}", y);

    // mut conveys the intent to readers that other parts of the
    // code are going to be changing this variable's value.
    // also, in larger data structures mutating an instance in place
    // maybe faster than copying and returning newly allocated instances.
    // while for smaller structures, creating newer instances and allowing
    // a more functional style maybe be easier to practice and reason with,
    // a worthwhile trade-off, for gaining extra clarity.

    //CONSTANTS

    // while this sounds like variables are constants by default, it's not
    // the case, as constants in rust are not only immutable by default,
    // they are always immutable.

    // constants are defined using the const keyword, instead of the let
    // keyword, while the type annotation is mandatory. constants can be
    // declared in any scope, including the global scope, and may only be
    // set to a constant expression, not a function call, or any value that
    // is calculated in runtime, their definitions are explicit throughout.

    const MAXIMUM: u32 = 100_000;

    //SHADOWING

    // shadowing is the act of declaring a new variable with the same name as
    // a previously existing variable, where this new variable shadows the pre-
    // -vious variable, in all subsequent sections of the code that follow the
    // shadowing.

    let z: i32 = 5 * MAXIMUM as i32;

    let z: i64 = z as i64 + 1;

    let z: i64 = z*z + 3*z;

    println!("the value of z is: {}",z);

    // shadowing is different than marking a variable as mut, as with mut we can assign
    // without let, but in shadowing we are explicitly writing over the variable instance,
    // with a new instance of different value, in mut the instance remains the same,
    // with this, shadowing allows us to do things like, change the associated type, while still
    // keeping the variable immutable for everything else.

    let spaces = "    "; //4 spaces
    let spaces = spaces.len();
    println!("spaces is {}",spaces);

    // if we had used mut, we would not have been able to change the type associated with space.

}
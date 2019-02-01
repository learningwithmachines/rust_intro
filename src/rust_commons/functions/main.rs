//functions

fn main(){

    // in rust, main function is the entry-point of programs,
    // and is defined using the "fn" keyword
    println!("this is our main function!");

    // for function names, rust uses snake case, i.e all letters
    // are in lowercase and "_" is used to separate words.
    not_main();

}

fn not_main(){

    // rust doesnt care where the function is defined,
    // as long as they are defined and in scope.
    println!("this is not our main function!");

    do_sth(999);
}

fn do_sth(x: i64){

    // in rust annotation for parameters are mandatory,
    // and are defined inside the function definition itself.
    println!("the value of x is: {}", x);
    show_and_express();
    println!("the value returned is: {}", return_sth());
}

fn show_and_express(){

    // inside the function body, we can have statements and
    // expressions, statements are instruction and expressions
    // are evaluations.
    let y = 6; // a statement
    // function definitions are also statements

    // let x = (let y = 6); => wont work, as statements cannot
    // return values, as let y = 6; , doesnt return any value,
    // and so, x won't have anything to bind to.

    let x = {
        let y = y * 3;
        y + 32
    };

    // expression can be part of statements, calling a function,
    // or calling a macro are both expressions, blocks are also
    // expressions, also expression don't include semicolons, if we
    // added a semicolon to y+32 in the block above we would have made
    // it into a statement.

    println!("the value of x from the expression is: {}", x);
}

fn return_sth() -> i32 {

    let x = simple_return();
    return x;
    // functions can be made to return a specific value-type by using
    // "->" keyword, in rust the return value from a function is by'
    // default the last expression in the block of the function's body,
    // a value can be returned earlier, through the use of return keyword.

    // anything below the return keyword, if specified is rendered unreachable.
}

fn simple_return() -> i32 {
    5
    // perfectly valid
}
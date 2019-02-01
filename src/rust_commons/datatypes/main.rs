//data types

fn main(){

    // since rust is a statically typed language, the compiler needs to
    // know the types of all variables at compile time, it can usually infer
    // what type we want to use based on the value, and how it's used, but
    // in cases when many types are possible, type annotations are a must.

    let guess: u32 = "42".parse().expect("Not a Number!");

    // SCALAR TYPES

    // represents a single value, rust has 4 primary scalar types
    // integers, floating points, booleans and characters

    // integers have 2 variants, signed and unsigned (i & u)
    // and come in sizes of 8, 16, 32, 64 and 128 bits, and a last arch
    // variant, depending on the computer architecture (usually 32 and 64)
    // additionally, they can be written in any of the following forms,
    // Decimal(98_22), Hex(0xff), Octal(0o77), Binary(0b'11110011')
    // and Byte(b'A'), '_' is a visual separator

    // using larger values, than what's specified by the type, will lead to
    // overflow, and the program will panic, in debug mode, in release modes,
    // overflow is not checked for, and values are wrapped around, i.e.
    // passing 256 to 'u8' integer variable, makes it 0, 257 makes it 1 and so on...

    // for floating points rust has 2 types, f32 and f64, f64 is the default type.

    let float_x = 2.0; //f64

    let float_y: f32 = 23.32; //f32

    println!("{}, {}, {}",guess, float_x, float_y);

    // Numeric Operations
    // rust supports all mathematical operations, like add, subtract, multiply,
    // divide and modulo.

    let sum = float_x as f32 + 2.0 as f32;
    let diff = float_y - 3.0;
    let mul = float_x as f32 * float_y as f32;
    let div = float_y / float_x as f32;
    let rem = float_y % float_x as f32;

    println!("sum={}, diff={}, mul={}, div={}, rem={}",sum,diff,mul,div,rem);

    // boolean type
    // usually used along with conditionals like 'if'

    let t = true;
    let f = false;
    println!("using booleans: {}", t == f);

    // character type
    // specified in single quotes, as opposed to String types which are in "".

    let happy_cat = '😻';
    println!("here's a happy cat, {}", happy_cat);

    // COMPOUND TYPES

    // compound types can group multiple values into one type, rust has 2 compound
    // types, tuples and arrays.

    let tup: (i32, f64, u8) = (500, 25.55, 255);
    // tuples are created by specifying a comma separated list of values inside
    // parentheses, each position has a defined type, and types inside the tuple don't
    // have to be of the same type, annotation is optional.

    // elements inside a tuple can be pattern matched to destructure a tuple value.

    let (xx, yy, zz) = tup;
    println!("de-structed tuple scalars, {}, {}, {}",xx, yy, zz);

    // in addition to destructing by pattern matching, tuple elements can also be
    // extracted directly by using period (.), operator followed by the index of
    // the value that we want to access.

    println!("element extracted tuple scalars, {}, {}, {}",tup.0, tup.1, tup.2);

    // the array type
    // unlike tuples, array elements all share a common type, and have a fixed length.
    // they are useful when we want to preallocate our data in stacks, for variable
    // length arrays, rust provides vectors, which unlike arrays, can be grown or shrunk.

    let arr_0 = [1, 2, 3, 4]; // i32 array of length 4, no annotation.
    let arr_1: [i32; 4] = [1,2,3,4];  // same i32 array but with annotation.

    println!("first elements of arrays arr_0 and arr_1: {}, {}", arr_0[0], arr_1[0]);
    // care has to be taken to not overflow the index, or we get and
    // index out of bounds error during runtime, and that is bad, very bad.

}
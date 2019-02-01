//ownership

fn main() {

    // allows rust to make memory safety guarantees without
    // needing a garbage collector.

    // all programs have to manage the way the computer's memory
    // is used while running, some use garbage collectors that
    // are constantly on the lookout for memory references that
    // are no longer being used, while in other languages, the
    // memory allocation has to be defined explicitly by the
    // programmer. rust does things a bit differently, the memory in
    // rust is managed through a system of ownership and rules that
    // allow the compiler to do checks at compile time.

    // for a systems programming language like rust, managing memory
    // is crucial, and strongly affects the programs behavior.

    // stacks use lifo scheme, that allows for fast additions and
    // deletions, as all deletions and additions always take place at
    // the top, the downside is that all that data must take up a known
    // and fixed space.

    // for data with variable or unknown size, a heap is used, a heap
    // is less organized, as when a new block of data is put in a heap,
    // the os has to find an empty spot within the heap that is big enough
    // and mark it as being in used, returning a pointer which points to
    // the address of that location, this process is know as allocation.

    // stacks don't have to allocate data, as all data in the stack is
    // of a known and fixed size, and a pointer can simply be stored in
    // the stack and data can be retrieved by following the pointer.

    // so, accessing the data on a heap is slower, as the processor has
    // to constantly look for pointers and then follow them, if the data
    // is spread out too much, it slows it even further, plus allocating
    // large amounts of data in the heap also takes time on it's own.

    // rust's ownership system tries to handles all this,
    // i.e. keeping track of what parts of code are using what data
    // on the heap, thereby minimizing duplicate data on the heap,
    // and cleaning up unused data on the heap to avoid running out of space.

    // OWNERSHIP RULES
    // Each value in rust has a variable that's called it's owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value is dropped.

    // VARIABLE SCOPE

    { // s is not a valid entity here, as it's still not defined
        let s = "hello"; // now s is valid

        // we can do stuff with s here onwards
        print!("string s in scope is => {} \nExiting Scope!",s);
    } // drop scope, s is invalid again.
    // rust calls the "drop" function to return the block back to the os.
    println!("\t scope dropped, string s is no more!\n");

    {
        println!("reallocating string s, from none ");
        let mut s = String::from("hello");
        // String::from allows us to use the namespace from String.
        // and request the os for memory to store it in the heap.
        // we can mutate this string now, like so, using the allocated
        // memory in the heap.
        s.push_str(", world!");
        println!("string s is: {}\n",s);
    }
    // on exiting the scope we return all this memory back to the os.
    // if we forget to do this, we waste memory, if we do this too early
    // we have an invalid variable, even doing this twice can cause bugs.
    // for proper allocation, we must exactly pair one allocate with one free.
    // in rust, this is done automatically on exiting the scope, through
    // the built-in drop function, that's triggered at the closing "}".

    {
        let x = 5;
        let y = x;
        // both x and y are allocated on the stack, as they are of fixed size
        // that is known at compile time.
        print!("Fixed Type, Original and Copied variables are both present");
        println!(", and exist in stack: x={}, y={}",x,y);

        let s1 = String::from("hello");
        let s2 = s1;
        // although this looks similar to the above code, for a complex type
        // like String, the stack stores the pointer, the length and the capacity
        // the actual data inside the string is stored in the heap as an indexed list.
        // the length denotes the memory content in bytes that the string is currently holding,
        // the capacity is the total amount of memory allocated to it by the os. when the copy s2
        // is made, we only copy the data that is on the stack, while the data on the heap remains
        // as is. see figure 4.2 at: https://doc.rust-lang.org/book/2018-edition/img/trpl04-02.svg

        // as both s1 and s2 are going to go out of scope at the same time, they will both try to
        // free the same memory, which can lead to memory corruption and security vulnerabilities.
        // so what rust does is it invalidates s1 when s2 is created, so there's no need to free
        // anything when s1 goes out of scope.

        // so instead of making copies, rust moves the stack and heap memory of s1 into s2.
        // the behavior can be checked by trying to print s1.

        // println!("{}, world!", s1);
        // error message: note: move occurs because `s1` has type `std::string::String`,
        // ...which does not implement the `Copy` trait
        print!("Complex Type, Original var s1 = Invalidated after copy!");
        println!(" and transferred over to s2={}\n",s2); // this will work.

        // this also means that rust will never automatically create deep copies of our data
        // ensuring that any automatic copying remains inexpensive in runtime.
    }

    {
        // on the other hand, if we really want to deeply copy the heap data of the String,
        // we can use the clone method, like so. keeping in mind that it may be expensive on runtime.

        let s1 = String::from("hello");
        print!("s1 created, value is: {}\n",s1);
        let s2 = s1.clone();

        println!("Deep Copying, Complex Type s1 into s2");
        println!("s1 = {}, s2 = {} \nDone, Exiting Scope!\n", s1, s2); // this will work
    }

    {
        // copying stack only data
        // for type with fixed and know size, the data is automatically copied on assignment

        let x  = 5;
        let y = x;
        println!("For Fixed Types with copy trait and no Drop! trait");
        println!("E.g: var x copied to y, x={}, y={}",x,y);

        // for such type, rust utilizes a special trait called copy, it is not available on
        // types that implement the Drop trait in any form, or in any part, or if the type
        // need something special to happen when the values go out of scope.

        // copy trait is available for all integer, floating point, boolean, character and
        // tuple types strictly containing simple types. a tuple with any String wont have
        // the copy trait.
    }

    // ownership and functions
    // as the semantics for passing a value to a function are similar to those for
    // assigning a value to a variable, passing a variable to a function will either
    // move or copy, just like assigning does.

    println!("Demonstrating Ownerships\n");

    let s1 = gives_ownership();
    println!("This String was created by an external function,\nand passed over to main:\t{}\n",s1);

    let s2 = String::from("Hello, from the Inside!"); // s comes in scope
    takes_ownership(s2); // s moves to function, so is no longer valid here

    let mut s3 = String::from("Hello, from the Inside!");
    let (s3, len) = takes_and_gives_back(s3);
    println!("This string belonged to main, was passed over to an external function shadowed");
    println!("and returned back with an additional variable\t\t{} Length={}\n",s3, len);

    let x:i128 = 54665134841589113413484198; // x comes into scope
    makes_copy(x); // x moves into function, but as i128 has copy trait, it will


}

fn takes_ownership(some_string: String) {
    println!("this string was passed over from main\nwhere it no longer belongs\t{}\n", some_string);
    // some_string will go out of scope and drop will be called, freeing memory
}

fn makes_copy(some_int: i128) {

    println!("This Fixed Type Integer wad passed to an external function:\t{}", some_int)
    // some_int will go out of scope
}

fn gives_ownership() -> String {

    let some_string = String::from("Hello, from the outside!");

    some_string         //some string is returned and moves over to the calling function.

}

fn takes_and_gives_back(a_string: String) -> (String,usize) {

    let mut tempstr:String = a_string.clone();
    tempstr.push_str(", and shadowed from the outside!");
    let a_string = tempstr.clone();
    let length = a_string.len();

    (a_string,length)

    //a_string and length are returned and moved over to the calling function.

}
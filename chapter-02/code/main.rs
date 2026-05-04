fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print_add() {
    let a = 10;
    let b: i32 = 32;
    let c = 44i32;
    let sum = add(add(a, b), c);
    println!("Sum: {}", sum);
}

fn print_floating_point() {
    let a = 0.32;
    let b = 0.64f32;
    let c: f32 = 0.1;
    let sum = a + b + c;
    println!("Float sum: {}", sum)
}

fn compare_nums() {
    let a: i32 = 10;
    let b: u16 = 3;
    if a > b.into() { // try_into() for safer type case checking
        println!("a > b, i.e. {} > {}", a, b)
    }
}

fn loops() {
    // Create an anonymous loop and print 3 times
    for _ in 0..3 {
        print!(".");
    }
    println!(" anonymous loop complete x3.");

    // Iterate through values of a container
    let container = [16, 10, 1993, 22, 09, 1998, 05, 05, 2006];
    for year in container {
        print!("{}, ", year);
    }
    println!(" iterated through array.");

    for year in &container {
        print!("{}, ", year);
    }
    println!(" iterated through array reference.");

    let mut container2 = [16, 10, 1993, 22, 09, 1998, 05, 05, 2006];
    for year in &mut container2 {
        *year = &*year * 2;
    }
    for year in container2 {
        print!("{}, ", year);
    }
    println!(" iterated through mutated values.");

    // Accessing indices, avoid managing index variables, use iterator
    // NOTE: you can do the following but it incurs runtime BOUNDS CHECKING overhead
    for i in 0..container2.len() {
        if i == 4 {
            continue // for fun
        }
        print!("{}: {} | ", i, container2[i]);
    }
    println!(" printed indices.");

    let mut i = 0;
    while i < 10 {
        print!(".");
        i += 1;
    }
    println!(" printed 10 dots using while")
}

fn enumerated_loop() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("Line num: {}", line_num);
        }
    }

}

// match with range matching
fn match_range() {
    let age = 31;
    print!("Age is {}, so: ", age);
    match age {
        0 => println!("Not born."),
        1 ..= 3 => println!("Toddler"),
        4 ..= 12 => println!("Child"),
        13 ..= 19 => println!("Teens"),
        30 | 40 => println!("Midlife crisis"),
        _ => println!("Adult"),
    }

}

// Skipped: lifetime annotations
// Skipped: references

// Generic functions!
fn add_generic<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn print_generic_add() {
    let t: i32 = 10;
    let s: i32 = 20;
    println!("i32 sum: {}", add_generic(t, s));
    let a: i64 = 10;
    let b: i64 = 50;
    println!("i64 sum: {}", add_generic(a, b));
}
// Different types of string
// String uses dynamic memory allocation to store the text that it represents. 
// &str values avoids a memory allocation
// Some other types may be encountered in your travels. Here’s a short list:a   char—A single character encoded as 4 bytes. The internal representation of  char is equivalent to UCS-4/UTF-32. This differs from &str and String,  which encodes single characters as UTF-8. Conversion does impose a pen-  alty, but it means that char values are of fixed-width and are, therefore, eas-  ier for the compiler to reason about. Characters encoded as UTF-8 can span  1 to 4 bytes.   [u8]—A slice of raw bytes, usually found when dealing with streams of  binary data.   Vec<u8>—A vector of raw bytes, usually created when consuming [u8] data.  String is to Vec<u8> as str is to [u8].   std::ffi::OSString—A platform-native string. It’s behavior is close to String  but without a guarantee that it’s encoded as UTF-8 and that it won’t contain  the zero byte (0x00).   std::path::Path—A string-like type that is dedicated to handling filesys-  tem paths.  

// Arrays, slices and vectors
// Arrays are fixed size, statically sized -- the size is a type.
// Slices are dynamically sized and can be views into an array
// Vector seems the most common higher level construct to use







// Grep lite (as branch)
// Adding 3p code
// Using regexp
// Generate 3P docs
// rust-up
// CLI args
// Reading from files
// Reading input via stdin



fn main() {
    let arg1 = std::env::args().nth(1);
    if arg1.is_none() {
        println!("No argument provided.");
        return;
    }

    let arg = arg1.unwrap();

    match arg.as_str() {
        "add" => print_add(),
        "float" => print_floating_point(),
        "compare" => compare_nums(),
        "loops" => loops(),
        "enumerated-loop" => enumerated_loop(),
        "match" => match_range(),
        "generic-add" => print_generic_add(),
        _ => println!("Unknown argument: {}", arg),
    }
}
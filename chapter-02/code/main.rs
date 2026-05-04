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
// Arrays, slices and vectors





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
        "match" => match_range(),
        "generic-add" => print_generic_add(),
        _ => println!("Unknown argument: {}", arg),
    }
}
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
        _ => println!("Unknown argument: {}", arg),
    }
}
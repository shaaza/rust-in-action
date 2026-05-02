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

fn main() {
    print_add()
}
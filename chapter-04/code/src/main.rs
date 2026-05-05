#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(_sat_id: i32) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    println!("sat_a: {:?}", check_status(sat_a));
    println!("sat_b: {:?}", check_status(sat_b));
    println!("sat_c: {:?}", check_status(sat_c));
}

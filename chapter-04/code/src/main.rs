#[derive(Debug)]
enum StatusMessage {
    Ok,
}

struct CubeSat {
    id: i32,
}

fn check_status(_sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    println!("sat_a: {:?}", check_status(sat_a));
    println!("sat_b: {:?}", check_status(sat_b));
    println!("sat_c: {:?}", check_status(sat_c));

    println!("sat_a: {:?}", check_status(sat_a));
    println!("sat_b: {:?}", check_status(sat_b));
    println!("sat_c: {:?}", check_status(sat_c));
}

mod stub_file_api;
use stub_file_api::prototype_file_api;

mod file_api;
use file_api::struct_file_api;


fn main() {
    let arg1 = std::env::args().nth(1);
    if arg1.is_none() {
        println!("no args given");
        return;
    }

    let arg = arg1.unwrap();
    match arg.as_str() {
        "prototype_file_api" => prototype_file_api(),
        "file_api_struct" => struct_file_api(),
        _ => println!("unknown arg"),
    }
}

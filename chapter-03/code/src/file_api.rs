#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

pub fn struct_file_api() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    println!("Struct: {:?}", f1);
    println!("{} is {} bytes long", &f1.name, &f1.data.len());
}
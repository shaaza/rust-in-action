#![allow(unused_variables)]
use rand::RngExt;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn one_in(denominator: u32) -> bool {
    // Returns true one out of 'denominator' times
    if denominator == 0 {
        return false; // Avoid division by zero
    }
    rand::rng().random_range(0..denominator) == 0
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone(); // make a copy so File owns its own data
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(5) {
        let err = String::from("file open failure random");
        return Err(err);
    }

    Ok(f)
}

fn close(f: File) -> bool {
    true
}

pub fn struct_file_api() {
    let f2 = File::new_with_data("f2.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];
    let f2 = open(f2).expect("failed to open file");
    let f2_length = f2.read(&mut buffer);

    let text = String::from_utf8_lossy(&buffer);

    println!("Struct: {:?}", f2);
    println!(
        "{} is {} bytes long, read {} bytes",
        &f2.name,
        &f2.data.len(),
        f2_length
    );
    println!("{}", text);

    close(f2);
}

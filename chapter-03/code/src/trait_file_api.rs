//! Trait-based file API example that moves read behavior behind a trait.

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

/// Represents an in-memory file whose bytes can be read through the `Read` trait.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
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
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

pub fn trait_file_api() {
    let f = File::new_with_data("f3.txt", &vec![114, 117, 115, 116, 33]);
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}

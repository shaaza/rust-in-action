#![allow(unused_variables)]

#[derive(Debug)]
struct File {
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

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}


pub fn struct_file_api() {
    let mut f2 = File::new_with_data("f2.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];
    open(&mut f2);
    let f2_length = f2.read(&mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("Struct: {:?}", f2);
    println!("{} is {} bytes long, read {} bytes", &f2.name, &f2.data.len(), f2_length);
    println!("{}", text);
}
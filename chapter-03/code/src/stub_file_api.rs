#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn close(f: &mut File) -> bool {
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

pub fn prototype_file_api() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
    read(&mut f1, &mut vec![]);
}

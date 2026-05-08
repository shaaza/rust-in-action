use std::path::PathBuf;

pub trait Store {
    fn get(&self, key: &str);
    fn insert(&mut self, key: &str, value: &str);
    fn update(&mut self, key: &str, value: &str);
    fn delete(&mut self, key: &str);
}

pub struct KVStore {
    filepath: PathBuf,
}

impl KVStore {
    pub fn open(filepath: PathBuf) -> Self {
        Self { filepath }
    }
}

impl Store for KVStore {
    fn get(&self, key: &str) {
        println!("get {key} from {:?}", self.filepath);
    }

    fn insert(&mut self, key: &str, value: &str) {
        println!("insert {key}={value} into {:?}", self.filepath);
    }

    fn update(&mut self, key: &str, value: &str) {
        println!("update {key}={value} in {:?}", self.filepath);
    }

    fn delete(&mut self, key: &str) {
        println!("delete {key} from {:?}", self.filepath);
    }
}

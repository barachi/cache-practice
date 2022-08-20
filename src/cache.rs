use serde::{de, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache<T> {
    data: HashMap<String, T>,
}
impl<T> Cache<T> {
    pub fn new() -> Cache<T> {
        Cache {
            data: HashMap::<String, T>::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&mut self, key: &str) -> Option<&T> {
        match self.data.get(key) {
            Some(data) => Some(&data),
            None => None,
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn save(&mut self)
    where
        T: serde::Serialize,
    {
        let path = "cache.json";

        let mut file = match File::create(&path) {
            Err(e) => panic!("create faild: {}", e),
            Ok(file) => file,
        };
        let json_data = serde_json::to_string(&self).unwrap();

        match file.write_all(json_data.as_bytes()) {
            Err(e) => panic!("write failed: {}", e),
            Ok(_) => println!("write success"),
        }
    }
    pub fn load(&mut self)
    where
        for<'a> T: de::Deserialize<'a>,
    {
        let path = "cache.json";
        let content = fs::read_to_string(path).expect("Failed to load JSON");
        let mut tmp = Cache {
            data: HashMap::<String, T>::new(),
        };
        tmp = serde_json::from_str(&content).unwrap();
        self.data = tmp.data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn getset() {
        let mut cache = Cache::<String>::new();
        cache.set("test", String::from("TEST"));
        assert_eq!(cache.get("test"), Some(&String::from("TEST")));

        cache.set("test", String::from("HOGE"));
        assert_eq!(cache.get("test"), Some(&String::from("HOGE")));
    }

    #[test]
    fn delete() {
        let mut cache = Cache::new();
        cache.set("test", String::from("TEST"));
        cache.delete("test");
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn clear() {
        let mut cache = Cache::new();
        cache.set("test1", String::from("TEST1"));
        cache.set("test2", String::from("TEST2"));

        cache.clear();
        assert!(cache.get("test1").is_none());
        assert!(cache.get("test2").is_none());
    }

    #[test]
    fn saveload() {
        let mut cache_one = Cache::<String>::new();
        cache_one.set("test", String::from("TEST"));
        assert_eq!(cache_one.get("test"), Some(&String::from("TEST")));

        cache_one.set("test", String::from("HOGE"));
        assert_eq!(cache_one.get("test"), Some(&String::from("HOGE")));

        cache_one.save();
        let mut cache_two = Cache::<String>::new();
        cache_two.load();
        assert_eq!(cache_one.get("test"), cache_two.get("test"));
    }
}

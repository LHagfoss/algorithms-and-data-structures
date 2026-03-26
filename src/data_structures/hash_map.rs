use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Debug)]
struct Entry {
    key: String,
    value: i32,
}

pub struct SimpleHashMap {
    buckets: Vec<Vec<Entry>>,
    num_entries: usize,
}

impl SimpleHashMap {
    pub fn new(capacity: usize) -> Self {
        SimpleHashMap {
            buckets: vec![Vec::new(); capacity],
            num_entries: 0,
        }
    }

    pub fn hash_key(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();

        key.hash(&mut hasher);

        let hash_value = hasher.finish() as usize;

        hash_value % self.buckets.len()
    }

    pub fn insert(&mut self, key: String, value: i32) {
        if self.num_entries >= self.buckets.capacity() {
            return;
        }

        let index = self.hash_key(&key);
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.key == key {
                entry.value = value;
                
                return;
            }
        }

        bucket.push(Entry { key, value });
        self.num_entries += 1;
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let index = self.hash_key(key);
        let bucket = &self.buckets[index];

        for entry in bucket.iter() {
            if entry.key == key {
                return Some(entry.value);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &str) -> bool {
        let index = self.hash_key(key);
        let bucket = &mut self.buckets[index];

        if let Some(pos) = bucket.iter().position(|e| e.key == key) {
            bucket.remove(pos);
            self.num_entries -= 1;
            return true
        }

        false
    }
}
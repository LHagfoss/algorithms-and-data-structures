use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;
use std::borrow::Borrow;

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct SimpleHashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    num_entries: usize,
}

impl <K, V> SimpleHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        SimpleHashMap {
            buckets: (0..16).map(|_| Vec::new()).collect(),
            num_entries: 0,
        }
    }

    fn get_bucket_index<Q>(key: &Q, capacity: usize) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % capacity
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.num_entries >= self.buckets.len() * 3 / 4 {
            self.resize();
        }

        let index = Self::get_bucket_index(&key, self.buckets.len());
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.key == key {
                return Some(mem::replace(&mut entry.value, value));
            }
        }

        bucket.push(Entry { key, value });
        self.num_entries += 1;
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::get_bucket_index(key, self.buckets.len());
        let bucket = &self.buckets[index];

        for entry in bucket {
            if entry.key.borrow() == key {
                return Some(&entry.value);
            }
        }

        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::get_bucket_index(key, self.buckets.len());
        let bucket = &mut self.buckets[index];

        if let Some(pos) = bucket.iter().position(|e| e.key.borrow() == key) {
            let entry = bucket.remove(pos);
            self.num_entries -= 1;
            return Some(entry.value);
        }

        None
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_buckets: Vec<Vec<Entry<K, V>>> = (0..new_capacity).map(|_| Vec::new()).collect();

        for bucket in self.buckets.iter_mut() {
            for entry in bucket.drain(..) {
                let new_index = Self::get_bucket_index(&entry.key, new_capacity);
                new_buckets[new_index].push(entry);
            }
        }

        self.buckets = new_buckets;
    }

    pub fn len(&self) -> usize {
        self.num_entries
    }

    pub fn is_empty(&self) -> bool {
        self.num_entries == 0
    }
}
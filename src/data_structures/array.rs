use crate::algorithms::binary_search::{is_in_unsorted, SortingAlgorithm};

/// Static Array, static size of 10, length `usize`
pub struct StaticArray {
    data: [i32; 10],
    length: usize
}

impl StaticArray {
    /// New instance of StaticArray
    pub fn new() -> Self {
        Self {
            data: [0; 10],
            length: 0,
        }
    }

    pub fn push(&mut self, value: i32) -> Result<(), &'static str> {
        if self.length >= self.data.len() {
            return Err("Array is full");
        }

        self.data[self.length] = value;
        self.length += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        self.length -= 1;
        Some(self.data[self.length])
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.length {
            return None
        }

        Some(self.data[index])
    }

    pub fn delete_at(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.length {
            return Err("Index out of bounds");
        }

        for i in index..(self.length - 1) {
            self.data[i] = self.data[i + 1];
        }

        self.length -= 1;
        Ok(())
    }

    pub fn contains(&self, target: i32) -> bool {
        let mut temp_slice = self.data[0..self.length].to_vec();

        is_in_unsorted(&mut temp_slice, target, SortingAlgorithm::MergeSort)
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
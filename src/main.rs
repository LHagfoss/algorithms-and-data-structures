use crate::algorithms::binary_search::{is_in_sorted, is_in_unsorted, SortingAlgorithm};

pub mod algorithms;
pub mod data_structures;

fn main() {
    let sorted_array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut unsorted_array: [i32; 10] = [5, 4, 2, 6, 10, 8, 4, 2, 5, 4];

    println!("{}", is_in_sorted(&sorted_array, 4));
    println!("{}", is_in_unsorted(&mut unsorted_array, 4, SortingAlgorithm::MergeSort));
}

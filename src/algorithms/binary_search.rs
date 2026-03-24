use crate::algorithms::sorting::{bubble_sort, insertion_sort, merge_sort};

/// Enum for choosing algorithm for testing in binary search `is_in_unsorted`
pub enum SortingAlgorithm {
    /// Insertion Sort
    /// Time complexity - O(n^2)
    /// Space complexity - O(1)
    InsertionSort,

    /// Bubble Sort
    /// Time complexity - O(n^2)
    /// Space complexity - O(1)
    BubbleSort,

    /// Merge Sort
    /// Time complexity - O(n log n)
    /// Space complexity - O(n)
    MergeSort,
}

/// Takes in Sorted array and target Integer and uses binary search to find if number exists in array
pub fn is_in_sorted(arr: &[i32], target: i32) -> bool {
    if arr.is_empty() {
        return false;
    }

    binary_search(arr, target)
}

/// Takes in unsorted array and target Integer, then sorts it and uses binary search to find if number exists in array
pub fn is_in_unsorted(arr: &mut [i32], target: i32, sorting_algorithm: SortingAlgorithm) -> bool {
    if arr.is_empty() {
        return false;
    }

    match sorting_algorithm {
        SortingAlgorithm::InsertionSort => {
            insertion_sort(arr);
        },
        SortingAlgorithm::BubbleSort => {
            bubble_sort(arr);
        }
        SortingAlgorithm::MergeSort => {
            merge_sort(arr);
        }
    }

    binary_search(arr, target)
}


/// Searches for number in array
fn binary_search(arr: &[i32], target: i32) -> bool {
    let mut low = 0usize;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_value = arr[mid];

        if mid_value == target {
            return true;
        } else if mid_value < target {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }

            high = mid - 1;
        }
    }

    false
}

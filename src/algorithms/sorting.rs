/// Merge sort
/// Time complexity O(n log n)
/// Space complexity O(n)
pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let mut temp = arr.to_vec();
    merge_sort_recursive(arr, &mut temp, 0, arr.len());
}

fn merge_sort_recursive(arr: &mut [i32], temp: &mut [i32], start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }

    let mid = start + (end - start) / 2;
    merge_sort_recursive(arr, temp, start, mid);
    merge_sort_recursive(arr, temp, mid, end);
    merge(arr, temp, start, mid, end);
}

fn merge(arr: &mut [i32], temp: &mut [i32], start: usize, mid: usize, end: usize) {
    let mut left = start;
    let mut right = mid;
    let mut write = start;

    while left < mid && right < end {
        if arr[left] <= arr[right] {
            temp[write] = arr[left];
            left += 1;
        } else {
            temp[write] = arr[right];
            right += 1;
        }
        write += 1;
    }

    while left < mid {
        temp[write] = arr[left];
        left += 1;
        write += 1;
    }

    while right < end {
        temp[write] = arr[right];
        right += 1;
        write += 1;
    }

    arr[start..end].copy_from_slice(&temp[start..end]);
}

/// Insertion sort
/// Time complexity of O(n^2)
/// Space complexity of O(1)
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let current = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > current {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = current;
    }
}

/// Bubble sort
/// Time complexity of O(n^2)
/// Space complexity of O(1)
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    for end in (1..n).rev() {
        let mut swapped = false;

        for i in 0..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

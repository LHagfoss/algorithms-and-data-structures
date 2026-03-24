use crate::data_structures::array::StaticArray;

pub mod algorithms;
pub mod data_structures;

fn main() {
    let mut sorted_array = StaticArray::new();

    for i in 0..10 {
        println!("pushed {}", i + 1);
        sorted_array.push(i + 1).expect("Error pushing");
    }

    println!("{}", sorted_array.is_empty());

    if let Some(value) = sorted_array.pop() {
        println!("popped {}", value);
    } else {
        eprintln!("Stack is empty");
    }

    if let Some(value) = sorted_array.get(1) {
        println!("index[1] = {}", value)
    } else {
        eprintln!("Index out of bounds")
    }

    println!("{:?}", sorted_array.delete_at(0));
}

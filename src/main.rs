use crate::data_structures::hash_map::SimpleHashMap;

pub mod algorithms;
pub mod data_structures;

fn main() {
    // let mut sorted_array = StaticArray::new();
    //
    // for i in 0..10 {
    //     println!("pushed {i + 1}");
    //     sorted_array.push(i + 1).expect("Error pushing");
    // }
    //
    // println!("{}", sorted_array.is_empty());
    //
    // if let Some(value) = sorted_array.pop() {
    //     println!("popped {value}");
    // } else {
    //     eprintln!("Stack is empty");
    // }
    //
    // if let Some(value) = sorted_array.get(1) {
    //     println!("index[1] = {value}")
    // } else {
    //     eprintln!("Index out of bounds")
    // }
    //
    // println!("{:?}", sorted_array.delete_at(0));

    // let mut linked_list = SinglyLinkedList::new();
    //
    // linked_list.push_back(1);
    // linked_list.push_back(2);
    // linked_list.push_back(3);
    // linked_list.push_back(4);
    // linked_list.push_back(5);
    // linked_list.push_back(6);
    //
    // linked_list.print();
    //
    // println!("{}", linked_list.contains(1)); // Should work
    // println!("{}", linked_list.contains(7)); // should fail
    //
    // println!("{}", linked_list.contains(0));

    let mut simple_hash_map = SimpleHashMap::new(10);

    simple_hash_map.insert("lucas".into(), 18);
    simple_hash_map.insert("herman".into(), 18);
    simple_hash_map.insert("saga".into(), 17);
    simple_hash_map.insert("jp".into(), 19);

    if let Some(age) = simple_hash_map.get("lucas") {
        println!("{}", age);
    }

    if let Some(age) = simple_hash_map.get("saga") {
        println!("{}", age);
    }

    simple_hash_map.remove("lucas");

    if let Some(age) = simple_hash_map.get("lucas") {
        println!("{}", age);
    } else {
        println!("no existo");
    }
}
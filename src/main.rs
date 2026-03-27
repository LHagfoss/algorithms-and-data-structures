use crate::data_structures::binary_search_tree::BinarySearchTree;

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

    // let mut simple_hash_map: SimpleHashMap<String, i32> = SimpleHashMap::new();
    //
    // simple_hash_map.insert("lucas".to_string(), 18);
    // simple_hash_map.insert("herman".to_string(), 18);
    // simple_hash_map.insert("saga".to_string(), 17);
    // simple_hash_map.insert("jp".to_string(), 19);
    //
    // if let Some(age) = simple_hash_map.get("lucas") {
    //     println!("{}", age);
    // }
    //
    // if let Some(age) = simple_hash_map.get("saga") {
    //     println!("{}", age);
    // }
    //
    // simple_hash_map.remove("lucas");
    //
    // if let Some(age) = simple_hash_map.get("lucas") {
    //     println!("{}", age);
    // } else {
    //     println!("no existo");
    // }

    let mut binary_search_tree: BinarySearchTree<i32> = BinarySearchTree::new();

    binary_search_tree.insert(5);
    binary_search_tree.insert(10);
    binary_search_tree.insert(20);
    binary_search_tree.insert(15);
    binary_search_tree.insert(3);
    binary_search_tree.insert(8);

    println!("{}", binary_search_tree.search(5)); // true
    println!("{}", binary_search_tree.search(7)); // false

    if let Some(exists) = binary_search_tree.min() {
        println!("min {}", exists);
    }

    if let Some(exists) = binary_search_tree.max() {
        println!("max {}", exists);

    }
}
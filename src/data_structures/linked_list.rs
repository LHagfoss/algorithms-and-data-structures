pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>
}

pub  struct SinglyLinkedList {
    head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        println!("pushed {value} to new tail node");
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn contains(&self, target: i32) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.value == target {
                return true;
            }

            current = &node.next;
        }

        false
    }

    pub fn print(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            println!("{}", node.value);

            current = &node.next;
        }
    }
}


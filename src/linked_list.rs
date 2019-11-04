struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn create_node(data: T) -> Node<T>{
        Node {data: data, next: None}
    }

    fn add_node(&mut self, data: T) {
        let next_node = &mut self.next;
        match next_node {
            Some(next_node) => {
                next_node.add_node(data);
            }
            None => {
                let new_node = Box::new(Node::create_node(data));
                self.next = Some(new_node);
            }
        }
    }

    fn get_data(&self) -> &T {
        &self.data
    }

    fn next_node(&self) -> &Option<Box<Node<T>>>{
        &self.next
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn create_linked_list() -> LinkedList<T> {
        LinkedList {length: 0, head: None}
    }

    pub fn add_data(&mut self, data: T) {
        self.length += 1;
        match &mut self.head {
            Some(head) => {
                head.add_node(data);
            }
            None => {
                let new_node = Box::new(Node::create_node(data));
                self.head = Some(new_node);
            }
        }
    }

    pub fn print_list(&self) 
    where T: std::fmt::Display {
        let mut traversal_node = &self.head;
        loop {
            match traversal_node {
                Some(node) => {
                    println!("Node has data: {}", node.get_data());
                    traversal_node = node.next_node();
                }
                None => {
                    break;
                }
            }
        }
    }
}
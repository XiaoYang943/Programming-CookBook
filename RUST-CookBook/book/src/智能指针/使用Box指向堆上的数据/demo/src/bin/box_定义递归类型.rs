#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        let child = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        match child {
            Some(node) => node.insert(value),
            None => *child = Some(Box::new(Node::new(value))),
        }
    }

    fn contains(&self, value: i32) -> bool {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => self
                .left
                .as_deref()
                .is_some_and(|node| node.contains(value)),
            std::cmp::Ordering::Greater => self
                .right
                .as_deref()
                .is_some_and(|node| node.contains(value)),
        }
    }
}

fn main() {
    let mut root = Node::new(8);
    root.insert(3);
    root.insert(10);
    root.insert(6);

    assert!(root.contains(6));
    assert!(!root.contains(7));
}

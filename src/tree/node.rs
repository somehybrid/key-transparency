use sha2::{Digest, Sha256};

pub struct Node {
    pub leaf: bool,
    pub value: [u8; 32],
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf(value: [u8; 32]) -> Self {
        Node {
            value,
            left: None,
            right: None,
            leaf: true,
        }
    }

    pub fn new_branch(left: Box<Node>, right: Box<Node>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.value);
        hasher.update(&right.value);
        let value = hasher.finalize();

        Node {
            value: value.into(),
            left: Some(left),
            right: Some(right),
            leaf: false,
        }
    }
}

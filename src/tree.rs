use core::fmt::Debug;
use digest::Digest;
use digest::Output;

#[derive(Clone)]
pub struct Node<H: Digest + Clone> {
    pub value: Output<H>,
}

impl<H: Digest + Clone> Node<H> {
    pub fn new(value: &[u8]) -> Self {
        let mut hasher = H::new();
        hasher.update(value);
        let value = hasher.finalize();

        Node { value }
    }
}

impl<H: Digest + Clone> Debug for Node<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ value: ... }}")
    }
}

impl<H: Digest + Clone> PartialEq for Node<H> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Clone)]
pub struct Tree<H: Digest + Clone> {
    nodes: Vec<Vec<Node<H>>>,
    leaves_cache: Vec<Node<H>>,
}

impl<H: Digest + Clone> Tree<H> {
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            leaves_cache: Vec::new(),
        }
    }

    fn from(nodes: Vec<Vec<Node<H>>>) -> Tree<H> {
        Tree {
            nodes,
            leaves_cache: Vec::new(),
        }
    }

    fn from_leaf_nodes(leaves: &mut Vec<Node<H>>) -> Tree<H> {
        let mut tree: Vec<Vec<Node<H>>> = Vec::new();
        let mut current_layer: Vec<Node<H>> = Vec::new();

        current_layer.append(leaves);

        tree.push(current_layer.clone());

        let mut layer_num = 0usize;
        while current_layer.len() > 1 {
            current_layer.clear();

            for i in 0..current_layer.len() / 2 {
                let left = &tree[layer_num][i * 2];
                let right = &tree[layer_num][i * 2 + 1];

                let mut hasher = H::new();
                hasher.update(&left.value);
                hasher.update(&right.value);
                let value = hasher.finalize();

                current_layer.push(Node { value });
            }

            layer_num += 1;
        }

        Tree::from(tree)
    }

    pub fn from_leaves(leaves: Vec<&[u8]>) -> Tree<H> {
        let mut nodes = leaves.iter().map(|x| Node::new(x)).collect();
        Tree::from_leaf_nodes(&mut nodes)
    }

    pub fn insert(&mut self, value: &[u8]) {
        self.leaves_cache.push(Node::new(value));
    }

    pub fn commit(&mut self) {
        let mut branch = Tree::from_leaf_nodes(&mut self.leaves_cache.clone());
        self.leaves_cache.clear();
        println!("branch: {:?}", branch);

        if branch.nodes.len() > self.nodes.len() {
            for _ in 0..(branch.nodes.len() - self.nodes.len()) {
                self.nodes.push(Vec::new());
            }
        }

        for (tree_layer, branch_layer) in self.nodes.iter_mut().zip(branch.nodes.iter_mut()) {
            tree_layer.append(branch_layer);
        }
    }
}

impl<H: Digest + Clone> Debug for Tree<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree {{ nodes: {:?} }}", self.nodes)
    }
}

impl<H: Digest + Clone> PartialEq for Tree<H> {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Sha256;

    #[test]
    fn test_tree() {
        let mut tree: Tree<Sha256> = Tree::new();
        tree.insert(&[1, 2, 3]);
        tree.insert(&[4, 5, 6]);
        tree.insert(&[7, 8, 9]);

        tree.commit();

        tree.insert(&[10, 11, 12]);

        tree.commit();

        let mut nodes = vec![
            Node::new(&[1, 2, 3]),
            Node::new(&[4, 5, 6]),
            Node::new(&[7, 8, 9]),
            Node::new(&[10, 11, 12]),
        ];

        assert_eq!(Tree::from_leaf_nodes(&mut nodes), tree);
    }
}

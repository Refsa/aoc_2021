use crate::runner::Runner;

#[derive(Default)]
pub struct AOC18 {
    lines: Vec<Tree>,
}

impl Runner for AOC18 {
    fn parse(&mut self, input: &Vec<String>) {
        let mut lines = Vec::new();

        for l in input.iter().map(|e| e.chars().collect::<Vec<char>>()) {
            let mut tree = Tree::default();
            let mut tree_stack = vec![];

            for c in l {
                match c {
                    '[' => match tree_stack.last() {
                        Some(&parent) => tree_stack.push(tree.add_child(parent, None)),
                        None => tree_stack.push(tree.get_root()),
                    },
                    ']' => {
                        tree_stack.pop();
                    }
                    ',' => {}
                    _ => {
                        let val = c as u8 - 48;
                        let parent = *tree_stack.last().unwrap();
                        tree.add_child(parent, Some(val));
                    }
                }
            }

            lines.push(tree);
        }

        self.lines = lines;
    }

    fn run_p1(&self) -> usize {
        let mut l_tree = self.lines[0].clone();
        l_tree.split_l();

        let r_tree = self.lines[1].clone();
        l_tree.nodes[2] = Node::Parent { idx: 2 };
        l_tree.insert_tree(l_tree.nodes[2], r_tree.nodes[1], &r_tree);
        l_tree.insert_tree(l_tree.nodes[2], r_tree.nodes[2], &r_tree);

        println!("{:#?}", l_tree);

        0
    }

    fn run_p2(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Empty { idx: usize },
    Parent { idx: usize },
    Child { idx: usize, val: u8 },
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty { idx: 0 }
    }
}

impl Node {
    fn left_idx(&self) -> usize {
        self.idx() * 2 + 1
    }

    fn right_idx(&self) -> usize {
        self.idx() * 2 + 2
    }

    fn idx(&self) -> usize {
        match self {
            Node::Parent { idx, .. } | Node::Child { idx, .. } | Node::Empty { idx, .. } => *idx,
            _ => unreachable!(),
        }
    }

    fn is_leaf(&self) -> bool {
        if let Node::Child { .. } = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
struct Tree {
    nodes: Vec<Node>,
}

impl Default for Tree {
    fn default() -> Self {
        let tree = Self { nodes: vec![] };
        tree
    }
}

impl Tree {
    fn get_root(&mut self) -> Node {
        if self.nodes.len() == 0 {
            let root = Node::Parent { idx: 0 };
            self.nodes.push(root);
        }

        self.nodes[0]
    }

    fn get_left(&self, node: Node) -> Option<Node> {
        let idx = node.left_idx();
        if idx >= self.nodes.len() {
            None
        } else {
            Some(self.nodes[idx])
        }
    }

    fn get_right(&self, node: Node) -> Option<Node> {
        let idx = node.right_idx();
        if idx >= self.nodes.len() {
            None
        } else {
            Some(self.nodes[idx])
        }
    }

    fn get_parent(&self, node: Node) -> Node {
        self.nodes[node.idx() / 2]
    }

    fn add_child(&mut self, node: Node, val: Option<u8>) -> Node {
        self.ensure_len(node.left_idx());
        self.ensure_len(node.right_idx());

        if node.left_idx() >= self.nodes.len() {
            self.insert_left(node, val)
        } else if node.right_idx() >= self.nodes.len() {
            self.insert_right(node, val)
        } else if let Some(Node::Empty { idx }) = self.get_left(node) {
            self.nodes[idx] = self.create_node(idx, val);
            self.nodes[idx]
        } else if let Some(Node::Empty { idx }) = self.get_right(node) {
            self.nodes[idx] = self.create_node(idx, val);
            self.nodes[idx]
        } else {
            unreachable!()
        }
    }

    fn insert_left(&mut self, parent: Node, val: Option<u8>) -> Node {
        let idx = parent.left_idx();
        if idx >= self.nodes.len() {
            self.extend(idx - self.nodes.len() + 1);
        }

        let new_node = self.create_node(idx, val);
        self.nodes[idx] = new_node;
        new_node
    }

    fn insert_right(&mut self, parent: Node, val: Option<u8>) -> Node {
        let idx = parent.right_idx();
        if idx >= self.nodes.len() {
            self.extend(idx - self.nodes.len() + 1);
        }

        let new_node = self.create_node(idx, val);
        self.nodes[idx] = new_node;
        new_node
    }

    fn split_l(&mut self) {
        let mut tree = Tree::default();
        let new_root = tree.get_root();
        let right_root = tree.insert_left(new_root, None);

        let root = self.get_root();
        tree.insert_tree(right_root, self.get_left(root).unwrap(), self);
        tree.insert_tree(right_root, self.get_right(root).unwrap(), self);
        self.nodes = tree.nodes;
    }

    fn insert_tree(&mut self, parent: Node, node: Node, other: &Tree) {
        match other.nodes[node.idx()] {
            Node::Child { val, .. } => {
                self.add_child(parent, Some(val));
            }
            Node::Empty { .. } => {
                self.add_child(parent, None);
            }
            Node::Parent { .. } => {
                let p = self.add_child(parent, None);

                self.ensure_len(p.left_idx());
                if let Some(n) = other.get_left(p) {
                    self.insert_tree(p, n, other);
                }

                self.ensure_len(p.right_idx());
                if let Some(n) = other.get_right(p) {
                    self.insert_tree(p, n, other);
                }
            }
        }
    }

    fn create_node(&self, idx: usize, val: Option<u8>) -> Node {
        if let Some(val) = val {
            Node::Child { val: val, idx: idx }
        } else {
            Node::Parent { idx: idx }
        }
    }

    fn ensure_len(&mut self, size: usize) {
        if size >= self.nodes.len() {
            self.extend(size - self.nodes.len() + 1);
        }
    }

    fn extend(&mut self, by: usize) {
        let s = self.nodes.len();
        for i in 0..by {
            self.nodes.push(Node::Empty { idx: s + i });
        }
    }
}

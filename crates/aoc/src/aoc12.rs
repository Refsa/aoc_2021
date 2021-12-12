use crate::runner::Runner;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

type NodeIndex = usize;
type EdgeIndex = usize;
type EdgeCost = usize;

#[derive(Debug)]
enum Node {
    Entrance(NodeIndex, String),
    Exit(NodeIndex, String),
    SmallCave(NodeIndex, String),
    BigCave(NodeIndex, String),
}

#[derive(Default)]
struct Edge {
    node_a: NodeIndex,
    node_b: NodeIndex,
    edge_cost: EdgeCost,
}

#[derive(Default)]
struct Graph {
    entrance: NodeIndex,
    exit: NodeIndex,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    edge_lookup: HashMap<NodeIndex, Vec<EdgeIndex>>,
    node_lookup: HashMap<String, NodeIndex>,
}

impl Graph {
    fn add_node(&mut self, ident: &str) -> NodeIndex {
        let idx = self.nodes.len();
        let node = match ident {
            "start" => {
                self.entrance = idx;
                Node::Entrance(idx, ident.to_string())
            }
            "end" => {
                self.exit = idx;
                Node::Exit(idx, ident.to_string())
            }
            _ if (ident.chars().nth(0).unwrap() as u8) < 97 => {
                Node::BigCave(idx, ident.to_string())
            }
            _ => Node::SmallCave(idx, ident.to_string()),
        };

        self.nodes.push(node);
        self.node_lookup.insert(ident.to_string(), idx);

        idx
    }

    fn add_edge(&mut self, idx_a: NodeIndex, idx_b: NodeIndex) {
        if idx_a >= self.nodes.len() || idx_b >= self.nodes.len() {
            panic!("trying to add edge between non-existing nodes");
        }

        let edge = Edge {
            node_a: idx_a,
            node_b: idx_b,
            edge_cost: 0,
        };
        let edge_idx = self.edges.len();

        let a_lookup = {
            if !self.edge_lookup.contains_key(&idx_a) {
                self.edge_lookup.insert(idx_a, Vec::new());
            }
            self.edge_lookup.get_mut(&idx_a).unwrap()
        };
        a_lookup.push(edge_idx);

        let b_lookup = {
            if !self.edge_lookup.contains_key(&idx_b) {
                self.edge_lookup.insert(idx_b, Vec::new());
            }
            self.edge_lookup.get_mut(&idx_b).unwrap()
        };
        b_lookup.push(edge_idx);

        self.edges.push(edge)
    }

    fn get_node_by_name(&self, ident: &str) -> Option<&NodeIndex> {
        self.node_lookup.get(ident)
    }
}

#[derive(Default)]
pub struct AOC12 {
    graph: Graph,
}

impl Runner for AOC12 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        let mut graph = Graph::default();

        for l in input {
            let (a, b) = l.split_once('-').unwrap();
            let a_idx = if let Some(idx) = graph.get_node_by_name(a) {
                *idx
            } else {
                graph.add_node(a)
            };
            let b_idx = if let Some(idx) = graph.get_node_by_name(b) {
                *idx
            } else {
                graph.add_node(b)
            };

            graph.add_edge(a_idx, b_idx);
        }
        /* println!();
        for (name, node) in &graph.node_lookup {
            println!("{} : {:?}", name, graph.nodes[*node]);
        }

        println!();
        for edge in &graph.edges {
            println!("{:?} - {:?}", graph.nodes[edge.node_a], graph.nodes[edge.node_b]);
        }

        println!();
        for (nidx, eidxs) in &graph.edge_lookup {
            println!("{:?} - {:?}", graph.nodes[*nidx], eidxs);
        } */

        self.graph = graph;
    }
    fn run_p1(&self) -> usize {
        find_paths_p1(&self.graph).len()
    }
    fn run_p2(&self) -> usize {
        find_paths_p2(&self.graph).len()
    }
}

fn find_paths_p1(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();

    find_path_p1(graph, graph.entrance, HashSet::new(), vec![], &mut paths);

    return paths;
}

fn find_path_p1(
    graph: &Graph,
    current: NodeIndex,
    mut visited: HashSet<NodeIndex>,
    mut path: Vec<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);

    match &graph.nodes[current] {
        Node::Entrance(_, _) => (),
        Node::Exit(_, _) => {
            paths.push(path);
            return;
        }
        Node::SmallCave(_, _) => {
            let _ = visited.insert(current);
        }
        Node::BigCave(_, _) => (),
    }

    for &n in graph.edge_lookup.get(&current).unwrap() {
        let edge = &graph.edges[n];
        let target = if edge.node_a == current {
            edge.node_b
        } else {
            edge.node_a
        };

        if target == graph.entrance || visited.contains(&target) {
            continue;
        }

        find_path_p1(graph, target, visited.clone(), path.clone(), paths);
    }
}

fn find_paths_p2(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();

    find_path_p2(graph, graph.entrance, 0, HashSet::new(), vec![], &mut paths);

    return paths;
}

fn find_path_p2(
    graph: &Graph,
    current: NodeIndex,
    mut small_visit_count: u8,
    mut visited: HashSet<NodeIndex>,
    mut path: Vec<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);

    match &graph.nodes[current] {
        Node::Entrance(_, _) => (),
        Node::Exit(_, _) => {
            paths.push(path);
            return;
        }
        Node::SmallCave(_, _) => {
            if !visited.insert(current) {
                small_visit_count += 1;
            }
        }
        Node::BigCave(_, _) => (),
    }

    for &n in graph.edge_lookup.get(&current).unwrap() {
        let edge = &graph.edges[n];
        let target = if edge.node_a == current {
            edge.node_b
        } else {
            edge.node_a
        };

        if target == graph.entrance || (visited.contains(&target) && small_visit_count > 0) {
            continue;
        }

        find_path_p2(graph, target, small_visit_count, visited.clone(), path.clone(), paths);
    }
}

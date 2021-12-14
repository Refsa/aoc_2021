use crate::runner::Runner;
use std::collections::HashMap;
use std::collections::HashSet;

type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(Debug)]
enum Node {
    Entrance(NodeIndex),
    Exit(NodeIndex),
    SmallCave(NodeIndex),
    BigCave(NodeIndex),
}

#[derive(Default)]
struct Edge {
    node_a: NodeIndex,
    node_b: NodeIndex,
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
                Node::Entrance(idx)
            }
            "end" => {
                self.exit = idx;
                Node::Exit(idx)
            }
            // find upper case based on ASCII index
            _ if (ident.chars().nth(0).unwrap() as u8) < 97 => Node::BigCave(idx),
            _ => Node::SmallCave(idx),
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

        self.graph = graph;
    }
    fn run_p1(&self) -> usize {
        let mut paths = Vec::new();

        find_paths_p1(
            &self.graph,
            self.graph.entrance,
            HashSet::new(),
            vec![],
            &mut paths,
        );

        paths.len()
    }
    fn run_p2(&self) -> usize {
        let mut paths = Vec::new();

        find_paths_p2(
            &self.graph,
            self.graph.entrance,
            0,
            HashSet::new(),
            vec![],
            &mut paths,
        );

        paths.len()
    }
}

fn find_paths_p1(
    graph: &Graph,
    current: NodeIndex,
    mut visited: HashSet<NodeIndex>,
    mut path: Vec<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);

    match &graph.nodes[current] {
        Node::Exit(_) => {
            paths.push(path);
            return;
        }
        Node::SmallCave(_) => {
            let _ = visited.insert(current);
        }
        _ => (),
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

        find_paths_p1(graph, target, visited.clone(), path.clone(), paths);
    }
}

fn find_paths_p2(
    graph: &Graph,
    current: NodeIndex,
    mut small_visit_count: u8,
    mut visited: HashSet<NodeIndex>,
    mut path: Vec<NodeIndex>,
    paths: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);

    match &graph.nodes[current] {
        Node::Exit(_) => {
            paths.push(path);
            return;
        }
        Node::SmallCave(_) => {
            if !visited.insert(current) {
                small_visit_count += 1;
            }
        }
        _ => (),
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

        find_paths_p2(
            graph,
            target,
            small_visit_count,
            visited.clone(),
            path.clone(),
            paths,
        );
    }
}

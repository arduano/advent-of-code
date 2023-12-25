use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Binary,
};

use shared::*;

const INPUT: &str = day_input!();

// jqt: rhn xhk nvd
// rsh: frs pzl lsr
// xhk: hfx
// cmg: qnr nvd lhk bvb
// rhn: xhk bvb hfx
// bvb: xhk hfx
// pzl: lsr hfx nvd
// qnr: nvd
// ntq: jqt hfx bvb xhk
// nvd: lhk
// lsr: lhk
// rzs: qnr cmg lsr rsh
// frs: qnr lhk lsr

#[derive(Debug)]
struct Component {
    name: String,
    children: Vec<String>,
}

fn parse_input() -> Vec<Component> {
    let mut components = Vec::new();
    for line in INPUT.lines() {
        let (name, children) = line.split_at_str(": ");

        let children = children
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        components.push(Component {
            name: name.to_string(),
            children,
        });
    }
    components
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NodeId(u32);

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NodeId").field(&self.0).finish()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EdgeId(NodeId, NodeId);

impl EdgeId {
    pub fn new(a: NodeId, b: NodeId) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

impl std::fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EdgeId")
            .field(&self.0 .0)
            .field(&self.1 .0)
            .finish()
    }
}

impl std::fmt::Display for EdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.0, self.1))
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    edges: HashSet<NodeId>,
}

#[derive(Debug, Clone)]
struct Graph<N, E> {
    node_id_counter: u32,
    nodes: HashMap<NodeId, Node<N>>,
    edges: HashMap<EdgeId, E>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            node_id_counter: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, data: N) -> NodeId {
        let id = NodeId(self.node_id_counter);
        self.node_id_counter += 1;

        self.nodes.insert(
            id,
            Node {
                data,
                edges: HashSet::new(),
            },
        );

        id
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId, data: E) -> EdgeId {
        let id = EdgeId::new(a, b);

        self.edges.insert(id, data);

        self.nodes.get_mut(&a).unwrap().edges.insert(b);
        self.nodes.get_mut(&b).unwrap().edges.insert(a);

        id
    }

    pub fn map_nodes_edges<N2, E2, F, G>(&self, mut f: F, mut g: G) -> Graph<N2, E2>
    where
        F: FnMut(NodeId, &N) -> N2,
        G: FnMut(EdgeId, &E) -> E2,
    {
        let mut graph = Graph::new();

        for (id, node) in &self.nodes {
            let new_node = Node {
                data: f(*id, &node.data),
                edges: node.edges.clone(),
            };
            graph.nodes.insert(*id, new_node);
        }

        for (id, edge) in &self.edges {
            let new_edge = g(*id, edge);
            graph.edges.insert(*id, new_edge);
        }

        graph
    }

    pub fn iter_edge_node_values<'a>(&'a self, edge: EdgeId) -> impl Iterator<Item = &'a N> {
        [
            &self.nodes.get(&edge.0).unwrap().data,
            &self.nodes.get(&edge.1).unwrap().data,
        ]
        .into_iter()
    }

    fn bfs_walk<F, P: Clone>(&self, start: NodeId, mut f: F, initial: P)
    where
        F: FnMut(NodeId, &N, P) -> P,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_front((start, initial));

        while let Some((node_id, val)) = queue.pop_back() {
            if visited.contains(&node_id) {
                continue;
            }

            visited.insert(node_id);

            let node = self.nodes.get(&node_id).unwrap();

            let next = f(node_id, &node.data, val.clone());

            for &other_node_id in &node.edges {
                if !visited.contains(&other_node_id) {
                    queue.push_front((other_node_id, next.clone()));
                }
            }
        }
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node<N>> {
        self.nodes.get(&id)
    }

    pub fn iter_node_edge_values<'a>(&'a self, node: NodeId) -> impl Iterator<Item = &'a E> {
        self.nodes
            .get(&node)
            .unwrap()
            .edges
            .iter()
            .map(move |other_node_id| {
                let edge_id = EdgeId::new(node, *other_node_id);
                self.edges.get(&edge_id).unwrap()
            })
    }

    pub fn get_node_value(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(&id).map(|node| &node.data)
    }

    pub fn get_node_value_mut(&mut self, id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(&id).map(|node| &mut node.data)
    }

    pub fn iter_neighbors(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.get(&id).unwrap().edges.iter().copied()
    }

    pub fn get_edge(&self, id: EdgeId) -> Option<&E> {
        self.edges.get(&id)
    }

    pub fn get_edge_mut(&mut self, id: EdgeId) -> Option<&mut E> {
        self.edges.get_mut(&id)
    }

    pub fn print_node_values(&self)
    where
        N: std::fmt::Debug,
    {
        for (id, node) in &self.nodes {
            println!("{}: {:?}", id.0, node.data);
        }
    }

    pub fn print_edge_values(&self)
    where
        E: std::fmt::Debug,
    {
        for (id, edge) in &self.edges {
            println!("{}-{}: {:?}", id.0 .0, id.1 .0, edge);
        }
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = (NodeId, &N)> {
        self.nodes.iter().map(|(id, node)| (*id, &node.data))
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (EdgeId, &E)> {
        self.edges.iter().map(|(id, edge)| (*id, edge))
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        self.edges.remove(&id);
        self.nodes.get_mut(&id.0).unwrap().edges.remove(&id.1);
        self.nodes.get_mut(&id.1).unwrap().edges.remove(&id.0);
    }
}

fn build_graph(components: &[Component]) -> Graph<String, ()> {
    let mut graph = Graph::new();

    let mut node_ids = HashMap::new();

    for component in components {
        node_ids
            .entry(&component.name)
            .or_insert_with(|| graph.add_node(component.name.clone()));

        for child in &component.children {
            node_ids
                .entry(child)
                .or_insert_with(|| graph.add_node(child.clone()));
        }
    }

    dbg!(&node_ids);

    for component in components {
        let node_id = node_ids[&component.name];

        for child in &component.children {
            let child_id = node_ids[child];

            graph.add_edge(node_id, child_id, ());
        }
    }

    graph
}

fn as_astar_graph<N, E>(graph: &Graph<N, E>, start: NodeId) -> Graph<u32, ()> {
    let mut astar_graph = graph.map_nodes_edges(|_, _| 0, |_, _| ());
    graph.bfs_walk(
        start,
        |id, _, val| {
            *astar_graph.get_node_value_mut(id).unwrap() = val;
            val + 1
        },
        0,
    );

    astar_graph
}

fn as_astar_edge_uses_graph<N, E>(graph: &Graph<N, E>, start: NodeId) -> Graph<u32, u32> {
    let astar = as_astar_graph(graph, start);
    let mut astar_edge_uses = astar.map_nodes_edges(|_, _| 1, |_, _| 0);

    let mut remaining_heads = BinaryHeap::<(u32, NodeId)>::new();

    for (id, &astar_dist) in astar.iter_nodes() {
        remaining_heads.push((astar_dist, id));
    }

    while let Some((_, node_id)) = remaining_heads.pop() {
        let neighbors = astar.iter_neighbors(node_id);

        let mut smallest_neighbor = None;
        let mut smallest_neighbor_dist = std::u32::MAX;

        for neighbor in neighbors {
            let neighbor_dist = *astar.get_node_value(neighbor).unwrap();

            if neighbor_dist < smallest_neighbor_dist {
                smallest_neighbor = Some(neighbor);
                smallest_neighbor_dist = neighbor_dist;
            }
        }

        let Some(smallest_neighbor) = smallest_neighbor else {
            continue;
        };

        let current_node_value = *astar_edge_uses.get_node_value(node_id).unwrap();

        let edge_id = EdgeId::new(node_id, smallest_neighbor);
        *astar_edge_uses.get_edge_mut(edge_id).unwrap() += current_node_value;
        *astar_edge_uses
            .get_node_value_mut(smallest_neighbor)
            .unwrap() += current_node_value;
    }

    astar_edge_uses
}

fn count_graph_nodes_from<N, E>(graph: &Graph<N, E>, start: NodeId) -> usize {
    let mut count = 0;

    graph.bfs_walk(
        start,
        |_, _, _| {
            count += 1;
            ()
        },
        (),
    );

    count
}

fn part1() {
    let input = parse_input();

    let graph = build_graph(&input);

    let mut edge_values_sum_graph = graph.map_nodes_edges(|_, _| (), |_, _| 0);

    for (id, _) in graph.iter_nodes() {
        let astar_edges = as_astar_edge_uses_graph(&graph, id);

        for (id, val) in astar_edges.iter_edges() {
            *edge_values_sum_graph.get_edge_mut(id).unwrap() += val;
        }
    }

    let mut edge_uses = Vec::new();
    for (id, val) in edge_values_sum_graph.iter_edges() {
        edge_uses.push((val, id));
    }

    edge_uses.sort();
    edge_uses.reverse();

    for (val, id) in &edge_uses {
        println!("{}: {}", id, val);
    }

    let mut node_count = graph.nodes.len();

    let mut resulting_count = 0;

    'outer: for i in 2..node_count {
        for j in 1..i {
            for k in 0..j {
                let (val1, id1) = &edge_uses[i];
                let (val2, id2) = &edge_uses[j];
                let (val3, id3) = &edge_uses[k];

                let mut test_graph = graph.clone();

                test_graph.remove_edge(*id1);
                test_graph.remove_edge(*id2);
                test_graph.remove_edge(*id3);

                let graph_size = count_graph_nodes_from(&test_graph, NodeId(0));

                if graph_size < node_count {
                    println!("Removed {}: {}", id1, val1);
                    println!("Removed {}: {}", id2, val2);
                    println!("Removed {}: {}", id3, val3);
                    resulting_count = graph_size;
                    break 'outer;
                }
            }
        }
    }

    let other_count = node_count - resulting_count;

    let product = other_count * resulting_count;

    println!("Part 1: {}", product)
}

fn part2() {
    println!("Part 2: {}", -1)
}

fn main() {
    part1();
    part2();
}

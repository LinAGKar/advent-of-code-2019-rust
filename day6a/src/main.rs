use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

struct Node<'a> {
    parents: HashSet<&'a str>,
    children: HashSet<&'a str>,
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        Node {
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }
}

struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph {
            nodes: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, parent: &'a str, child: &'a str) {
        self.nodes.entry(parent).or_insert_with(Node::new).children.insert(child);
        self.nodes.entry(child).or_insert_with(Node::new).parents.insert(parent);
    }
    
    fn calc_root(&self) -> &'a str {
        let mut id = self.nodes.keys().next().unwrap();
        while let Some(parent) = self.nodes.get(id).unwrap().parents.iter().next() {
            id = parent;
        }
        id
    }

    fn count_orbits(&self, root: &str) -> i32 {
        let mut orbits = HashMap::new();
        let mut to_count = Vec::new();
        to_count.push(root);
        orbits.insert(root, 0);
        while !to_count.is_empty() {
            to_count = to_count.iter().flat_map(|x| {
                let &this_orbits = orbits.get(x).unwrap();
                self.nodes.get(x).unwrap().children.iter().map(|&y| {
                    orbits.insert(y, this_orbits + 1);
                    y
                }).collect::<Vec<&str>>()
            }).collect();
        }
        orbits.values().fold(0, |acc, x| acc + x)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut graph = Graph::new();
    for i in input.split_whitespace() {
        let mut edge = i.split(')');
        let parent = edge.next().unwrap();
        let child = edge.next().unwrap();
        graph.add_edge(parent, child);
    }
    println!("{}", graph.count_orbits(graph.calc_root()));
}

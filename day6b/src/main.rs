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

    fn parent_of(&self, id: &str) -> &str {
        return self.nodes.get(id).unwrap().parents.iter().next().unwrap()
    }

    fn distance(&self, source: &str, target: &str) -> u32 {
        let mut to_check = HashSet::new();
        let mut checked = HashSet::new();
        let mut distance = 1;
        to_check.insert(source);
        checked.insert(source);
        if source == target {
            return 0;
        }
        loop {
            let mut new_to_check = HashSet::new();
            for i in to_check {
                let node = self.nodes.get(i).unwrap();
                for &j in node.parents.iter().chain(node.children.iter()) {
                    if checked.contains(j) {
                        continue;
                    }
                    if j == target {
                        return distance;
                    }
                    checked.insert(j);
                    new_to_check.insert(j);
                }
            }
            to_check = new_to_check;
            distance += 1;
        }
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
    
    println!("{}", graph.distance(graph.parent_of("YOU"), graph.parent_of("SAN")));
}

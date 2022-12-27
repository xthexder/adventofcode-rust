use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cell::Cell;

#[derive(Clone, Debug)]
struct Node<'main> {
    name: &'main str,
    big: bool,
    visit_count: Cell<u32>,

    neighbors: Vec<usize>,
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Node<'_>) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for Node<'_> {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl <'main> Node<'main> {
    fn new(name: &str) -> Node {
        Node {
            name: name,
            big: name.chars().next().unwrap_or('a').is_ascii_uppercase(),
            visit_count: Cell::new(0),
            neighbors: vec!(),
        }
    }

    fn traverse<'a>(&self, nodes: &[Node<'main>], allow_twice: bool) -> u32 {
        let mut paths = 0;
        let this_twice = allow_twice && !self.big && self.visit_count.get() == 1 && self.name != "start" && self.name != "end";
        if self.big || self.visit_count.get() == 0 || this_twice {
            self.visit_count.set(self.visit_count.get() + 1);

            if self.name == "end" {
                paths += 1;
            }

            for node in self.neighbors.to_vec() {
                paths += nodes[node].traverse(nodes, allow_twice && !this_twice);
            }

            self.visit_count.set(self.visit_count.get() - 1);
        }
        paths
    }
}

fn part1<'main>(root: &Node<'main>, nodes: &[Node<'main>]) -> io::Result<()> {
    let paths = root.traverse(nodes, false);
    println!("Part 1: {}", paths);
    return Ok(())
}
fn part2<'main>(root: &Node<'main>, nodes: &[Node<'main>]) -> io::Result<()> {
    let paths = root.traverse(nodes, true);
    println!("Part 2: {}", paths);
    return Ok(())
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    let mut nodes: Vec<Node> = vec!();
    let connections = contents.lines().filter_map(|line| line.split_once('-')).collect::<Vec<_>>();
    for (parent, child) in &connections {
        if nodes.iter().find(|x| x == parent).is_none() {
            nodes.push(Node::new(parent));
        }
        if nodes.iter().find(|x| x == child).is_none() {
            nodes.push(Node::new(child));
        }
    }
    for (parent_name, child_name) in connections {
        let parent = nodes.iter().position(|x| x == parent_name).unwrap();
        let child = nodes.iter().position(|x| x == child_name).unwrap();
        if !nodes[parent].neighbors.contains(&child) {
            nodes[parent].neighbors.push(child.clone());
        }
        if !nodes[child].neighbors.contains(&parent) {
            nodes[child].neighbors.push(parent.clone());
        }
    }

    let root = nodes.iter().position(|x| x == "start").unwrap();
    part1(&nodes[root], &nodes)?;
    part2(&nodes[root], &nodes)
}

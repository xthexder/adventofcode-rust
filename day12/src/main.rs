use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cell::Cell;

#[derive(Clone, Debug)]
struct Node<'main> {
    name: &'main str,
    big: bool,
    allow_twice: Cell<bool>,
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
            allow_twice: Cell::new(false),
            visit_count: Cell::new(0),
            neighbors: vec!(),
        }
    }

    fn traverse<'a>(&self, nodes: &[Node<'main>], path_list: &'a mut Vec<Vec<&'main str>>, path: &[&'main str]) {
        if self.big || self.visit_count.get() == 0 || (self.allow_twice.get() && self.visit_count.get() == 1) {
            self.visit_count.set(self.visit_count.get() + 1);

            let mut new_path = path.to_vec();
            new_path.push(self.name);

            if self.name == "end" {
                path_list.push(new_path.clone());
            }

            for node in self.neighbors.to_vec() {
                nodes[node].traverse(nodes, path_list, &new_path);
            }

            self.visit_count.set(self.visit_count.get() - 1);
        }
    }
}

fn part1<'main>(root: &Node<'main>, nodes: &[Node<'main>]) -> io::Result<()> {
    let mut paths: Vec<Vec<&'main str>> = vec!();
    root.traverse(nodes, &mut paths, &[]);
    // for path in &paths {
    //     println!("Path: {:?}", path);
    // }
    println!("Part 1: {}", paths.len());
    return Ok(())
}
fn part2<'main>(root: &Node<'main>, nodes: &[Node<'main>]) -> io::Result<()> {
    let mut paths: Vec<Vec<&'main str>> = vec!();
    for node in nodes {
        if !node.big && node.name != "start" && node.name != "end" {
            node.allow_twice.set(true);
            root.traverse(nodes, &mut paths, &[]);
            node.allow_twice.set(false);
        }
    }
    // This is a really slow hack of a solution...
    paths.sort();
    paths.dedup();
    println!("Part 2: {}", paths.len());
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

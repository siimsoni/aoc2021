use crate::parser::{NodeKind, NodeLink, NodeSize};
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct PathItem {
    pub node: NodeKind,
    pub id: Box<[u8]>,
}

impl Hash for PathItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for PathItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PathItem {}

pub fn solve(input: &Vec<NodeLink>) -> usize {
    let mut counter: Vec<u8> = Vec::new();
    counter.push(255); // separator
    let mut links = HashMap::new();
    for link in input {
        links.entry(link.from.clone()).or_insert(Vec::new()).push(link.to.clone());
        links.entry(link.to.clone()).or_insert(Vec::new()).push(link.from.clone());
    }

    let mut result = 0;

    if let Some(paths) = get_paths(&NodeKind::Start, &links, Vec::new(), &mut counter) {
        result += paths;
    }

    result
}

pub fn insert(
    mut set: Vec<PathItem>,
    node: NodeKind,
    counter_as_bytes: &[u8],
) -> Vec<PathItem> {
    set.push(match node {
        NodeKind::Start => PathItem {
            node,
            id: Box::new(*(b"start")),
        },
        NodeKind::End => PathItem {
            node,
            id: Box::new(*(b"end")),
        },
        NodeKind::Regular { ref id, size } => {
            let id = id.clone();
            match size {
                NodeSize::Small => PathItem { node, id },
                NodeSize::Large => PathItem {
                    node,
                    id: Box::from([&id, counter_as_bytes].concat()),
                },
            }
        },
    });
    set
}

/*
 * We need unique byte array to append to IDs
 */
fn increment(counter: &mut Vec<u8>) {
    if let Some(last) = counter.pop() {
        if last == u8::MAX {
            counter.push(last);
            counter.push(0);
        } else {
            counter.push(last + 1);
        }
    } else {
        counter.push(0);
    }
}

fn get_paths(node: &NodeKind, link_map: &HashMap<NodeKind, Vec<NodeKind>>, mut path: Vec<PathItem>, counter: &mut Vec<u8>) -> Option<usize> {
    let path_item = match node {
        NodeKind::End => PathItem{node: node.clone(), id: Box::from(*b"end")},
        NodeKind::Start => PathItem{node: node.clone(), id: Box::from(*b"start")},
        NodeKind::Regular { id, size } => {
            match size {
                NodeSize::Small => PathItem{node: node.clone(), id: id.clone()},
                NodeSize::Large => {
                    increment(counter);
                    let item = PathItem{node: node.clone(), id: Box::from([id, counter.as_slice()].concat())};
                    item
                }
            }
        }
    };
    if path.contains(&path_item) {
        return None;
    }

    path.push(path_item);

    let mut result = 0;

    if let NodeKind::End = node {
        return Some(1);
    }

    if let Some(links) = link_map.get(node) {
       for link in links {
           if let Some(paths) = get_paths(link, link_map, path.clone(), counter) {
               result += paths;
           }
       }
    } else {
        return None;
    }

    Some(result)
}
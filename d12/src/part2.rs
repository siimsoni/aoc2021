use crate::parser::{NodeKind, NodeLink, NodeSize};
use std::collections::{HashMap};

pub fn solve(input: &Vec<NodeLink>) -> usize {
    let mut links = HashMap::new();
    for link in input {
        links.entry(link.from.clone()).or_insert(Vec::new()).push(link.to.clone());
        links.entry(link.to.clone()).or_insert(Vec::new()).push(link.from.clone());
    }

    let mut result = 0;

    if let Some(paths) = get_paths(&NodeKind::Start, &links, &HashMap::new(), false, Vec::new()) {
        result += paths;
    }

    result
}

fn get_paths(node: &NodeKind, link_map: &HashMap<NodeKind, Vec<NodeKind>>, counts: &HashMap<NodeKind,usize>, mut visited_twice: bool, mut stack: Vec<NodeKind>) -> Option<usize> {
    stack.push(node.clone());
    match node {
        NodeKind::Start => {
            if let Some(_) = counts.get(node) {
                return Some(0);
            }
            let mut counts = counts.clone();
            counts.insert(node.clone(), 1);
            return Some(get_links(node, link_map, &counts, visited_twice, stack));
        },
        NodeKind::End => {
            return Some(1)
        },
        NodeKind::Regular {id: _, size} => {
            match size {
                NodeSize::Small => {
                    if let Some(count) = counts.get(node) {
                        if *count >= 1 {
                            if visited_twice {
                                return Some(0);
                            } else {
                                visited_twice = true;
                            }
                        }
                    }
                    let mut counts = counts.clone();
                    *counts.entry(node.clone()).or_insert(0) += 1;
                    return Some(get_links(node, link_map, &counts, visited_twice, stack))
                },
                NodeSize::Large => {
                    return Some(get_links(node, link_map, &counts, visited_twice, stack))
                },
            }
        }
    }
}

fn get_links(node: &NodeKind, link_map: &HashMap<NodeKind, Vec<NodeKind>>, counts: &HashMap<NodeKind,usize>, visited_twice: bool, stack: Vec<NodeKind>) -> usize {
    let mut result = 0;
    if let Some(links) = link_map.get(node) {
        for link in links {
            if let Some(paths) = get_paths(link, link_map, counts, visited_twice, stack.clone()) {
                result += paths;
            }
        }
    }
    result
}

// fn print(stack: Vec<NodeKind>) {
//     for node in stack {
//         match node {
//             NodeKind::Start => print!("start,"),
//             NodeKind::End => print!("end"),
//             NodeKind::Regular { id, size: _ } => {
//                 unsafe {
//                     print!("{},", std::str::from_boxed_utf8_unchecked(id));
//                 }
//             },
//         }
//     }
//     print!("\n");
// }
use std::{
    cmp::Eq,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn bfs<T: Hash + Eq + Copy>(
    start: T,
    end: T,
    neighbor_fn: &dyn Fn(T) -> HashSet<T>,
) -> HashMap<T, Option<T>> {
    let mut parent_map = HashMap::new();
    parent_map.insert(start, None);
    let mut frontier = vec![start];
    let mut new_frontier: Vec<T> = Vec::new();
    while !frontier.is_empty() {
        if parent_map.contains_key(&end) {
            break;
        }
        frontier.iter().for_each(|u| {
            neighbor_fn(*u).iter().for_each(|v| {
                if !parent_map.contains_key(v) {
                    parent_map.insert(*v, Some(*u));
                    new_frontier.push(*v);
                }
            })
        });
        frontier = new_frontier;
        new_frontier = Vec::new();
    }

    parent_map
}

pub fn shortest_path<T: Hash + Eq + Copy>(
    start: T,
    end: T,
    neighbor_fn: &dyn Fn(T) -> HashSet<T>,
) -> Vec<T> {
    let parent_map = bfs(start, end, neighbor_fn);
    let mut path = Vec::new();
    let mut curr = &end;
    loop {
        match parent_map.get(curr) {
            None => {
                break;
            }
            Some(Some(parent)) => {
                path.push(*curr);
                curr = parent;
            }
            Some(None) => {
                path.push(*curr);
                break;
            }
        }
    }
    path.reverse();
    path
}

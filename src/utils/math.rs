use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    pos: (i32, i32),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn heuristic(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

pub fn astar(
    start: (i32, i32),
    goal: (i32, i32),
    obstacles: &std::collections::HashSet<(i32, i32)>,
    grid_w: i32,
    grid_h: i32,
) -> Option<Vec<(i32, i32)>> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), u32> = HashMap::new();

    g_score.insert(start, 0);
    open.push(Node {
        cost: heuristic(start, goal),
        pos: start,
    });

    while let Some(current) = open.pop() {
        if current.pos == goal {
            let mut path = Vec::new();
            let mut cur = goal;
            while cur != start {
                path.push(cur);
                cur = came_from[&cur];
            }
            path.reverse();
            return Some(path);
        }

        let current_g = g_score[&current.pos];
        let neighbors = [
            (current.pos.0, current.pos.1 - 1),
            (current.pos.0, current.pos.1 + 1),
            (current.pos.0 - 1, current.pos.1),
            (current.pos.0 + 1, current.pos.1),
        ];

        for next in neighbors {
            if next.0 < 0 || next.0 >= grid_w || next.1 < 0 || next.1 >= grid_h {
                continue;
            }
            if obstacles.contains(&next) {
                continue;
            }
            let tentative = current_g + 1;
            if tentative < *g_score.get(&next).unwrap_or(&u32::MAX) {
                came_from.insert(next, current.pos);
                g_score.insert(next, tentative);
                open.push(Node {
                    cost: tentative + heuristic(next, goal),
                    pos: next,
                });
            }
        }
    }

    None
}

use std::collections::HashSet;
use crate::utils::math::astar;

pub struct SnakeAi;

impl SnakeAi {
    pub fn greedy_step(
        head: (i32, i32),
        food: (i32, i32),
        obstacles: &HashSet<(i32, i32)>,
        grid_w: i32,
        grid_h: i32,
    ) -> Option<(i32, i32)> {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut best_dir = None;
        let mut best_dist = i32::MAX;

        for dir in dirs {
            let next = (head.0 + dir.0, head.1 + dir.1);
            if next.0 < 0 || next.0 >= grid_w || next.1 < 0 || next.1 >= grid_h {
                continue;
            }
            if obstacles.contains(&next) {
                continue;
            }
            let dist = (next.0 - food.0).abs() + (next.1 - food.1).abs();
            if dist < best_dist {
                best_dist = dist;
                best_dir = Some(dir);
            }
        }

        best_dir
    }

    pub fn astar_step(
        head: (i32, i32),
        food: (i32, i32),
        obstacles: &HashSet<(i32, i32)>,
        grid_w: i32,
        grid_h: i32,
    ) -> Option<(i32, i32)> {
        // Try A* first
        if let Some(path) = astar(head, food, obstacles, grid_w, grid_h) {
            if let Some(first) = path.first() {
                return Some((first.0 - head.0, first.1 - head.1));
            }
        }

        // Fallback to greedy
        Self::greedy_step(head, food, obstacles, grid_w, grid_h)
    }
}

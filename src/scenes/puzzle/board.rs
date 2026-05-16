pub const GRID: usize = 4;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PuzzleState {
    Playing,
    Won,
    Lost,
}

#[derive(Clone)]
pub struct TileMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub merged: bool,
    pub value: u32,
}

pub struct MoveResult {
    pub tile_moves: Vec<TileMove>,
    pub new_tile: ((usize, usize), u32),
    pub score_gained: u32,
}

pub struct Board {
    pub cells: [[u32; GRID]; GRID],
    pub score: u32,
    pub best_score: u32,
    pub state: PuzzleState,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            cells: [[0; GRID]; GRID],
            score: 0,
            best_score: 0,
            state: PuzzleState::Playing,
        };
        board.spawn_random();
        board.spawn_random();
        board
    }

    pub fn reset(&mut self) {
        self.cells = [[0; GRID]; GRID];
        self.score = 0;
        self.state = PuzzleState::Playing;
        self.spawn_random();
        self.spawn_random();
    }

    pub fn slide(&mut self, dir: Direction) -> Option<MoveResult> {
        let mut moves = Vec::new();
        let mut score_gained = 0;
        let mut moved = false;

        match dir {
            Direction::Left => {
                for r in 0..GRID {
                    let (changed, line_moves, line_score) = slide_line_left(&self.cells[r], r, dir);
                    if changed {
                        moved = true;
                        for m in &line_moves {
                            self.cells[m.to.1][m.to.0] = m.value;
                        }
                        // Clear source positions that aren't destinations
                        for c in 0..GRID {
                            let is_dest = line_moves.iter().any(|m| m.to == (c, r));
                            if !is_dest {
                                self.cells[r][c] = 0;
                            }
                        }
                    }
                    moves.extend(line_moves);
                    score_gained += line_score;
                }
            }
            Direction::Right => {
                for r in 0..GRID {
                    let (changed, line_moves, line_score) = slide_line_right(&self.cells[r], r, dir);
                    if changed {
                        moved = true;
                        for m in &line_moves {
                            self.cells[m.to.1][m.to.0] = m.value;
                        }
                        for c in 0..GRID {
                            let is_dest = line_moves.iter().any(|m| m.to == (c, r));
                            if !is_dest {
                                self.cells[r][c] = 0;
                            }
                        }
                    }
                    moves.extend(line_moves);
                    score_gained += line_score;
                }
            }
            Direction::Up => {
                for c in 0..GRID {
                    let col: [u32; GRID] = [self.cells[0][c], self.cells[1][c], self.cells[2][c], self.cells[3][c]];
                    let (changed, line_moves, line_score) = slide_line_up(&col, c, dir);
                    if changed {
                        moved = true;
                        for m in &line_moves {
                            self.cells[m.to.1][m.to.0] = m.value;
                        }
                        for r in 0..GRID {
                            let is_dest = line_moves.iter().any(|m| m.to == (c, r));
                            if !is_dest {
                                self.cells[r][c] = 0;
                            }
                        }
                    }
                    moves.extend(line_moves);
                    score_gained += line_score;
                }
            }
            Direction::Down => {
                for c in 0..GRID {
                    let col: [u32; GRID] = [self.cells[0][c], self.cells[1][c], self.cells[2][c], self.cells[3][c]];
                    let (changed, line_moves, line_score) = slide_line_down(&col, c, dir);
                    if changed {
                        moved = true;
                        for m in &line_moves {
                            self.cells[m.to.1][m.to.0] = m.value;
                        }
                        for r in 0..GRID {
                            let is_dest = line_moves.iter().any(|m| m.to == (c, r));
                            if !is_dest {
                                self.cells[r][c] = 0;
                            }
                        }
                    }
                    moves.extend(line_moves);
                    score_gained += line_score;
                }
            }
        }

        if !moved {
            return None;
        }

        self.score += score_gained;
        if self.score > self.best_score {
            self.best_score = self.score;
        }

        let new_tile = self.spawn_random();
        self.check_state();

        Some(MoveResult {
            tile_moves: moves,
            new_tile,
            score_gained,
        })
    }

    fn spawn_random(&mut self) -> ((usize, usize), u32) {
        let mut empty = Vec::new();
        for r in 0..GRID {
            for c in 0..GRID {
                if self.cells[r][c] == 0 {
                    empty.push((r, c));
                }
            }
        }
        if empty.is_empty() {
            return ((0, 0), 0);
        }
        let idx = macroquad::rand::gen_range(0, empty.len());
        let (r, c) = empty[idx];
        let value = if macroquad::rand::gen_range(0.0, 1.0) < 0.9 { 2 } else { 4 };
        self.cells[r][c] = value;
        ((c, r), value)
    }

    fn check_state(&mut self) {
        // Check for 2048
        for r in 0..GRID {
            for c in 0..GRID {
                if self.cells[r][c] >= 2048 {
                    self.state = PuzzleState::Won;
                    return;
                }
            }
        }
        // Check for any moves possible
        for r in 0..GRID {
            for c in 0..GRID {
                if self.cells[r][c] == 0 {
                    return; // Still have empty cells
                }
                if c + 1 < GRID && self.cells[r][c] == self.cells[r][c + 1] {
                    return; // Can merge horizontally
                }
                if r + 1 < GRID && self.cells[r][c] == self.cells[r + 1][c] {
                    return; // Can merge vertically
                }
            }
        }
        self.state = PuzzleState::Lost;
    }
}

fn slide_line_left(line: &[u32; GRID], row: usize, _dir: Direction) -> (bool, Vec<TileMove>, u32) {
    let mut tiles: Vec<u32> = line.iter().copied().filter(|&x| x != 0).collect();
    let mut moves = Vec::new();
    let mut score = 0;
    let mut merged = vec![false; tiles.len()];

    // Merge pass
    let mut i = 0;
    while i + 1 < tiles.len() {
        if tiles[i] == tiles[i + 1] && !merged[i] {
            tiles[i] *= 2;
            score += tiles[i];
            merged[i] = true;
            tiles.remove(i + 1);
            merged.remove(i + 1);
        }
        i += 1;
    }

    // Build result
    let mut result = [0u32; GRID];
    for (i, &val) in tiles.iter().enumerate() {
        result[i] = val;
    }

    // Track moves
    let mut src_idx = 0;
    for (dst_col, &val) in result.iter().enumerate() {
        if val == 0 {
            continue;
        }
        // Find source
        while src_idx < GRID && line[src_idx] == 0 {
            src_idx += 1;
        }
        let is_merge = dst_col < tiles.len() && merged.get(dst_col).copied().unwrap_or(false);
        moves.push(TileMove {
            from: (src_idx, row),
            to: (dst_col, row),
            merged: is_merge,
            value: val,
        });
        src_idx += 1;
    }

    let changed = result != *line;
    (changed, moves, score)
}

fn slide_line_right(line: &[u32; GRID], row: usize, _dir: Direction) -> (bool, Vec<TileMove>, u32) {
    let mut tiles: Vec<u32> = line.iter().copied().filter(|&x| x != 0).collect();
    let mut moves = Vec::new();
    let mut score = 0;
    let mut merged = vec![false; tiles.len()];

    // Merge pass (right to left)
    let mut i = tiles.len();
    while i > 1 {
        i -= 1;
        if tiles[i] == tiles[i - 1] && !merged[i] {
            tiles[i] *= 2;
            score += tiles[i];
            merged[i] = true;
            tiles.remove(i - 1);
            merged.remove(i - 1);
            i -= 1;
        }
    }

    let mut result = [0u32; GRID];
    let offset = GRID - tiles.len();
    for (i, &val) in tiles.iter().enumerate() {
        result[offset + i] = val;
    }

    let mut src_idx = GRID;
    for dst_col in (0..GRID).rev() {
        if result[dst_col] == 0 {
            continue;
        }
        src_idx -= 1;
        while src_idx > 0 && line[src_idx] == 0 {
            src_idx -= 1;
        }
        let merge_idx = dst_col - offset;
        let is_merge = merged.get(merge_idx).copied().unwrap_or(false);
        moves.push(TileMove {
            from: (src_idx, row),
            to: (dst_col, row),
            merged: is_merge,
            value: result[dst_col],
        });
    }

    let changed = result != *line;
    (changed, moves, score)
}

fn slide_line_up(col: &[u32; GRID], col_idx: usize, _dir: Direction) -> (bool, Vec<TileMove>, u32) {
    let mut tiles: Vec<u32> = col.iter().copied().filter(|&x| x != 0).collect();
    let mut moves = Vec::new();
    let mut score = 0;
    let mut merged = vec![false; tiles.len()];

    let mut i = 0;
    while i + 1 < tiles.len() {
        if tiles[i] == tiles[i + 1] && !merged[i] {
            tiles[i] *= 2;
            score += tiles[i];
            merged[i] = true;
            tiles.remove(i + 1);
            merged.remove(i + 1);
        }
        i += 1;
    }

    let mut result = [0u32; GRID];
    for (i, &val) in tiles.iter().enumerate() {
        result[i] = val;
    }

    let mut src_idx = 0;
    for (dst_row, &val) in result.iter().enumerate() {
        if val == 0 {
            continue;
        }
        while src_idx < GRID && col[src_idx] == 0 {
            src_idx += 1;
        }
        let is_merge = merged.get(dst_row).copied().unwrap_or(false);
        moves.push(TileMove {
            from: (col_idx, src_idx),
            to: (col_idx, dst_row),
            merged: is_merge,
            value: val,
        });
        src_idx += 1;
    }

    let changed = result != *col;
    (changed, moves, score)
}

fn slide_line_down(col: &[u32; GRID], col_idx: usize, _dir: Direction) -> (bool, Vec<TileMove>, u32) {
    let mut tiles: Vec<u32> = col.iter().copied().filter(|&x| x != 0).collect();
    let mut moves = Vec::new();
    let mut score = 0;
    let mut merged = vec![false; tiles.len()];

    let mut i = tiles.len();
    while i > 1 {
        i -= 1;
        if tiles[i] == tiles[i - 1] && !merged[i] {
            tiles[i] *= 2;
            score += tiles[i];
            merged[i] = true;
            tiles.remove(i - 1);
            merged.remove(i - 1);
            i -= 1;
        }
    }

    let mut result = [0u32; GRID];
    let offset = GRID - tiles.len();
    for (i, &val) in tiles.iter().enumerate() {
        result[offset + i] = val;
    }

    let mut src_idx = GRID;
    for dst_row in (0..GRID).rev() {
        if result[dst_row] == 0 {
            continue;
        }
        src_idx -= 1;
        while src_idx > 0 && col[src_idx] == 0 {
            src_idx -= 1;
        }
        let merge_idx = dst_row - offset;
        let is_merge = merged.get(merge_idx).copied().unwrap_or(false);
        moves.push(TileMove {
            from: (col_idx, src_idx),
            to: (col_idx, dst_row),
            merged: is_merge,
            value: result[dst_row],
        });
    }

    let changed = result != *col;
    (changed, moves, score)
}

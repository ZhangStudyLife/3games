use macroquad::prelude::*;
use crate::scenes::tank::game::WallType;

pub struct WallData {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub wall_type: WallType,
}

pub struct Level {
    pub player_spawn: Vec2,
    pub enemy_spawns: Vec<Vec2>,
    pub walls: Vec<WallData>,
    pub base_pos: Vec2,
    pub max_enemies: u32,
    pub total_enemies: u32,
}

impl Level {
    pub fn load(level: u8) -> Self {
        match level {
            1 => Self::level_1(),
            2 => Self::level_2(),
            3 => Self::level_3(),
            _ => Self::level_1(),
        }
    }

    fn level_1() -> Self {
        let mut walls = Vec::new();
        // Brick walls in the middle area
        for i in 0..6 {
            let x = 120.0 + i as f32 * 130.0;
            walls.push(WallData { x, y: 200.0, w: 40.0, h: 40.0, wall_type: WallType::Brick });
            walls.push(WallData { x, y: 400.0, w: 40.0, h: 40.0, wall_type: WallType::Brick });
        }
        // Vertical walls for cover
        for i in 0..3 {
            let x = 200.0 + i as f32 * 250.0;
            walls.push(WallData { x, y: 300.0, w: 20.0, h: 60.0, wall_type: WallType::Brick });
        }
        // Base protection walls
        walls.push(WallData { x: 420.0, y: 650.0, w: 20.0, h: 50.0, wall_type: WallType::Brick });
        walls.push(WallData { x: 540.0, y: 650.0, w: 20.0, h: 50.0, wall_type: WallType::Brick });

        Self {
            player_spawn: Vec2::new(480.0, 550.0),
            enemy_spawns: vec![
                Vec2::new(100.0, 60.0),
                Vec2::new(480.0, 60.0),
                Vec2::new(860.0, 60.0),
            ],
            walls,
            base_pos: Vec2::new(480.0, 700.0),
            max_enemies: 3,
            total_enemies: 6,
        }
    }

    fn level_2() -> Self {
        let mut walls = Vec::new();
        // L-shaped wall structures
        for i in 0..5 {
            let x = 100.0 + i as f32 * 170.0;
            walls.push(WallData { x, y: 180.0, w: 60.0, h: 20.0, wall_type: WallType::Brick });
            walls.push(WallData { x, y: 180.0, w: 20.0, h: 60.0, wall_type: WallType::Brick });
            walls.push(WallData { x, y: 450.0, w: 60.0, h: 20.0, wall_type: WallType::Brick });
            walls.push(WallData { x, y: 450.0, w: 20.0, h: 60.0, wall_type: WallType::Brick });
        }
        // Steel walls
        walls.push(WallData { x: 250.0, y: 320.0, w: 60.0, h: 60.0, wall_type: WallType::Steel });
        walls.push(WallData { x: 650.0, y: 320.0, w: 60.0, h: 60.0, wall_type: WallType::Steel });

        Self {
            player_spawn: Vec2::new(480.0, 550.0),
            enemy_spawns: vec![
                Vec2::new(100.0, 60.0),
                Vec2::new(480.0, 60.0),
                Vec2::new(860.0, 60.0),
            ],
            walls,
            base_pos: Vec2::new(480.0, 700.0),
            max_enemies: 4,
            total_enemies: 8,
        }
    }

    fn level_3() -> Self {
        let mut walls = Vec::new();
        // Maze-like layout with more walls
        for r in 0..7 {
            for c in 0..9 {
                if (r + c) % 2 == 0 {
                    let x = 80.0 + c as f32 * 100.0;
                    let y = 120.0 + r as f32 * 80.0;
                    if rand::gen_range(0.0, 1.0) < 0.6 {
                        walls.push(WallData {
                            x, y, w: 40.0, h: 40.0,
                            wall_type: if rand::gen_range(0.0, 1.0) < 0.25 { WallType::Steel } else { WallType::Brick },
                        });
                    }
                }
            }
        }
        // Protect base with steel
        walls.push(WallData { x: 420.0, y: 650.0, w: 20.0, h: 50.0, wall_type: WallType::Steel });
        walls.push(WallData { x: 540.0, y: 650.0, w: 20.0, h: 50.0, wall_type: WallType::Steel });

        Self {
            player_spawn: Vec2::new(480.0, 550.0),
            enemy_spawns: vec![
                Vec2::new(100.0, 60.0),
                Vec2::new(480.0, 60.0),
                Vec2::new(860.0, 60.0),
            ],
            walls,
            base_pos: Vec2::new(480.0, 700.0),
            max_enemies: 5,
            total_enemies: 10,
        }
    }
}

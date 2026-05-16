use macroquad::prelude::*;
use crate::core::input::InputState;
use crate::scenes::tank::level::Level;
use crate::scenes::tank::ai::EnemyAi;
use crate::scenes::tank::collision::CollisionSystem;

#[derive(Clone, Copy, PartialEq)]
pub enum TankState {
    Playing,
    Paused,
    LevelComplete,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TankType {
    Basic,
    Fast,
    Heavy,
}

#[derive(Clone)]
pub struct Tank {
    pub pos: Vec2,
    pub direction: Direction,
    pub speed: f32,
    pub health: u32,
    pub max_health: u32,
    pub fire_cooldown: f32,
    pub fire_timer: f32,
    pub is_player: bool,
    pub tank_type: TankType,
    pub size: f32,
}

#[derive(Clone)]
pub struct Bullet {
    pub pos: Vec2,
    pub direction: Direction,
    pub speed: f32,
    pub owner_is_player: bool,
    pub damage: u32,
    pub alive: bool,
}

#[derive(Clone)]
pub struct Wall {
    pub rect: Rect,
    pub wall_type: WallType,
    pub health: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WallType {
    Brick,
    Steel,
}

pub struct TankGame {
    pub level: Level,
    pub player: Tank,
    pub enemies: Vec<Tank>,
    pub bullets: Vec<Bullet>,
    pub walls: Vec<Wall>,
    pub base_alive: bool,
    pub state: TankState,
    pub current_level: u8,
    pub enemies_remaining: u32,
    pub enemy_spawn_timer: f32,
    pub enemy_ais: Vec<EnemyAi>,
    pub explosions: Vec<(Vec2, f32)>,
    pub time: f32,
}

impl TankGame {
    pub fn new(level_num: u8) -> Self {
        let level = Level::load(level_num);
        let player_spawn = level.player_spawn;
        let base_pos = level.base_pos;
        let total_enemies = level.total_enemies;

        let player = Tank {
            pos: player_spawn,
            direction: Direction::Up,
            speed: 120.0,
            health: 3,
            max_health: 3,
            fire_cooldown: 0.4,
            fire_timer: 0.0,
            is_player: true,
            tank_type: TankType::Basic,
            size: 24.0,
        };

        let walls: Vec<Wall> = level.walls.iter().map(|w| Wall {
            rect: Rect::new(w.x, w.y, w.w, w.h),
            wall_type: w.wall_type,
            health: if w.wall_type == WallType::Steel { i32::MAX } else { 2 },
        }).collect();

        Self {
            level,
            player,
            enemies: Vec::new(),
            bullets: Vec::new(),
            walls,
            base_alive: true,
            state: TankState::Playing,
            current_level: level_num,
            enemies_remaining: total_enemies,
            enemy_spawn_timer: 2.0,
            enemy_ais: Vec::new(),
            explosions: Vec::new(),
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputState) {
        if self.state != TankState::Playing {
            return;
        }

        self.time += dt;

        // Player movement
        let mut moved = false;
        let mut new_dir = self.player.direction;
        if input.is_down(KeyCode::W) || input.is_down(KeyCode::Up) {
            new_dir = Direction::Up;
            moved = true;
        } else if input.is_down(KeyCode::S) || input.is_down(KeyCode::Down) {
            new_dir = Direction::Down;
            moved = true;
        } else if input.is_down(KeyCode::A) || input.is_down(KeyCode::Left) {
            new_dir = Direction::Left;
            moved = true;
        } else if input.is_down(KeyCode::D) || input.is_down(KeyCode::Right) {
            new_dir = Direction::Right;
            moved = true;
        }

        self.player.direction = new_dir;
        if moved {
            let new_pos = self.player.pos + new_dir.to_vec2() * self.player.speed * dt;
            if !CollisionSystem::tank_wall_collision(new_pos, self.player.size, &self.walls)
                && !CollisionSystem::tank_base_collision(new_pos, self.player.size, self.level.base_pos, 32.0)
            {
                self.player.pos = new_pos;
            }
        }

        // Player shooting
        self.player.fire_timer -= dt;
        if input.is_down(KeyCode::Space) && self.player.fire_timer <= 0.0 {
            self.player.fire_timer = self.player.fire_cooldown;
            let bullet_pos = self.player.pos + self.player.direction.to_vec2() * (self.player.size + 4.0);
            self.bullets.push(Bullet {
                pos: bullet_pos,
                direction: self.player.direction,
                speed: 300.0,
                owner_is_player: true,
                damage: 1,
                alive: true,
            });
        }

        // Spawn enemies
        self.enemy_spawn_timer -= dt;
        if self.enemy_spawn_timer <= 0.0 && self.enemies.len() < self.level.max_enemies as usize && self.enemies_remaining > 0 {
            self.spawn_enemy();
            self.enemy_spawn_timer = 3.0;
        }

        // Update enemies
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            if let Some(ai) = self.enemy_ais.get_mut(i) {
                ai.update(enemy, self.player.pos, self.level.base_pos, &self.walls, dt);
            }
            enemy.fire_timer -= dt;
            if enemy.fire_timer <= 0.0 && rand::gen_range(0.0, 1.0) < 0.02 {
                enemy.fire_timer = enemy.fire_cooldown;
                let bullet_pos = enemy.pos + enemy.direction.to_vec2() * (enemy.size + 4.0);
                self.bullets.push(Bullet {
                    pos: bullet_pos,
                    direction: enemy.direction,
                    speed: 200.0,
                    owner_is_player: false,
                    damage: 1,
                    alive: true,
                });
            }
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.pos += bullet.direction.to_vec2() * bullet.speed * dt;
        }

        // Bullet-wall collision
        for bullet in &mut self.bullets {
            if !bullet.alive {
                continue;
            }
            for wall in &mut self.walls {
                if wall.health <= 0 {
                    continue;
                }
                if bullet.pos.x >= wall.rect.x && bullet.pos.x <= wall.rect.x + wall.rect.w
                    && bullet.pos.y >= wall.rect.y && bullet.pos.y <= wall.rect.y + wall.rect.h
                {
                    bullet.alive = false;
                    if wall.wall_type == WallType::Brick {
                        wall.health -= bullet.damage as i32;
                        if wall.health <= 0 {
                            self.explosions.push((Vec2::new(wall.rect.x + wall.rect.w / 2.0, wall.rect.y + wall.rect.h / 2.0), 0.3));
                        }
                    } else {
                        self.explosions.push((bullet.pos, 0.2));
                    }
                    break;
                }
            }
        }

        // Bullet-tank collision
        for bullet in &mut self.bullets {
            if !bullet.alive {
                continue;
            }
            if bullet.owner_is_player {
                for enemy in &mut self.enemies {
                    if enemy.health > 0 && CollisionSystem::point_in_rect(bullet.pos, enemy.pos, enemy.size) {
                        bullet.alive = false;
                        enemy.health = enemy.health.saturating_sub(bullet.damage);
                        if enemy.health == 0 {
                            self.explosions.push((enemy.pos, 0.5));
                        }
                        break;
                    }
                }
            } else {
                if CollisionSystem::point_in_rect(bullet.pos, self.player.pos, self.player.size) {
                    bullet.alive = false;
                    self.player.health = self.player.health.saturating_sub(bullet.damage);
                    if self.player.health == 0 {
                        self.explosions.push((self.player.pos, 0.5));
                        self.state = TankState::GameOver;
                    }
                }
            }
        }

        // Bullet-base collision
        for bullet in &mut self.bullets {
            if !bullet.alive || bullet.owner_is_player {
                continue;
            }
            let base_rect = Rect::new(self.level.base_pos.x - 16.0, self.level.base_pos.y - 16.0, 32.0, 32.0);
            if bullet.pos.x >= base_rect.x && bullet.pos.x <= base_rect.x + base_rect.w
                && bullet.pos.y >= base_rect.y && bullet.pos.y <= base_rect.y + base_rect.h
            {
                bullet.alive = false;
                self.base_alive = false;
                self.explosions.push((self.level.base_pos, 0.8));
                self.state = TankState::GameOver;
            }
        }

        // Remove dead bullets and enemies
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.health > 0);
        self.enemy_ais.truncate(self.enemies.len());

        // Check level complete
        if self.enemies.is_empty() && self.enemies_remaining == 0 {
            self.state = TankState::LevelComplete;
        }

        // Update explosions
        for exp in &mut self.explosions {
            exp.1 -= dt;
        }
        self.explosions.retain(|e| e.1 > 0.0);
    }

    fn spawn_enemy(&mut self) {
        let spawn_idx = rand::gen_range(0, self.level.enemy_spawns.len());
        let spawn = self.level.enemy_spawns[spawn_idx];

        let (tank_type, speed, health, cooldown) = match rand::gen_range(0, 3) {
            0 => (TankType::Basic, 80.0, 1, 1.5),
            1 => (TankType::Fast, 150.0, 1, 0.8),
            _ => (TankType::Heavy, 50.0, 3, 2.0),
        };

        self.enemies.push(Tank {
            pos: spawn,
            direction: Direction::Down,
            speed,
            health,
            max_health: health,
            fire_cooldown: cooldown,
            fire_timer: cooldown,
            is_player: false,
            tank_type,
            size: 24.0,
        });
        self.enemy_ais.push(EnemyAi::new());
        self.enemies_remaining -= 1;
    }

    pub fn update_ai_only(&mut self, dt: f32) {
        if self.state != TankState::Playing {
            return;
        }

        self.time += dt;

        // Spawn enemies
        self.enemy_spawn_timer -= dt;
        if self.enemy_spawn_timer <= 0.0 && self.enemies.len() < self.level.max_enemies as usize && self.enemies_remaining > 0 {
            self.spawn_enemy();
            self.enemy_spawn_timer = 3.0;
        }

        // Update enemies
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            if let Some(ai) = self.enemy_ais.get_mut(i) {
                ai.update(enemy, self.player.pos, self.level.base_pos, &self.walls, dt);
            }
            enemy.fire_timer -= dt;
            if enemy.fire_timer <= 0.0 && rand::gen_range(0.0, 1.0) < 0.02 {
                enemy.fire_timer = enemy.fire_cooldown;
                let bullet_pos = enemy.pos + enemy.direction.to_vec2() * (enemy.size + 4.0);
                self.bullets.push(Bullet {
                    pos: bullet_pos,
                    direction: enemy.direction,
                    speed: 200.0,
                    owner_is_player: false,
                    damage: 1,
                    alive: true,
                });
            }
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.pos += bullet.direction.to_vec2() * bullet.speed * dt;
        }

        // Bullet-wall collision
        for bullet in &mut self.bullets {
            if !bullet.alive {
                continue;
            }
            for wall in &mut self.walls {
                if wall.health <= 0 {
                    continue;
                }
                if bullet.pos.x >= wall.rect.x && bullet.pos.x <= wall.rect.x + wall.rect.w
                    && bullet.pos.y >= wall.rect.y && bullet.pos.y <= wall.rect.y + wall.rect.h
                {
                    bullet.alive = false;
                    if wall.wall_type == WallType::Brick {
                        wall.health -= bullet.damage as i32;
                        if wall.health <= 0 {
                            self.explosions.push((Vec2::new(wall.rect.x + wall.rect.w / 2.0, wall.rect.y + wall.rect.h / 2.0), 0.3));
                        }
                    } else {
                        self.explosions.push((bullet.pos, 0.2));
                    }
                    break;
                }
            }
        }

        // Bullet-tank collision
        for bullet in &mut self.bullets {
            if !bullet.alive {
                continue;
            }
            if bullet.owner_is_player {
                for enemy in &mut self.enemies {
                    if enemy.health > 0 && CollisionSystem::point_in_rect(bullet.pos, enemy.pos, enemy.size) {
                        bullet.alive = false;
                        enemy.health = enemy.health.saturating_sub(bullet.damage);
                        if enemy.health == 0 {
                            self.explosions.push((enemy.pos, 0.5));
                        }
                        break;
                    }
                }
            } else {
                if CollisionSystem::point_in_rect(bullet.pos, self.player.pos, self.player.size) {
                    bullet.alive = false;
                    self.player.health = self.player.health.saturating_sub(bullet.damage);
                    if self.player.health == 0 {
                        self.explosions.push((self.player.pos, 0.5));
                        self.state = TankState::GameOver;
                    }
                }
            }
        }

        // Bullet-base collision
        for bullet in &mut self.bullets {
            if !bullet.alive || bullet.owner_is_player {
                continue;
            }
            let base_rect = Rect::new(self.level.base_pos.x - 16.0, self.level.base_pos.y - 16.0, 32.0, 32.0);
            if bullet.pos.x >= base_rect.x && bullet.pos.x <= base_rect.x + base_rect.w
                && bullet.pos.y >= base_rect.y && bullet.pos.y <= base_rect.y + base_rect.h
            {
                bullet.alive = false;
                self.base_alive = false;
                self.explosions.push((self.level.base_pos, 0.8));
                self.state = TankState::GameOver;
            }
        }

        // Remove dead bullets and enemies
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.health > 0);
        self.enemy_ais.truncate(self.enemies.len());

        // Check level complete
        if self.enemies.is_empty() && self.enemies_remaining == 0 {
            self.state = TankState::LevelComplete;
        }

        // Update explosions
        for exp in &mut self.explosions {
            exp.1 -= dt;
        }
        self.explosions.retain(|e| e.1 > 0.0);
    }
}

use macroquad::prelude::*;

pub const BG_DARK: Color = Color::new(0.1, 0.1, 0.12, 1.0);
pub const BG_MEDIUM: Color = Color::new(0.15, 0.15, 0.18, 1.0);
pub const BG_LIGHT: Color = Color::new(0.2, 0.2, 0.23, 1.0);

pub const ACCENT_BLUE: Color = Color::new(0.3, 0.5, 0.9, 1.0);
pub const ACCENT_GREEN: Color = Color::new(0.3, 0.8, 0.4, 1.0);
pub const ACCENT_RED: Color = Color::new(0.9, 0.3, 0.3, 1.0);
pub const ACCENT_YELLOW: Color = Color::new(0.95, 0.8, 0.2, 1.0);
pub const ACCENT_ORANGE: Color = Color::new(0.95, 0.6, 0.2, 1.0);
pub const ACCENT_PURPLE: Color = Color::new(0.6, 0.3, 0.9, 1.0);

pub const TEXT_PRIMARY: Color = Color::new(0.95, 0.95, 0.95, 1.0);
pub const TEXT_SECONDARY: Color = Color::new(0.7, 0.7, 0.75, 1.0);
pub const TEXT_DIM: Color = Color::new(0.5, 0.5, 0.55, 1.0);

pub const SNAKE_HEAD: Color = Color::new(0.2, 0.7, 0.3, 1.0);
pub const SNAKE_BODY: Color = Color::new(0.3, 0.8, 0.4, 1.0);
pub const SNAKE_TAIL: Color = Color::new(0.5, 0.9, 0.6, 1.0);
pub const FOOD_COLOR: Color = Color::new(0.9, 0.3, 0.3, 1.0);

pub const TILE_COLORS: &[(u32, Color)] = &[
    (2, Color::new(0.93, 0.90, 0.86, 1.0)),
    (4, Color::new(0.93, 0.88, 0.78, 1.0)),
    (8, Color::new(0.95, 0.69, 0.47, 1.0)),
    (16, Color::new(0.96, 0.58, 0.39, 1.0)),
    (32, Color::new(0.96, 0.49, 0.37, 1.0)),
    (64, Color::new(0.96, 0.37, 0.23, 1.0)),
    (128, Color::new(0.93, 0.81, 0.45, 1.0)),
    (256, Color::new(0.93, 0.80, 0.38, 1.0)),
    (512, Color::new(0.93, 0.78, 0.31, 1.0)),
    (1024, Color::new(0.93, 0.77, 0.25, 1.0)),
    (2048, Color::new(0.93, 0.76, 0.18, 1.0)),
];

pub const TILE_BG: Color = Color::new(0.80, 0.76, 0.70, 1.0);
pub const BOARD_BG: Color = Color::new(0.73, 0.69, 0.64, 1.0);

pub const TANK_PLAYER: Color = Color::new(0.3, 0.7, 0.3, 1.0);
pub const TANK_ENEMY: Color = Color::new(0.8, 0.3, 0.3, 1.0);
pub const TANK_HEAVY: Color = Color::new(0.7, 0.5, 0.2, 1.0);
pub const BULLET_COLOR: Color = Color::new(1.0, 1.0, 0.8, 1.0);
pub const WALL_BRICK: Color = Color::new(0.8, 0.5, 0.3, 1.0);
pub const WALL_STEEL: Color = Color::new(0.7, 0.7, 0.75, 1.0);
pub const WATER_COLOR: Color = Color::new(0.2, 0.4, 0.8, 1.0);
pub const BASE_COLOR: Color = Color::new(0.9, 0.8, 0.2, 1.0);

pub fn tile_color(value: u32) -> Color {
    for &(v, c) in TILE_COLORS.iter().rev() {
        if value >= v {
            return c;
        }
    }
    TILE_COLORS[0].1
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

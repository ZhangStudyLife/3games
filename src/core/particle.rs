use macroquad::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
    pub color: Color,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn emit(&mut self, pos: Vec2, count: u32, speed_range: (f32, f32), life_range: (f32, f32), size_range: (f32, f32), colors: &[Color]) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(speed_range.0, speed_range.1);
            let life = rand::gen_range(life_range.0, life_range.1);
            let size = rand::gen_range(size_range.0, size_range.1);
            let color = colors[rand::gen_range(0, colors.len())];

            self.particles.push(Particle {
                pos,
                vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                life,
                max_life: life,
                size,
                color,
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.vel *= 0.98;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self) {
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(p.pos.x, p.pos.y, p.size * alpha, color);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }
}

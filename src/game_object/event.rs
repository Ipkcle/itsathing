use ggez::graphics::{Vector2};

#[derive(Clone, Copy)]
pub enum Event {
    Collision {
        penetration: Vector2,
        elasticity: f32,
    },
    NoCollision,
    Impulse(Vector2),
    Damage(i32),
    None,
}

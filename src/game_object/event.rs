use ggez::graphics::Vector2;

#[derive(Clone, Copy)]
pub struct Collision {
    penetration: Vector2,
    elasticity: f32,
}

#[derive(Clone, Copy)]
pub enum Event {
    Collision(Collision),
    Impulse(Vector2),
    Damage(i32),
}

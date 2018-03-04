use ggez::graphics::Vector2;

#[derive(Clone, Copy)]
pub enum Event {
    Impulse(Vector2),
    Damage(i32),
}

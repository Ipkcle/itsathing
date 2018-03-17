use ggez::graphics::Vector2;
use ggez::graphics::Point2;

#[derive(Clone, Copy)]
pub enum Event {
    ImpulseFrom { 
        from: Point2,
        magnitude: f32,
    },
    Impulse(Vector2),
    Damage(i32),
}

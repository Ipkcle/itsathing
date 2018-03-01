use super::Object;
use super::event::Event;
use ggez::graphics::Vector2;

pub struct Hitbox(Vector2);

impl Hitbox {
    pub fn new(size: Vector2) -> Self {
        Hitbox(size)
    }

    pub fn vec(&self) -> Vector2 {
        self.0
    }
}

fn get_hitbox_distances<T: Object, U: Object>(
    object_1: &T,
    object_2: &U,
) -> Option<(f32, f32, f32, f32)> {
    if let (Some(hitbox_1), Some(hitbox_2)) = (object_1.get_hitbox(), object_2.get_hitbox()) {
        let (position_1, position_2) = (object_1.get_position(), object_2.get_position());
        let h1 = position_1 + hitbox_1.vec();
        let h2 = position_2 + hitbox_2.vec();

        //TODO move into vectors for more concise code.
        let dx1 = h2.x - position_1.x;
        let dx2 = h1.x - position_2.x;
        let dy1 = h2.y - position_1.y;
        let dy2 = h1.y - position_2.y;

        Some((dx1, dx2, dy1, dy2))
    } else {
        None
    }
}

pub fn is_intersecting<T: Object, U: Object>(
    object_1: &T,
    object_2: &U,
) -> bool {
    if let Some((dx1, dx2, dy1, dy2)) = get_hitbox_distances(object_1, object_2) {
        (dx1 > 0.0) & (dx2 > 0.0) & (dy1 > 0.0) & (dy2 > 0.0)
    } else {
        false
    }
}

pub fn create_collision_event<T: Object, U: Object>(
    object_1: &mut T,
    object_2: &mut U,
) -> Event {
    if let (Some(hitbox_1), Some(hitbox_2)) = (object_1.get_hitbox(), object_2.get_hitbox()) {
        let (position_1, position_2) = (object_1.get_position(), object_2.get_position());
        let h1 = position_1 + hitbox_1.vec();
        let h2 = position_2 + hitbox_2.vec();

        //TODO move into vectors for more concise code.
        let dx1 = h2.x - position_1.x;
        let dx2 = h1.x - position_2.x;
        let dy1 = h2.y - position_1.y;
        let dy2 = h1.y - position_2.y;

        if (dx1 > 0.0) & (dx2 > 0.0) & (dy1 > 0.0) & (dy2 > 0.0) {
            let px = match dx1.abs() < dx2.abs() {
                true => -1.0 * dx1,
                false => dx2,
            };
            let py = match dy1.abs() < dy2.abs() {
                true => -1.0 * dy1,
                false => dy2,
            };
            Event::Collision {
                penetration: match px.abs() > py.abs() {
                    true => Vector2::new(0.0, py),
                    false => Vector2::new(px, 0.0),
                },
                elasticity: 0.0,
            }
        } else {
            Event::NoCollision
        }
    } else {
        Event::None
    }
}

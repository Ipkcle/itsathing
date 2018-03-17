use ggez::graphics::Vector2;
use super::HasHitbox;
use super::HasPhysics;
use super::Object;
use utils::get_two;

#[derive(Clone, Copy)]
pub struct Collision {
    penetration: Vector2,
}

impl Collision {
    pub fn new(penetration: Vector2) -> Self {
        Self { penetration }
    }

    pub fn get_penetration(&self) -> Vector2 {
        self.penetration
    }
}

pub struct Hitbox(Vector2);

impl Hitbox {
    pub fn new(size: Vector2) -> Self {
        Hitbox(size)
    }

    pub fn vec(&self) -> Vector2 {
        self.0
    }
}

fn get_hitbox_distances<T: HasHitbox, U: HasHitbox>(
    object_1: &T,
    object_2: &U,
) -> (f32, f32, f32, f32) {
    let (hitbox_1, hitbox_2) = (object_1.get_hitbox(), object_2.get_hitbox());
    let (position_1, position_2) = (object_1.get_position(), object_2.get_position());
    let h1 = position_1 + hitbox_1.vec();
    let h2 = position_2 + hitbox_2.vec();

    //TODO move into vectors for more concise code.
    let dx1 = h2.x - position_1.x;
    let dx2 = h1.x - position_2.x;
    let dy1 = h2.y - position_1.y;
    let dy2 = h1.y - position_2.y;

    (dx1, dx2, dy1, dy2)
}

pub fn get_distance<T: Object, U: Object>(object_1: &T, object_2: &U) -> f32 {
    (object_1.get_position() - object_2.get_position()).norm()
}

pub fn is_intersecting<T: HasHitbox, U: HasHitbox>(object_1: &T, object_2: &U) -> bool {
    let (dx1, dx2, dy1, dy2) = get_hitbox_distances(object_1, object_2);
    (dx1 > 0.0) & (dx2 > 0.0) & (dy1 > 0.0) & (dy2 > 0.0)
}

pub fn find_penetration<T: HasHitbox, U: HasHitbox>(object_1: &T, object_2: &U) -> Vector2 {
    let (hitbox_1, hitbox_2) = (object_1.get_hitbox(), object_2.get_hitbox());
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
        match px.abs() > py.abs() {
            true => Vector2::new(0.0, py),
            false => Vector2::new(px, 0.0),
        }
    } else {
        Vector2::new(0.0, 0.0)
    }
}

pub fn create_collision<T: HasPhysics, U: HasPhysics>(object_1: &T, object_2: &U) -> Collision {
    Collision::new(find_penetration(object_1, object_2))
}

pub fn object_vec_physics<T: HasPhysics, U: HasPhysics>(
    dt: f32,
    object_1: &mut T,
    list: &mut Vec<U>,
) {
    for object_2 in list.iter_mut() {
        let collision = create_collision(object_1, object_2);
        object_1.recieve_collision(dt, collision);
    }
}

pub fn vec_vec_physics<T: HasPhysics, U: HasPhysics>(
    dt: f32,
    objects_1: &mut Vec<T>,
    objects_2: &mut Vec<U>,
) {
    for object_1 in objects_1.iter_mut() {
        for object_2 in objects_2.iter_mut() {
            let collision = create_collision(object_1, object_2);
            object_1.recieve_collision(dt, collision);
        }
    }
}


pub fn vec_physics<T: HasPhysics>(dt: f32, list: &mut Vec<T>) {
    let n = list.len();
    for x in 0..n {
        for y in 0..n {
            if let Ok((object_1, object_2)) = get_two(list, x, y) {
                let collision = create_collision(object_1, object_2);
                object_1.recieve_collision(dt, collision);
            }
        }
    }
}

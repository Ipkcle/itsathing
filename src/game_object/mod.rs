mod physics;
pub mod bullet;
pub mod event;
pub mod collision;
pub mod player;
pub mod mob;
pub mod block;
pub mod basic_cuboid;

use ggez::graphics::Point2;
use ggez::graphics::Vector2;
use ggez::graphics::Color;
use self::collision::Hitbox;
use self::event::Event;
use assets::DrawableAsset;

// structs
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectID {
    value: u32,
}

impl ObjectID {
    pub fn new(value: u32) -> ObjectID {
        ObjectID { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

//traits

pub trait CanSetMoveTarget: Object {
    fn set_target(&mut self, target: Point2);
}

pub trait CanShoot: Object {
    fn shoot(&mut self) -> Option<bullet::Bullet>;
}

pub trait Renderable: HasBoundingBox {
    fn get_drawable_asset(&self) -> DrawableAsset;
    fn get_color(&self) -> Option<Color>;
}

pub trait HasBoundingBox: Object {
    fn get_bounding_box(&self) -> Vector2;

    fn get_center_position(&self) -> Point2 {
        self.get_position() + 0.5 * self.get_bounding_box()
    }
}

pub trait HasHitbox: Object {
    fn get_hitbox(&self) -> &Hitbox;
}

pub trait HasPhysics: HasHitbox {
    fn get_elasticity(&self) -> f32 {
        0.0
    }

    fn recieve_collision(&mut self, _dt: f32, _collision: collision::Collision) {
        //do nothing
    }
}

pub trait RecievesCollisionEvents: HasHitbox + CanRecieveEvents {
    fn recieve_collision_event(&mut self, dt: f32, event: Event) {
        self.recieve_event(dt, event);
    }
}

pub trait CanRecieveEvents: Object {
    fn recieve_event(&mut self, dt: f32, event: Event);

    fn recieve_events(&mut self, dt: f32, events: Vec<Event>) {
        for event in events {
            self.recieve_event(dt, event);
        }
    }
}

pub trait HasCollisionEvents: HasHitbox {
    fn create_collision_event<T: HasHitbox + CanRecieveEvents>(&mut self, object: &T)
        -> Vec<Event>;
}

pub trait Object {
    fn get_position(&self) -> Point2;

    fn step(&mut self, _dt: f32) {
        //do nothing
    }

    fn should_delete(&self) -> bool {
        false
    }

    fn get_id(&self) -> ObjectID {
        ObjectID::new(0)
    }
}

//impl blocks
impl<T> HasBoundingBox for T
where
    T: HasHitbox,
{
    fn get_bounding_box(&self) -> Vector2 {
        self.get_hitbox().vec()
    }
}

//functions
pub fn object_vec_collision_events<T: HasHitbox + CanRecieveEvents, U: HasCollisionEvents>(
    dt: f32,
    object_1: &mut T,
    list: &mut Vec<U>,
) {
    for object_2 in list.iter_mut() {
        let events = object_2.create_collision_event(object_1);
        for event in events {
            object_1.recieve_event(dt, event);
        }
    }
}

pub fn vec_vec_collision_events<T: HasHitbox + CanRecieveEvents, U: HasCollisionEvents>(
    dt: f32,
    objects_1: &mut Vec<T>,
    objects_2: &mut Vec<U>,
) {
    for object_1 in objects_1.iter_mut() {
        for object_2 in objects_2.iter_mut() {
            let events = object_2.create_collision_event(object_1);
            for event in events {
                object_1.recieve_event(dt, event);
            }
        }
    }
}

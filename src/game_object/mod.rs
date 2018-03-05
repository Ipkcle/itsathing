mod physics;
pub mod bullet;
pub mod event;
pub mod collision;
pub mod player;
pub mod mob;
pub mod block;

use ggez::graphics::Point2;
use ggez::graphics::Color;
use self::collision::Hitbox;
use self::event::Event;
use assets::DrawableAsset;

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

pub trait Renderable: Object {
    fn get_drawable_asset(&self) -> DrawableAsset;
    fn get_color(&self) -> Color;
}

pub trait HasHitbox: Object {
    fn get_hitbox(&self) -> &Hitbox;
}

pub trait HasCollisionEvents: HasHitbox {
    fn create_collision_event<T: HasHitbox + CanRecieveEvents>(&mut self, object: &T)
        -> Vec<Event>;
}

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
pub trait HasPhysics: HasHitbox {
    fn get_elasticity(&self) -> f32 {
        0.0
    }

    fn recieve_collision(&mut self, dt: f32, collision: collision::Collision) {
        //do nothing
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

mod physics;
pub mod bullet;
pub mod event;
pub mod collision;

use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use self::physics::ActorPhysics;
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

fn apply_collision_events() {

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

pub struct Block {
    mesh: DrawableAsset,
    position: Point2,
    hitbox: Hitbox,
}

impl Block {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            mesh: DrawableAsset::Block,
            position: Point2::new(x, y),
            hitbox: Hitbox::new(Vector2::new(20.0, 30.0)),
        }
    }
}

impl Renderable for Block {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.mesh
    }

    fn get_color(&self) -> Color {
        Color::new(0.3, 0.7, 0.3, 0.7)
    }
}

impl HasHitbox for Block {
    fn get_hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

impl HasPhysics for Block {}

impl Object for Block {
    fn get_position(&self) -> Point2 {
        self.position
    }
}

pub struct Mob {
    walk_acceleration: f32,
    mesh: DrawableAsset,
    hitbox: Hitbox,
    physics: ActorPhysics,
    position: Point2,
    health: i32,
    time_since_hurt: f32,
    time_since_shot: f32,
    id: ObjectID,
    color: Color,
    target: Option<Point2>,
    shoot_direction: Vector2,
}

impl Mob {
    pub fn player() -> Self {
        //TODO write this in terms of max speed and seconds until max speed is hit. You need
        //calculus.
        let accel = 2500.0;
        let max_speed = 250.0;
        let drag_constant = accel / max_speed;
        // ex. with v^2 drag
        // accel_drag = v^2*C
        // C = accel_drag/(v^2)
        // want constant speed, so accel == accel_drag
        // accel = 100, v = 10
        // 100 = 10^2*C
        // C = 1
        Self {
            walk_acceleration: accel,
            mesh: DrawableAsset::Player,
            hitbox: Hitbox::new(Vector2::new(10.0, 10.0)),
            physics: ActorPhysics::new(drag_constant),
            position: Point2::new(0.0, 0.0),
            health: 5,
            time_since_hurt: 300.0,
            time_since_shot: 300.0,
            id: ObjectID::new(1),
            color: Color::new(0.3, 0.7, 0.7, 0.7),
            target: None,
            shoot_direction: Vector2::new(0.0, 0.0),
        }
    }

    pub fn dummy(position: Point2) -> Self {
        Self {
            walk_acceleration: 1000.0,
            mesh: DrawableAsset::Player,
            hitbox: Hitbox::new(Vector2::new(10.0, 10.0)),
            physics: ActorPhysics::new(10.0),
            position,
            health: 3,
            time_since_hurt: 300.0,
            time_since_shot: 300.0,
            id: ObjectID::new(0),
            color: Color::from((222, 184, 135, 200)),
            target: Some(Point2::new(0.0, 0.0)),
            shoot_direction: Vector2::new(0.0, 0.0),
        }
    }

    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
    }

    pub fn shoot(&mut self) -> Option<bullet::Bullet> {
        if self.time_since_shot >= 0.2 {
            self.time_since_shot = 0.0;
            Some(bullet::Bullet::new(
                self.position + 0.5 * self.hitbox.vec(),
                500.0 * self.shoot_direction.normalize(),
                Color::new(0.9, 0.9, 0.9, 1.0),
                vec![self.get_id()],
            ))
        } else {
            None
        }
    }

    pub fn set_shoot_direction(&mut self, direction: Vector2) {
        self.shoot_direction = direction;
    }

    pub fn set_target(&mut self, target: Point2) {
        self.target = Some(target);
        self.shoot_direction = target - self.position;
    }

    pub fn set_movement(&mut self, direction: Vector2) {
        if let Some(unit_vector) = direction.try_normalize(0.0) {
            self.physics
                .set_acceleration(self.walk_acceleration * unit_vector);
        } else {
            self.physics.set_acceleration(Vector2::new(0.0, 0.0));
        }
    }
}
impl HasHitbox for Mob {
    fn get_hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

impl HasPhysics for Mob {
    fn recieve_collision(&mut self, dt: f32, collision: collision::Collision) {
        let p = collision.get_penetration();
        self.position -= p;
        let mut v = self.physics.get_velocity();
        match (p.x == 0.0, p.y == 0.0) {
            (false, true) => v.x = 0.0,
            (true, false) => v.y = 0.0,
            _ => (),
        }
        self.physics.set_velocity(v);
    }
}

impl Renderable for Mob {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.mesh
    }

    fn get_color(&self) -> Color {
        if self.time_since_hurt < 0.15 {
            Color::new(0.9, 0.4, 0.4, 0.7)
        } else {
            self.color
        }
    }
}

impl CanRecieveEvents for Mob {
    fn recieve_event(&mut self, _dt: f32, event: Event) {
        match event {
            Event::Damage(damage) => {
                self.health -= damage;
                self.time_since_hurt = 0.0
            }
            Event::Impulse(vector) => self.physics.add_impulse(vector),
            _ => (),
        }
    }
}

impl Object for Mob {
    fn get_id(&self) -> ObjectID {
        self.id
    }

    fn get_position(&self) -> Point2 {
        self.position
    }

    fn step(&mut self, dt: f32) {
        self.physics.step(dt);
        self.update_position(dt);
        if let Some(t) = self.target {
            let pos = self.position;
            self.set_movement(t - pos);
        }
        if self.time_since_hurt < 200.0 {
            self.time_since_hurt += dt;
        }
        if self.time_since_shot < 200.0 {
            self.time_since_shot += dt;
        }
    }

    fn should_delete(&self) -> bool {
        self.health <= 0
    }
}

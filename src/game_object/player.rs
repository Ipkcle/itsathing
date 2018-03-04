use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use super::Object;
use super::ObjectID;
use super::bullet::Bullet;
use super::collision;
use assets::DrawableAsset;

pub struct MobData {
    walk_acceleration: f32,
    mesh: DrawableAsset,
    hitbox: Hitbox,
    physics: ActorPhysics,
    position: Point2,
    color: Color,
}
pub struct Player {
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
    shoot_direction: Vector2,
}

impl Player {
    pub fn new() -> Self {
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
        Player {
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
            shoot_direction: Vector2::new(0.0, 0.0),
        }
    }

    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
    }

    pub fn shoot(&mut self) -> Option<Bullet> {
        if self.time_since_shot >= 0.2 {
            self.time_since_shot = 0.0;
            Some(Bullet::new(
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

    pub fn set_movement(&mut self, direction: Vector2) {
        if let Some(unit_vector) = direction.try_normalize(0.0) {
            self.physics
                .set_acceleration(self.walk_acceleration * unit_vector);
        } else {
            self.physics.set_acceleration(Vector2::new(0.0, 0.0));
        }
    }
}

impl Object for Player {
    fn create_collision_event<T: Object>(&mut self, object: &T) -> Vec<Event> {
        vec![Event::Collision {
            penetration: collision::find_penetration(object, self),
            elasticity: 0.0,
        }]
    }
    fn get_id(&self) -> ObjectID {
        self.id
    }
    fn get_hitbox(&self) -> Option<&Hitbox> {
        Some(&self.hitbox)
    }

    fn get_drawable_asset(&self) -> Option<DrawableAsset> {
        Some(self.mesh)
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

    fn get_color(&self) -> Option<Color> {
        if self.time_since_hurt < 0.15 {
            Some(Color::new(0.9, 0.4, 0.4, 0.7))
        } else {
            Some(self.color)
        }
    }

    fn should_delete(&self) -> bool {
        self.health <= 0
    }

    fn recieve_event(&mut self, _dt: f32, event: Event) {
        match event {
            Event::Collision { penetration: p, .. } => {
                self.position -= p;
                let mut v = self.physics.get_velocity();
                match (p.x == 0.0, p.y == 0.0) {
                    (false, true) => v.x = 0.0,
                    (true, false) => v.y = 0.0,
                    _ => (),
                }
                self.physics.set_velocity(v);
            }
            Event::Damage(damage) => {
                self.health -= damage;
                self.time_since_hurt = 0.0
            }
            Event::Impulse(vector) => self.physics.add_impulse(vector),
            _ => (),
        }
    }
}

use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use super::mob::*;
use super::*;
use assets::DrawableAsset;

pub struct BasicCuboid {
    pub walk_acceleration: f32,
    pub mesh: DrawableAsset,
    pub hitbox: Hitbox,
    pub physics: ActorPhysics,
    pub position: Point2,
    pub health: i32,
    pub time_since_hurt: f32,
    pub time_since_shot: f32,
    pub id: ObjectID,
    pub color: Color,
    pub target: Option<Point2>,
    pub shoot_direction: Vector2,
    pub blacklist: Vec<ObjectID>,
}

impl BasicCuboid {
    pub fn new(
        walk_acceleration: f32,
        mesh: DrawableAsset,
        hitbox_vec: Vector2,
        drag_constant: f32,
        position: Point2,
        health: i32,
        id: ObjectID,
        color: Color,
    ) -> Self {
        Self {
            walk_acceleration,
            mesh,
            hitbox: Hitbox::new(hitbox_vec),
            physics: ActorPhysics::new(drag_constant),
            position,
            health,
            time_since_hurt: 300.0,
            time_since_shot: 300.0,
            id,
            color,
            target: None,
            shoot_direction: Vector2::new(0.0, 0.0),
            blacklist: vec![ObjectID::new(1)],
        }
    }
    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
    }
}

impl CanShoot for BasicCuboid {
    fn shoot(&mut self) -> Option<bullet::Bullet> {
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

    fn set_shoot_direction(&mut self, direction: Vector2) {
        self.shoot_direction = direction;
    }
}

impl CanSetMovement for BasicCuboid {
    fn set_target(&mut self, target: Point2) {
        self.target = Some(target);
        self.shoot_direction = target - self.position;
    }

    fn set_movement(&mut self, direction: Vector2) {
        if let Some(unit_vector) = direction.try_normalize(0.0) {
            self.physics
                .set_acceleration(self.walk_acceleration * unit_vector);
        } else {
            self.physics.set_acceleration(Vector2::new(0.0, 0.0));
        }
    }
}
impl HasHitbox for BasicCuboid {
    fn get_hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

impl HasPhysics for BasicCuboid {
    fn recieve_collision(&mut self, _dt: f32, collision: collision::Collision) {
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

impl HasCollisionEvents for BasicCuboid {
    fn create_collision_event<T: HasHitbox>(&mut self, object: &T) -> Vec<Event> {
        if super::collision::is_intersecting(self, object) {
            if self.blacklist.iter().any(|x| *x == object.get_id()) {
                let mut effects = Vec::new();
                effects.push(Event::Damage(1));
                effects.push(Event::Impulse(
                    2000.0 * (object.get_position() - self.get_position()).normalize(),
                ));
                effects
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}

impl Renderable for BasicCuboid {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.mesh
    }

    fn get_color(&self) -> Option<Color> {
        Some(if self.time_since_hurt < 0.15 {
            Color::new(0.9, 0.4, 0.4, 0.7)
        } else {
            self.color
        })
    }
}

impl CanRecieveEvents for BasicCuboid {
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

impl Object for BasicCuboid {
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

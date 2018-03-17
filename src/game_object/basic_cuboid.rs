use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use super::*;
use assets::DrawableAsset;

pub struct BasicCuboid {
    walk_acceleration: f32,
    mesh: DrawableAsset,
    hitbox: Hitbox,
    physics: ActorPhysics,
    position: Point2,
    health: i32,
    time_since_hurt: f32,
    id: ObjectID,
    color: Color,
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
            id,
            color,
        }
    }
    pub fn get_health(&self) -> i32 {
        self.health
    }
    pub fn set_movement(&mut self, direction: Vector2) {
        if let Some(unit_vector) = direction.try_normalize(0.0) {
            self.physics
                .set_acceleration(self.walk_acceleration * unit_vector);
        } else {
            self.physics.set_acceleration(Vector2::new(0.0, 0.0));
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
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
            Event::ImpulseFrom {
                from,
                magnitude,
            } =>  { self.physics.add_impulse(magnitude*(self.position - from).normalize()) },
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
        if self.time_since_hurt < 200.0 {
            self.time_since_hurt += dt;
        }
    }

    fn should_delete(&self) -> bool {
        self.health <= 0
    }
}

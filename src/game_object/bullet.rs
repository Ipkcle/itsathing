use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use super::Object;
use super::HasCollision;
use super::Renderable;
use super::ObjectID;
use assets::DrawableAsset;

pub struct Bullet {
    hitbox: Hitbox,
    mesh: DrawableAsset,
    position: Point2,
    physics: ActorPhysics,
    lifetime: f32,
    max_lifetime: f32,
    effects: Vec<Event>,
    color: Color,
    whitelist: Vec<ObjectID>,
}

impl Bullet {
    pub fn new(position: Point2, velocity: Vector2, color: Color, whitelist: Vec<ObjectID>) -> Self {
        let mut effects = Vec::new();
        effects.push(Event::Damage(1));
        effects.push(Event::Impulse(400.0 * velocity.normalize()));
        let mut bullet = Bullet {
            hitbox: Hitbox::new(Vector2::new(2.0, 2.0)),
            mesh: DrawableAsset::Bullet,
            position,
            physics: ActorPhysics::new(0.0),
            lifetime: 0.0,
            max_lifetime: 1.0,
            effects,
            color,
            whitelist,
        };
        bullet.physics.set_velocity(velocity);
        bullet
    }

    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
    }

    fn get_whitelist(&self) -> Vec<ObjectID> {
        self.whitelist.clone()
    }

    fn mark_for_deletion(&mut self) {
        self.lifetime += 1000.0;
    }

    fn get_effects(&self) -> Vec<Event> {
        self.effects.clone()
    }
}

impl Renderable for Bullet {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.mesh
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

impl HasCollision for Bullet {
    fn create_collision_event<T: HasCollision>(&mut self, object: &T) -> Vec<Event> {
        if super::collision::is_intersecting(self, object) {
        if !self.get_whitelist().iter().any(|x| *x == object.get_id()) {
                self.mark_for_deletion();
                self.get_effects()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    fn get_hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

impl Object for Bullet {
    fn should_delete(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }


    fn get_position(&self) -> Point2 {
        self.position
    }

    fn step(&mut self, dt: f32) {
        self.lifetime += dt;
        self.physics.step(dt);
        self.update_position(dt);
    }
}

use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::*;
use super::mob::IsMob;
use super::basic_cuboid::BasicCuboid;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use assets::DrawableAsset;

pub struct Player {
    implementation: BasicCuboid,
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
        Self {
            implementation: BasicCuboid::new(
                accel,
                DrawableAsset::Player,
                Vector2::new(10.0, 10.0),
                drag_constant,
                Point2::new(0.0, 0.0),
                30,
                ObjectID::new(1),
                Color::new(0.3, 0.7, 0.7, 0.7),
            ),
        }
    }
    pub fn shoot(&mut self) -> Option<bullet::Bullet> {
        if self.implementation.time_since_shot >= 0.10 {
            self.implementation.time_since_shot = 0.0;
            Some(bullet::Bullet::new(
                self.implementation.position + 0.5 * self.implementation.hitbox.vec(),
                500.0 * self.implementation.shoot_direction.normalize(),
                Color::new(0.9, 0.9, 0.9, 1.0),
                vec![self.get_id()],
            ))
        } else {
            None
        }
    }

    pub fn set_shoot_direction(&mut self, direction: Vector2) {
        self.implementation.shoot_direction = direction;
    }
}

impl IsMob for Player {
    type Implmementation = BasicCuboid;
    fn get_mob_mut(&mut self) -> &mut Self::Implmementation {
        &mut self.implementation
    }
    fn get_mob(&self) -> &Self::Implmementation {
        &self.implementation
    }
}

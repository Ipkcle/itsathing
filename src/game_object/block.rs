use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::physics::ActorPhysics;
use super::collision::Hitbox;
use super::event::Event;
use super::Object;
use super::HasHitbox;
use super::HasBoundingBox;
use super::HasPhysics;
use super::CanRecieveEvents;
use super::Renderable;
use super::ObjectID;
use assets::DrawableAsset;
use super::bullet;

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
            hitbox: Hitbox::new(Vector2::new(20.0, 20.0)),
        }
    }

    pub fn wallh(x: f32, y: f32) -> Self {
        Self {
            mesh: DrawableAsset::Wallh,
            position: Point2::new(x, y),
            hitbox: Hitbox::new(Vector2::new(400.0, 20.0)),
        }
    }

    pub fn wallv(x: f32, y: f32) -> Self {
        Self {
            mesh: DrawableAsset::Wallv,
            position: Point2::new(x, y),
            hitbox: Hitbox::new(Vector2::new(20.0, 400.0)),
        }
    }

}

impl Renderable for Block {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.mesh
    }

    fn get_color(&self) -> Option<Color> {
        Some(Color::new(0.3, 0.7, 0.3, 0.7))
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

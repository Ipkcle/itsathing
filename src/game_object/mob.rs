use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::collision::Hitbox;
use super::event::Event;
use super::basic_cuboid::BasicCuboid;
use super::*;
use assets::DrawableAsset;

pub trait CanSetMovement {
    fn set_target(&mut self, target: Point2);
    fn set_movement(&mut self, direction: Vector2);
}

pub trait CanShoot {
    fn shoot(&mut self) -> Option<bullet::Bullet>;
    fn set_shoot_direction(&mut self, direction: Vector2);
}

pub trait IsMob {
    type Implmementation: HasPhysics
        + HasCollisionEvents
        + Renderable
        + CanRecieveEvents
        + CanSetMovement
        + CanShoot;
    fn get_mob_mut(&mut self) -> &mut Self::Implmementation;
    fn get_mob(&self) -> &Self::Implmementation;
}

impl<T: IsMob> CanShoot for T {
    fn shoot(&mut self) -> Option<bullet::Bullet> {
        self.get_mob_mut().shoot()
    }

    fn set_shoot_direction(&mut self, direction: Vector2) {
        self.get_mob_mut().set_shoot_direction(direction);
    }
}

impl<T: IsMob> CanSetMovement for T {
    fn set_movement(&mut self, direction: Vector2) {
        self.get_mob_mut().set_movement(direction);
    }

    fn set_target(&mut self, target: Point2) {
        self.get_mob_mut().set_target(target);
    }
}

impl<T: IsMob> Object for T {
    fn get_position(&self) -> Point2 {
        self.get_mob().get_position()
    }

    fn step(&mut self, dt: f32) {
        self.get_mob_mut().step(dt);
    }

    fn should_delete(&self) -> bool {
        self.get_mob().should_delete()
    }

    fn get_id(&self) -> ObjectID {
        self.get_mob().get_id()
    }
}

impl<T: IsMob> HasHitbox for T {
    fn get_hitbox(&self) -> &Hitbox {
        self.get_mob().get_hitbox()
    }
}

impl<T: IsMob> HasPhysics for T {
    fn get_elasticity(&self) -> f32 {
        self.get_mob().get_elasticity()
    }

    fn recieve_collision(&mut self, dt: f32, collision: collision::Collision) {
        self.get_mob_mut().recieve_collision(dt, collision);
    }
}

impl<T: IsMob> Renderable for T {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.get_mob().get_drawable_asset()
    }

    fn get_color(&self) -> Option<Color> {
        self.get_mob().get_color()
    }
}

impl<T: IsMob> HasCollisionEvents for T {
    fn create_collision_event<O: HasHitbox + CanRecieveEvents>(
        &mut self,
        object: &O,
    ) -> Vec<Event> {
        self.get_mob_mut().create_collision_event(object)
    }
}

impl<T: IsMob> CanRecieveEvents for T {
    fn recieve_event(&mut self, dt: f32, event: Event) {
        self.get_mob_mut().recieve_event(dt, event);
    }

    fn recieve_events(&mut self, dt: f32, events: Vec<Event>) {
        self.get_mob_mut().recieve_events(dt, events);
    }
}

pub struct Mob {
    implementation: BasicCuboid,
}

impl Mob {
    pub fn dummy(position: Point2) -> Self {
        Self {
            implementation: BasicCuboid::new(
                1000.0,
                DrawableAsset::Player,
                Vector2::new(10.0, 10.0),
                2.0,
                position,
                5,
                ObjectID::new(0),
                Color::from((222, 184, 135, 200)),
            ),
        }
    }
}

impl IsMob for Mob {
    type Implmementation = BasicCuboid;
    fn get_mob_mut(&mut self) -> &mut Self::Implmementation {
        &mut self.implementation
    }
    fn get_mob(&self) -> &Self::Implmementation {
        &self.implementation
    }
}

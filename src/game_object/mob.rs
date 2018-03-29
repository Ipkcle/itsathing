use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use super::collision::Hitbox;
use super::event::Event;
use super::basic_cuboid::BasicCuboid;
use super::*;
use assets::DrawableAsset;

pub trait IsMob {
    type Implmementation: HasPhysics + Renderable + CanRecieveEvents;
    fn get_mob_mut(&mut self) -> &mut Self::Implmementation;
    fn get_mob(&self) -> &Self::Implmementation;
    fn pre_step(&mut self, _dt: f32) {
        /* do nothing */
    }
    fn post_step(&mut self, _dt: f32) {
        /* do nothing */
    }
}

impl<T: IsMob> Object for T {
    fn get_position(&self) -> Point2 {
        self.get_mob().get_position()
    }

    fn step(&mut self, dt: f32) {
        self.pre_step(dt);
        self.get_mob_mut().step(dt);
        self.post_step(dt);
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

impl<T: IsMob> RecievesCollisionEvents for T {}

impl<T: IsMob> Renderable for T {
    fn get_drawable_asset(&self) -> DrawableAsset {
        self.get_mob().get_drawable_asset()
    }

    fn get_color(&self) -> Option<Color> {
        self.get_mob().get_color()
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

pub struct Dummy {
    implementation: BasicCuboid,
    blacklist: Vec<ObjectID>,
    target: Option<Point2>,
}

impl Dummy {
    pub fn new(position: Point2) -> Self {
        Dummy {
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
            blacklist: vec![ObjectID::new(1)],
            target: None,
        }
    }
}

impl HasCollisionEvents for Dummy {
    fn create_collision_event(&mut self, id: ObjectID) -> Vec<Event> {
        //TODO move the blacklist thing to the other function place maybe?
        if self.blacklist.iter().any(|x| *x == id) {
            let mut effects = Vec::new();
            effects.push(Event::Damage(1));
            effects.push(Event::ImpulseFrom {
                from: self.get_position(),
                magnitude: 2000.0
            });
            effects
        } else {
            Vec::new()
        }
    }
}

impl CanSetMoveTarget for Dummy {
    fn set_target(&mut self, target: Point2) {
        self.target = Some(target);
    }
}

impl IsMob for Dummy {
    type Implmementation = BasicCuboid;
    fn post_step(&mut self, _dt: f32) {
        if let Some(t) = self.target {
            let pos = self.get_position();
            self.implementation.set_movement(t - pos);
        }
    }
    fn get_mob_mut(&mut self) -> &mut Self::Implmementation {
        &mut self.implementation
    }
    fn get_mob(&self) -> &Self::Implmementation {
        &self.implementation
    }
}

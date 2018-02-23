mod physics;
pub mod event;
use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Color;
use self::physics::ActorPhysics;
use self::collision::Hitbox;
use self::event::Event;
use assets::DrawableAsset;

pub trait Object {
    fn get_drawable_asset(&self) -> Option<DrawableAsset>;

    fn get_color(&self) -> Option<Color> {
        None
    }

    fn get_hitbox(&self) -> Option<&Hitbox> {
        None
    }

    fn get_position(&self) -> Point2;

    fn recieve_event(&mut self, _dt: f32, _event: Event) {
        //do nothing
    }

    fn step(&mut self, _dt: f32) {
        //do nothing
    }
}

pub struct Block {
    mesh: DrawableAsset,
    position: Point2,
    hitbox: Hitbox,
    is_colliding: bool,
}

impl Block {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            mesh: DrawableAsset::Block,
            position: Point2::new(x, y),
            hitbox: Hitbox::new(Vector2::new(20.0, 30.0)),
            is_colliding: false,
        }
    }
}

impl Object for Block {
    fn get_hitbox(&self) -> Option<&Hitbox> {
        Some(&self.hitbox)
    }

    fn get_drawable_asset(&self) -> Option<DrawableAsset> {
        Some(self.mesh)
    }

    fn get_position(&self) -> Point2 {
        self.position
    }

    fn recieve_event(&mut self, _dt: f32, event: Event) {
        self.is_colliding = match event {
            Event::Collision { .. } => true,
            Event::NoCollision => false,
            _ => self.is_colliding,
        }
    }

    fn get_color(&self) -> Option<Color> {
        Some(match self.is_colliding {
            true => Color::new(0.7, 0.3, 0.3, 0.7),
            false => Color::new(0.3, 0.7, 0.3, 0.7),
        })
    }
}

pub struct Projectile {
    hitbox: Hitbox,
    mesh: DrawableAsset,
    position: Point2,
    physics: ActorPhysics,
    lifetime: f32,
    max_lifetime: f32,
    effects: Vec<Event>,
    color: Color,
}

impl Projectile {
    pub fn bullet(position: Point2, velocity: Vector2, color: Color) -> Self {
        let mut effects = Vec::new();
        effects.push(Event::Damage(1));
        let mut bullet = Self {
            hitbox: Hitbox::new(Vector2::new(2.0, 2.0)),
            mesh: DrawableAsset::Bullet,
            position,
            physics: ActorPhysics::new(0.0),
            lifetime: 0.0,
            max_lifetime: 1.0,
            effects,
            color,
        };
        bullet.physics.set_velocity(velocity);
        bullet
    }

    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
    }

    pub fn get_effects(&self) -> Vec<Event> {
        self.effects.clone()
    }

    pub fn should_delete(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }
}

impl Object for Projectile {
    fn get_hitbox(&self) -> Option<&Hitbox> {
        Some(&self.hitbox)
    }

    fn get_drawable_asset(&self) -> Option<DrawableAsset> {
        Some(self.mesh)
    }

    fn get_position(&self) -> Point2 {
        self.position
    }

    fn get_color(&self) -> Option<Color> {
        Some(self.color)
    }

    fn step(&mut self, dt: f32) {
        self.lifetime += dt;
        self.physics.step(dt);
        self.update_position(dt);
    }
}

pub struct Mob {
    walk_acceleration: f32,
    mesh: DrawableAsset,
    hitbox: Hitbox,
    physics: ActorPhysics,
    position: Point2,
    health: i32,
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
            health: 1,
        }
    }

    fn update_position(&mut self, dt: f32) {
        if self.physics.get_velocity().norm() > 10.0 {
            self.position += self.physics.get_velocity() * dt;
        }
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

impl Object for Mob {
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
    }

    fn get_color(&self) -> Option<Color> {
        Some(Color::new(0.3, 0.7, 0.7, 0.7))
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
            Event::Damage(damage) => {self.health -= damage},
            Event::Impulse(vector) => {self.physics.add_impulse(vector)}
            _ => (),
        }
    }
}

pub mod collision {
    use super::Object;
    use super::event::Event;
    use ggez::graphics::Vector2;

    pub struct Hitbox(Vector2);

    impl Hitbox {
        pub fn new(size: Vector2) -> Self {
            Hitbox(size)
        }

        pub fn vec(&self) -> Vector2 {
            self.0
        }
    }

    pub fn is_intersecting<T: Object, U: Object>(
        object_1: &T,
        object_2: &T,
    ) -> bool {
        if let (Some(hitbox_1), Some(hitbox_2)) = (object_1.get_hitbox(), object_2.get_hitbox()) {
            let (position_1, position_2) = (object_1.get_position(), object_2.get_position());
            let h1 = position_1 + hitbox_1.vec();
            let h2 = position_2 + hitbox_2.vec();

            //TODO move into vectors for more concise code.
            let dx1 = h2.x - position_1.x;
            let dx2 = h1.x - position_2.x;
            let dy1 = h2.y - position_1.y;
            let dy2 = h1.y - position_2.y;

            (dx1 > 0.0) & (dx2 > 0.0) & (dy1 > 0.0) & (dy2 > 0.0)
        } else {
            false
        }
    }

    pub fn create_collision_event<T: Object, U: Object>(
        object_1: &mut T,
        object_2: &mut U,
    ) -> Event {
        if let (Some(hitbox_1), Some(hitbox_2)) = (object_1.get_hitbox(), object_2.get_hitbox()) {
            let (position_1, position_2) = (object_1.get_position(), object_2.get_position());
            let h1 = position_1 + hitbox_1.vec();
            let h2 = position_2 + hitbox_2.vec();

            //TODO move into vectors for more concise code.
            let dx1 = h2.x - position_1.x;
            let dx2 = h1.x - position_2.x;
            let dy1 = h2.y - position_1.y;
            let dy2 = h1.y - position_2.y;

            if (dx1 > 0.0) & (dx2 > 0.0) & (dy1 > 0.0) & (dy2 > 0.0) {
                let px = match dx1.abs() < dx2.abs() {
                    true => -1.0 * dx1,
                    false => dx2,
                };
                let py = match dy1.abs() < dy2.abs() {
                    true => -1.0 * dy1,
                    false => dy2,
                };
                Event::Collision {
                    penetration: match px.abs() > py.abs() {
                        true => Vector2::new(0.0, py),
                        false => Vector2::new(px, 0.0),
                    },
                    elasticity: 0.0,
                }
            } else {
                Event::NoCollision
            }
        } else {
            Event::None
        }
    }
}

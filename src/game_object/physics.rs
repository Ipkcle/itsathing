use ggez::graphics::{Point2, Vector2};

struct Drag {
    drag_constant: f32,
    pub acceleration: Vector2,
}

impl Drag {
    pub fn new(drag_constant: f32) -> Self {
        Self {
            drag_constant,
            acceleration: Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_drag_constant(&self) -> f32 {
        self.drag_constant
    }
}

pub struct ActorPhysics {
    velocity: Vector2,
    acceleration: Vector2,
    drag: Drag,
}

impl ActorPhysics {
    pub fn new(drag_constant: f32) -> Self {
        ActorPhysics {
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            drag: Drag::new(drag_constant),
        }
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    pub fn set_acceleration(&mut self, accel: Vector2) {
        self.acceleration = accel;
    }
    
    pub fn set_velocity(&mut self, vel: Vector2) {
        self.velocity = vel;
    }

    pub fn add_impulse(&mut self, impulse: Vector2) {
        self.velocity += impulse;
    }

    fn calculate_drag_acceleration(&mut self) {
        let c = self.drag.get_drag_constant();
        self.drag.acceleration = -1.0 * self.velocity * c;
    }

    fn calculate_velocity(&mut self, dt: f32) {
        self.velocity += (self.acceleration + self.drag.acceleration) * dt;
        // if self.velocity.norm() < 0.001 { self.velocity = Vector2::zeros(); };
    }

    pub fn step(&mut self, dt: f32) {
        self.calculate_drag_acceleration();
        self.calculate_velocity(dt);
    }
}

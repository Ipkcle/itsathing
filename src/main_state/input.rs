use ggez::graphics::{Vector2};

//enums
enum Action {
    Item,
    None,
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DirectionInputScalar {
    Positive,
    Negative,
}

impl DirectionInputScalar {
    pub fn get_value(&self) -> f32 {
        match *self {
            DirectionInputScalar::Positive => 1.0,
            DirectionInputScalar::Negative => -1.0,
        }
    }
}


// DirectionInputStack struct
pub struct DirectionInputStack {
    x_input_stack: Vec<DirectionInputScalar>,
    y_input_stack: Vec<DirectionInputScalar>,
}

impl DirectionInputStack {
    pub fn new() -> Self {
        Self {
            x_input_stack: Vec::new(),
            y_input_stack: Vec::new(),
        }
    }

    fn get_input_stack(&mut self, axis: Axis) -> &mut Vec<DirectionInputScalar> {
        match axis {
            Axis::X => &mut self.x_input_stack,
            Axis::Y => &mut self.y_input_stack,
        }
    }

    pub fn get_direction_old(&self) -> Vector2 {
        let mut x_vec = Vector2::zeros();
        let mut y_vec = Vector2::zeros();
        if let Some(x_magnitude) = self.x_input_stack.first() {
            x_vec = Vector2::new(x_magnitude.get_value(), 0.0);
        }
        if let Some(y_magnitude) = self.y_input_stack.first() {
            y_vec = Vector2::new(0.0, y_magnitude.get_value());
        }
        (x_vec + y_vec)
    }

    pub fn get_direction_recent(&self) -> Vector2 {
        let mut x_vec = Vector2::zeros();
        let mut y_vec = Vector2::zeros();
        if let Some(x_magnitude) = self.x_input_stack.last() {
            x_vec = Vector2::new(x_magnitude.get_value(), 0.0);
        }
        if let Some(y_magnitude) = self.y_input_stack.last() {
            y_vec = Vector2::new(0.0, y_magnitude.get_value());
        }
        (x_vec + y_vec)
    }

    pub fn is_active(&self) -> bool {
        !(self.x_input_stack.is_empty() && self.y_input_stack.is_empty())
    }

    pub fn deactivate_direction(&mut self, direction: DirectionInputScalar, axis: Axis) {
        self.get_input_stack(axis)
            .retain(|element| *element != direction);
    }

    pub fn activate_direction(&mut self, direction: DirectionInputScalar, axis: Axis) {
        if !self.get_input_stack(axis).contains(&direction) {
            self.get_input_stack(axis).push(direction);
        }
    }
}

//Input struct
pub struct Input {
    action: Action,
    pub move_stack: DirectionInputStack,
    pub shoot_stack: DirectionInputStack,
}

impl Input {
    pub fn new() -> Self {
        Self {
            action: Action::None,
            move_stack: DirectionInputStack::new(),
            shoot_stack: DirectionInputStack::new(),
        }
    }
}

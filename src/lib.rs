// Imports
use nannou::prelude::*;

// A physics object that can be moved around.
#[derive(Clone, Debug)]
pub struct Blob {
    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
    color: Rgb,
    size: f32,
}

impl Blob {
    // Make new blob object
    pub fn new(position_current: Vec2, size: f32, color: Rgb) -> Self {
        Self {
            position_current,
            position_old: position_current,
            acceleration: Vec2::ZERO,
            color,
            size,
        }
    }

    // Update blob's position
    fn update_position(&mut self, time: f32) {
        // Set velocity
        let velocity: Vec2 = self.position_current - self.position_old;

        // Update old position
        self.position_old = self.position_current;

        // Do the verlet physics
        self.position_current = self.position_current + velocity + self.acceleration * time * time;

        // Reset acceleration
        self.acceleration = Vec2::ZERO;
    }

    // Apply acceleration force to blob
    fn update_acceleration(&mut self, acceleration: Vec2) {
        // Calculate the acceleration
        self.acceleration = self.acceleration + acceleration
    }

    fn update_constraint(&mut self, constraints_position: Vec2, constraints_radius: f32) {
        let mut to_next = constraints_position - self.position_current;
        if to_next.length() > constraints_radius - (self.size / 2.0) {
            to_next = to_next.clamp_length_max(constraints_radius - (self.size / 2.0));
            let offset = (constraints_position - self.position_current) - to_next;
            self.position_current += offset
        }
    }

    fn update_collision(&mut self, other: &mut Blob) {
        println!("{:#?}", other.acceleration)
    }

    // Show blob to the screen
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position_current)
            .radius(self.size / 2.0)
            .color(self.color);
    }
}

// The constraint properties
struct Constraint {
    position: Vec2,
    radius: f32,
}

impl Constraint {
    fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }
}

// The physics solver
pub struct Solver {
    blobs: Vec<Blob>,
    gravity: Vec2,
    constraint: Constraint,
}

impl Solver {
    // Make new solver object
    pub fn new(blobs: Vec<Blob>) -> Self {
        Self {
            blobs,
            gravity: Vec2::new(0.0, -1000.0),
            constraint: Constraint::new(Vec2::ZERO, 200.0),
        }
    }

    // Update all blobs in the solver
    pub fn update(&mut self, time: f32) {
        self.solve_acceleration();
        self.solve_constraint();
        self.solve_collision();
        self.solve_position(time);
    }

    // Update blob's gravity
    fn solve_acceleration(&mut self) {
        for blob in &mut self.blobs {
            blob.update_acceleration(self.gravity);
        }
    }

    // Update blob's constraint
    fn solve_constraint(&mut self) {
        for blob in &mut self.blobs {
            blob.update_constraint(self.constraint.position, self.constraint.radius);
        }
    }

    // Update blob's collision
    fn solve_collision(&mut self) {
        let object_count = self.blobs.len();
        for i in 0..object_count {
            let first = &mut self.blobs[i];
            for j in 0..object_count {
                if i == j {
                    continue;
                };
                let second = &mut self.blobs[j];
                first.update_collision(second)
            }
        }
    }

    // Update blob's position
    fn solve_position(&mut self, time: f32) {
        for blob in &mut self.blobs {
            blob.update_position(time);
        }
    }

    // Draw all the circles to the screen
    pub fn draw(&self, draw: &Draw) {
        // Draw the constraint
        draw.ellipse()
            .xy(self.constraint.position)
            .radius(self.constraint.radius)
            .color(BLACK);

        for blob in &self.blobs {
            blob.draw(draw);
        }
    }

    // Add a new blob to the solver
    pub fn add_blob(&mut self, blob: Blob) {
        self.blobs.push(blob);
    }

    // Remove a blob from the solver
    pub fn remove_blob(&mut self, index: usize) {
        self.blobs.remove(index);
    }
}

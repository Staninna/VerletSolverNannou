// Imports
use nannou::prelude::*;

// A physics object that can be moved around.
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
        // Calculate the distance between the blob and the constraint
        let mut to_next = constraints_position - self.position_current;

        // Move the blob inside the constraint if it's too far away
        if to_next.length() > constraints_radius - (self.size / 2.0) {
            to_next = to_next.clamp_length_max(constraints_radius - (self.size / 2.0));
            let offset = (constraints_position - self.position_current) - to_next;
            self.position_current += offset
        }
    }

    fn update_collision(&mut self, other: &mut Blob) {
        // Calculate the distance between the blob and the other blob
        let collision_axis = self.position_current - other.position_current;
        let distance = collision_axis.length();

        // Check if the blobs are close enough to collide
        let minimum_distance = self.size / 2.0 + other.size / 2.0;
        if distance < minimum_distance {
            // Calculate the collision normal
            let n = collision_axis / distance;
            let delta = minimum_distance - distance;

            // Move the blobs apart
            self.position_current += 0.5 * delta * n;
            other.position_current -= 0.5 * delta * n;
        }
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
pub struct Constraint {
    position: Vec2,
    radius: f32,
}

impl Constraint {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }
}

// The physics solver
pub struct Solver {
    blobs: Vec<Blob>,
    gravity: Vec2,
    constraint: Constraint,
    iterations: i32,
    time: f32,
}

impl Solver {
    // Make new solver object
    pub fn new(
        blobs: Vec<Blob>,
        gravity: Vec2,
        constraint: Constraint,
        iterations: i32,
        time: f32,
    ) -> Self {
        Self {
            blobs,
            gravity,
            constraint,
            iterations,
            time,
        }
    }

    // Update all blobs in the solver
    pub fn update(&mut self) {
        let sub_time_step = self.time / self.iterations as f32;
        for _ in 0..self.iterations {
            self.solve_acceleration();
            self.solve_constraint();
            self.solve_collision();
            self.solve_position(sub_time_step);
        }
    }

    // Update blob's gravity
    fn solve_acceleration(&mut self) {
        // Calculate the gravity acceleration
        for blob in &mut self.blobs {
            blob.update_acceleration(self.gravity);
        }
    }

    // Update blob's constraint
    fn solve_constraint(&mut self) {
        // Calculate the constraint force
        for blob in &mut self.blobs {
            blob.update_constraint(self.constraint.position, self.constraint.radius);
        }
    }

    // Update blob's collision
    fn solve_collision(&mut self) {
        // Calculate the collision force
        let mut blobs = self.blobs.as_mut_slice();
        while let [first, tail @ ..] = blobs {
            for second in tail.iter_mut() {
                first.update_collision(second);
            }
            blobs = tail
        }
    }

    // Update blob's position
    fn solve_position(&mut self, time: f32) {
        // Update all blobs positions
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
            .color(BLACK); // TODO make dynamic

        // Draw all the blobs
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

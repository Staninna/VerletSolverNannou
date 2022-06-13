// Imports
use nannou::prelude::*;

// A physics object
pub struct Blob {
    pub position_current: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
}

impl Blob {
    // Make new blob object
    pub fn new(position_current: Vec2) -> Self {
        Self {
            position_current,
            position_old: position_current,
            acceleration: Vec2::new(0.0, 0.0),
        }
    }

    // Update blob's position
    fn update_position(&mut self, time: f32) {
        // Set velocity
        let velocity: Vec2 = self.position_current - self.position_old;
        let time: Vec2 = Vec2::new(time, time);

        // Update old position
        self.position_old = self.position_current;

        // Do the verlet physics
        self.position_current = self.position_current + velocity + self.acceleration * time * time;

        // Reset acceleration
        self.acceleration = Vec2::new(0.0, 0.0);
    }

    // Apply acceleration force to blob
    fn update_acceleration(&mut self, acceleration: Vec2) {
        // Calculate the acceleration
        self.acceleration = self.acceleration + acceleration
    }

    // Show blob to the screen
    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.position_current);
    }
}

// TODO add function to add and remove blobs from the solver
// The physics solver
pub struct Solver {
    pub gravity: Vec2, // vec2(0.0, 9.81)
    pub blobs: Vec<Blob>,
}

impl Solver {
    // Make new solver object
    pub fn new(blobs: Vec<Blob>, gravity: Vec2) -> Self {
        Self { gravity, blobs }
    }

    // Update all blobs in the solver
    pub fn update(&mut self, time: f32) {
        // Loop over all blobs in the solver
        for blob in &mut self.blobs {
            // Update blob's gravity
            blob.update_acceleration(self.gravity);

            // println!("{}", blob.position_current);
            // Update blob's position
            blob.update_position(time);
        }
    }

    // Show all blobs in the solver
    pub fn draw(&self, draw: &Draw) {
        for blob in self.blobs.iter() {
            blob.draw(draw);
        }
    }
}

// TODO https://youtu.be/lS_qeBy3aQI?t=129

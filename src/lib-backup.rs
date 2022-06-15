// Imports
use nannou::prelude::*;

// A physics object
pub struct Blob {
    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
    size: f32,
}

impl Blob {
    // Make new blob object
    pub fn new(position_current: Vec2) -> Self {
        Self {
            position_current,
            position_old: position_current,
            acceleration: Vec2::ZERO,
            size: 50.0,
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

    fn update_constraints(&mut self) {
        // Set up the constraint radius
        let position = Vec2::ZERO;
        let radius = 2000.0;

        // Calculate the distance between the blob and the constraint
        let to_blob = self.position_current - position;
        let distance = ((self.position_current.x * to_blob.x).pow(2) as f32
            + (self.position_current.y * to_blob.y).pow(2) as f32)
            .sqrt();

        // If the blob not inside move blob back
        if distance > (radius - self.size) {
            let to_move = to_blob / distance;
            self.position_current = position + to_move * (distance - self.size);
        }
    }

    // Show blob to the screen
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position_current)
            .wh(vec2(self.size, self.size));
    }
}

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

            // Update blob's constraints
            blob.update_constraints();

            // Update blob's position
            blob.update_position(time);
        }
    }

    // Show all blobs in the solver
    pub fn draw(&self, draw: &Draw) {
        // Draw to window background
        draw.background().color(GRAY);

        // Draw the constraint
        draw.ellipse()
            .xy(vec2(0.0, 0.0))
            .wh(vec2(150.0, 150.0))
            .color(BLACK);

        for blob in self.blobs.iter() {
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

// TODO https://youtu.be/lS_qeBy3aQI?t=129

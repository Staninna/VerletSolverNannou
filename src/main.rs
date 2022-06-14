// Import
use nannou::prelude::*;
use std::time::Instant;
use verlet_solver_nannou::*;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    solver: Solver,
}

// Ran before drawing
fn model(app: &App) -> Model {
    // Make new window
    app.new_window().size(512, 512).view(view).build().unwrap();

    let mut blobs = Vec::new();
    for _ in 0..1 {
        blobs.push(Blob::new(Vec2::new(20.0, 0.0)));
    }

    // Return model struct
    Model {
        solver: Solver::new(blobs, vec2(0.0, 0.0)),
    }
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.solver.update(0.01);
}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    // Debug info
    let start = Instant::now();

    // Setup window draw surface
    let draw = app.draw();

    // Draw to window background
    draw.background().color(BLACK);

    // Draw all the blobs in the solver
    model.solver.draw(&draw);

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();

    // Debug info
    let duration = start.elapsed();
    println!("FPS: {:?}", duration);
}

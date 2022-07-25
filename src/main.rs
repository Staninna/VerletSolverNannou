// Import
use nannou::prelude::*;
use verlet_solver_nannou::*;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    solver: Solver,
}

// Ran before launching app
fn model(app: &App) -> Model {
    // Make new window
    app.new_window().size(512, 512).view(view).build().unwrap();

    // Generate blobs
    let mut blobs: Vec<Blob> = Vec::new();

    blobs.push(Blob::new(
        vec2(170.0, 0.0),
        30.0,
        Rgb::new(255.0, 0.0, 90.0),
    ));
    blobs.push(Blob::new(
        vec2(-150.0, 20.0),
        30.0,
        Rgb::new(255.0, 0.0, 90.0),
    ));
    blobs.push(Blob::new(
        vec2(170.0, 0.0),
        30.0,
        Rgb::new(255.0, 0.0, 90.0),
    ));

    Model {
        solver: Solver::new(blobs),
    }
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.solver.update(0.01);
}

// Draw window
fn view(app: &App, model: &Model, frame: Frame) {
    // Setup window draw surface
    let draw = app.draw();

    // Draw to window background
    draw.background().color(GRAY);

    // Draw the frame
    model.solver.draw(&draw);

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}

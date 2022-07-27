// Import
use nannou::prelude::*;
use rand::Rng;
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

    // Make random blobs
    let mut rng = rand::thread_rng();

    // Set boundaries
    let position_bound = 0.0..100.0;
    let size_bound = 0.0..25.0;
    let color_bound = 0.0..1.0;

    for _ in 0..150 {
        blobs.push(Blob::new(
            vec2(
                rng.gen_range(position_bound.clone()),
                rng.gen_range(position_bound.clone()),
            ),
            rng.gen_range(size_bound.clone()),
            Rgb::new(
                rng.gen_range(color_bound.clone()),
                rng.gen_range(color_bound.clone()),
                rng.gen_range(color_bound.clone()),
            ),
        ));
    }

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

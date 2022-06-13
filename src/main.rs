// Import
use nannou::prelude::*;
use verlet_solver_nannou::*;

// Entry point
fn main() {
    nannou::app(model).update(update).run();
}

// App data struct
struct Model {
    slover: Slover,
}

// Ran before drawing
fn model(app: &App) -> Model {
    // Make new window
    app.new_window().size(512, 512).view(view).build().unwrap();

    let mut blobs = Vec::new();
    for _ in 0..1000 {
        blobs.push(Blob::new(Vec2::new(0.0, 0.0)));
    }

    // Return model struct
    Model {
        slover: Slover::new(blobs, vec2(0.0, -1.0)),
    }
}

// Update app data struct
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.slover.update(0.1);
}

// Real main function
fn view(app: &App, model: &Model, frame: Frame) {
    // Setup window draw surface
    let draw = app.draw();

    draw.background().color(BLACK);

    model.slover.show(&draw);

    draw.to_frame(app, &frame).unwrap();
}

extern crate nannou;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .fullscreen()
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let max_iterations = 500000;
    let scale = 500.0;
    let mut rng = rand::thread_rng();
    let win = app.window_rect();

    let offset_x = 0.0;
    let offset_y = -500.0;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut xn;
    let mut yn;
    
    let mut valid_points: Vec<Point2> = Vec::with_capacity(max_iterations);

    for _ in 0..max_iterations {
        let r: f32 = rng.gen_range(0.0..1.0);
        if r < 0.01 {
            xn = 0.0;
            yn = 0.16 * y;
        } else if r < 0.86 {
            xn = 0.85 * x + 0.04 * y;
            yn = -0.04 * x + 0.85 * y + 1.6
        } else if r < 0.93 {
            xn = 0.2 * x - 0.26 * y;
            yn = 0.23 * x + 0.22 * y + 1.6;
        } else {
            xn = -0.15 * x + 0.28 * y;
            yn = 0.26 * x + 0.24 * y + 0.44;
        }
        let px = (xn * scale) + offset_x;
        let py = (yn * scale + win.bottom()) + offset_y;

        if px > win.left() && px < win.right() && py > win.bottom() && py < win.top() {
            valid_points.push(pt2(px, py));
        }

        x = xn;
        y = yn;
    }

    for point in valid_points {
        draw.ellipse().xy(point).radius(0.1).color(GREEN);
    }

    draw.to_frame(app, &frame).unwrap();
}

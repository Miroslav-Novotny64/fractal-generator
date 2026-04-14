extern crate nannou;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    scale: f32,
    points: Vec<Point2>,
    offset: Vec2, 
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .fullscreen()
        .build()
        .unwrap();
    Model { 
        scale: 500.0, 
        points: Vec::new(),
        offset: vec2(0.0, -1000.0), 
        x: 0.0,
        y: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let iterations_per_frame = 500000;
    let scale = model.scale;
    let offset = model.offset;
    let mut rng = rand::thread_rng();
    let win = app.window_rect();

    let mut x = model.x;
    let mut y = model.y;
    let mut xn;
    let mut yn;
    
    for _ in 0..iterations_per_frame {
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
        let px = (xn * scale) + offset.x;
        let py = (yn * scale) + offset.y;

        if px > win.left() && px < win.right() && py > win.bottom() && py < win.top() {
            model.points.push(pt2(px, py));
        }

        x = xn;
        y = yn;
    }

    model.x = x;
    model.y = y;
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple: Some(event), .. } = event {
        match event {
            KeyPressed(key) => {
                match key {
                    Key::Equals | Key::Plus | Key::NumpadAdd => {
                        zoom(app, model, 1.1);
                        model.points.clear();
                        model.x = 0.0;
                        model.y = 0.0;
                    }
                    Key::Minus | Key::NumpadSubtract => {
                        zoom(app, model, 0.9);
                        model.points.clear();
                        model.x = 0.0;
                        model.y = 0.0;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn zoom(app: &App, model: &mut Model, factor: f32) {
    let win = app.window_rect();
    let center = vec2(win.x(), win.y());

    // Chceme, aby světový bod pod "center" zůstal po zoomu pod "center".
    // old: s = w * scale + offset
    // new: s = w * scale2 + offset2
    // pro s=center => offset2 = center - (center - offset) * (scale2/scale)
    let old_scale = model.scale;
    let new_scale = old_scale * factor;
    let ratio = new_scale / old_scale;

    model.offset = center - (center - model.offset) * ratio;
    model.scale = new_scale;

    model.points.clear();
    model.x = 0.0;
    model.y = 0.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for point in model.points.iter() {
        draw.ellipse().xy(*point).radius(0.1).color(GREEN);
    }

    draw.to_frame(app, &frame).unwrap();
}

extern crate nannou;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    scale: f32,
    points: Vec<(Point2, Rgba)>,
    offset: Vec2,
    x: f32,
    y: f32,
    fractal_name: String,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).fullscreen().build().unwrap();

    let fractal_name = "mandelbrot".to_string();

    // Scale a offset pro Mandelbrotovu množinu, aby byla hezky vidět hned po startu
    let scale = if fractal_name == "mandelbrot" {
        300.0
    } else {
        1000.0
    };
    let offset = if fractal_name == "mandelbrot" {
        vec2(-150.0, 0.0)
    } else if fractal_name == "fern" {
        vec2(0.0, -2000.0)
    } else if fractal_name == "sierpinski" {
        vec2(-300.0, -350.0)
    } else {
        vec2(0.0, 0.0)
    };

    Model {
        scale,
        points: Vec::new(),
        offset,
        x: 0.0,
        y: 0.0,
        fractal_name,
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

    if model.fractal_name == "fern" {
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
                model.points.push((pt2(px, py), srgba(0.0, 1.0, 0.0, 1.0).into()));
            }

            x = xn;
            y = yn;
        }
    } else if model.fractal_name == "sierpinski" {
        for _ in 0..iterations_per_frame {
            let r: f32 = rng.gen_range(0.0..1.0);
            if r < 0.3333333333333333 {
                xn = 0.5 * x;
                yn = 0.5 * y;
            } else if r < 0.6666666666666666 {
                xn = 0.5 * x + 0.5;
                yn = 0.5 * y;
            } else {
                xn = 0.5 * x + 0.25;
                yn = 0.5 * y + 0.5;
            }
            let px = (xn * scale) + offset.x;
            let py = (yn * scale) + offset.y;

            if px > win.left() && px < win.right() && py > win.bottom() && py < win.top() {
                model.points.push((pt2(px, py), srgba(0.0, 1.0, 0.0, 1.0).into()));
            }

            x = xn;
            y = yn;
        }
    } else if model.fractal_name == "mandelbrot" {
        if model.points.is_empty() {
            let bailout = 2.0;
            let max_iteration = 100;

            // Dva cykly pro výšku a šířku obrazovky
            for py_int in (win.bottom() as i32)..(win.top() as i32) {
                for px_int in (win.left() as i32)..(win.right() as i32) {
                    let px = px_int as f32;
                    let py = py_int as f32;

                    // Mapování na komplexní rovinu: C = complex(x,y)
                    let cx = (px - offset.x) / scale;
                    let cy = (py - offset.y) / scale;

                    // z = (0,0)
                    let mut zx = 0.0;
                    let mut zy = 0.0;
                    let mut počet = 0;

                    // repeat ... until abs(z) > bailout or počet > 1000
                    while zx * zx + zy * zy <= bailout * bailout && počet < max_iteration {
                        // z = (z^2) + C
                        let xtemp = zx * zx - zy * zy + cx;
                        zy = 2.0 * zx * zy + cy;
                        zx = xtemp;
                        počet += 1;
                    }

                    // plot (x, y, colour(počet))
                    let barva = colour(počet, max_iteration);
                    model.points.push((pt2(px, py), barva));
                }
            }
        }
    }

    model.x = x;
    model.y = y;
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(event),
        ..
    } = event
    {
        match event {
            KeyPressed(key) => match key {
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
            },
            _ => (),
        }
    }
}

// Funkce colour(počet) podle požadavku v pseudokódu
fn colour(počet: i32, max_iteration: i32) -> Rgba {
    if počet >= max_iteration {
        return rgba(0.0, 0.0, 0.0, 1.0);
    }
    // Modro-bílá barevná škála
    let t = počet as f32 / max_iteration as f32;
    // Začíná sytě modrou (t=0) a přechází do bílé (t=1)
    srgba(t, t, 0.5 + 0.5 * t, 1.0).into()
}

fn zoom(app: &App, model: &mut Model, factor: f32) {
    let win = app.window_rect();
    let center = vec2(win.x(), win.y());

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

    // Kreslení všech bodů s jejich barvami
    // Používáme radius(0.6) aby se pixely mírně překrývaly a nevznikaly mezery
    for (point, color) in model.points.iter() {
        draw.ellipse().xy(*point).radius(0.6).color(*color);
    }

    draw.to_frame(app, &frame).unwrap();
}

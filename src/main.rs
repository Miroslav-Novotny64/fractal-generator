use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    scale: f32,
    points: Vec<(Point2, Rgba)>,
    grid: HashSet<(i32, i32)>,
    offset: Vec2,
    x: f32,
    y: f32,
    fractal_name: String,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event) // Pro předání událostí do egui
        .key_pressed(key_pressed)   // Klávesové zkratky
        .fullscreen()
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let fractal_name = "fern".to_string();

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
        scale: 1000.0,
        points: Vec::new(),
        grid: HashSet::new(),
        offset,
        x: 0.0,
        y: 0.0,
        fractal_name,
        egui,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let win = app.window_rect();
    let mut reset_requested = false;
    let mut zoom_requested: Option<f32> = None;
    let mut fractal_changed = false;
    let mut changed = false;

    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();

        egui::Window::new("Fractal Settings").show(&ctx, |ui| {
            ui.spacing_mut().slider_width = 400.0;
            ui.label("Nastavení zobrazení:");

            // Výběr fraktálu
            egui::ComboBox::from_label("Vyberte fraktál")
                .selected_text(&model.fractal_name)
                .show_ui(ui, |ui| {
                    let options = [("fern", "Barnsley Fern"), ("sierpinski", "Sierpinski Triangle"), ("mandelbrot", "Mandelbrot Set")];
                    for (id, label) in options {
                        if ui.selectable_value(&mut model.fractal_name, id.to_string(), label).changed() {
                            fractal_changed = true;
                        }
                    }
                });

            ui.separator();
            ui.label("Pohyb:");
            ui.horizontal(|ui| {
                if ui.button("⬅").clicked() {
                    model.offset.x += 100.0;
                    changed = true;
                }
                ui.vertical(|ui| {
                    if ui.button("⬆").clicked() {
                        model.offset.y += 100.0;
                        changed = true;
                    }
                    if ui.button("⬇").clicked() {
                        model.offset.y -= 100.0;
                        changed = true;
                    }
                });
                if ui.button("➡").clicked() {
                    model.offset.x -= 100.0;
                    changed = true;
                }
            });

            ui.separator();
            ui.label("Zoom:");
            ui.horizontal(|ui| {
                if ui.button("➕ Zvětšit").clicked() {
                    zoom_requested = Some(model.scale * 1.1);
                }
                if ui.button("➖ Zmenšit").clicked() {
                    zoom_requested = Some(model.scale * 0.9);
                }
            });

            ui.separator();
            if ui.button("Resetovat zobrazení").clicked() {
                reset_requested = true;
            }
        });
    }
    if fractal_changed || reset_requested {
        reset_model_view(model);
        changed = true;
    }

    if let Some(new_scale) = zoom_requested {
        apply_zoom(model, win, new_scale);
        changed = true;
    }

    if changed {
        model.points.clear();
        model.grid.clear();
        model.x = 0.0;
        model.y = 0.0;
    }

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
                let gx = (px / 5.0).floor() as i32;
                let gy = (py / 5.0).floor() as i32;
                if model.grid.insert((gx, gy)) {
                    model.points.push((pt2(px, py), srgba(0.0, 1.0, 0.0, 1.0).into()));
                }
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
                let gx = (px / 5.0).floor() as i32;
                let gy = (py / 5.0).floor() as i32;
                if model.grid.insert((gx, gy)) {
                    model.points.push((pt2(px, py), srgba(0.52, 0.70, 0.23, 1.0).into()));
                }
            }

            x = xn;
            y = yn;
        }
    } else if model.fractal_name == "mandelbrot" {
        if model.points.is_empty() {
            let bailout = 2.0;
            let max_iteration = 100;

            for py_int in (win.bottom() as i32..win.top() as i32).step_by(3) {
                for px_int in (win.left() as i32..win.right() as i32).step_by(3) {
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

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let win = app.window_rect();
    match key {
        Key::Equals | Key::Plus | Key::NumpadAdd => {
            let new_scale = model.scale * 1.1;
            apply_zoom(model, win, new_scale);
        }
        Key::Minus | Key::NumpadSubtract => {
            let new_scale = model.scale * 0.9;
            apply_zoom(model, win, new_scale);
        }
        _ => (),
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

fn apply_zoom(model: &mut Model, win: Rect, new_scale: f32) {
    let center = vec2(win.x(), win.y());
    let old_scale = model.scale;
    let ratio = new_scale / old_scale;

    model.offset = center - (center - model.offset) * ratio;
    model.scale = new_scale;

    model.points.clear();
    model.grid.clear();
    model.x = 0.0;
    model.y = 0.0;
}

fn reset_model_view(model: &mut Model) {
    model.offset = if model.fractal_name == "mandelbrot" {
        vec2(-150.0, 0.0)
    } else if model.fractal_name == "fern" {
        vec2(0.0, -2000.0)
    } else if model.fractal_name == "sierpinski" {
        vec2(-300.0, -350.0)
    } else {
        vec2(0.0, 0.0)
    };
    model.scale = if model.fractal_name == "mandelbrot" {
        300.0
    } else {
        1000.0
    };
    model.points.clear();
    model.grid.clear();
    model.x = 0.0;
    model.y = 0.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for (point, color) in &model.points {
        draw.ellipse().xy(*point).radius(1.0).color(*color);
    }

    draw.to_frame(app, &frame).unwrap();

    // Vykreslení UI panelu
    model.egui.draw_to_frame(&frame).unwrap();
}

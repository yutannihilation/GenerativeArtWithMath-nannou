use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    clicks: i32,
    clicked_frame: u64,
    needs_refresh: bool,
}

impl Model {
    fn threshold(&self) -> f32 {
        500.0 / 2.0.powi(self.clicks)
    }
}

fn model(app: &App) -> Model {
    let wd = 500;
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(wd, wd + 30)
        .mouse_released(mouse_released)
        .resized(resized)
        .view(view)
        .build()
        .unwrap();

    Model {
        clicks: 0,
        clicked_frame: 0,
        needs_refresh: true,
    }
}

fn mouse_released(app: &App, model: &mut Model, _button: MouseButton) {
    println!("mouse pressed, threshold is {}", model.threshold());
    model.clicks += 1;
    model.clicked_frame = app.elapsed_frames();
    model.needs_refresh = true;
}

fn resized(app: &App, model: &mut Model, dim: Vector2) {
    println!(
        "Frame {}: window resized to ({}, {})",
        app.elapsed_frames(),
        dim.x,
        dim.y
    );
    model.clicked_frame = app.elapsed_frames();
    model.needs_refresh = true;
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.clicked_frame != app.elapsed_frames() {
        if model.needs_refresh {
            println!("Refreshed?");
        }
        model.needs_refresh = false;
    }
}

fn draw_rect(draw: &Draw, r: &Rect) {
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .stroke_weight(3.0)
        .stroke(STEELBLUE)
        .color(rgba(random(), random(), random(), 0.6));
}

fn div_square(draw: &Draw, win: &Rect, ratio: f32, threshold: f32) {
    let mut wd = if ratio > 1.0 { win.h() } else { win.w() };
    let mut r = Rect::from_w_h(wd, wd).bottom_left_of(*win);

    let mut itr = if ratio > 1.0 { 0 } else { 1 };
    while wd > 0.1 && itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                if wd > threshold {
                    div_rect(draw, &r, ratio, threshold);
                } else {
                    draw_rect(draw, &r);
                }
                if r.right() + r.w() >= win.right() {
                    break 'x;
                }
                r = r.right_of(r);
            }
            wd = win.right() - r.right();
            r = Rect::from_w_h(wd, wd).right_of(r).align_bottom_of(r);
        } else {
            'y: loop {
                if wd > threshold {
                    div_rect(draw, &r, ratio, threshold);
                } else {
                    draw_rect(draw, &r);
                }
                if r.top() + r.h() >= win.top() {
                    break 'y;
                }
                r = r.above(r);
            }
            wd = win.top() - r.top();
            r = Rect::from_w_h(wd, wd).above(r).align_left_of(r);
        }
    }
}

fn div_rect(draw: &Draw, win: &Rect, ratio: f32, threshold: f32) {
    let mut wd = win.w();
    let mut r = Rect::from_w_h(wd * ratio, wd).bottom_left_of(*win);

    let mut itr = 0;
    while wd > 0.1 && itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                div_square(draw, &r, ratio, threshold);
                if r.right() + r.w() >= win.right() + 0.1 {
                    break 'x;
                }
                r = r.right_of(r);
            }
            wd = win.right() - r.right();
            r = Rect::from_w_h(wd, wd / ratio)
                .right_of(r)
                .align_bottom_of(r);
        } else {
            'y: loop {
                div_square(draw, &r, ratio, threshold);
                if r.top() + r.h() >= win.top() + 0.1 {
                    break 'y;
                }
                r = r.above(r);
            }
            wd = win.top() - r.top();
            r = Rect::from_w_h(wd * ratio, wd).above(r).align_left_of(r);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // draw only once
    let draw = app.draw();

    if !model.needs_refresh {
        return;
    }

    println!("Drawing (Frame: {})", app.elapsed_frames());
    draw.background().color(WHITE);

    let num_a = 10;
    let num_b = 6;
    let ratio = num_b as f32 / num_a as f32;

    let win = app.window_rect();
    div_rect(&draw, &win, ratio, model.threshold());
    draw.to_frame(app, &frame).unwrap();
}

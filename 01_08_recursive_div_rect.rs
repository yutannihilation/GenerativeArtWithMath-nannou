use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    threshold: f32,
}

fn model(app: &App) -> Model {
    let wd = 500;
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(wd, wd + 30)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    Model {
        threshold: wd as f32,
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    println!("mouse pressed, threshold is {}", model.threshold);
    model.threshold *= 0.8;
}

fn update(_app: &App, _model: &mut Model, update: Update) {
    println!("{:?}", update);
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

    draw.background().color(WHITE);

    let num_a = 10;
    let num_b = 6;
    let ratio = num_b as f32 / num_a as f32;

    let win = app.window_rect();
    div_rect(&draw, &win, ratio, model.threshold);
    draw.to_frame(app, &frame).unwrap();
}

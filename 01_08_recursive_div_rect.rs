use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::sketch(view).size(500, 530).run();
}

fn draw_rect(draw: &Draw, r: &Rect) {
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .stroke_weight(3.0)
        .stroke(STEELBLUE)
        .color(rgba(random(), random(), random(), 0.6));
}

fn div_square(draw: &Draw, win: &Rect, ratio: f32) {
    let mut wd = if ratio > 1.0 { win.h() } else { win.w() };
    let mut r = Rect::from_w_h(wd, wd).bottom_left_of(*win);

    let mut itr = if ratio > 1.0 { 0 } else { 1 };
    while wd > 0.1 && itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                if wd > 100.0 {
                    div_rect(draw, &r, ratio);
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
                if wd > 100.0 {
                    div_rect(draw, &r, ratio);
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

fn div_rect(draw: &Draw, win: &Rect, ratio: f32) {
    let mut wd = win.w();
    let mut r = Rect::from_w_h(wd * ratio, wd).bottom_left_of(*win);

    let mut itr = 0;
    while wd > 0.1 && itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                div_square(draw, &r, ratio);
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
                div_square(draw, &r, ratio);
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

fn view(app: &App, frame: Frame) {
    // draw only once
    app.set_loop_mode(LoopMode::loop_once());
    let draw = app.draw();

    draw.background().color(WHITE);

    let num_a = 10;
    let num_b = 6;
    let ratio = num_b as f32 / num_a as f32;

    let win = app.window_rect();
    println!("window is {} x {}", win.w(), win.h());
    div_rect(&draw, &win, ratio);
    draw.to_frame(app, &frame).unwrap();
}

use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

fn view(app: &App, frame: Frame) {
    // draw only once
    app.set_loop_mode(LoopMode::loop_once());
    let draw = app.draw();

    draw.background().color(WHITE);

    let num_a = 10.0;
    let num_b = 6.0;
    let ratio = num_b / num_a;

    let win = app.window_rect();
    let mut wd = win.w();
    let x_offset = win.w() / 2.0;
    let y_offset = win.h() / 2.0;
    let mut r = Rect::from_w_h(wd * ratio, wd).bottom_left_of(win);

    let mut itr = 0;
    'outer: while wd > 0.1 && itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                draw.rect()
                    .xy(r.xy())
                    .wh(r.wh())
                    .stroke_weight(3.0)
                    .stroke(STEELBLUE)
                    .color(rgba(random(), random(), random(), 0.6));
                if r.right() + x_offset + r.w() >= win.w() + 0.1 {
                    break 'x;
                }
                r = r.right_of(r);
            }
            wd = win.w() - (r.right() + x_offset);
            r = Rect::from_w_h(wd, wd / ratio)
                .right_of(r)
                .align_bottom_of(r);
        } else {
            'y: loop {
                draw.rect()
                    .xy(r.xy())
                    .wh(r.wh())
                    .stroke_weight(3.0)
                    .stroke(STEELBLUE)
                    .color(rgba(random(), random(), random(), 0.6));
                if r.top() + y_offset + r.h() >= win.h() + 0.1 {
                    break 'y;
                }
                r = r.above(r);
            }
            wd = win.h() - (r.top() + y_offset);
            r = Rect::from_w_h(wd * ratio, wd).above(r).align_left_of(r);
        }
        if wd == 0.0 {
            break 'outer;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

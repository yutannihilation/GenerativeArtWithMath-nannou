use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::sketch(view).size(500, 500).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let num_a_tmp = 10.0;
    let num_b_tmp = 7.0;
    let scalar = 50.0;
    let num_a = num_a_tmp * scalar;
    let num_b = num_b_tmp * scalar;

    let mut wd = num_b;
    let win = app.window_rect();
    let x_offset = win.w() / 2.0;
    let y_offset = win.h() / 2.0;
    let mut r = Rect::from_w_h(wd, wd).bottom_left_of(win);

    let mut itr = 0;
    'outer: while itr < 100 {
        itr += 1;
        if itr % 2 == 1 {
            'x: loop {
                draw.rect()
                    .xy(r.xy())
                    .wh(r.wh())
                    .stroke_weight(3.0)
                    .stroke(STEELBLUE)
                    .color(rgba(random(), random(), random(), 0.6));
                if r.right() + x_offset + r.w() >= num_a {
                    break 'x;
                }
                r = r.right_of(r);
            }
            wd = num_a - (r.right() + x_offset);
            r = Rect::from_w_h(wd, wd).right_of(r).align_bottom_of(r);
        } else {
            'y: loop {
                draw.rect()
                    .xy(r.xy())
                    .wh(r.wh())
                    .stroke_weight(3.0)
                    .stroke(STEELBLUE)
                    .color(rgba(random(), random(), random(), 0.6));
                if r.top() + y_offset + r.h() >= num_b {
                    break 'y;
                }
                r = r.above(r);
            }
            wd = num_b - (r.top() + y_offset);
            r = Rect::from_w_h(wd, wd).above(r).align_left_of(r);
        }
        if wd == 0.0 {
            break 'outer;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

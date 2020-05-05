use nannou::prelude::*;

mod fib;
use crate::fib::Fibonacci;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    clicks: u32,
    current_frame: u64,
    needs_refresh: bool,
    scale: f32,
    rotate: f32,
}

fn model(app: &App) -> Model {
    let wd = 500;
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(wd, wd + 30)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        clicks: 0,
        current_frame: 1,
        needs_refresh: true,
        scale: 1.0,
        rotate: 0.0,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        Resized(_) => {}
        MouseReleased(_) => {
            model.clicks += 1;
        }
        _ => return,
    }

    model.current_frame = app.elapsed_frames();
    model.needs_refresh = true;
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.current_frame != app.elapsed_frames() {
        model.needs_refresh = false;
    }
    model.scale *= 0.995;
    model.rotate = 2.0 * PI * (app.elapsed_frames() % 200) as f32 / 200.0
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().rotate(model.rotate).scale(model.scale);

    draw.background().color(WHITE);

    let fib = Fibonacci::new(model.clicks);
    let mut r = Rect::from_w_h(0.0, 0.0);
    // move above
    let dir = vec2(1.0, 0.0);
    for (n, w) in fib.enumerate() {
        match n {
            0 => r = Rect::from_w_h(1.0, 1.0),
            _ => {
                let pt = r.corner_at_index((n % 4) as u8).unwrap();
                let rot = ((4 - n) % 4) as f32 / 2.0 * PI;
                r = Rect::from_corners(
                    pt,
                    pt + dir.rotate(rot) * w + dir.rotate(rot + PI / 2.0) * w,
                );
            }
        }
        draw.rect()
            .xy(r.xy())
            .wh(r.wh())
            .stroke_weight(40.0)
            .stroke_color(BLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}

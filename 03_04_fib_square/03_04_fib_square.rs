use nannou::prelude::*;
use nannou::ui::prelude::*;

mod fib;
use crate::fib::Fibonacci;

use geom::rect::Rect;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    ui: Ui,
    widgets: Widgets,
    clicks: u32,
    current_frame: u64,
    needs_refresh: bool,
    scale: f32,
    rotation: f32,
    // stroke: f32,
}

struct Widgets {
    scale: widget::Id,
    rotation: widget::Id,
    // stroke: widget::Id,
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

    let mut ui = app.new_ui().build().unwrap();

    let widgets = Widgets {
        scale: ui.generate_widget_id(),
        rotation: ui.generate_widget_id(),
        // stroke: ui.generate_widget_id(),
    };

    Model {
        ui,
        widgets,
        clicks: 0,
        current_frame: 1,
        needs_refresh: true,
        scale: 1.0,
        rotation: 0.0,
        // stroke: 5.0,
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
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.scale.log10(), -4.0, 2.0)
        .top_left_with_margin(20.0)
        .label("Scale")
        .set(model.widgets.scale, ui)
    {
        model.scale = 10.0.powf(value);
    }

    for value in slider(model.rotation, 0.0, 2.0 * PI)
        .down(10.0)
        .label("Rotation")
        .set(model.widgets.rotation, ui)
    {
        model.rotation = value;
    }
    model.scale *= 0.998;
    model.rotation += 2.0 * PI / 300.0;
    model.rotation %= 2.0 * PI;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw().rotate(model.rotation).scale(model.scale);

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
    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

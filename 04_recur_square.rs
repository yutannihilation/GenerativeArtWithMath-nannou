use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    corners: i32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(700, 700)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();
    Model { corners: 3 }
}

fn mouse_pressed(app: &App, model: &mut Model, btn: MouseButton) {
    match btn {
        MouseButton::Left => model.corners += 1,
        MouseButton::Right => model.corners -= 1,
        _ => {}
    }
}

fn get_gapped_quad(p: &Vec<Point2>, gap: f32) -> Vec<Point2> {
    (0..p.len())
        .map(|i| {
            let i_next = (i + 1) % p.len();
            let v = p[i_next] - p[i];
            p[i] + v * gap
        })
        .collect()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let radius = 300.0;
    let mut points: Vec<Point2> = (0..model.corners)
        .map(|i| {
            let radian = 2.0 * PI * i as f32 / model.corners as f32;
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            pt2(x, y)
        })
        .collect();

    loop {
        draw.polygon()
            .stroke_weight(2.0)
            .stroke_color(BLACK)
            .points(points.iter().copied());

        let gap = 0.50 + 0.499 * (app.elapsed_frames() as f32 / 200.0).cos();
        points = get_gapped_quad(&points, gap);

        if (points[0] - points[1]).magnitude() < 3.0 {
            break;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

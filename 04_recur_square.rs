use nannou::geom::quad::Quad;
use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {}

fn model(app: &App) -> Model {
    Model {}
}

fn get_gapped_point(q: &Quad<Point2>, gap: f32, i: usize) -> Point2 {
    let i_next = (i + 1) % 4;
    let v = q[i_next] - q[i];
    q[i] + v * gap
}

fn get_gapped_quad(q: &Quad<Point2>, gap: f32) -> Quad<Point2> {
    let q = Quad([
        get_gapped_point(q, gap, 0),
        get_gapped_point(q, gap, 1),
        get_gapped_point(q, gap, 2),
        get_gapped_point(q, gap, 3),
    ]);
    q
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let r = Rect::from_w_h(600.0, 600.0);
    let mut q = r.corners();

    loop {
        draw.quad()
            .stroke_weight(2.0)
            .stroke_color(BLACK)
            .points(q[0], q[1], q[2], q[3]);

        let gap = 0.50 + 0.499 * (app.elapsed_frames() as f32 / 200.0).cos();
        q = get_gapped_quad(&q, gap);
        if (q[0] - q[1]).magnitude() < 10.0 {
            break;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

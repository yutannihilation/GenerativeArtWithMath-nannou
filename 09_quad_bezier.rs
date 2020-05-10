use nannou::prelude::*;

const STEP_POINT: i32 = 5;
const STEP_LINE: i32 = 20;
const STEP_LINE_ADDITION: i32 = 10;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    index: usize,
    last_clicked_frame: u64,
    points: [Point2; 3],
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(500, 500)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    Model {
        index: 0,
        last_clicked_frame: 0,
        points: [pt2(0.0, 0.0), pt2(0.0, 0.0), pt2(0.0, 0.0)],
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {}
        _ => return,
    }

    model.index %= 3;
    model.points[model.index % 3] = pt2(app.mouse.x, app.mouse.y);
    model.index += 1;
    model.last_clicked_frame = app.elapsed_frames();
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.index >= 3 && app.elapsed_frames() - model.last_clicked_frame > 500 {
        model.index = 0;
    }
}

fn draw_point(draw: &Draw, point: &Point2) -> i32 {
    draw.ellipse().xy(*point).w_h(10.0, 10.0).color(GREY);

    return STEP_POINT;
}

fn draw_line(draw: &Draw, start: &Point2, end: &Point2, cur_step: i32, steps: i32) -> i32 {
    let vec = *end - *start;
    let ratio = clamp(cur_step as f32 / steps as f32, 0.0, 1.0);
    draw.line()
        .start(*start)
        .end(*start + vec * ratio)
        .weight(1.8)
        .color(GREY);

    return STEP_LINE;
}

fn draw_additional_lines(
    draw: &Draw,
    start: &Point2,
    mid: &Point2,
    end: &Point2,
    steps: i32,
) -> i32 {
    let mut steps_left = steps;

    let vec1: Vector2 = (*mid - *start) / 10.0;
    let vec2: Vector2 = (*end - *mid) / 10.0;

    for i in 1..10 {
        draw_line(
            draw,
            &(*start + vec1 * i as f32),
            &(*mid + vec2 * i as f32),
            steps_left,
            STEP_LINE_ADDITION,
        );
        steps_left -= 7;
        if steps_left < 0 {
            break;
        }
    }
    return STEP_LINE_ADDITION + 7 * 9;
}

fn do_draw(draw: &Draw, points: &[Point2; 3], steps: u64) {
    let mut steps_left = steps as i32;
    steps_left -= draw_point(draw, &points[0]);
    if steps_left <= 0 {
        return;
    }

    steps_left -= draw_point(draw, &points[1]);
    if steps_left <= 0 {
        return;
    }

    steps_left -= draw_point(draw, &points[2]);
    if steps_left <= 0 {
        return;
    }

    steps_left -= draw_line(draw, &points[0], &points[1], steps_left, STEP_LINE);
    if steps_left <= 0 {
        return;
    }

    steps_left -= draw_line(draw, &points[1], &points[2], steps_left, STEP_LINE);
    if steps_left <= 0 {
        return;
    }

    steps_left -= draw_additional_lines(draw, &points[0], &points[1], &points[2], steps_left);
    if steps_left <= 0 {
        return;
    }

    let builder = nannou::geom::path::Builder::new();

    let path = builder
        .move_to(points[0])
        .quadratic_bezier_to(points[1], points[2])
        .build();

    draw.path()
        .stroke()
        .weight(3.0)
        .rgba(0.99, 0.03, 0.24, 0.87)
        .events(path.iter());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    match model.index {
        3 => do_draw(
            &draw,
            &model.points,
            app.elapsed_frames() - model.last_clicked_frame,
        ),
        _ => {}
    }
    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}

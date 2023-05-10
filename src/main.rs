use nannou::prelude::*;

const STEPS: f32 = 20.;

const LINE_WEIGHT: f32 = 4.5;

struct Model;

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let mut draw = app.draw();
    let background_color = Rgb8::new(0, 83, 156);
    draw.background().color(background_color);

    let start = Point2::new(0. - window.x.end, 0. + window.y.end);
    // im so sorry i know i know
    for x in ((start.x as i32)..(window.x.end as i32)).step_by(STEPS as usize) {
        for y in ((-window.y.end as i32)..(start.y as i32 + STEPS as i32))
            .step_by(STEPS as usize)
            .rev()
        {
            draw_line(&mut draw, Point2::new(x as f32, y as f32), STEPS);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_line(draw: &mut Draw, start: Point2, step: f32) {
    let line_color = Rgb8::new(238, 164, 127);
    let fifty_fifty = random_range(0.0, 1.0) > 0.5;
    if fifty_fifty {
        draw.line()
            .start(start)
            .end(start + Point2::new(step, -step))
            .color(line_color)
            .weight(LINE_WEIGHT);
        draw.ellipse()
            .x_y(start.x + step, start.y - step)
            .radius(LINE_WEIGHT / 2.)
            .color(line_color);
    } else {
        draw.line()
            .start(start + Point2::new(step, 0.))
            .end(start + Point2::new(0., -step))
            .color(line_color)
            .weight(LINE_WEIGHT);
        draw.ellipse()
            .x_y(start.x + step, start.y - step)
            .radius(LINE_WEIGHT / 2.)
            .color(line_color);
    }
}

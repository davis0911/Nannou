//Nannou program to display a moving sine curve

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    phase: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();// used to display the output on a new screem
    Model { _window, phase: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.phase += -0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let y = (model.phase ).sin() * 30.0;//set the position of the red dot

    //Draw the X-Axis
    draw.line().start(pt2(win.left(), 0.0)).end(pt2(win.right(), 0.0)).stroke_weight(2.0).color(WHITE);

    //Draw the Y-Axis
    draw.line().start(pt2(0.0, win.top())).end(pt2(0.0, win.bottom())).stroke_weight(2.0).color(WHITE);

    let points = (0..450).map(|i| {
        let x = i as f32 / 15.0;
        let y = (x + model.phase).sin();
        pt2(x + 5.0, y+5.0) * 30.0 //setting the scale
    });

    draw.polyline().weight(3.0).points(points).color(WHITE);//Draw the sine curve by joining all the points together
    draw.ellipse().x_y(150.0, y+150.0).radius(5.0).color(RED);

    //Draw the frame
    draw.to_frame(app, &frame).unwrap();
}
use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mouse_x: f32, //variable to track movement of mouse on the screen
}

fn model(app: &App) -> Model {
    app.new_window() .view(view) .build().unwrap();//opens up a new window to display the output
    Model { mouse_x: 0.0 } //initializa mouse_z to 0
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mouse_x = app.mouse.x; //updating mouse_x based on the movement of the mouse
}

//Defining a recursive function to draw a fractal tree
fn draw_branch(app: &App,draw: &Draw, start: Point2, length: f32, angle: f32, mouse_x: f32) {
    if length < 4.0 {
        return;
    }//if the length of the branch is less than 4 units then the tree stops branching

    let win=app.window_rect();
    let end_x = start.x + length * angle.cos();
    let end_y = start.y + length * angle.sin();
    let end = pt2(end_x, end_y);
    draw.line()
        .start(start)
        .end(end)
        .weight(2.0) 
        .color(WHITE);
    let new_length = length * 0.67; 
    //Changing the angle of branching based on the movement of the mouse
    let branch_angle = map_range(mouse_x, win.left(), win.right(), -25.0, 135.0).to_radians();

    draw_branch(app,draw, end, new_length, angle + branch_angle, mouse_x);//recursive function
    draw_branch(app,draw, end, new_length, angle - branch_angle, mouse_x);//recursive function
    }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let start = pt2(0.0, -300.0);//set starting point of base branch
    let length = 200.0;//set length of base branch to 200 units
    let angle = 90.0.to_radians(); //sets the base branch to be vertical
    draw_branch(app,&draw, start, length, angle, model.mouse_x);
    draw.to_frame(app, &frame).unwrap();
}

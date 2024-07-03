use nannou::prelude::*;

const OFFSET:f32=PI/24.0;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    angle_x: f32,
    angle_y: f32,
    angle:f32
  
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model { angle_x: 0.0,angle_y: 0.0,angle:0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
    model.angle_x += 0.035;
    model.angle_y += 0.035;
    model.angle+=0.035;
}

fn view(app: &App, model: & Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    
    let size = 200.0;
    let half_size = size / 2.0;
    //Defining all the vertices of a cube
    let vertices = vec![
        pt3(-half_size, -half_size, half_size),
        pt3(half_size, -half_size, half_size),
        pt3(half_size, half_size, half_size),
        pt3(-half_size, half_size, half_size),
        pt3(-half_size, -half_size, -half_size),
        pt3(half_size, -half_size, -half_size),
        pt3(half_size, half_size, -half_size),
        pt3(-half_size, half_size, -half_size)
    ]; 
        // Apply rotation around the x-axis
    let rotation_x = model.angle_x + (OFFSET as f32);
    let cos_theta_x = rotation_x.cos();
    let sin_theta_x = rotation_x.sin();

        //Apply rotation around the y axis
    let rotation_y = model.angle_x / 2.0 + (OFFSET as f32);
    let cos_theta_y = rotation_y.cos();
    let sin_theta_y = rotation_y.sin();

        let rotate_point = |point: Point3| -> Point3 {
             
            let x = point.x;
            let y = point.y;
            let z = point.z;
    
           //Rotating about the X-Axis
            let rotated_y = y * cos_theta_x - z * sin_theta_x;
            let rotated_z = y * sin_theta_x + z * cos_theta_x;
    
          //Rotating about the Y-Axis
            let rotated_x = x * cos_theta_y + rotated_z * sin_theta_y;
            let rotated_z = -x * sin_theta_y + rotated_z * cos_theta_y;
    
            pt3(rotated_x, rotated_y, rotated_z)
        };
    
        let rotated_vertices: Vec<_> = vertices.iter().map(|&p| rotate_point(p)).collect();

        //Drawing each face of the cube
        draw.quad()
        .points(
            rotated_vertices[0],
            rotated_vertices[1],
            rotated_vertices[2],
            rotated_vertices[3],  
        );//.x_y_z(0.0,0.0,0.0);
       // .rotate(model.angle);
        
        
        //.color(custom_color);
        
          draw.quad()
        .points(
            rotated_vertices[4],
            rotated_vertices[5],
            rotated_vertices[6],
            rotated_vertices[7],
        );//.x_y_z(0.0,0.0,0.0);
       // .rotate(model.angle);
        //.color(custom_color);
     draw.quad()
        .points(
            rotated_vertices[2],
            rotated_vertices[3],
            rotated_vertices[7],
            rotated_vertices[6],
         );//.x_y_z(0.0,0.0,0.0);
         //.rotate(model.angle);
        //.color(custom_color);
    draw.quad()
        .points(
            rotated_vertices[0],
            rotated_vertices[1],
            rotated_vertices[5],
            rotated_vertices[4],
           );//.x_y_z(0.0,0.0,0.0);
           //.rotate(model.angle);
        //.color(custom_color);
    draw.quad()
        .points(
            rotated_vertices[1],
            rotated_vertices[2],
            rotated_vertices[6],
            rotated_vertices[5],
        );//.x_y_z(0.0,0.0,0.0);
        //.rotate(model.angle);
        //.color(custom_color);
    draw.quad()
        .points(
            rotated_vertices[2],
            rotated_vertices[3],
            rotated_vertices[7],
            rotated_vertices[4],
           );//.x_y_z(0.0,0.0,0.0);
           //.rotate(model.angle);
        //.color(custom_color);

        //Draw the frame
    draw.to_frame(app, &frame).unwrap();
    }

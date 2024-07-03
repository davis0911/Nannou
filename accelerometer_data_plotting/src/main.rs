//Nannou program to visualize accelerometer data
use nannou::prelude::*;

struct Model {
    imu_data: Vec<Vector3>,
    current_frame: usize,
    frame_counter: usize,
}

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
     app.new_window().view(view).build().unwrap();//opens up a new window to display the output

    // Parse the IMU data
    let imu_data = vec![
        Vector3 { x: 84.0, y: -104.0, z: -1032.0 },
        Vector3 { x: 92.0, y: -104.0, z: -1056.0 },
        Vector3 { x: 80.0, y: -96.0, z: -1012.0 },
        Vector3 { x: 84.0, y: -104.0, z: -1032.0 },
        Vector3 { x: 92.0, y: -104.0, z: -1056.0 },
        Vector3 { x: 76.0, y: -100.0, z: -1024.0 },
        Vector3 { x: 108.0, y: -92.0, z: -1024.0 },
        Vector3 { x: 56.0, y: -124.0, z: -1044.0 },
        Vector3 { x: 84.0, y: -116.0, z: -1056.0 },
        Vector3 { x: 100.0, y: -112.0, z: -1092.0 },
        Vector3 { x: 72.0, y: -100.0, z: -1008.0 },
        Vector3 { x: 32.0, y: -120.0, z: -1000.0 },
        Vector3 { x: 52.0, y: -132.0, z: -1064.0 },
        Vector3 { x: 100.0, y: -84.0, z: -1000.0 },
        Vector3 { x: 116.0, y: -96.0, z: -1012.0 },
        Vector3 { x: 108.0, y: -100.0, z: -1032.0 },
        Vector3 { x: 100.0, y: -108.0, z: -1052.0 },
        Vector3 { x: 96.0, y: -144.0, z: -1112.0 },
        Vector3 { x: -40.0, y: -200.0, z: -1108.0 },
        Vector3 { x: -28.0, y: -152.0, z: -1196.0 },
        Vector3 { x: -140.0, y: -224.0, z: -1164.0 },
        Vector3 { x: -312.0, y: -208.0, z: -784.0 },
        Vector3 { x: -388.0, y: -252.0, z: -1000.0 },
        Vector3 { x: -516.0, y: -288.0, z: -664.0 },
        Vector3 { x: -680.0, y: -308.0, z: -736.0 },
        Vector3 { x: -696.0, y: -296.0, z: -520.0 },
        Vector3 { x: -808.0, y: -300.0, z: -344.0 },
        Vector3 { x: -1032.0, y: -252.0, z: -828.0 },
        Vector3 { x: -908.0, y: -272.0, z: -104.0 },
        Vector3 { x: -1172.0, y: -284.0, z: -288.0 },
        Vector3 { x: -832.0, y: -396.0, z: 228.0 },
        Vector3 { x: -900.0, y: -348.0, z: 296.0 },
        Vector3 { x: -800.0, y: -412.0, z: 436.0 },
        Vector3 { x: -764.0, y: -384.0, z: 444.0 },
        Vector3 { x: -644.0, y: -296.0, z: 476.0 },
        Vector3 { x: -948.0, y: -388.0, z: 628.0 },
        Vector3 { x: -760.0, y: -284.0, z: 524.0 },
        Vector3 { x: -604.0, y: -328.0, z: 592.0 },
        Vector3 { x: -572.0, y: -268.0, z: 720.0 },
        Vector3 { x: -660.0, y: -216.0, z: 784.0 },
        Vector3 { x: -608.0, y: -196.0, z: 876.0 },
        Vector3 { x: -424.0, y: -156.0, z: 876.0 },
        Vector3 { x: -492.0, y: -156.0, z: 992.0 },
        Vector3 { x: -352.0, y: -100.0, z: 940.0 },
        Vector3 { x: -324.0, y: -180.0, z: 996.0 },
        Vector3 { x: -144.0, y: -144.0, z: 896.0 },
        Vector3 { x: -328.0, y: -132.0, z: 1008.0 },
        Vector3 { x: -376.0, y: -76.0, z: 1032.0 },
        Vector3 { x: -348.0, y: -124.0, z: 880.0 },
        Vector3 { x: -400.0, y: -172.0, z: 948.0 },
        Vector3 { x: -544.0, y: -108.0, z: 1012.0 },
        Vector3 { x: -856.0, y: 28.0, z: 524.0 },
        Vector3 { x: -924.0, y: -120.0, z: 472.0 },
        Vector3 { x: -964.0, y: 16.0, z: -104.0 },
        Vector3 { x: -1072.0, y: -100.0, z: -148.0 },
        Vector3 { x: -968.0, y: -56.0, z: -556.0 },
        Vector3 { x: -648.0, y: -156.0, z: -812.0 },
        Vector3 { x: -692.0, y: -164.0, z: -1056.0 },
        Vector3 { x: -292.0, y: 32.0, z: -732.0 },
        Vector3 { x: -152.0, y: -152.0, z: -1072.0 },
        Vector3 { x: 184.0, y: -140.0, z: -1136.0 },
        Vector3 { x: 280.0, y: -12.0, z: -928.0 },
        Vector3 { x: 692.0, y: 32.0, z: -856.0 },
        Vector3 { x: 768.0, y: 100.0, z: -696.0 },
        Vector3 { x: 980.0, y: 72.0, z: -656.0 },
        Vector3 { x: 900.0, y: 76.0, z: -544.0 },
        Vector3 { x: 892.0, y: -8.0, z: -560.0 },
        Vector3 { x: 884.0, y: 56.0, z: -376.0 },
        Vector3 { x: 1436.0, y: 88.0, z: -344.0 },
        Vector3 { x: 788.0, y: -16.0, z: -276.0 },
        Vector3 { x: 732.0, y: -28.0, z: 116.0 },
        Vector3 { x: 1280.0, y: 32.0, z: 16.0 },
        Vector3 { x: 1004.0, y: 4.0, z: 196.0 },
        Vector3 { x: 856.0, y: 88.0, z: 380.0 },
        Vector3 { x: 732.0, y: 112.0, z: 504.0 },
        Vector3 { x: 764.0, y: 116.0, z: 444.0 },
        Vector3 { x: 732.0, y: 100.0, z: 600.0 },
        Vector3 { x: 524.0, y: 168.0, z: 852.0 },
        Vector3 { x: 412.0, y: 216.0, z: 924.0 },
        Vector3 { x: 616.0, y: 208.0, z: 816.0 },
        Vector3 { x: 408.0, y: 316.0, z: 832.0 },
        Vector3 { x: 316.0, y: 292.0, z: 964.0 },
    ];

    Model {
        imu_data,
        current_frame: 0,
        frame_counter: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    model.frame_counter += 1;

   //Reducing the frame rate by one third for smooth display of data
    if model.frame_counter % 3 == 0 {
        model.current_frame = (model.current_frame + 1) % model.imu_data.len();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    //Draw the Y-Axis
    draw.line().start(pt2(0.0, 0.0)).end(pt2(0.0, 350.0)).color(GREEN).stroke_weight(2.0);
    draw.line().start(pt2(0.0, 0.0)).end(pt2(0.0, -350.0)).color(GREEN).stroke_weight(2.0);


    let cosine = (PI / 4.0).cos();
    let sine = (PI / 4.0).sin();
    //Draw the Z-Axis
    draw.line().start(pt2(0.0, 0.0)).end(pt2(350.0 * cosine, 350.0 * sine)).color(RED).stroke_weight(2.0);
    draw.line().start(pt2(0.0, 0.0)).end(pt2(-350.0 * cosine, -350.0 * sine)).color(RED).stroke_weight(2.0);

    //Draw the X-Axis
    draw.line().start(pt2(0.0, 0.0)).end(pt2(350.0 , 0.0)).color(BLUE).stroke_weight(2.0);
    draw.line().start(pt2(0.0, 0.0)).end(pt2(-350.0 , 0.0)).color(BLUE).stroke_weight(2.0);

    // Get the current IMU data
    let imu_reading = &model.imu_data[model.current_frame];

    // Normalize the IMU data
    let norm = (imu_reading.x.powi(2) + imu_reading.y.powi(2) + imu_reading.z.powi(2)).sqrt();
    let normalized = if norm != 0.0 {
        Vector3 {
            x: imu_reading.x / norm,
            y: imu_reading.y / norm,
            z: imu_reading.z / norm,
        }
    } else {
        Vector3 {
            x: imu_reading.x,
            y: imu_reading.y,
            z: imu_reading.z,
        }
    };

    // Define the arrow length
    let arrow_length = 300.0;
    
    //Apply the transformation to convert a 3D point into a 2D point based on how the pseudo axes have been defined
    let end = pt2((normalized.x-normalized.z*0.707) * arrow_length, (normalized.y-normalized.z*0.707)* arrow_length);

    // Draw the arrow
    draw.arrow()
        .start_cap_round()
        .end_cap_round()
        .color(PINK)
        .start(pt2(0.0, 0.0))
        .end(end);

    // Draw the frame
    draw.to_frame(app, &frame).unwrap();
}



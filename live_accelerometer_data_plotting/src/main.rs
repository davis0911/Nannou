//Nannou program to take real time accelerometer data and plot it 

use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use nannou::prelude::*;

struct Model {
    imu_data: Arc<Mutex<Vec<Vec3>>>,
    current_frame: usize,
    frame_counter: usize,
}



fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();//opens up a new window to display the output
    let imu_data = Arc::new(Mutex::new(vec![vec3 ( 0.0, 0.0, 0.0 )]));
    let current_frame = 0;
    let frame_counter = 0;

    // Start a thread for serial communication
    let imu_data_clone = Arc::clone(&imu_data);
    thread::spawn(move || {
        let port_name = "COM5"; // Since the COM5 port is used
        let baud_rate = 115200;//Defining the baud rate based on the device(bbc microbit) used

        // Attempt to open the serial port
        match serialport::new(port_name, baud_rate)
            .timeout(Duration::from_secs(5)) // Timeout for reading
            .open()
        {
            Ok(mut port) => {
                println!("Successfully connected to {}", port_name);

                // Buffer to store incoming data
                let mut serial_buf: Vec<u8> = vec![0; 128];

                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            // Process the received bytes
                            if t > 0 {
                                let data_str = String::from_utf8_lossy(&serial_buf[..t]);
                                let mut iter = data_str.split(',');
                                if let (Some(a_str), Some(b_str), Some(c_str)) =
                                    (iter.next(), iter.next(), iter.next())
                                {
                                    if let (Ok(a), Ok(b), Ok(c)) = (
                                        a_str.trim().parse::<i32>(),
                                        b_str.trim().parse::<i32>(),
                                        c_str.trim().parse::<i32>(),
                                    ) {
                                        println!("Parsed integers: x={}, y={}, z={}", a, b, c);
                                        let incoming_vector = vec3(
                                           a as f32,
                                             b as f32,
                                            c as f32,
                                        );
                                        let mut data = imu_data_clone.lock().unwrap();
                                        data.push(incoming_vector);
                                    } else {
                                        eprintln!("Failed to parse integers from received data");
                                    }
                                } else {
                                    eprintln!("Received data format error: not enough parts");
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            println!("Timeout reading from serial port");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Error reading from serial port: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open serial port {}: {}", port_name, e);
                std::process::exit(1);
            }
        }
    });

    Model {
        imu_data,
        current_frame,
        frame_counter,
        
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
    model.frame_counter += 1;
    //Reducing the frame rate by one fourth for smooth display of data
        if model.frame_counter % 4 == 0  {
            model.current_frame = model.current_frame + 1;
        
        }
    }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // Get the current IMU data
    let imu_data = model.imu_data.lock().unwrap();
    if !imu_data.is_empty()  { 
        let imu_reading = &imu_data[model.current_frame];

        // Normalize the IMU data
        let norm = (imu_reading.x.powi(2) + imu_reading.y.powi(2) + imu_reading.z.powi(2)).sqrt();
        let normalized = if norm != 0.0 {
            vec3(
                 imu_reading.x / norm,
                 imu_reading.y / norm,
                 imu_reading.z / norm,
            )
        } else {
            vec3 (
                 imu_reading.x,
                 imu_reading.y,
                 imu_reading.z,
            )
        };

        // Define the arrow length
        let arrow_length = 200.0;
        let end = pt2(
            (normalized.x - normalized.z * 0.707) * arrow_length,
            (normalized.y - normalized.z * 0.707) * arrow_length,
        );

        // Draw the arrow
        draw.arrow()
            .start_cap_round()
            .end_cap_round()
            .color(PINK)
            .start(pt2(0.0, 0.0))
            .end(end);
    }

    // Draw the frame
    draw.to_frame(app, &frame).unwrap();
}

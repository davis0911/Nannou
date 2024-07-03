//Nannou program for circle collision with swapping velocities

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}
struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mass: f32,
}

impl Ball {
    fn new(x: f32, y: f32, r: f32) -> Self {
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)).normalize() * 3.0;
        let mass = r * 0.1;
        Ball {
            position: vec2(x, y),
            velocity,
            radius: r,
            mass,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    //Function to check collision of a ball with the boundary
    fn check_boundary_collision(&mut self, win: &Rect) {
        if self.position.x > win.right() - self.radius {
            self.position.x = win.right() - self.radius;
            self.velocity.x *= -1.0;
        } else if self.position.x < win.left() + self.radius {
            self.position.x = win.left() + self.radius;
            self.velocity.x *= -1.0;
        }

        if self.position.y > win.top() - self.radius {
            self.position.y = win.top() - self.radius;
            self.velocity.y *= -1.0;
        } else if self.position.y < win.bottom() + self.radius {
            self.position.y = win.bottom() + self.radius;
            self.velocity.y *= -1.0;
        }
    }

    //Function to check collision between two balls
    fn check_collision(&mut self, other: &mut Ball) {
        // Get distances between the balls components
        let distance_vect = other.position - self.position;

        // Calculate magnitude of the vector separating the balls
        let distance_mag = (distance_vect.x.powi(2) + distance_vect.y.powi(2)).sqrt();

        // Minimum distance before they are touching
        let min_distance = self.radius + other.radius;

        if distance_mag < min_distance {
            let distance_correction = (min_distance - distance_mag) / 2.0;
            let correction_vector = distance_vect.normalize() * distance_correction;
            other.position += correction_vector;
            self.position -= correction_vector;

            // get angle of distanceVect
            let theta = distance_vect.angle();

            //calculate tringnometric values of the angle
            let sine = theta.sin();
            let cosine = theta.cos();

    // b_Temp will hold rotated ball positions. You just need to worry about bTemp[1] position
            let mut b_temp = [vec2(0.0, 0.0); 2];
            b_temp[1] = vec2(cosine * distance_vect.x + sine * distance_vect.y, cosine * distance_vect.y - sine * distance_vect.x);

            // rotate Temporary velocities
            let mut v_temp = [vec2(0.0, 0.0); 2];
            v_temp[0] = vec2(cosine * self.velocity.x + sine * self.velocity.y, cosine * self.velocity.y - sine * self.velocity.x);
            v_temp[1] = vec2(cosine * other.velocity.x + sine * other.velocity.y, cosine * other.velocity.y - sine * other.velocity.x);

            //Using 1D conservation of momentum to calculate the final velocity along the x axis
            let mut v_final = [vec2(0.0, 0.0); 2];
            v_final[0].x = ((self.mass - other.mass) * v_temp[0].x + 2.0 * other.mass * v_temp[1].x) / (self.mass + other.mass);
            v_final[0].y = v_temp[0].y;

            v_final[1].x = ((other.mass - self.mass) * v_temp[1].x + 2.0 * self.mass * v_temp[0].x) / (self.mass + other.mass);
            v_final[1].y = v_temp[1].y;

            //To avoid clumping
            b_temp[0].x += v_final[0].x;
            b_temp[1].x += v_final[1].x;

            let mut b_final = [vec2(0.0, 0.0); 2];
            b_final[0] = vec2(cosine * b_temp[0].x - sine * b_temp[0].y, cosine * b_temp[0].y + sine * b_temp[0].x);
            b_final[1] = vec2(cosine * b_temp[1].x - sine * b_temp[1].y, cosine * b_temp[1].y + sine * b_temp[1].x);

            // update balls to screen position
            other.position = self.position + b_final[1];
            self.position += b_final[0];

             // update velocities
            self.velocity = vec2(cosine * v_final[0].x - sine * v_final[0].y, cosine * v_final[0].y + sine * v_final[0].x);
            other.velocity = vec2(cosine * v_final[1].x - sine * v_final[1].y, cosine * v_final[1].y + sine * v_final[1].x);
        }
    }
//Function to draw the circle
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(WHITE);
    }
}

struct Model {
    balls: Vec<Ball>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();//display output on new window
    let balls = vec![Ball::new(100.0, 400.0, 20.0), Ball::new(700.0, 400.0, 80.0)];//setting the position and radius of the 2 balls
    Model { balls }

}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    //Iterating over the two balls to update their position and check for boundary collison 
    for ball in model.balls.iter_mut() {
        ball.update();
        ball.check_boundary_collision(&win);
    }

    let (ball1, ball2) = model.balls.split_at_mut(1);
   
    ball1[0].check_collision(&mut ball2[0]);//function to perform collison when the balls collide
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for ball in &model.balls {
        ball.display(&draw);
    }

    //Draw the frame
    draw.to_frame(app, &frame).unwrap();
}


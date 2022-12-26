mod vector;

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use vector::Vector;


fn main() {
    //We need this window object to create a window.
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0)
    };

    //Run the loop.
    window.run_loop(win);
}

//This is the window handler.
//It handles the window events and have the objects that we want to draw and the logic.
struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D){
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p.draw(graphics);
      
        self.p2.update();
        self.p2.draw(graphics);

        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector, //This vector is the position of the pendulum.
    position: Vector, //This vector is the position of the ball.
    angle: f32,//This is the angle of the pendulum.
    angular_velocity: f32,
    angular_acceleration: f32,
    r: f32, //The length of the pendulum
    //m: f32, //The mass of the ball.
    g: f32, //The gravity.
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum { 
            // We need to set the origin of the pendulum
            origin: Vector::new(x, y), 
            //We'll set the position when we update the pendulum,
            //For now we'll set it to a default value.
            position: Vector::new(0.0, 0.0),
            //We'll set the angle to 1.0 radian.
            angle: 1.0,
            //The pendulum is not moving in the beginning
            angular_velocity: 0.0,
            //The pendulum is not accelerating in the beginning
            angular_acceleration: 0.0, 
            r,
            //The mass of the ball is 1.0 for this exemple
            //m: 1.0, 
            //The gravity id 1.5 for this example, but play with it
            g: 1.5, 
        }
    }
    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        //The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocity += self.angular_acceleration;
        //The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;
        //The position is the polar coordinates translated to Cartesian coordinates.
        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());
        //The final position of the ball is the canvas is the origin of the
        //pendulum plus the postion vector.
        self.position.add(&self.origin);

    }
    fn draw(&self, graphics: &mut Graphics2D) { 
        //We need to draw the line of the pendulum first.
        //It takes the start and end position of the line, the width of the line and the color.
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle(
            (self.position.x, self.position.y),
            30.0,
            Color::RED);
    }
}